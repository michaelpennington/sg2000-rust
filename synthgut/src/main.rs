#![feature(impl_trait_in_assoc_type)]
#![no_std]
#![no_main]

mod logger;

use core::{fmt::Write, panic::PanicInfo};
use embassy_executor::Spawner;
use embassy_time::Timer;
use log::info;
use riscv::asm::nop;
use sg2000_hal::{
    Config,
    irq::{enable_irq, set_handler},
    mailbox::{Channel, Cpu, Mailbox},
    pac::interrupt::ExternalInterrupt,
    peripherals::{self, Mailboxes},
    rpmsg::RpmsgDevice,
    uart::{self, Uart},
};

const LED_MASK: u32 = 1 << 29;
const INPUT_MASK: u32 = 1 << 15;
const BUILD_TIME: &str = include!(concat!(env!("OUT_DIR"), "/timestamp.rs"));

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

// Uart1 here refers to the wrapper type
const BANNER: &str = "##############################################################";

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    let peripherals = sg2000_hal::init(Config);

    let gpio0 = peripherals.gpio0;
    let gpio1 = peripherals.gpio1;
    let plic = peripherals.plic;
    let uart1 = peripherals.uart1;
    let mailbox = peripherals.mailboxes;

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

    logger::init().unwrap();
    info!("Logger initialized!");

    // Uart::new takes &pac::Uart1. uart1 is &mut peripherals::Uart1, which derefs to pac::Uart1.
    let mut uart1p = Uart::new(uart1, uart::Config::default().with_add_cr(true)).unwrap();

    writeln!(uart1p, "{BANNER}");
    writeln!(uart1p, "# Synthgut 0.1.0, built {BUILD_TIME} #");
    writeln!(uart1p, "{BANNER}\n");

    let tx_mailbox = Mailbox::new(mailbox, Channel::Ch1, Cpu::C906_0);
    let rx_mailbox = Mailbox::new(unsafe { Mailboxes::steal() }, Channel::Ch0, Cpu::C906_1);

    let mut rpmsg = unsafe { RpmsgDevice::new(tx_mailbox, rx_mailbox) };
    writeln!(uart1p, "Waiting for Linux Host (DRIVER_OK)...");

    while !rpmsg.is_driver_ok() {
        Timer::after_millis(10).await;
    }

    writeln!(uart1p, "Host online. Announcing service...");
    let src_addr = 0x400;
    match rpmsg.announce("rpmsg-tty", src_addr) {
        Ok(_) => writeln!(uart1p, "service 'rpmsg-tty' announced at {src_addr:#010X}"),
        Err(e) => writeln!(uart1p, "Announcement failed: {e}"),
    }
    Timer::after_millis(500).await;

    info!("Hello from the remote core!");

    spawner.spawn(print_hellos()).unwrap();
    loop {
        rpmsg.poll(|src, dst, data| {
            if let Ok(s) = core::str::from_utf8(data) {
                writeln!(uart1p, "RX [{}->{}]: {}", src, dst, s);
            } else {
                writeln!(uart1p, "RX [{}->{}]: {} bytes binary", src, dst, data.len());
            }
        });
        while let Some(msg) = logger::pop_log() {
            // Note: send() might fail if the ring is full, but for logs we
            // might just have to drop them or retry briefly.
            if rpmsg.send(src_addr, src_addr, msg.as_bytes()).is_err() {
                // If we can't send via RPMsg, maybe fallback to UART or drop
                // We shouldn't log::error! here as it would loop recursively.
                break;
            }
        }
        Timer::after_millis(2).await;
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
