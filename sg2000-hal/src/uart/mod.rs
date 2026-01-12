use core::{
    cell::RefCell,
    fmt,
    future::poll_fn,
    marker::PhantomData,
    task::{Poll, Waker},
};

pub mod config;

use critical_section::Mutex;
use embedded_io::{ErrorType, ReadReady};
use embedded_io::{Read as BlockingRead, Write as BlockingWrite};
use embedded_io_async::{Read as AsyncRead, Write as AsyncWrite};
use sg2000_pac::{
    interrupt::ExternalInterrupt,
    uart0::{self, iir::IntStatus},
};

use crate::{
    Async, Blocking, DriverMode,
    clock::ClockSource,
    irq::{enable_irq, set_handler},
    peripherals::{Uart0, Uart1, Uart2, Uart3, Uart4},
    private::Sealed,
    uart::config::ConfigError,
};

pub use crate::uart::config::Config;

pub struct Uart<'a, Dm: DriverMode> {
    uart: AnyUart<'a>,
    phantom: PhantomData<Dm>,
    add_cr: bool,
}

const BUFFER_SIZE: usize = 128;

static UART_TX_DATA: Mutex<RefCell<UartTxData>> = Mutex::new(RefCell::new(UartTxData::default()));
static UART_RX_DATA: Mutex<RefCell<UartRxData>> = Mutex::new(RefCell::new(UartRxData::default()));

#[derive(Debug)]
struct UartTxData {
    rd_ptr: usize,
    wt_ptr: usize,
    buffer: [u8; BUFFER_SIZE],
    waker: Option<Waker>,
}

#[derive(Debug)]
struct UartRxData {
    rd_ptr: usize,
    wt_ptr: usize,
    buffer: [u8; BUFFER_SIZE],
    waker: Option<Waker>,
}

impl UartTxData {
    const fn default() -> Self {
        Self {
            rd_ptr: 0,
            wt_ptr: 0,
            buffer: [0; BUFFER_SIZE],
            waker: None,
        }
    }
}

impl UartRxData {
    const fn default() -> Self {
        Self {
            rd_ptr: 0,
            wt_ptr: 0,
            buffer: [0; BUFFER_SIZE],
            waker: None,
        }
    }
}

pub trait Instance: Sealed + Sized {
    fn degrade<'a>(self) -> AnyUart<'a>
    where
        Self: 'a;
}

impl Instance for Uart0<'_> {
    fn degrade<'a>(self) -> AnyUart<'a>
    where
        Self: 'a,
    {
        AnyUart {
            inner: Uarts::Uart0(self),
        }
    }
}
impl Sealed for Uart0<'_> {}
impl Instance for Uart1<'_> {
    fn degrade<'a>(self) -> AnyUart<'a>
    where
        Self: 'a,
    {
        AnyUart {
            inner: Uarts::Uart1(self),
        }
    }
}
impl Sealed for Uart1<'_> {}
impl Instance for Uart2<'_> {
    fn degrade<'a>(self) -> AnyUart<'a>
    where
        Self: 'a,
    {
        AnyUart {
            inner: Uarts::Uart2(self),
        }
    }
}
impl Sealed for Uart2<'_> {}
impl Instance for Uart3<'_> {
    fn degrade<'a>(self) -> AnyUart<'a>
    where
        Self: 'a,
    {
        AnyUart {
            inner: Uarts::Uart3(self),
        }
    }
}
impl Sealed for Uart3<'_> {}
impl Instance for Uart4<'_> {
    fn degrade<'a>(self) -> AnyUart<'a>
    where
        Self: 'a,
    {
        AnyUart {
            inner: Uarts::Uart4(self),
        }
    }
}
impl Sealed for Uart4<'_> {}

// #[derive()]
pub struct AnyUart<'a> {
    inner: Uarts<'a>,
}

// impl core::ops::

