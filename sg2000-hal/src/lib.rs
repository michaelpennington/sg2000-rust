#![no_std]

pub use sg2000_pac as pac;
pub mod resource_table;

#[cfg(feature = "embassy")]
pub mod timer;
