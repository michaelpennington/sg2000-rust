#![no_std]

pub use sg2000_pac as pac;
pub mod irq;
pub mod resource_table;
pub mod uart;

#[cfg(feature = "embassy")]
pub mod timer;

pub struct Config;

impl Default for Config {
    fn default() -> Self {
        Self
    }
}

pub fn init(_config: Config) -> pac::Peripherals {
    let peripherals = pac::Peripherals::take().unwrap();

    unsafe {
        xuantie_riscv::register::mhcr::set_ie();

        riscv::interrupt::enable();
        riscv::register::mie::set_mext();
        riscv::register::mie::set_mtimer();
    }
    peripherals
        .plic
        .priority_threshold(0)
        .write(|w| unsafe { w.bits(0) });

    peripherals
}
