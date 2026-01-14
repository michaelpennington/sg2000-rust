#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    plic_prio: [PlicPrio; 1024],
    plic_ip: [PlicIp; 32],
    _reserved2: [u8; 0x0f80],
    plic_h0_mie: [PlicH0Mie; 32],
    plic_h0_sie: [PlicH0Sie; 32],
    plic_h1_mie: [PlicH1Mie; 32],
    plic_h1_sie: [PlicH1Sie; 32],
    _reserved6: [u8; 0x001f_ddfc],
    plic_per: PlicPer,
    plic_h0_mth: PlicH0Mth,
    plic_h0_mclaim: PlicH0Mclaim,
    _reserved9: [u8; 0x0ff8],
    plic_h0_sth: PlicH0Sth,
    plic_h0_sclaim: PlicH0Sclaim,
    _reserved11: [u8; 0x0ff8],
    plic_h1_mth: PlicH1Mth,
    plic_h1_mclaim: PlicH1Mclaim,
    _reserved13: [u8; 0x0ff8],
    plic_h1_sth: PlicH1Sth,
    plic_h1_sclaim: PlicH1Sclaim,
}
impl RegisterBlock {
    #[doc = "0x00..0x1000 - Interrupt Priority Register (1-1023)"]
    #[inline(always)]
    pub const fn plic_prio(&self, n: usize) -> &PlicPrio {
        &self.plic_prio[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x1000 - Interrupt Priority Register (1-1023)"]
    #[inline(always)]
    pub fn plic_prio_iter(&self) -> impl Iterator<Item = &PlicPrio> {
        self.plic_prio.iter()
    }
    #[doc = "0x1000..0x1080 - Interrupt Pending Register"]
    #[inline(always)]
    pub const fn plic_ip(&self, n: usize) -> &PlicIp {
        &self.plic_ip[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1000..0x1080 - Interrupt Pending Register"]
    #[inline(always)]
    pub fn plic_ip_iter(&self) -> impl Iterator<Item = &PlicIp> {
        self.plic_ip.iter()
    }
    #[doc = "0x2000..0x2080 - M-mode Interrupt Enable Register"]
    #[inline(always)]
    pub const fn plic_h0_mie(&self, n: usize) -> &PlicH0Mie {
        &self.plic_h0_mie[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2000..0x2080 - M-mode Interrupt Enable Register"]
    #[inline(always)]
    pub fn plic_h0_mie_iter(&self) -> impl Iterator<Item = &PlicH0Mie> {
        self.plic_h0_mie.iter()
    }
    #[doc = "0x2080..0x2100 - S-mode Interrupt Enable Register"]
    #[inline(always)]
    pub const fn plic_h0_sie(&self, n: usize) -> &PlicH0Sie {
        &self.plic_h0_sie[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2080..0x2100 - S-mode Interrupt Enable Register"]
    #[inline(always)]
    pub fn plic_h0_sie_iter(&self) -> impl Iterator<Item = &PlicH0Sie> {
        self.plic_h0_sie.iter()
    }
    #[doc = "0x2100..0x2180 - Hart 1 M-mode Interrupt Enable Register"]
    #[inline(always)]
    pub const fn plic_h1_mie(&self, n: usize) -> &PlicH1Mie {
        &self.plic_h1_mie[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2100..0x2180 - Hart 1 M-mode Interrupt Enable Register"]
    #[inline(always)]
    pub fn plic_h1_mie_iter(&self) -> impl Iterator<Item = &PlicH1Mie> {
        self.plic_h1_mie.iter()
    }
    #[doc = "0x2180..0x2200 - Hart 1 S-mode Interrupt Enable Register"]
    #[inline(always)]
    pub const fn plic_h1_sie(&self, n: usize) -> &PlicH1Sie {
        &self.plic_h1_sie[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2180..0x2200 - Hart 1 S-mode Interrupt Enable Register"]
    #[inline(always)]
    pub fn plic_h1_sie_iter(&self) -> impl Iterator<Item = &PlicH1Sie> {
        self.plic_h1_sie.iter()
    }
    #[doc = "0x1ffffc - PLIC Permission Control Register"]
    #[inline(always)]
    pub const fn plic_per(&self) -> &PlicPer {
        &self.plic_per
    }
    #[doc = "0x200000 - M-mode Interrupt Threshold Register"]
    #[inline(always)]
    pub const fn plic_h0_mth(&self) -> &PlicH0Mth {
        &self.plic_h0_mth
    }
    #[doc = "0x200004 - M-mode Interrupt Claim/Complete Register"]
    #[inline(always)]
    pub const fn plic_h0_mclaim(&self) -> &PlicH0Mclaim {
        &self.plic_h0_mclaim
    }
    #[doc = "0x201000 - S-mode Interrupt Threshold Register"]
    #[inline(always)]
    pub const fn plic_h0_sth(&self) -> &PlicH0Sth {
        &self.plic_h0_sth
    }
    #[doc = "0x201004 - S-mode Interrupt Claim/Complete Register"]
    #[inline(always)]
    pub const fn plic_h0_sclaim(&self) -> &PlicH0Sclaim {
        &self.plic_h0_sclaim
    }
    #[doc = "0x202000 - Hart 1 M-mode Interrupt Threshold Register"]
    #[inline(always)]
    pub const fn plic_h1_mth(&self) -> &PlicH1Mth {
        &self.plic_h1_mth
    }
    #[doc = "0x202004 - Hart 1 M-mode Interrupt Claim/Complete Register"]
    #[inline(always)]
    pub const fn plic_h1_mclaim(&self) -> &PlicH1Mclaim {
        &self.plic_h1_mclaim
    }
    #[doc = "0x203000 - Hart 1 S-mode Interrupt Threshold Register"]
    #[inline(always)]
    pub const fn plic_h1_sth(&self) -> &PlicH1Sth {
        &self.plic_h1_sth
    }
    #[doc = "0x203004 - Hart 1 S-mode Interrupt Claim/Complete Register"]
    #[inline(always)]
    pub const fn plic_h1_sclaim(&self) -> &PlicH1Sclaim {
        &self.plic_h1_sclaim
    }
}
#[doc = "PLIC_PRIO (rw) register accessor: Interrupt Priority Register (1-1023)\n\nYou can [`read`](crate::Reg::read) this register and get [`plic_prio::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plic_prio::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@plic_prio`] module"]
#[doc(alias = "PLIC_PRIO")]
pub type PlicPrio = crate::Reg<plic_prio::PlicPrioSpec>;
#[doc = "Interrupt Priority Register (1-1023)"]
pub mod plic_prio;
#[doc = "PLIC_IP (rw) register accessor: Interrupt Pending Register\n\nYou can [`read`](crate::Reg::read) this register and get [`plic_ip::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plic_ip::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@plic_ip`] module"]
#[doc(alias = "PLIC_IP")]
pub type PlicIp = crate::Reg<plic_ip::PlicIpSpec>;
#[doc = "Interrupt Pending Register"]
pub mod plic_ip;
#[doc = "PLIC_H0_MIE (rw) register accessor: M-mode Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`plic_h0_mie::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plic_h0_mie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@plic_h0_mie`] module"]
#[doc(alias = "PLIC_H0_MIE")]
pub type PlicH0Mie = crate::Reg<plic_h0_mie::PlicH0MieSpec>;
#[doc = "M-mode Interrupt Enable Register"]
pub mod plic_h0_mie;
#[doc = "PLIC_H0_SIE (rw) register accessor: S-mode Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`plic_h0_sie::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plic_h0_sie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@plic_h0_sie`] module"]
#[doc(alias = "PLIC_H0_SIE")]
pub type PlicH0Sie = crate::Reg<plic_h0_sie::PlicH0SieSpec>;
#[doc = "S-mode Interrupt Enable Register"]
pub mod plic_h0_sie;
#[doc = "PLIC_H1_MIE (rw) register accessor: Hart 1 M-mode Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`plic_h1_mie::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plic_h1_mie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@plic_h1_mie`] module"]
#[doc(alias = "PLIC_H1_MIE")]
pub type PlicH1Mie = crate::Reg<plic_h1_mie::PlicH1MieSpec>;
#[doc = "Hart 1 M-mode Interrupt Enable Register"]
pub mod plic_h1_mie;
#[doc = "PLIC_H1_SIE (rw) register accessor: Hart 1 S-mode Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`plic_h1_sie::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plic_h1_sie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@plic_h1_sie`] module"]
#[doc(alias = "PLIC_H1_SIE")]
pub type PlicH1Sie = crate::Reg<plic_h1_sie::PlicH1SieSpec>;
#[doc = "Hart 1 S-mode Interrupt Enable Register"]
pub mod plic_h1_sie;
#[doc = "PLIC_PER (rw) register accessor: PLIC Permission Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`plic_per::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plic_per::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@plic_per`] module"]
#[doc(alias = "PLIC_PER")]
pub type PlicPer = crate::Reg<plic_per::PlicPerSpec>;
#[doc = "PLIC Permission Control Register"]
pub mod plic_per;
#[doc = "PLIC_H0_MTH (rw) register accessor: M-mode Interrupt Threshold Register\n\nYou can [`read`](crate::Reg::read) this register and get [`plic_h0_mth::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plic_h0_mth::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@plic_h0_mth`] module"]
#[doc(alias = "PLIC_H0_MTH")]
pub type PlicH0Mth = crate::Reg<plic_h0_mth::PlicH0MthSpec>;
#[doc = "M-mode Interrupt Threshold Register"]
pub mod plic_h0_mth;
#[doc = "PLIC_H0_MCLAIM (rw) register accessor: M-mode Interrupt Claim/Complete Register\n\nYou can [`read`](crate::Reg::read) this register and get [`plic_h0_mclaim::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plic_h0_mclaim::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@plic_h0_mclaim`] module"]
#[doc(alias = "PLIC_H0_MCLAIM")]
pub type PlicH0Mclaim = crate::Reg<plic_h0_mclaim::PlicH0MclaimSpec>;
#[doc = "M-mode Interrupt Claim/Complete Register"]
pub mod plic_h0_mclaim;
#[doc = "PLIC_H0_STH (rw) register accessor: S-mode Interrupt Threshold Register\n\nYou can [`read`](crate::Reg::read) this register and get [`plic_h0_sth::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plic_h0_sth::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@plic_h0_sth`] module"]
#[doc(alias = "PLIC_H0_STH")]
pub type PlicH0Sth = crate::Reg<plic_h0_sth::PlicH0SthSpec>;
#[doc = "S-mode Interrupt Threshold Register"]
pub mod plic_h0_sth;
#[doc = "PLIC_H0_SCLAIM (rw) register accessor: S-mode Interrupt Claim/Complete Register\n\nYou can [`read`](crate::Reg::read) this register and get [`plic_h0_sclaim::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plic_h0_sclaim::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@plic_h0_sclaim`] module"]
#[doc(alias = "PLIC_H0_SCLAIM")]
pub type PlicH0Sclaim = crate::Reg<plic_h0_sclaim::PlicH0SclaimSpec>;
#[doc = "S-mode Interrupt Claim/Complete Register"]
pub mod plic_h0_sclaim;
#[doc = "PLIC_H1_MTH (rw) register accessor: Hart 1 M-mode Interrupt Threshold Register\n\nYou can [`read`](crate::Reg::read) this register and get [`plic_h1_mth::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plic_h1_mth::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@plic_h1_mth`] module"]
#[doc(alias = "PLIC_H1_MTH")]
pub type PlicH1Mth = crate::Reg<plic_h1_mth::PlicH1MthSpec>;
#[doc = "Hart 1 M-mode Interrupt Threshold Register"]
pub mod plic_h1_mth;
#[doc = "PLIC_H1_MCLAIM (rw) register accessor: Hart 1 M-mode Interrupt Claim/Complete Register\n\nYou can [`read`](crate::Reg::read) this register and get [`plic_h1_mclaim::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plic_h1_mclaim::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@plic_h1_mclaim`] module"]
#[doc(alias = "PLIC_H1_MCLAIM")]
pub type PlicH1Mclaim = crate::Reg<plic_h1_mclaim::PlicH1MclaimSpec>;
#[doc = "Hart 1 M-mode Interrupt Claim/Complete Register"]
pub mod plic_h1_mclaim;
#[doc = "PLIC_H1_STH (rw) register accessor: Hart 1 S-mode Interrupt Threshold Register\n\nYou can [`read`](crate::Reg::read) this register and get [`plic_h1_sth::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plic_h1_sth::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@plic_h1_sth`] module"]
#[doc(alias = "PLIC_H1_STH")]
pub type PlicH1Sth = crate::Reg<plic_h1_sth::PlicH1SthSpec>;
#[doc = "Hart 1 S-mode Interrupt Threshold Register"]
pub mod plic_h1_sth;
#[doc = "PLIC_H1_SCLAIM (rw) register accessor: Hart 1 S-mode Interrupt Claim/Complete Register\n\nYou can [`read`](crate::Reg::read) this register and get [`plic_h1_sclaim::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plic_h1_sclaim::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@plic_h1_sclaim`] module"]
#[doc(alias = "PLIC_H1_SCLAIM")]
pub type PlicH1Sclaim = crate::Reg<plic_h1_sclaim::PlicH1SclaimSpec>;
#[doc = "Hart 1 S-mode Interrupt Claim/Complete Register"]
pub mod plic_h1_sclaim;
