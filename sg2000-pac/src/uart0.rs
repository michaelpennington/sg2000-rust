#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved_0_dll: [u8; 0x04],
    _reserved_1_dlh: [u8; 0x04],
    _reserved_2_fcr: [u8; 0x04],
    lcr: Lcr,
    mcr: Mcr,
    lsr: Lsr,
    msr: Msr,
    _reserved7: [u8; 0x04],
    lpdll: Lpdll,
    lpdlh: Lpdlh,
    _reserved9: [u8; 0x08],
    _reserved_9_srbr: [u8; 0x04],
    _reserved10: [u8; 0x3c],
    far: Far,
    tfr: Tfr,
    rfw: Rfw,
    usr: Usr,
    tfl: Tfl,
    rfl: Rfl,
    srr: Srr,
    srts: Srts,
    sbcr: Sbcr,
    sdmam: Sdmam,
    sfe: Sfe,
    srt: Srt,
    stet: Stet,
    htx: Htx,
    dmasa: Dmasa,
}
impl RegisterBlock {
    #[doc = "0x00 - Divisor Latch Low byte Register"]
    #[inline(always)]
    pub const fn dll(&self) -> &Dll {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x00 - Receive Buffer Register, Transmit Holding Register"]
    #[inline(always)]
    pub const fn rbr_thr(&self) -> &RbrThr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x04 - Divisor Latch high byte Register"]
    #[inline(always)]
    pub const fn dlh(&self) -> &Dlh {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x04 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x08 - FIFO Control Register"]
    #[inline(always)]
    pub const fn fcr(&self) -> &Fcr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x08 - Interrupt Identification Register"]
    #[inline(always)]
    pub const fn iir(&self) -> &Iir {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x0c - Line Control Register"]
    #[inline(always)]
    pub const fn lcr(&self) -> &Lcr {
        &self.lcr
    }
    #[doc = "0x10 - Modem Control Register"]
    #[inline(always)]
    pub const fn mcr(&self) -> &Mcr {
        &self.mcr
    }
    #[doc = "0x14 - Line Status Register"]
    #[inline(always)]
    pub const fn lsr(&self) -> &Lsr {
        &self.lsr
    }
    #[doc = "0x18 - Modem Status Register"]
    #[inline(always)]
    pub const fn msr(&self) -> &Msr {
        &self.msr
    }
    #[doc = "0x20 - Low Power Divisor Latch (Low) Register"]
    #[inline(always)]
    pub const fn lpdll(&self) -> &Lpdll {
        &self.lpdll
    }
    #[doc = "0x24 - Low Power Divisor Latch (High) Register"]
    #[inline(always)]
    pub const fn lpdlh(&self) -> &Lpdlh {
        &self.lpdlh
    }
    #[doc = "0x30 - Shadow Transmit Buffer Register"]
    #[inline(always)]
    pub const fn sthr(&self) -> &Sthr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(48).cast() }
    }
    #[doc = "0x30 - Shadow Receive Buffer Register"]
    #[inline(always)]
    pub const fn srbr(&self) -> &Srbr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(48).cast() }
    }
    #[doc = "0x70 - FIFO Access Register"]
    #[inline(always)]
    pub const fn far(&self) -> &Far {
        &self.far
    }
    #[doc = "0x74 - Transmit FIFO Read"]
    #[inline(always)]
    pub const fn tfr(&self) -> &Tfr {
        &self.tfr
    }
    #[doc = "0x78 - Receive FIFO Write"]
    #[inline(always)]
    pub const fn rfw(&self) -> &Rfw {
        &self.rfw
    }
    #[doc = "0x7c - UART Status Register"]
    #[inline(always)]
    pub const fn usr(&self) -> &Usr {
        &self.usr
    }
    #[doc = "0x80 - Transmit FIFO Level"]
    #[inline(always)]
    pub const fn tfl(&self) -> &Tfl {
        &self.tfl
    }
    #[doc = "0x84 - Receive FIFO Level"]
    #[inline(always)]
    pub const fn rfl(&self) -> &Rfl {
        &self.rfl
    }
    #[doc = "0x88 - Software Reset Register"]
    #[inline(always)]
    pub const fn srr(&self) -> &Srr {
        &self.srr
    }
    #[doc = "0x8c - Shadow Request to Send"]
    #[inline(always)]
    pub const fn srts(&self) -> &Srts {
        &self.srts
    }
    #[doc = "0x90 - Shadow Break Control Register"]
    #[inline(always)]
    pub const fn sbcr(&self) -> &Sbcr {
        &self.sbcr
    }
    #[doc = "0x94 - Shadow DMA Mode"]
    #[inline(always)]
    pub const fn sdmam(&self) -> &Sdmam {
        &self.sdmam
    }
    #[doc = "0x98 - Shadow FIFO Enable"]
    #[inline(always)]
    pub const fn sfe(&self) -> &Sfe {
        &self.sfe
    }
    #[doc = "0x9c - Shadow RCVR Trigger"]
    #[inline(always)]
    pub const fn srt(&self) -> &Srt {
        &self.srt
    }
    #[doc = "0xa0 - Shadow TX Empty Trigger"]
    #[inline(always)]
    pub const fn stet(&self) -> &Stet {
        &self.stet
    }
    #[doc = "0xa4 - Halt TX"]
    #[inline(always)]
    pub const fn htx(&self) -> &Htx {
        &self.htx
    }
    #[doc = "0xa8 - DMA Software Acknowledge"]
    #[inline(always)]
    pub const fn dmasa(&self) -> &Dmasa {
        &self.dmasa
    }
}
#[doc = "RBR_THR (rw) register accessor: Receive Buffer Register, Transmit Holding Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rbr_thr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rbr_thr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rbr_thr`] module"]
#[doc(alias = "RBR_THR")]
pub type RbrThr = crate::Reg<rbr_thr::RbrThrSpec>;
#[doc = "Receive Buffer Register, Transmit Holding Register"]
pub mod rbr_thr;
#[doc = "DLL (rw) register accessor: Divisor Latch Low byte Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dll::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dll::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dll`] module"]
#[doc(alias = "DLL")]
pub type Dll = crate::Reg<dll::DllSpec>;
#[doc = "Divisor Latch Low byte Register"]
pub mod dll;
#[doc = "IER (rw) register accessor: Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`] module"]
#[doc(alias = "IER")]
pub type Ier = crate::Reg<ier::IerSpec>;
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "DLH (rw) register accessor: Divisor Latch high byte Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dlh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dlh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dlh`] module"]
#[doc(alias = "DLH")]
pub type Dlh = crate::Reg<dlh::DlhSpec>;
#[doc = "Divisor Latch high byte Register"]
pub mod dlh;
#[doc = "IIR (r) register accessor: Interrupt Identification Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iir::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iir`] module"]
#[doc(alias = "IIR")]
pub type Iir = crate::Reg<iir::IirSpec>;
#[doc = "Interrupt Identification Register"]
pub mod iir;
#[doc = "FCR (w) register accessor: FIFO Control Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcr`] module"]
#[doc(alias = "FCR")]
pub type Fcr = crate::Reg<fcr::FcrSpec>;
#[doc = "FIFO Control Register"]
pub mod fcr;
#[doc = "LCR (rw) register accessor: Line Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcr`] module"]
#[doc(alias = "LCR")]
pub type Lcr = crate::Reg<lcr::LcrSpec>;
#[doc = "Line Control Register"]
pub mod lcr;
#[doc = "MCR (rw) register accessor: Modem Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcr`] module"]
#[doc(alias = "MCR")]
pub type Mcr = crate::Reg<mcr::McrSpec>;
#[doc = "Modem Control Register"]
pub mod mcr;
#[doc = "LSR (r) register accessor: Line Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lsr`] module"]
#[doc(alias = "LSR")]
pub type Lsr = crate::Reg<lsr::LsrSpec>;
#[doc = "Line Status Register"]
pub mod lsr;
#[doc = "MSR (r) register accessor: Modem Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`msr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msr`] module"]
#[doc(alias = "MSR")]
pub type Msr = crate::Reg<msr::MsrSpec>;
#[doc = "Modem Status Register"]
pub mod msr;
#[doc = "LPDLL (rw) register accessor: Low Power Divisor Latch (Low) Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lpdll::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpdll::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpdll`] module"]
#[doc(alias = "LPDLL")]
pub type Lpdll = crate::Reg<lpdll::LpdllSpec>;
#[doc = "Low Power Divisor Latch (Low) Register"]
pub mod lpdll;
#[doc = "LPDLH (rw) register accessor: Low Power Divisor Latch (High) Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lpdlh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpdlh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpdlh`] module"]
#[doc(alias = "LPDLH")]
pub type Lpdlh = crate::Reg<lpdlh::LpdlhSpec>;
#[doc = "Low Power Divisor Latch (High) Register"]
pub mod lpdlh;
#[doc = "SRBR (r) register accessor: Shadow Receive Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`srbr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srbr`] module"]
#[doc(alias = "SRBR")]
pub type Srbr = crate::Reg<srbr::SrbrSpec>;
#[doc = "Shadow Receive Buffer Register"]
pub mod srbr;
#[doc = "STHR (w) register accessor: Shadow Transmit Buffer Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sthr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sthr`] module"]
#[doc(alias = "STHR")]
pub type Sthr = crate::Reg<sthr::SthrSpec>;
#[doc = "Shadow Transmit Buffer Register"]
pub mod sthr;
#[doc = "FAR (rw) register accessor: FIFO Access Register\n\nYou can [`read`](crate::Reg::read) this register and get [`far::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`far::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@far`] module"]
#[doc(alias = "FAR")]
pub type Far = crate::Reg<far::FarSpec>;
#[doc = "FIFO Access Register"]
pub mod far;
#[doc = "TFR (rw) register accessor: Transmit FIFO Read\n\nYou can [`read`](crate::Reg::read) this register and get [`tfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tfr`] module"]
#[doc(alias = "TFR")]
pub type Tfr = crate::Reg<tfr::TfrSpec>;
#[doc = "Transmit FIFO Read"]
pub mod tfr;
#[doc = "RFW (rw) register accessor: Receive FIFO Write\n\nYou can [`read`](crate::Reg::read) this register and get [`rfw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfw`] module"]
#[doc(alias = "RFW")]
pub type Rfw = crate::Reg<rfw::RfwSpec>;
#[doc = "Receive FIFO Write"]
pub mod rfw;
#[doc = "USR (r) register accessor: UART Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usr`] module"]
#[doc(alias = "USR")]
pub type Usr = crate::Reg<usr::UsrSpec>;
#[doc = "UART Status Register"]
pub mod usr;
#[doc = "TFL (r) register accessor: Transmit FIFO Level\n\nYou can [`read`](crate::Reg::read) this register and get [`tfl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tfl`] module"]
#[doc(alias = "TFL")]
pub type Tfl = crate::Reg<tfl::TflSpec>;
#[doc = "Transmit FIFO Level"]
pub mod tfl;
#[doc = "RFL (r) register accessor: Receive FIFO Level\n\nYou can [`read`](crate::Reg::read) this register and get [`rfl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfl`] module"]
#[doc(alias = "RFL")]
pub type Rfl = crate::Reg<rfl::RflSpec>;
#[doc = "Receive FIFO Level"]
pub mod rfl;
#[doc = "SRR (rw) register accessor: Software Reset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`srr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srr`] module"]
#[doc(alias = "SRR")]
pub type Srr = crate::Reg<srr::SrrSpec>;
#[doc = "Software Reset Register"]
pub mod srr;
#[doc = "SRTS (rw) register accessor: Shadow Request to Send\n\nYou can [`read`](crate::Reg::read) this register and get [`srts::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srts`] module"]
#[doc(alias = "SRTS")]
pub type Srts = crate::Reg<srts::SrtsSpec>;
#[doc = "Shadow Request to Send"]
pub mod srts;
#[doc = "SBCR (rw) register accessor: Shadow Break Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sbcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbcr`] module"]
#[doc(alias = "SBCR")]
pub type Sbcr = crate::Reg<sbcr::SbcrSpec>;
#[doc = "Shadow Break Control Register"]
pub mod sbcr;
#[doc = "SDMAM (rw) register accessor: Shadow DMA Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`sdmam::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdmam::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmam`] module"]
#[doc(alias = "SDMAM")]
pub type Sdmam = crate::Reg<sdmam::SdmamSpec>;
#[doc = "Shadow DMA Mode"]
pub mod sdmam;
#[doc = "SFE (rw) register accessor: Shadow FIFO Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`sfe::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfe::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfe`] module"]
#[doc(alias = "SFE")]
pub type Sfe = crate::Reg<sfe::SfeSpec>;
#[doc = "Shadow FIFO Enable"]
pub mod sfe;
#[doc = "SRT (rw) register accessor: Shadow RCVR Trigger\n\nYou can [`read`](crate::Reg::read) this register and get [`srt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srt`] module"]
#[doc(alias = "SRT")]
pub type Srt = crate::Reg<srt::SrtSpec>;
#[doc = "Shadow RCVR Trigger"]
pub mod srt;
#[doc = "STET (rw) register accessor: Shadow TX Empty Trigger\n\nYou can [`read`](crate::Reg::read) this register and get [`stet::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stet::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stet`] module"]
#[doc(alias = "STET")]
pub type Stet = crate::Reg<stet::StetSpec>;
#[doc = "Shadow TX Empty Trigger"]
pub mod stet;
#[doc = "HTX (rw) register accessor: Halt TX\n\nYou can [`read`](crate::Reg::read) this register and get [`htx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`htx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@htx`] module"]
#[doc(alias = "HTX")]
pub type Htx = crate::Reg<htx::HtxSpec>;
#[doc = "Halt TX"]
pub mod htx;
#[doc = "DMASA (rw) register accessor: DMA Software Acknowledge\n\nYou can [`read`](crate::Reg::read) this register and get [`dmasa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmasa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmasa`] module"]
#[doc(alias = "DMASA")]
pub type Dmasa = crate::Reg<dmasa::DmasaSpec>;
#[doc = "DMA Software Acknowledge"]
pub mod dmasa;
