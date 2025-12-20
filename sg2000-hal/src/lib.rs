#![no_std]

pub use sg2000_pac as pac;

#[cfg(feature = "embassy")]
pub mod timer;