enum Uarts<'a> {
    Uart0(Uart0<'a>),
    Uart1(Uart1<'a>),
    Uart2(Uart2<'a>),
    Uart3(Uart3<'a>),
    Uart4(Uart4<'a>),
}

impl<'a> core::ops::Deref for AnyUart<'a> {
    type Target = uart0::RegisterBlock;

    fn deref(&self) -> &Self::Target {
        match &self.inner {
            Uarts::Uart0(uart0) => uart0.inner.deref(),
            Uarts::Uart1(uart1) => uart1.inner.deref(),
            Uarts::Uart2(uart2) => uart2.inner.deref(),
            Uarts::Uart3(uart3) => uart3.inner.deref(),
            Uarts::Uart4(uart4) => uart4.inner.deref(),
        }
    }
}

#[derive(Debug)]
pub enum UartError {
    Config(ConfigError),
    BufferOverrun,
    Framing,
    Parity,
    RxFifoOverrun,
    Other,
}

impl core::fmt::Display for UartError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Config(config_error) => write!(f, "{config_error}"),
            Self::BufferOverrun => write!(f, "UART software buffer overrun"),
            Self::Framing => write!(f, "Uart framing error"),
            Self::Parity => write!(f, "UART parity error"),
            Self::RxFifoOverrun => write!(f, "UART hardware RX FIFO overrun"),
            Self::Other => write!(f, "Other UART error"),
        }
    }
}

impl core::error::Error for UartError {}

impl embedded_io::Error for UartError {
    fn kind(&self) -> embedded_io::ErrorKind {
        match self {
            Self::Config(config_error) => config_error.kind(),
            Self::Parity | Self::Framing => embedded_io::ErrorKind::InvalidData,
            Self::RxFifoOverrun | Self::Other | Self::BufferOverrun => {
                embedded_io::ErrorKind::Other
            }
        }
    }
}

impl From<ConfigError> for UartError {
    fn from(value: ConfigError) -> Self {
        Self::Config(value)
    }
}

impl<'a, Dm: DriverMode> ErrorType for Uart<'a, Dm> {
    type Error = UartError;
}

impl<'a, Dm: DriverMode> Uart<'a, Dm> {
    pub fn read_divisor(&self) -> u16 {
        self.uart.lcr().modify(|_, w| w.div_latch().set_bit());

        let low = self.uart.dll().read().bits();
        let high = self.uart.dlh().read().bits();

        self.uart.lcr().modify(|_, w| w.div_latch().clear_bit());

        (high as u16) << 8 | (low as u16)
    }
}

impl<'a> Uart<'a, Blocking> {
    pub fn new<I: Instance + 'a>(uart: I, config: Config) -> Result<Self, UartError> {
        let uart = uart.degrade();
        uart.ier().write(|w| unsafe { w.bits(0x00) });

        config.validate()?;

        let divisor = (config.clock().hz().as_hz() as u32 + 8 * config.baud_rate())
            / (16 * config.baud_rate());
        let divisor_low = (divisor & 0xFF) as u8;
        let divisor_high = ((divisor >> 8) & 0xFF) as u8;

        uart.lcr().modify(|_, w| w.div_latch().set_bit());

        uart.dll().write(|w| unsafe { w.dll().bits(divisor_low) });
        uart.dlh().write(|w| unsafe { w.dlh().bits(divisor_high) });

        uart.lcr().modify(|_, w| {
            w.div_latch().clear_bit();
            match config.data_len() {
                config::DataLen::Five => w.data_len().five(),
                config::DataLen::Six => w.data_len().six(),
                config::DataLen::Seven => w.data_len().seven(),
                config::DataLen::Eight => w.data_len().eight(),
            };
            match config.stop_bits() {
                config::StopBits::One => w.stop_bits().clear_bit(),
                config::StopBits::OnePFive | config::StopBits::Two => w.stop_bits().set_bit(),
            };
            match config.parity() {
                config::ParityMode::None => w.parity_en().clear_bit(),
                config::ParityMode::Odd => w.parity_en().set_bit(),
                config::ParityMode::Even => w.parity_en().set_bit().par_even().set_bit(),
                config::ParityMode::Mark => w.parity_en().set_bit().stick_par().set_bit(),
                config::ParityMode::Space => w
                    .parity_en()
                    .set_bit()
                    .par_even()
                    .set_bit()
                    .stick_par()
                    .set_bit(),
            }
        });

        uart.fcr().write(|w| {
            w.fifo_en()
                .set_bit()
                .rxfifo_rst()
                .set_bit()
                .txfifo_rst()
                .set_bit()
                .tx_empty_trig()
                .quarter()
                .rx_trig()
                .char1()
        });

        Ok(Self {
            uart,
            phantom: PhantomData,
            add_cr: config.add_cr(),
        })
    }

    pub fn write_str(&mut self, s: &str) {
        if self.add_cr {
            let mut s = s;
            while let Some(next_newline) = s.find('\n') {
                let _ = self.write_all(&s.as_bytes()[..next_newline]);
                let _ = self.write_all(b"\r\n");
                s = &s[next_newline + 1..];
            }
            let _ = self.write_all(s.as_bytes());
        } else {
            let _ = self.write_all(s.as_bytes());
        }
    }

    pub fn write_fmt(&mut self, args: core::fmt::Arguments<'_>) {
        use core::fmt::Write;

        let mut buf = heapless::String::<128>::new();

        let _ = buf.write_fmt(args);

        self.write_str(&buf);
    }

    pub fn into_async(self) -> Uart<'a, Async> {
        let plic = unsafe { sg2000_pac::Plic::steal() };
        set_handler(ExternalInterrupt::UART1, uart1_handler);
        enable_irq(&plic, ExternalInterrupt::UART1);
        self.uart.ier().write(|w| w.rx_data().set_bit());
        Uart {
            uart: self.uart,
            phantom: PhantomData,
            add_cr: self.add_cr,
        }
    }

    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize, UartError> {
        BlockingRead::read(self, buf)
    }
}

