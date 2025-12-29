#![no_std]

use core::marker::PhantomData;

pub use sg2000_pac as pac;
pub mod clock;
pub mod irq;
pub mod peripherals;
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

pub struct Blocking;
impl private::Sealed for Blocking {}
impl DriverMode for Blocking {}

pub struct Async(PhantomData<*const ()>);
impl private::Sealed for Async {}
impl DriverMode for Async {}

pub trait DriverMode: private::Sealed {}

pub(crate) mod private {
    pub trait Sealed {}
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
