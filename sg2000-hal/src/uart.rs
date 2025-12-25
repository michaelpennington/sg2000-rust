use core::fmt;

use sg2000_pac::uart0;

pub struct UartWriter<'a> {
    uart: &'a uart0::RegisterBlock,
    add_cr: bool,
}

impl<'a> UartWriter<'a> {
    pub fn new(uart: &'a uart0::RegisterBlock, add_cr: bool) -> Self {
        UartWriter { uart, add_cr }
    }

    fn putc_blocking(&self, value: u8) {
        while !self.uart.lsr().read().tx_empty().bit() {}
        unsafe { self.uart.rbr_thr().write(|w| w.rbr_thr().bits(value)) };
    }
}

impl<'a> fmt::Write for UartWriter<'a> {
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