impl<'a> Uart<'a, Async> {
    pub async fn write_str(&mut self, s: &str) {
        if self.add_cr {
            let mut s = s;
            while let Some(next_newline) = s.find('\n') {
                let _ = self.write_all(&s.as_bytes()[..next_newline]).await;
                let _ = self.write_all(b"\r\n").await;
                s = &s[next_newline + 1..];
            }
            let _ = self.write_all(s.as_bytes()).await;
        } else {
            let _ = self.write_all(s.as_bytes()).await;
        }
    }

    pub fn flush(&self) {
        loop {
            let empty = critical_section::with(|cs| {
                let data = UART_TX_DATA.borrow_ref(cs);
                data.rd_ptr == data.wt_ptr
            });
            if empty {
                break;
            }
        }

        while !self.uart.usr().read().tx_fifo_empty().bit_is_set() {}
    }

    pub fn write_fmt(
        &mut self,
        args: core::fmt::Arguments<'_>,
    ) -> impl core::future::Future<Output = ()> + '_ {
        use core::fmt::Write;

        let mut buf = heapless::String::<128>::new();

        let _ = buf.write_fmt(args);

        async move {
            self.write_str(&buf).await;
        }
    }
}

impl<'a> BlockingWrite for Uart<'a, Blocking> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        let uart = &self.uart;
        let mut count = 0;
        if self.add_cr {
            for &b in buf {
                if b == b'\n' {
                    while !uart.usr().read().tx_fifo_not_full().bit_is_set() {}
                    uart.rbr_thr().write(|w| unsafe { w.rbr_thr().bits(b'\r') });
                }
                while !uart.usr().read().tx_fifo_not_full().bit_is_set() {}
                uart.rbr_thr().write(|w| unsafe { w.rbr_thr().bits(b) });
                count += 1;
            }
        } else {
            for &b in buf {
                while !uart.usr().read().tx_fifo_not_full().bit_is_set() {}
                uart.rbr_thr().write(|w| unsafe { w.rbr_thr().bits(b) });
                count += 1;
            }
        }

        Ok(count)
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        while !self.uart.usr().read().tx_fifo_empty().bit_is_set() {}
        Ok(())
    }
}

impl<'a> BlockingRead for Uart<'a, Blocking> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        let uart = &self.uart;
        while !uart.usr().read().rx_fifo_not_empty().bit_is_set() {}
        for (i, byte) in buf.iter_mut().enumerate() {
            *byte = (uart.rbr_thr().read().bits() & 0xFF) as u8;
            if uart.usr().read().rx_fifo_not_empty().bit_is_clear() {
                return Ok(i + 1);
            }
        }
        Ok(buf.len())
    }
}

impl<'a> ReadReady for Uart<'a, Blocking> {
    fn read_ready(&mut self) -> Result<bool, Self::Error> {
        Ok(self.uart.usr().read().rx_fifo_not_empty().bit_is_set())
    }
}

impl<'a> AsyncRead for Uart<'a, Async> {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        let uart = &self.uart;
        poll_fn(|cx| {
            critical_section::with(|cs| {
                let mut uart_rx_data = UART_RX_DATA.borrow_ref_mut(cs);
                if uart_rx_data.wt_ptr == uart_rx_data.rd_ptr {
                    uart_rx_data.waker = Some(cx.waker().clone());
                    uart.ier().modify(|_, w| w.rx_data().set_bit());
                    return Poll::Pending;
                }
                let mut count = 0;
                let mut it = buf
                    .iter_mut()
                    .zip(uart_rx_data.rd_ptr..=uart_rx_data.wt_ptr)
                    .enumerate()
                    .peekable();
                while let Some((i, (byte, idx))) = it.next() {
                    if it.peek().is_none() {
                        count = i;
                        uart_rx_data.rd_ptr = idx;
                    }
                    *byte = uart_rx_data.buffer[idx % BUFFER_SIZE];
                }
                Poll::Ready(Ok(count))
            })
        })
        .await
    }
}

