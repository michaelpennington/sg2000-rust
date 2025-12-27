#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    msip0: Msip0,
    _reserved1: [u8; 0x3ffc],
    mtimecmpl0: Mtimecmpl0,
    mtimecmph0: Mtimecmph0,
    _reserved3: [u8; 0x7ff8],
    ssip0: Ssip0,
    _reserved4: [u8; 0x0ffc],
    stimecmpl0: Stimecmpl0,
    stimecmph0: Stimecmph0,
}
impl RegisterBlock {
    #[doc = "0x00 - M-mode Software Interrupt Pending"]
    #[inline(always)]
    pub const fn msip0(&self) -> &Msip0 {
        &self.msip0
    }
    #[doc = "0x4000 - M-mode Timer Compare Low"]
    #[inline(always)]
    pub const fn mtimecmpl0(&self) -> &Mtimecmpl0 {
        &self.mtimecmpl0
    }
    #[doc = "0x4004 - M-mode Timer Compare High"]
    #[inline(always)]
    pub const fn mtimecmph0(&self) -> &Mtimecmph0 {
        &self.mtimecmph0
    }
    #[doc = "0xc000 - S-mode Software Interrupt Pending"]
    #[inline(always)]
    pub const fn ssip0(&self) -> &Ssip0 {
        &self.ssip0
    }
    #[doc = "0xd000 - S-mode Timer Compare Low"]
    #[inline(always)]
    pub const fn stimecmpl0(&self) -> &Stimecmpl0 {
        &self.stimecmpl0
    }
    #[doc = "0xd004 - S-mode Timer Compare High"]
    #[inline(always)]
    pub const fn stimecmph0(&self) -> &Stimecmph0 {
        &self.stimecmph0
    }
}
#[doc = "MSIP0 (rw) register accessor: M-mode Software Interrupt Pending\n\nYou can [`read`](crate::Reg::read) this register and get [`msip0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`msip0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msip0`] module"]
#[doc(alias = "MSIP0")]
pub type Msip0 = crate::Reg<msip0::Msip0Spec>;
#[doc = "M-mode Software Interrupt Pending"]
pub mod msip0;
#[doc = "MTIMECMPL0 (rw) register accessor: M-mode Timer Compare Low\n\nYou can [`read`](crate::Reg::read) this register and get [`mtimecmpl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtimecmpl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtimecmpl0`] module"]
#[doc(alias = "MTIMECMPL0")]
pub type Mtimecmpl0 = crate::Reg<mtimecmpl0::Mtimecmpl0Spec>;
#[doc = "M-mode Timer Compare Low"]
pub mod mtimecmpl0;
#[doc = "MTIMECMPH0 (rw) register accessor: M-mode Timer Compare High\n\nYou can [`read`](crate::Reg::read) this register and get [`mtimecmph0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtimecmph0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtimecmph0`] module"]
#[doc(alias = "MTIMECMPH0")]
pub type Mtimecmph0 = crate::Reg<mtimecmph0::Mtimecmph0Spec>;
#[doc = "M-mode Timer Compare High"]
pub mod mtimecmph0;
#[doc = "SSIP0 (rw) register accessor: S-mode Software Interrupt Pending\n\nYou can [`read`](crate::Reg::read) this register and get [`ssip0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssip0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssip0`] module"]
#[doc(alias = "SSIP0")]
pub type Ssip0 = crate::Reg<ssip0::Ssip0Spec>;
#[doc = "S-mode Software Interrupt Pending"]
pub mod ssip0;
#[doc = "STIMECMPL0 (rw) register accessor: S-mode Timer Compare Low\n\nYou can [`read`](crate::Reg::read) this register and get [`stimecmpl0::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stimecmpl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stimecmpl0`] module"]
#[doc(alias = "STIMECMPL0")]
pub type Stimecmpl0 = crate::Reg<stimecmpl0::Stimecmpl0Spec>;
#[doc = "S-mode Timer Compare Low"]
pub mod stimecmpl0;
#[doc = "STIMECMPH0 (rw) register accessor: S-mode Timer Compare High\n\nYou can [`read`](crate::Reg::read) this register and get [`stimecmph0::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stimecmph0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stimecmph0`] module"]
#[doc(alias = "STIMECMPH0")]
pub type Stimecmph0 = crate::Reg<stimecmph0::Stimecmph0Spec>;
#[doc = "S-mode Timer Compare High"]
pub mod stimecmph0;
