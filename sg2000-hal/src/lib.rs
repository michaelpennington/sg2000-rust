#![no_std]

pub use sg2000_pac as pac;
pub mod clock;
pub mod irq;
pub mod resource_table;
pub mod uart;
pub mod peripherals;

#[cfg(feature = "embassy")]
pub mod timer;

pub struct Config;

impl Default for Config {
    fn default() -> Self {
        Self
    }
}

pub fn init(_config: Config) -> peripherals::Peripherals {
    let peripherals = peripherals::Peripherals::take().unwrap();

    unsafe {
        xuantie_riscv::register::mhcr::set_ie();

        riscv::interrupt::enable();
        riscv::register::mie::set_mext();
        riscv::register::mie::set_mtimer();
    }
    peripherals
        .plic
        .plic_prio(0)
        .write(|w| unsafe { w.bits(0) });

    peripherals
}