impl<'a> AsyncWrite for Uart<'a, Async> {
    async fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        let uart = &self.uart;
        poll_fn(|cx| {
            critical_section::with(|cs| {
                let mut uart_tx_data = UART_TX_DATA.borrow_ref_mut(cs);

                if uart_tx_data.wt_ptr - uart_tx_data.rd_ptr >= BUFFER_SIZE {
                    uart_tx_data.waker = Some(cx.waker().clone());
                    uart.ier().modify(|_, w| w.tx_empty().set_bit());
                    return Poll::Pending;
                }
                let mut count = 0;
                for &byte in buf {
                    if uart_tx_data.wt_ptr - uart_tx_data.rd_ptr >= BUFFER_SIZE {
                        uart_tx_data.waker = Some(cx.waker().clone());
                        break;
                    }
                    let wt_idx = uart_tx_data.wt_ptr;
                    uart_tx_data.buffer[wt_idx % BUFFER_SIZE] = byte;
                    uart_tx_data.wt_ptr = wt_idx + 1;
                    count += 1;
                }

                uart.ier().modify(|_, w| w.tx_empty().set_bit());

                Poll::Ready(Ok(count))
            })
        })
        .await
    }

    async fn flush(&mut self) -> Result<(), Self::Error> {
        let uart = &self.uart;

        poll_fn(|cx| {
            critical_section::with(|cs| {
                let mut uart_data = UART_TX_DATA.borrow_ref_mut(cs);

                if uart_data.rd_ptr != uart_data.wt_ptr {
                    uart_data.waker = Some(cx.waker().clone());
                    uart.ier().write(|w| w.tx_empty().set_bit());
                    return Poll::Pending;
                }

                if uart.usr().read().tx_fifo_empty().bit_is_set() {
                    Poll::Ready(Ok(()))
                } else {
                    uart_data.waker = Some(cx.waker().clone());
                    uart.ier().write(|w| w.tx_empty().set_bit());
                    Poll::Pending
                }
            })
        })
        .await
    }
}

fn uart1_handler() {
    let uart = unsafe { sg2000_pac::Uart1::steal() };
    let iir = uart.iir().read();

    match iir.int_id().variant() {
        Some(IntStatus::Thrempty) => {
            critical_section::with(|cs| {
                let mut uart_tx_data = UART_TX_DATA.borrow_ref_mut(cs);
                // TODO: read number of bytes in FIFO from TFL rather than checking each time
                while uart_tx_data.rd_ptr < uart_tx_data.wt_ptr
                    && uart.usr().read().tx_fifo_not_full().bit_is_set()
                {
                    uart.rbr_thr().write(|w| unsafe {
                        w.rbr_thr()
                            .bits(uart_tx_data.buffer[uart_tx_data.rd_ptr % BUFFER_SIZE])
                    });
                    uart_tx_data.rd_ptr += 1;
                }

                if uart_tx_data.rd_ptr == uart_tx_data.wt_ptr {
                    uart.ier().modify(|_, w| w.tx_empty().clear_bit());
                }

                if let Some(waker) = uart_tx_data.waker.take() {
                    waker.wake();
                }
            });
        }
        Some(IntStatus::DataAvailable) => critical_section::with(|cs| {
            let mut uart_rx_data = UART_RX_DATA.borrow_ref_mut(cs);
            let mut wt_ptr = uart_rx_data.wt_ptr;
            while wt_ptr < uart_rx_data.rd_ptr + BUFFER_SIZE
                && uart.usr().read().rx_fifo_not_empty().bit_is_set()
            {
                uart_rx_data.buffer[wt_ptr % BUFFER_SIZE] =
                    (uart.rbr_thr().read().bits() & 0xFF) as u8;
                wt_ptr += 1;
            }
            uart_rx_data.wt_ptr = wt_ptr;

            if uart.usr().read().rx_fifo_not_empty().bit_is_clear() {
                uart.ier().modify(|_, w| w.rx_data().clear_bit());
            }

            if let Some(waker) = uart_rx_data.waker.take() {
                waker.wake();
            }
        }),
        _ => {}
    }
}
