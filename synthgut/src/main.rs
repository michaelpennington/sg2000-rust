#![feature(impl_trait_in_assoc_type)]
#![no_std]
#![no_main]

use core::{fmt::Write, panic::PanicInfo};
use embassy_executor::Spawner;
use embassy_time::Timer;
use riscv::asm::nop;
use sg2000_hal::{
    Config,
    irq::{enable_irq, set_handler},
    mailbox::{Channel, Cpu, Mailbox},
    pac::interrupt::ExternalInterrupt,
    peripherals::{self, Mailboxes},
    resource_table::{RESOURCE_TABLE, VRING_ALIGN, VRING_NUM},
    rpmsg::{RPMSG_ADDR_NS, RpmsgHeader, RpmsgNsMsg},
    uart::{self, Uart},
    virtio::VirtQueue,
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
async fn main(_spawner: Spawner) -> ! {
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

        while (core::ptr::read_volatile(&RESOURCE_TABLE.rpmsg_vdev.status) & 0x4) == 0 {
            Timer::after_millis(10).await;
        }
    }

    let mut tx_mailbox = Mailbox::new(mailbox, Channel::Ch1, Cpu::C906_0);
    let rx_mailbox = Mailbox::new(unsafe { Mailboxes::steal() }, Channel::Ch0, Cpu::C906_1);

    Timer::after_secs(1).await;

    // Uart::new takes &pac::Uart1. uart1 is &mut peripherals::Uart1, which derefs to pac::Uart1.
    let mut uart1p = Uart::new(uart1, uart::Config::default().with_add_cr(true)).unwrap();
    let mut tx_queue =
        unsafe { VirtQueue::from_resource_table_addr(0x8F528000, VRING_NUM as u16, VRING_ALIGN) };
    let mut rx_queue =
        unsafe { VirtQueue::from_resource_table_addr(0x8F52C000, VRING_NUM as u16, VRING_ALIGN) };

    writeln!(uart1p, "{BANNER}");
    writeln!(uart1p, "# Synthgut 0.1.0, built {BUILD_TIME} #");
    writeln!(
        uart1p,
        "# VirtIO Driver OK. Rings Initialized                        #"
    );
    writeln!(uart1p, "{BANNER}\n");
    writeln!(uart1p, "rx_queue = {rx_queue:?}");
    writeln!(uart1p, "tx_queue = {tx_queue:?}");

    let mut tx_desc_idx = None;
    while tx_desc_idx.is_none() {
        tx_desc_idx = tx_queue.get_avail_buf();
    }
    let desc_idx = tx_desc_idx.unwrap();
    writeln!(uart1p, "Got tx_desc_idx {desc_idx}");

    let buffer = unsafe { tx_queue.get_buf_slice(desc_idx) };

    let src_addr = 0x400;
    let payload = RpmsgNsMsg::new("rpmsg-client-sample", src_addr);

    let header = RpmsgHeader {
        src: src_addr,
        dst: RPMSG_ADDR_NS,
        reserved: 0,
        len: size_of::<RpmsgNsMsg>() as u16,
        flags: 0,
    };

    unsafe {
        let head_ptr = buffer.as_mut_ptr() as *mut RpmsgHeader;
        head_ptr.write(header);

        writeln!(uart1p, "Wrote header {:?} into {:?}", header, head_ptr);

        let payload_ptr = buffer.as_mut_ptr().add(size_of::<RpmsgHeader>()) as *mut RpmsgNsMsg;
        payload_ptr.write(payload);
        writeln!(uart1p, "Wrote payload {:?} into {:?}", payload_ptr, payload);
    }

    let total_len = size_of::<RpmsgHeader>() + size_of::<RpmsgNsMsg>();
    tx_queue.add_used_buf(desc_idx, total_len as u32);

    writeln!(uart1p, "Sent Name Service Announcement. Kicking Host...");
    if !tx_mailbox.send_data(0) {
        panic!("Failed to send `0` on tx_mailbox");
    }

    loop {
        if rx_mailbox.is_pending()
            && let Some(d) = rx_mailbox.read_data()
        {
            writeln!(uart1p, "You've got mail! {d:010X}");
        }
        if let Some(desc_idx) = rx_queue.get_avail_buf() {
            let buf = unsafe { rx_queue.get_buf_slice(desc_idx) };

            let header = unsafe { &*(buf.as_ptr() as *const RpmsgHeader) };
            let payload_len = header.len as usize;

            if buf.len() >= 16 + payload_len {
                let msg_data = &buf[16..16 + payload_len];
                if let Ok(s) = core::str::from_utf8(msg_data) {
                    writeln!(uart1p, "RX: {}", s);
                }
            }

            rx_queue.add_used_buf(desc_idx, 0);

            if !tx_mailbox.send_data(1) {
                panic!("Failed to send `1` on tx_mailbox");
            }
        }

        Timer::after_millis(2).await;
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
