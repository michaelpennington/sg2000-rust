#[repr(C)]
#[doc = "Interrupt Enable Registers"]
#[doc(alias = "enable")]
pub struct Enable {
    bits: [Bits; 32],
}
impl Enable {
    #[doc = "0x00..0x80 - Interrupt Enable Bits"]
    #[inline(always)]
    pub const fn bits(&self, n: usize) -> &Bits {
        &self.bits[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x80 - Interrupt Enable Bits"]
    #[inline(always)]
    pub fn bits_iter(&self) -> impl Iterator<Item = &Bits> {
        self.bits.iter()
    }
}
#[doc = "bits (rw) register accessor: Interrupt Enable Bits\n\nYou can [`read`](crate::Reg::read) this register and get [`bits::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bits::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bits`] module"]
#[doc(alias = "bits")]
pub type Bits = crate::Reg<bits::BitsSpec>;
#[doc = "Interrupt Enable Bits"]
pub mod bits;
