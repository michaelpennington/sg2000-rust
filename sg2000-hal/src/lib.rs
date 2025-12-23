#![no_std]

pub use sg2000_pac as pac;
pub mod resource_table;
pub mod uart;

#[cfg(feature = "embassy")]
pub mod timer;
