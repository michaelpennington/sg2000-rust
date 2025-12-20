use core::{cell::RefCell, task::Waker};

use critical_section::Mutex;
use embassy_time_driver::{Driver, TICK_HZ};

// Ratio between the selected tick rate and the clock rate (25Mhz)
const TICK_RATIO: u64 = 25_000_000 / TICK_HZ;

static WAKER: Mutex<RefCell<Option<Waker>>> = Mutex::new(RefCell::new(None));

struct Sg2000TimerDriver {}

impl Driver for Sg2000TimerDriver {
    fn now(&self) -> u64 {
        riscv::register::time::read64() / TICK_RATIO
    }

    fn schedule_wake(&self, at: u64, waker: &core::task::Waker) {
        critical_section::with(|cs| {
            let mut waker_slot = WAKER.borrow(cs).borrow_mut();
            *waker_slot = Some(waker.clone());

            let timestamp = at * TICK_RATIO;
            unsafe {
                let mtimecmp = sg2000_pac::Mtimecmp::steal();
                mtimecmp
                    .mtimecmp_high()
                    .write(|w| w.bits((timestamp >> 32) as u32));
                mtimecmp.mtimecmp_low().write(|w| w.bits(timestamp as u32));
            }
        });
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn MachineTimer() {
    critical_section::with(|cs| {
        unsafe {
            // "clear" the interrupt by settin mtimecmp to the maximal value
            let mtimecmp = sg2000_pac::Mtimecmp::steal();
            mtimecmp.mtimecmp_high().write(|w| w.bits(u32::MAX));
            mtimecmp.mtimecmp_low().write(|w| w.bits(u32::MAX));
        }
        let mut waker_slot = WAKER.borrow(cs).borrow_mut();
        if let Some(waker) = waker_slot.take() {
            waker.wake();
        }
    })
}

embassy_time_driver::time_driver_impl!(static DRIVER: Sg2000TimerDriver = Sg2000TimerDriver{});
