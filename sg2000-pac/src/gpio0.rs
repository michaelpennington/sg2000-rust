#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dr: Dr,
    ddr: Ddr,
    _reserved2: [u8; 0x28],
    inten: Inten,
    intmask: Intmask,
    inttype_level: InttypeLevel,
    int_polarity: IntPolarity,
    intstatus: Intstatus,
    raw_intstatus: RawIntstatus,
    debounce: Debounce,
    eoi: Eoi,
    ext: Ext,
    _reserved11: [u8; 0x0c],
    ls_sync: LsSync,
}
impl RegisterBlock {
    #[doc = "0x00 - Data Register"]
    #[inline(always)]
    pub const fn dr(&self) -> &Dr {
        &self.dr
    }
    #[doc = "0x04 - Data Direction Register"]
    #[inline(always)]
    pub const fn ddr(&self) -> &Ddr {
        &self.ddr
    }
    #[doc = "0x30 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn inten(&self) -> &Inten {
        &self.inten
    }
    #[doc = "0x34 - Interrupt Mask Register"]
    #[inline(always)]
    pub const fn intmask(&self) -> &Intmask {
        &self.intmask
    }
    #[doc = "0x38 - Interrupt Level Register"]
    #[inline(always)]
    pub const fn inttype_level(&self) -> &InttypeLevel {
        &self.inttype_level
    }
    #[doc = "0x3c - Interrupt Polarity Register"]
    #[inline(always)]
    pub const fn int_polarity(&self) -> &IntPolarity {
        &self.int_polarity
    }
    #[doc = "0x40 - Interrupt Status Register"]
    #[inline(always)]
    pub const fn intstatus(&self) -> &Intstatus {
        &self.intstatus
    }
    #[doc = "0x44 - Raw (pre-masking) Interrupt Status Register"]
    #[inline(always)]
    pub const fn raw_intstatus(&self) -> &RawIntstatus {
        &self.raw_intstatus
    }
    #[doc = "0x48 - Debounce Enable Register"]
    #[inline(always)]
    pub const fn debounce(&self) -> &Debounce {
        &self.debounce
    }
    #[doc = "0x4c - Clear Interrupt Register"]
    #[inline(always)]
    pub const fn eoi(&self) -> &Eoi {
        &self.eoi
    }
    #[doc = "0x50 - External Port Register"]
    #[inline(always)]
    pub const fn ext(&self) -> &Ext {
        &self.ext
    }
    #[doc = "0x60 - Level-Sensitive Synchronization Enable Register"]
    #[inline(always)]
    pub const fn ls_sync(&self) -> &LsSync {
        &self.ls_sync
    }
}
#[doc = "DR (rw) register accessor: Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr`] module"]
#[doc(alias = "DR")]
pub type Dr = crate::Reg<dr::DrSpec>;
#[doc = "Data Register"]
pub mod dr;
#[doc = "DDR (rw) register accessor: Data Direction Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr`] module"]
#[doc(alias = "DDR")]
pub type Ddr = crate::Reg<ddr::DdrSpec>;
#[doc = "Data Direction Register"]
pub mod ddr;
#[doc = "INTEN (rw) register accessor: Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten`] module"]
#[doc(alias = "INTEN")]
pub type Inten = crate::Reg<inten::IntenSpec>;
#[doc = "Interrupt Enable Register"]
pub mod inten;
#[doc = "INTMASK (rw) register accessor: Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intmask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intmask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intmask`] module"]
#[doc(alias = "INTMASK")]
pub type Intmask = crate::Reg<intmask::IntmaskSpec>;
#[doc = "Interrupt Mask Register"]
pub mod intmask;
#[doc = "INTTYPE_LEVEL (rw) register accessor: Interrupt Level Register\n\nYou can [`read`](crate::Reg::read) this register and get [`inttype_level::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inttype_level::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inttype_level`] module"]
#[doc(alias = "INTTYPE_LEVEL")]
pub type InttypeLevel = crate::Reg<inttype_level::InttypeLevelSpec>;
#[doc = "Interrupt Level Register"]
pub mod inttype_level;
#[doc = "INT_POLARITY (rw) register accessor: Interrupt Polarity Register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_polarity::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_polarity::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_polarity`] module"]
#[doc(alias = "INT_POLARITY")]
pub type IntPolarity = crate::Reg<int_polarity::IntPolaritySpec>;
#[doc = "Interrupt Polarity Register"]
pub mod int_polarity;
#[doc = "INTSTATUS (r) register accessor: Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intstatus`] module"]
#[doc(alias = "INTSTATUS")]
pub type Intstatus = crate::Reg<intstatus::IntstatusSpec>;
#[doc = "Interrupt Status Register"]
pub mod intstatus;
#[doc = "RAW_INTSTATUS (r) register accessor: Raw (pre-masking) Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`raw_intstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@raw_intstatus`] module"]
#[doc(alias = "RAW_INTSTATUS")]
pub type RawIntstatus = crate::Reg<raw_intstatus::RawIntstatusSpec>;
#[doc = "Raw (pre-masking) Interrupt Status Register"]
pub mod raw_intstatus;
#[doc = "DEBOUNCE (rw) register accessor: Debounce Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`debounce::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debounce::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debounce`] module"]
#[doc(alias = "DEBOUNCE")]
pub type Debounce = crate::Reg<debounce::DebounceSpec>;
#[doc = "Debounce Enable Register"]
pub mod debounce;
#[doc = "EOI (rw) register accessor: Clear Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`eoi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eoi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eoi`] module"]
#[doc(alias = "EOI")]
pub type Eoi = crate::Reg<eoi::EoiSpec>;
#[doc = "Clear Interrupt Register"]
pub mod eoi;
#[doc = "EXT (r) register accessor: External Port Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ext::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext`] module"]
#[doc(alias = "EXT")]
pub type Ext = crate::Reg<ext::ExtSpec>;
#[doc = "External Port Register"]
pub mod ext;
#[doc = "LS_SYNC (rw) register accessor: Level-Sensitive Synchronization Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ls_sync::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ls_sync::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ls_sync`] module"]
#[doc(alias = "LS_SYNC")]
pub type LsSync = crate::Reg<ls_sync::LsSyncSpec>;
#[doc = "Level-Sensitive Synchronization Enable Register"]
pub mod ls_sync;
