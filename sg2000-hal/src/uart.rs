use core::{cell::RefCell, fmt};

use critical_section::Mutex;
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

#[derive(Clone, Copy, Debug)]
struct UartData {
    rd_ptr: usize,
    wt_ptr: usize,
    buffer: [u8; BUFFER_SIZE],
}

impl UartData {
    const fn default() -> Self {
        Self {
            rd_ptr: 0,
            wt_ptr: 0,
            buffer: [0; BUFFER_SIZE],
        }
    }
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

    #[allow(dead_code)]
    fn putc_blocking(&self, value: u8) {
        while !self.uart.lsr().read().tx_empty().bit() {}
        unsafe { self.uart.rbr_thr().write(|w| w.rbr_thr().bits(value)) };
    }

    fn putc_async(&self, value: u8) {
        critical_section::with(|cs| {
            let mut uart_data = UART_DATA.borrow_ref_mut(cs);

            if uart_data.rd_ptr == uart_data.wt_ptr
                && self.uart.usr().read().tx_fifo_not_full().bit_is_set()
            {
                self.uart
                    .rbr_thr()
                    .write(|w| unsafe { w.rbr_thr().bits(value) });
            } else {
                let wt_ptr = uart_data.wt_ptr;
                uart_data.buffer[wt_ptr % BUFFER_SIZE] = value;
                uart_data.wt_ptr = wt_ptr + 1;

                self.uart.ier().write(|w| w.tx_empty().set_bit());
            }
        });
    }

    pub fn read_divisor(&self) -> u16 {
        self.uart.lcr().modify(|_, w| w.div_latch().set_bit());

        let low = self.uart.dll().read().bits();
        let high = self.uart.dlh().read().bits();

        self.uart.lcr().modify(|_, w| w.div_latch().clear_bit());

        (high as u16) << 8 | (low as u16)
    }
}

fn uart1_handler() {
    let uart = unsafe { sg2000_pac::Uart1::steal() };
    let iir = uart.iir().read();

    if iir.int_id().variant() == Some(IntStatus::Thrempty) {
        critical_section::with(|cs| {
            let mut uart_data = UART_DATA.borrow_ref_mut(cs);
            if uart_data.rd_ptr == uart_data.wt_ptr {
                uart.ier().modify(|_, w| w.tx_empty().clear_bit());
                return;
            }
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
                uart.ier().modify(|_, w| w.tx_empty().clear_bit());
            }
        });
    }
}

impl<'a> fmt::Write for Uart<'a> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for b in s.bytes() {
            if b == b'\n' && self.add_cr {
                self.putc_async(b'\r');
            }
            self.putc_async(b);
        }
        Ok(())
    }
}
