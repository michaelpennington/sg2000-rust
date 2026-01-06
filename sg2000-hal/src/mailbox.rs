use core::{
    ops::Index,
    sync::atomic::{AtomicU8, Ordering},
};

use crate::peripherals::Mailboxes;

static LOCK_COUNT: Locks = Locks::new();

const SPINLOCK_INDEX: usize = 4;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(usize)]
pub enum Cpu {
    C906_0 = 1,
    C906_1 = 2,
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Channel {
    Ch0,
    Ch1,
    Ch2,
    Ch3,
    Ch4,
    Ch5,
    Ch6,
    Ch7,
}

struct Locks {
    locks: [AtomicU8; 8],
}

impl Locks {
    const fn new() -> Self {
        Self {
            locks: [
                AtomicU8::new(1),
                AtomicU8::new(1),
                AtomicU8::new(1),
                AtomicU8::new(1),
                AtomicU8::new(1),
                AtomicU8::new(1),
                AtomicU8::new(1),
                AtomicU8::new(1),
            ],
        }
    }
}

impl Index<usize> for Locks {
    type Output = AtomicU8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.locks[index]
    }
}

pub struct Mailbox<'a> {
    channel: Channel,
    mbox: Mailboxes<'a>,
    to_cpu: Cpu,
    mask: u8,
    last_lock: u8,
}

impl<'a> Mailbox<'a> {
    pub fn new(mbox: Mailboxes<'a>, channel: Channel, to_cpu: Cpu) -> Self {
        let mask = 1 << channel as u32;
        mbox.cpu_mbox_int(to_cpu as usize)
            .int_clr()
            .write(|w| unsafe { w.bits(mask) });
        mbox.cpu_mbox_en(to_cpu as usize)
            .modify(|r, w| unsafe { w.bits(r.bits() & !mask) });
        mbox.mbox_set().reset();
        Self {
            channel,
            mbox,
            to_cpu,
            mask,
            last_lock: 0,
        }
    }

    fn lock(&mut self) {
        let spinlock = self.mbox.spinlock(SPINLOCK_INDEX);
        let lock_val = LOCK_COUNT[SPINLOCK_INDEX].fetch_add(1, Ordering::Relaxed) as u32;

        loop {
            spinlock.write(|w| unsafe { w.bits(lock_val) });
            if spinlock.read().bits() == lock_val {
                self.last_lock = lock_val as u8;
                return;
            }

            core::hint::spin_loop();
        }
    }

    fn unlock(&self) -> bool {
        let spinlock = self.mbox.spinlock(SPINLOCK_INDEX);
        if spinlock.read().bits() == self.last_lock as u32 {
            spinlock.write(|w| unsafe { w.bits(self.last_lock as u32) });
            return true;
        }
        false
    }

    pub fn send_data(&mut self, data: u32) -> bool {
        self.lock();

        let cpu_mbox_int = self.mbox.cpu_mbox_int(self.to_cpu as usize);
        self.mbox
            .context(self.channel as usize)
            .write(|w| unsafe { w.bits(data) });
        cpu_mbox_int
            .int_clr()
            .write(|w| unsafe { w.bits(self.mask) });
        self.mbox
            .cpu_mbox_en(self.to_cpu as usize)
            .modify(|r, w| unsafe { w.bits(r.bits() | self.mask) });
        self.mbox.mbox_set().write(|w| unsafe { w.bits(self.mask) });
        if !self.unlock() {
            return false;
        }

        unsafe { xuantie_riscv::asm::sync() };
        true
    }

    pub fn is_pending(&self) -> bool {
        self.mbox
            .cpu_mbox_int(Cpu::C906_1 as usize)
            .int_int()
            .read()
            .bits()
            & self.mask
            != 0
    }

    pub fn read_data(&self) -> Option<u32> {
        if self.is_pending() {
            let msg = self.mbox.context(self.channel as usize).read().bits();
            self.mbox
                .cpu_mbox_int(Cpu::C906_1 as usize)
                .int_clr()
                .write(|w| unsafe { w.bits(self.mask) });
            self.mbox
                .cpu_mbox_en(Cpu::C906_1 as usize)
                .modify(|r, w| unsafe { w.bits(r.bits() & !self.mask) });
            Some(msg)
        } else {
            None
        }
    }
}
