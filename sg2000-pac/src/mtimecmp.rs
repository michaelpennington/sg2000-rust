#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    mtimecmp_low: MtimecmpLow,
    mtimecmp_high: MtimecmpHigh,
}
impl RegisterBlock {
    #[doc = "0x00 - Low Bits Of Mtimecmp"]
    #[inline(always)]
    pub const fn mtimecmp_low(&self) -> &MtimecmpLow {
        &self.mtimecmp_low
    }
    #[doc = "0x04 - Low Bits Of Mtimecmp"]
    #[inline(always)]
    pub const fn mtimecmp_high(&self) -> &MtimecmpHigh {
        &self.mtimecmp_high
    }
}
#[doc = "mtimecmp_low (rw) register accessor: Low Bits Of Mtimecmp\n\nYou can [`read`](crate::Reg::read) this register and get [`mtimecmp_low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtimecmp_low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtimecmp_low`] module"]
#[doc(alias = "mtimecmp_low")]
pub type MtimecmpLow = crate::Reg<mtimecmp_low::MtimecmpLowSpec>;
#[doc = "Low Bits Of Mtimecmp"]
pub mod mtimecmp_low;
#[doc = "mtimecmp_high (rw) register accessor: Low Bits Of Mtimecmp\n\nYou can [`read`](crate::Reg::read) this register and get [`mtimecmp_high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtimecmp_high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtimecmp_high`] module"]
#[doc(alias = "mtimecmp_high")]
pub type MtimecmpHigh = crate::Reg<mtimecmp_high::MtimecmpHighSpec>;
#[doc = "Low Bits Of Mtimecmp"]
pub mod mtimecmp_high;
