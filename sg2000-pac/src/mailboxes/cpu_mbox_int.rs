#[repr(C)]
#[doc = "Interrupts Registers"]
#[doc(alias = "cpu_mbox_int")]
pub struct CpuMboxInt {
    int_clr: IntClr,
    _reserved1: [u8; 0x03],
    int_mask: IntMask,
    _reserved2: [u8; 0x03],
    int_int: IntInt,
    _reserved3: [u8; 0x03],
    int_raw: IntRaw,
}
impl CpuMboxInt {
    #[doc = "0x00 - Interrupt Clear"]
    #[inline(always)]
    pub const fn int_clr(&self) -> &IntClr {
        &self.int_clr
    }
    #[doc = "0x04 - Interrupt Mask"]
    #[inline(always)]
    pub const fn int_mask(&self) -> &IntMask {
        &self.int_mask
    }
    #[doc = "0x08 - Interrupt Status"]
    #[inline(always)]
    pub const fn int_int(&self) -> &IntInt {
        &self.int_int
    }
    #[doc = "0x0c - Interrupt Raw"]
    #[inline(always)]
    pub const fn int_raw(&self) -> &IntRaw {
        &self.int_raw
    }
}
#[doc = "int_clr (rw) register accessor: Interrupt Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`int_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
#[doc(alias = "int_clr")]
pub type IntClr = crate::Reg<int_clr::IntClrSpec>;
#[doc = "Interrupt Clear"]
pub mod int_clr;
#[doc = "int_mask (rw) register accessor: Interrupt Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`int_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_mask`] module"]
#[doc(alias = "int_mask")]
pub type IntMask = crate::Reg<int_mask::IntMaskSpec>;
#[doc = "Interrupt Mask"]
pub mod int_mask;
#[doc = "int_int (rw) register accessor: Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`int_int::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_int::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_int`] module"]
#[doc(alias = "int_int")]
pub type IntInt = crate::Reg<int_int::IntIntSpec>;
#[doc = "Interrupt Status"]
pub mod int_int;
#[doc = "int_raw (rw) register accessor: Interrupt Raw\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
#[doc(alias = "int_raw")]
pub type IntRaw = crate::Reg<int_raw::IntRawSpec>;
#[doc = "Interrupt Raw"]
pub mod int_raw;
