use core::cell::RefCell;

use critical_section::{CriticalSection, Mutex};
use embassy_time_driver::{Driver, TICK_HZ};
use embassy_time_queue_utils::Queue;

// Ratio between the selected tick rate and the clock rate (25Mhz)
const TICK_RATIO: u64 = 25_000_000 / TICK_HZ;

struct Sg2000TimerDriver {
    queue: Mutex<RefCell<Queue>>,
}

impl Sg2000TimerDriver {
    const fn new() -> Self {
        Self {
            queue: Mutex::new(RefCell::new(Queue::new())),
        }
    }

    /// Sets the mtimecmp alarm.
    /// Returns `false` if the timestamp is in the past (alarm not set).
    /// Returns `true` if the alarm was successfully set for the future (or disabled if empty)
    fn set_alarm(&self, _cs: &CriticalSection, timestamp: u64) -> bool {
        let clint = unsafe { sg2000_pac::Clint::steal() };

        // If the queue is empty, embassy-time-queue-utils returns u64::MAX.
        // We set the comparator to max to effectively disable the interrupt.
        if timestamp == u64::MAX {
            clint.mtimecmph0().reset();
            clint.mtimecmpl0().reset();
            return true;
        }

        let cycles = timestamp * TICK_RATIO;
        let now = riscv::register::time::read64();

        if cycles <= now {
            return false;
        }
        // Safe update sequence for 64-bit timer on 32-bit bus:
        // 1. Set High to MAX to prevent spurious match while writing Low
        // 2. Write Low
        // 3. Write correct High
        clint.mtimecmph0().reset();
        clint
            .mtimecmpl0()
            .write(|w| unsafe { w.bits(cycles as u32) });
        clint
            .mtimecmph0()
            .write(|w| unsafe { w.bits((cycles >> 32) as u32) });
        true
    }
}

impl Driver for Sg2000TimerDriver {
    fn now(&self) -> u64 {
        riscv::register::time::read64() / TICK_RATIO
    }

    fn schedule_wake(&self, at: u64, waker: &core::task::Waker) {
        critical_section::with(|cs| {
            let mut queue = self.queue.borrow(cs).borrow_mut();
            // Queue::schedule_wake returns true if the new task is the *soonest* one.
            if queue.schedule_wake(at, waker) {
                let mut next = queue.next_expiration(self.now());

                while !self.set_alarm(&cs, next) {
                    next = queue.next_expiration(self.now());
                }
            }
        });
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn MachineTimer() {
    critical_section::with(|cs| {
        let mut queue = DRIVER.queue.borrow(cs).borrow_mut();

        // next_expiration(now) does two things:
        // 1. Pops and wakes any tasks that have expired relative to `now`.
        // 2. Returns the timestamp of the *next* upcoming event.
        let mut next = queue.next_expiration(DRIVER.now());

        // Attempt to set the alarm for that next event.
        // If it's already passed (set_alarm returns false), we loop to process that one too.
        while !DRIVER.set_alarm(&cs, next) {
            next = queue.next_expiration(DRIVER.now());
        }
    })
}

embassy_time_driver::time_driver_impl!(static DRIVER: Sg2000TimerDriver = Sg2000TimerDriver::new());
