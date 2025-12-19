#![feature(impl_trait_in_assoc_type)]
#![no_std]
#![no_main]

mod irq;
mod uart;

use core::{fmt::Write, panic::PanicInfo};

use crate::uart::UartWriter;
use embassy_executor::Spawner;
use sg2000_pac::Uart0;
use xuantie_riscv::register::mhcr;

const LED_MASK: u32 = 1 << 29;
const INPUT_MASK: u32 = 1 << 15;
// const INPUT_IRQ_NO: usize = sg2000_pac::interrupt::try_cause

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    riscv::interrupt::disable();
    let uart0 = unsafe { Uart0::steal() };
    let mut uart_writer = UartWriter::new(&uart0, true);
    let _ = writeln!(uart_writer, "{}", info);
    loop {}
}

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    unsafe { mhcr::set_ie() };

    let peripherals = sg2000_pac::Peripherals::take().unwrap();
    let gpio0 = peripherals.gpio0;
    let gpio1 = peripherals.gpio1;
    let plic = peripherals.plic;

    let uart0 = peripherals.uart0;

    unsafe {
        riscv::interrupt::enable();
        riscv::register::mie::set_mext();
        riscv::register::mie::set_mtimer();
        plic.priority_threshold(0).write(|w| w.bits(0));

        gpio0.ddr().modify(|r, w| w.bits(r.bits() | LED_MASK));

        gpio1
            .int_polarity()
            .modify(|r, w| w.bits(r.bits() | INPUT_MASK));
        gpio1.inten().modify(|r, w| w.bits(r.bits() | INPUT_MASK));
        // irq::enable_irq(&plic, INPUT_IRQ_NO);
    }

    loop {
        riscv::asm::wfi();
    }
}
