#![feature(impl_trait_in_assoc_type)]
#![no_std]
#![no_main]

use core::{fmt::Write, panic::PanicInfo};
use embassy_executor::Spawner;
use embassy_time::Timer;
use embedded_io_async::Read;
use log::info;
use sg2000_hal::{
    Config,
    logger::RpmsgLogger,
    peripherals,
    uart::{self, Uart},
};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    riscv::interrupt::disable();
    // Use the HAL wrapper's steal method
    let uart1 = unsafe { peripherals::Uart1::steal() };
    let mut uart_writer = Uart::new(uart1, uart::Config::default()).unwrap();
    let mut buf = heapless::String::<512>::new();
    let _ = write!(buf, "{info}");
    uart_writer.write_str(&buf);
    loop {}
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let peripherals = sg2000_hal::init(Config);

    let mut logger = RpmsgLogger::init();
    info!("Logger initialized!");

    logger.flush_log();
    let mut uart = Uart::new(
        peripherals.uart1,
        uart::Config::default().with_baud_rate(31250),
    )
    .unwrap()
    .into_async();
    let mut buf = [0u8; 128];

    // spawner.spawn(print_hellos()).unwrap();
    loop {
        logger.flush_log();
        if let Ok(num_bytes) = uart.read(&mut buf).await {
            for byte in &buf[..num_bytes] {
                if *byte >= 0x80 {
                    info!("\nMIDI Status: {byte:#04X}");
                } else {
                    info!(" Data: {byte:#04X}");
                }
            }
        }
        Timer::after_millis(1).await;
    }
}

#[embassy_executor::task]
async fn print_hellos() {
    loop {
        for i in 0..10 {
            info!("Hello {i}!");
            Timer::after_secs(1).await;
        }
    }
}
