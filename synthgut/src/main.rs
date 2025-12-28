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
    pac::interrupt::ExternalInterrupt,
    peripherals,
    uart::{self, Uart},
};
use static_cell::StaticCell;

const LED_MASK: u32 = 1 << 29;
const INPUT_MASK: u32 = 1 << 15;
const BUILD_TIME: &str = include!(concat!(env!("OUT_DIR"), "/timestamp.rs"));

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    riscv::interrupt::disable();
    // Use the HAL wrapper's steal method
    let uart1 = unsafe { peripherals::Uart1::steal() };
    let uart_writer = Uart::new(&uart1, true);
    let mut buf = heapless::String::<128>::new();
    let _ = write!(buf, "{info}");
    uart_writer.write_str_blocking(&buf);
    loop {}
}

// Uart1 here refers to the wrapper type
static UART1: StaticCell<peripherals::Uart1> = StaticCell::new();
const BANNER: &str = "##############################################################";

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    let peripherals = init(Config);

    let gpio0 = peripherals.gpio0;
    let gpio1 = peripherals.gpio1;
    let plic = peripherals.plic;
    let uart1 = UART1.init(peripherals.uart1);

    unsafe {
        gpio0.ddr().modify(|r, w| w.bits(r.bits() | LED_MASK));

        gpio1
            .int_polarity()
            .modify(|r, w| w.bits(r.bits() | INPUT_MASK));
        gpio1.inten().modify(|r, w| w.bits(r.bits() | INPUT_MASK));
        set_handler(ExternalInterrupt::GPIO1, gpio1_handler);
        // enable_irq takes &pac::Plic. plic is peripherals::Plic, which derefs to pac::Plic.
        enable_irq(&plic, ExternalInterrupt::GPIO1);
    }

    Timer::after_secs(1).await;

    // Uart::new takes &pac::Uart1. uart1 is &mut peripherals::Uart1, which derefs to pac::Uart1.
    let mut uart1p = Uart::new(uart1, true);
    uart1p.init(uart::Config::default()).unwrap();
    writeln!(uart1p, "{BANNER}").await;
    writeln!(uart1p, "# Synthgut 0.1.0, built {BUILD_TIME} #").await;
    writeln!(uart1p, "{BANNER}").await;

    uart1p.flush();

    spawner.spawn(print_hellos(uart1p)).unwrap();
    loop {
        unsafe { gpio0.dr().modify(|r, w| w.bits(r.bits() ^ LED_MASK)) };
        Timer::after_secs(1).await;
    }
}

#[embassy_executor::task]
async fn print_hellos(mut uart: Uart<'static>) {
    loop {
        for i in 0..10 {
            writeln!(uart, "HELLO {i}").await;
            Timer::after_secs(2).await;
        }
    }
}

fn gpio1_handler() {
    // Also update interrupt handler to use HAL peripherals steal
    let peripherals = unsafe { peripherals::Peripherals::steal() };
    let gpio0 = peripherals.gpio0;

    unsafe { gpio0.dr().modify(|r, w| w.bits(r.bits() ^ LED_MASK)) };

    for _ in 0..10000000 {
        nop();
        nop();
    }
}
