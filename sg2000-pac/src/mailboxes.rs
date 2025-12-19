#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cpu_mbox_en: (),
    _reserved1: [u8; 0x10],
    cpu_mbox_set: (),
    _reserved2: [u8; 0x50],
    mbox_set: MboxSet,
    _reserved3: [u8; 0x03],
    mbox_status: MboxStatus,
    _reserved4: [u8; 0x0b],
    cpu_mbox_status: (),
    _reserved5: [u8; 0x50],
    spinlock: [Spinlock; 8],
    _reserved6: [u8; 0x0320],
    context: [Context; 16],
}
impl RegisterBlock {
    #[doc = "0x00 - Mailbox Enable Bits"]
    #[inline(always)]
    pub const fn cpu_mbox_en(&self, n: usize) -> &CpuMboxEn {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4 * n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00 - Mailbox Enable Bits"]
    #[inline(always)]
    pub fn cpu_mbox_en_iter(&self) -> impl Iterator<Item = &CpuMboxEn> {
        (0..4).map(move |n| unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4 * n).cast() })
    }
    #[doc = "0x10..0x44 - Interrupts Registers"]
    #[inline(always)]
    pub const fn cpu_mbox_set(&self, n: usize) -> &CpuMboxSet {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(16)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x10..0x44 - Interrupts Registers"]
    #[inline(always)]
    pub fn cpu_mbox_set_iter(&self) -> impl Iterator<Item = &CpuMboxSet> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(16)
                .add(16 * n)
                .cast()
        })
    }
    #[doc = "0x60 - mbox_set"]
    #[inline(always)]
    pub const fn mbox_set(&self) -> &MboxSet {
        &self.mbox_set
    }
    #[doc = "0x64 - mbox_status"]
    #[inline(always)]
    pub const fn mbox_status(&self) -> &MboxStatus {
        &self.mbox_status
    }
    #[doc = "0x70 - cpu_mbox_status"]
    #[inline(always)]
    pub const fn cpu_mbox_status(&self, n: usize) -> &CpuMboxStatus {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(112)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x70 - cpu_mbox_status"]
    #[inline(always)]
    pub fn cpu_mbox_status_iter(&self) -> impl Iterator<Item = &CpuMboxStatus> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(112)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0xc0..0xe0 - Spinlock"]
    #[inline(always)]
    pub const fn spinlock(&self, n: usize) -> &Spinlock {
        &self.spinlock[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xc0..0xe0 - Spinlock"]
    #[inline(always)]
    pub fn spinlock_iter(&self) -> impl Iterator<Item = &Spinlock> {
        self.spinlock.iter()
    }
    #[doc = "0x400..0x440 - Data For The Messages"]
    #[inline(always)]
    pub const fn context(&self, n: usize) -> &Context {
        &self.context[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x400..0x440 - Data For The Messages"]
    #[inline(always)]
    pub fn context_iter(&self) -> impl Iterator<Item = &Context> {
        self.context.iter()
    }
}
#[doc = "cpu_mbox_en (rw) register accessor: Mailbox Enable Bits\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_mbox_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_mbox_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_mbox_en`] module"]
#[doc(alias = "cpu_mbox_en")]
pub type CpuMboxEn = crate::Reg<cpu_mbox_en::CpuMboxEnSpec>;
#[doc = "Mailbox Enable Bits"]
pub mod cpu_mbox_en;
#[doc = "Interrupts Registers"]
pub use self::cpu_mbox_set::CpuMboxSet;
#[doc = r"Cluster"]
#[doc = "Interrupts Registers"]
pub mod cpu_mbox_set;
#[doc = "mbox_set (rw) register accessor: mbox_set\n\nYou can [`read`](crate::Reg::read) this register and get [`mbox_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mbox_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mbox_set`] module"]
#[doc(alias = "mbox_set")]
pub type MboxSet = crate::Reg<mbox_set::MboxSetSpec>;
#[doc = "mbox_set"]
pub mod mbox_set;
#[doc = "mbox_status (rw) register accessor: mbox_status\n\nYou can [`read`](crate::Reg::read) this register and get [`mbox_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mbox_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mbox_status`] module"]
#[doc(alias = "mbox_status")]
pub type MboxStatus = crate::Reg<mbox_status::MboxStatusSpec>;
#[doc = "mbox_status"]
pub mod mbox_status;
#[doc = "cpu_mbox_status (rw) register accessor: cpu_mbox_status\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_mbox_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_mbox_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_mbox_status`] module"]
#[doc(alias = "cpu_mbox_status")]
pub type CpuMboxStatus = crate::Reg<cpu_mbox_status::CpuMboxStatusSpec>;
#[doc = "cpu_mbox_status"]
pub mod cpu_mbox_status;
#[doc = "spinlock (rw) register accessor: Spinlock\n\nYou can [`read`](crate::Reg::read) this register and get [`spinlock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spinlock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spinlock`] module"]
#[doc(alias = "spinlock")]
pub type Spinlock = crate::Reg<spinlock::SpinlockSpec>;
#[doc = "Spinlock"]
pub mod spinlock;
#[doc = "context (rw) register accessor: Data For The Messages\n\nYou can [`read`](crate::Reg::read) this register and get [`context::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`context::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@context`] module"]
#[doc(alias = "context")]
pub type Context = crate::Reg<context::ContextSpec>;
#[doc = "Data For The Messages"]
pub mod context;
