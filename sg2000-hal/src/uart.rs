use core::{
    cell::RefCell,
    fmt,
    future::poll_fn,
    task::{Poll, Waker},
};

use critical_section::Mutex;
use embedded_io::ErrorType;
use embedded_io_async::Write;
use sg2000_pac::{
    interrupt::ExternalInterrupt,
    uart0::{self, iir::IntStatus},
};

use crate::irq::{enable_irq, set_handler};

pub struct Uart<'a> {
    uart: &'a uart0::RegisterBlock,
    add_cr: bool,
}

const BUFFER_SIZE: usize = 128;

static UART_DATA: Mutex<RefCell<UartData>> = Mutex::new(RefCell::new(UartData::default()));

#[derive(Debug)]
struct UartData {
    rd_ptr: usize,
    wt_ptr: usize,
    buffer: [u8; BUFFER_SIZE],
    waker: Option<Waker>,
}

impl UartData {
    const fn default() -> Self {
        Self {
            rd_ptr: 0,
            wt_ptr: 0,
            buffer: [0; BUFFER_SIZE],
            waker: None,
        }
    }
}

#[derive(Debug)]
pub struct UartError;

impl core::fmt::Display for UartError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Uart error! Weed eater")
    }
}

impl core::error::Error for UartError {}

impl embedded_io_async::Error for UartError {
    fn kind(&self) -> embedded_io::ErrorKind {
        embedded_io_async::ErrorKind::Other
    }
}

impl<'a> ErrorType for Uart<'a> {
    type Error = UartError;
}

impl<'a> Uart<'a> {
    pub fn new(uart: &'a uart0::RegisterBlock, add_cr: bool) -> Self {
        Uart { uart, add_cr }
    }

    pub fn init(&self, source_clock_hz: u32, baud_rate: u32) {
        self.uart.ier().write(|w| unsafe { w.bits(0x00) });

        let divisor = (source_clock_hz + 8 * baud_rate) / (16 * baud_rate);
        let divisor_low = (divisor & 0xFF) as u8;
        let divisor_high = ((divisor >> 8) & 0xFF) as u8;

        self.uart.lcr().modify(|_, w| w.div_latch().set_bit());

        self.uart
            .dll()
            .write(|w| unsafe { w.dll().bits(divisor_low) });
        self.uart
            .dlh()
            .write(|w| unsafe { w.dlh().bits(divisor_high) });

        self.uart.lcr().write(|w| {
            w.div_latch()
                .clear_bit()
                .data_len()
                .eight()
                .stop_bits()
                .one()
                .parity_en()
                .clear_bit()
        });

        self.uart.fcr().write(|w| {
            w.fifo_en()
                .set_bit()
                .rxfifo_rst()
                .set_bit()
                .txfifo_rst()
                .set_bit()
                .tx_empty_trig()
                .quarter()
        });

        // self.uart.ier().write(|w| w.tx_empty().set_bit());

        let plic = unsafe { sg2000_pac::Plic::steal() };
        set_handler(ExternalInterrupt::UART1, uart1_handler);
        enable_irq(&plic, ExternalInterrupt::UART1);
    }

    fn putc_blocking(&self, value: u8) {
        while !self.uart.lsr().read().tx_empty().bit() {}
        unsafe { self.uart.rbr_thr().write(|w| w.rbr_thr().bits(value)) };
    }

    pub fn write_str_blocking(&self, s: &str) {
        if self.add_cr {
            for b in s.bytes() {
                if b == b'\n' {
                    self.putc_blocking(b'\r');
                }
                self.putc_blocking(b);
            }
        } else {
            for b in s.bytes() {
                self.putc_blocking(b);
            }
        }
    }

    pub async fn write_str(&mut self, s: &str) {
        if self.add_cr {
            for b in s.bytes() {
                if b == b'\n' {
                    let _ = self.write_all(b"\r\n").await;
                } else {
                    let _ = self.write_all(&[b]).await;
                }
            }
        } else {
            let _ = self.write_all(s.as_bytes()).await;
        }
    }

    pub fn flush(&self) {
        // 1. Wait for software buffer to drain completely
        loop {
            let empty = critical_section::with(|cs| {
                let data = UART_DATA.borrow_ref(cs);
                data.rd_ptr == data.wt_ptr
            });
            if empty {
                break;
            }
            // Optional: Insert a wfi() or yield here if you want to save power,
            // but for simple blocking flush, a tight loop is fine.
        }

        while !self.uart.usr().read().tx_fifo_empty().bit_is_set() {}
    }

    pub fn read_divisor(&self) -> u16 {
        self.uart.lcr().modify(|_, w| w.div_latch().set_bit());

        let low = self.uart.dll().read().bits();
        let high = self.uart.dlh().read().bits();

        self.uart.lcr().modify(|_, w| w.div_latch().clear_bit());

        (high as u16) << 8 | (low as u16)
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

impl<'a> Write for Uart<'a> {
    async fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        let uart = self.uart;
        poll_fn(|cx| {
            critical_section::with(|cs| {
                let mut uart_data = UART_DATA.borrow_ref_mut(cs);

                if uart_data.wt_ptr - uart_data.rd_ptr >= BUFFER_SIZE {
                    uart_data.waker = Some(cx.waker().clone());
                    uart.ier().write(|w| w.tx_empty().set_bit());
                    return Poll::Pending;
                }
                let mut count = 0;
                for &byte in buf {
                    if uart_data.wt_ptr - uart_data.rd_ptr >= BUFFER_SIZE {
                        uart_data.waker = Some(cx.waker().clone());
                        break;
                    }
                    let wt_idx = uart_data.wt_ptr;
                    uart_data.buffer[wt_idx % BUFFER_SIZE] = byte;
                    uart_data.wt_ptr = wt_idx + 1;
                    count += 1;
                }

                uart.ier().write(|w| w.tx_empty().set_bit());

                Poll::Ready(Ok(count))
            })
        })
        .await
    }

    async fn flush(&mut self) -> Result<(), Self::Error> {
        let uart = self.uart;

        poll_fn(|cx| {
            critical_section::with(|cs| {
                let mut uart_data = UART_DATA.borrow_ref_mut(cs);

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

    if iir.int_id().variant() == Some(IntStatus::Thrempty) {
        critical_section::with(|cs| {
            let mut uart_data = UART_DATA.borrow_ref_mut(cs);
            // TODO: read number of bytes in FIFO from TFL rather than checking each time
            while uart_data.rd_ptr < uart_data.wt_ptr
                && uart.usr().read().tx_fifo_not_full().bit_is_set()
            {
                uart.rbr_thr().write(|w| unsafe {
                    w.rbr_thr()
                        .bits(uart_data.buffer[uart_data.rd_ptr % BUFFER_SIZE])
                });
                uart_data.rd_ptr += 1;
            }

            if uart_data.rd_ptr == uart_data.wt_ptr {
                uart.ier().reset();
            }

            if let Some(waker) = uart_data.waker.take() {
                waker.wake();
            }
        });
    }
}
