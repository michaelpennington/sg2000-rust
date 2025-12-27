use sg2000_pac::{Plic, interrupt::ExternalInterrupt};

const DEFAULT_PRIORITY: u32 = 7;
const MAX_INTERRUPTS: usize = 128;

static mut IRQ_HANDLERS: [Option<fn()>; MAX_INTERRUPTS] = [None; MAX_INTERRUPTS];

pub fn set_handler(irq: ExternalInterrupt, handler: fn()) {
    let irq_no = irq as usize;
    if irq_no < MAX_INTERRUPTS {
        unsafe {
            IRQ_HANDLERS[irq_no] = Some(handler);
        }
    }
}

pub fn enable_irq(plic: &Plic, irq: ExternalInterrupt) {
    let irq_no = irq as usize;
    plic.plic_prio(irq_no)
        .write(|w| unsafe { w.bits(DEFAULT_PRIORITY) });
    let reg_index = irq_no / 32;
    let bit_index = irq_no % 32;

    plic.plic_h0_mie(reg_index)
        .modify(|r, w| unsafe { w.bits(r.bits() | (1 << bit_index)) });
}

#[unsafe(no_mangle)]
pub extern "C" fn MachineExternal() {
    let plic = unsafe { sg2000_pac::Plic::steal() };

    loop {
        let claim = plic.plic_h0_mclaim();
        let irq_no = claim.read().bits();

        if irq_no == 0 {
            break;
        }

        if (irq_no as usize) < MAX_INTERRUPTS
            && let Some(handler) = unsafe { IRQ_HANDLERS[irq_no as usize] }
        {
            handler();
        }

        claim.write(|w| unsafe { w.bits(irq_no) });
    }
}
