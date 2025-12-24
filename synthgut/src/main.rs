#![feature(impl_trait_in_assoc_type)]
#![no_std]
#![no_main]

use core::{fmt::Write, panic::PanicInfo};

use embassy_executor::Spawner;
use embassy_time::Timer;
use riscv::asm::nop;
use sg2000_hal::{
    Config, init,
    irq::{enable_irq, set_handler},
    pac::{Uart0, interrupt::ExternalInterrupt},
    uart::UartWriter,
};

const LED_MASK: u32 = 1 << 29;
const INPUT_MASK: u32 = 1 << 15;
const BUILD_TIME: &str = include!(concat!(env!("OUT_DIR"), "/timestamp.rs"));

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
    let peripherals = init(Config);

    let gpio0 = peripherals.gpio0;
    let gpio1 = peripherals.gpio1;
    let plic = peripherals.plic;
    let uart0 = peripherals.uart0;

    let mut uart_writer = UartWriter::new(&uart0, true);
    writeln!(uart_writer, "Synthgut 0.1.0, built {BUILD_TIME}").unwrap();

    unsafe {
        gpio0.ddr().modify(|r, w| w.bits(r.bits() | LED_MASK));

        gpio1
            .int_polarity()
            .modify(|r, w| w.bits(r.bits() | INPUT_MASK));
        gpio1.inten().modify(|r, w| w.bits(r.bits() | INPUT_MASK));
        set_handler(ExternalInterrupt::GPIO1, gpio1_handler);
        enable_irq(&plic, ExternalInterrupt::GPIO1);
    }

    spawner.spawn(print_hellos(uart0)).unwrap();

    loop {
        unsafe { gpio0.dr().modify(|r, w| w.bits(r.bits() ^ LED_MASK)) };
        Timer::after_secs(1).await;
    }
}

#[embassy_executor::task]
async fn print_hellos(uart0: Uart0) {
    let mut uart_writer = UartWriter::new(&uart0, true);

    loop {
        for i in 0..10 {
            writeln!(&mut uart_writer, "HELLO {i}").unwrap();
            Timer::after_secs(2).await
        }
    }
}

fn gpio1_handler() {
    let peripherals = unsafe { sg2000_hal::pac::Peripherals::steal() };
    let gpio0 = peripherals.gpio0;

    unsafe { gpio0.dr().modify(|r, w| w.bits(r.bits() ^ LED_MASK)) };

    for _ in 0..10000000 {
        nop();
        nop();
    }
}
