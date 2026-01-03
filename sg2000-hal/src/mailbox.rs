use crate::peripherals::Mailboxes;

/// Configuration based on the provided DTS
const LOCAL_CPU_ID: usize = 1; // MCU is CPU 1
// const REMOTE_CPU_ID: usize = 0; // Linux is CPU 0
const TX_BUF_CHANNEL: u8 = 0; // Channel used to Kick Linux (vq_tx)
const RX_DATA_CHANNEL: u8 = 1; // Channel Linux uses to Kick MCU (vq_rx)
// const KICK_LINUX_CHANNEL: u8 = 2;

pub struct Sg2000Mailbox<'a> {
    regs: Mailboxes<'a>,
}

impl<'a> Sg2000Mailbox<'a> {
    /// Initialize the mailbox driver.
    ///
    /// This configures the local CPU to receive interrupts on the RX_CHANNEL.
    /// It does NOT enable the PLIC interrupt (must be done separately).
    pub fn new(regs: Mailboxes<'a>) -> Self {
        let mbox = Self { regs };

        // 1. Unmask the specific RX channel for the Local CPU.
        //    We assume '0' in int_mask enables the interrupt (standard behavior).
        //    We read-modify-write to preserve other settings if necessary.
        mbox.regs
            .cpu_mbox_set(LOCAL_CPU_ID)
            .int_mask()
            .modify(|r, w| unsafe {
                // Clear the bit corresponding to RX_CHANNEL to UNMASK it
                w.bits(r.bits() & !((1 << RX_DATA_CHANNEL) | (1 << TX_BUF_CHANNEL)))
            });

        // 2. Enable routing of the RX channel to the Local CPU.
        //    Set the bit in cpu_mbox_en.
        mbox.regs.cpu_mbox_en(LOCAL_CPU_ID).modify(|r, w| unsafe {
            w.bits(r.bits() | (1 << RX_DATA_CHANNEL) | (1 << TX_BUF_CHANNEL))
        });

        mbox
    }

    /// Kick the remote processor.
    ///
    /// This asserts the interrupt for the TX channel. The remote processor (Linux)
    /// must have its cpu_mbox_en and int_mask configured to listen to this channel.
    pub fn kick(&mut self) {
        // Write to mbox_set to trigger the interrupt
        // The hardware will route this to the CPU that has this channel enabled.
        unsafe {
            self.regs.mbox_set().write(|w| w.bits(1 << TX_BUF_CHANNEL));
        }
    }

    /// Check if we have a pending interrupt for the RX channel and clear it.
    ///
    /// Returns true if the RPMsg vring should be processed.
    pub fn check_and_clear_pending(&mut self) -> bool {
        // Read the Masked Interrupt Status for the Local CPU
        let status = self.regs.cpu_mbox_set(LOCAL_CPU_ID).int_int().read().bits();

        if (status & (1 << RX_DATA_CHANNEL)) != 0 {
            // Clear the interrupt by writing 1 to int_clr
            unsafe {
                self.regs
                    .cpu_mbox_set(LOCAL_CPU_ID)
                    .int_clr()
                    .write(|w| w.bits(1 << RX_DATA_CHANNEL));
            }
            return true;
        }

        false
    }
}

//Helper function to setup the PLIC for the Mailbox interrupt.
//Call this during system initialization.
// pub fn enable_mailbox_interrupt(_plic: &sg2000_pac::Plic) {
//     // The SG2000 PAC defines the mailbox interrupt as MBOX (ID 61)
//     let mbox_irq = ExternalInterrupt::MBOX;
//
//     // Note: You need to implement the PLIC enable logic specific to your context (Hart ID)
//     // context.enable_interrupt(mbox_irq);
//     // context.set_priority(mbox_irq, Priority::P1);
// }
