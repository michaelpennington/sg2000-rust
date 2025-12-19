#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    priority: [Priority; 1024],
    _reserved1: [u8; 0x1000],
    enable: [Enable; 15872],
    _reserved2: [u8; 0xe000],
    priority_threshold: (),
    _reserved3: [u8; 0x04],
    claim: (),
}
impl RegisterBlock {
    #[doc = "0x00..0x1000 - Interrupt priority"]
    #[inline(always)]
    pub const fn priority(&self, n: usize) -> &Priority {
        &self.priority[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x1000 - Interrupt priority"]
    #[inline(always)]
    pub fn priority_iter(&self) -> impl Iterator<Item = &Priority> {
        self.priority.iter()
    }
    #[doc = "0x2000..0x1f2000 - Interrupt Enable Registers"]
    #[inline(always)]
    pub const fn enable(&self, n: usize) -> &Enable {
        &self.enable[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2000..0x1f2000 - Interrupt Enable Registers"]
    #[inline(always)]
    pub fn enable_iter(&self) -> impl Iterator<Item = &Enable> {
        self.enable.iter()
    }
    #[doc = "0x200000..0x20f800 - Interrupt Priority Threshold"]
    #[inline(always)]
    pub const fn priority_threshold(&self, n: usize) -> &PriorityThreshold {
        #[allow(clippy::no_effect)]
        [(); 15872][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(2097152)
                .add(4096 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x200000..0x20f800 - Interrupt Priority Threshold"]
    #[inline(always)]
    pub fn priority_threshold_iter(&self) -> impl Iterator<Item = &PriorityThreshold> {
        (0..15872).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(2097152)
                .add(4096 * n)
                .cast()
        })
    }
    #[doc = "0x200004..0x20f804 - Interrupt Claim/Completion"]
    #[inline(always)]
    pub const fn claim(&self, n: usize) -> &Claim {
        #[allow(clippy::no_effect)]
        [(); 15872][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(2097156)
                .add(4096 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x200004..0x20f804 - Interrupt Claim/Completion"]
    #[inline(always)]
    pub fn claim_iter(&self) -> impl Iterator<Item = &Claim> {
        (0..15872).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(2097156)
                .add(4096 * n)
                .cast()
        })
    }
}
#[doc = "priority (rw) register accessor: Interrupt priority\n\nYou can [`read`](crate::Reg::read) this register and get [`priority::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@priority`] module"]
#[doc(alias = "priority")]
pub type Priority = crate::Reg<priority::PrioritySpec>;
#[doc = "Interrupt priority"]
pub mod priority;
#[doc = "Interrupt Enable Registers"]
pub use self::enable::Enable;
#[doc = r"Cluster"]
#[doc = "Interrupt Enable Registers"]
pub mod enable;
#[doc = "priority_threshold (rw) register accessor: Interrupt Priority Threshold\n\nYou can [`read`](crate::Reg::read) this register and get [`priority_threshold::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority_threshold::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@priority_threshold`] module"]
#[doc(alias = "priority_threshold")]
pub type PriorityThreshold = crate::Reg<priority_threshold::PriorityThresholdSpec>;
#[doc = "Interrupt Priority Threshold"]
pub mod priority_threshold;
#[doc = "claim (rw) register accessor: Interrupt Claim/Completion\n\nYou can [`read`](crate::Reg::read) this register and get [`claim::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`claim::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@claim`] module"]
#[doc(alias = "claim")]
pub type Claim = crate::Reg<claim::ClaimSpec>;
#[doc = "Interrupt Claim/Completion"]
pub mod claim;
