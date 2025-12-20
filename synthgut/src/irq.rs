use sg2000_pac::{Plic, interrupt::ExternalInterrupt};

const DEFAULT_PRIORITY: u32 = 7;
const HART_NO: usize = 0;

unsafe extern "C" {
    fn GPIO1();
}

#[unsafe(no_mangle)]
pub extern "C" fn MachineExternal() {
    let plic = unsafe { sg2000_pac::Plic::steal() };

    loop {
        // Claim the interrupt
        let claim = plic.claim(HART_NO);
        let irq_no = claim.read().bits();

        if irq_no == 0 {
            break;
        }

        match irq_no {
            i if i == ExternalInterrupt::GPIO1 as u32 => unsafe { GPIO1() },
            _ => {}
        }

        unsafe {
            claim.write(|w| w.bits(irq_no));
        }
    }
}

pub fn enable_irq(plic: &Plic, irq_no: usize) {
    unsafe {
        plic.priority(irq_no).write(|w| w.bits(DEFAULT_PRIORITY));

        let reg_index = irq_no / 32;
        let bit_index = irq_no % 32;

        plic.enable(HART_NO)
            .bits(reg_index)
            .modify(|r, w| w.bits(r.bits() | (1 << bit_index)));
    }
}
