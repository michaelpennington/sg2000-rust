use core::fmt;

use sg2000_pac::uart0;

pub struct Uart<'a> {
    pub uart: &'a uart0::RegisterBlock,
    add_cr: bool,
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
        });
    }

    fn putc_blocking(&self, value: u8) {
        while !self.uart.lsr().read().tx_empty().bit() {}
        unsafe { self.uart.rbr_thr().write(|w| w.rbr_thr().bits(value)) };
    }

    pub fn read_divisor(&self) -> u16 {
        self.uart.lcr().modify(|_, w| w.div_latch().set_bit());

        let low = self.uart.dll().read().bits();
        let high = self.uart.dlh().read().bits();

        self.uart.lcr().modify(|_, w| w.div_latch().clear_bit());

        (high as u16) << 8 | (low as u16)
    }
}

impl<'a> fmt::Write for Uart<'a> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for b in s.bytes() {
            if b == b'\n' && self.add_cr {
                self.putc_blocking(b'\r');
            }
            self.putc_blocking(b);
        }
        Ok(())
    }
}
