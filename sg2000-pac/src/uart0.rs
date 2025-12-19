#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    rbr_thr_dll: RbrThrDll,
    _reserved1: [u8; 0x10],
    lsr: Lsr,
}
impl RegisterBlock {
    #[doc = "0x00 - Receive Buffer,Transmit Holding or Divisor Latch Low byte Register"]
    #[inline(always)]
    pub const fn rbr_thr_dll(&self) -> &RbrThrDll {
        &self.rbr_thr_dll
    }
    #[doc = "0x14 - Line Status Register"]
    #[inline(always)]
    pub const fn lsr(&self) -> &Lsr {
        &self.lsr
    }
}
#[doc = "RBR_THR_DLL (rw) register accessor: Receive Buffer,Transmit Holding or Divisor Latch Low byte Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rbr_thr_dll::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rbr_thr_dll::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rbr_thr_dll`] module"]
#[doc(alias = "RBR_THR_DLL")]
pub type RbrThrDll = crate::Reg<rbr_thr_dll::RbrThrDllSpec>;
#[doc = "Receive Buffer,Transmit Holding or Divisor Latch Low byte Register"]
pub mod rbr_thr_dll;
#[doc = "LSR (r) register accessor: Line Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lsr`] module"]
#[doc(alias = "LSR")]
pub type Lsr = crate::Reg<lsr::LsrSpec>;
#[doc = "Line Status Register"]
pub mod lsr;
