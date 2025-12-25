#[doc = "Register `MSR` reader"]
pub type R = crate::R<MsrSpec>;
#[doc = "Field `DELTA_CTS` reader - Delta Clear to Send."]
pub type DeltaCtsR = crate::BitReader;
#[doc = "Field `CTS` reader - CTS"]
pub type CtsR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Delta Clear to Send."]
    #[inline(always)]
    pub fn delta_cts(&self) -> DeltaCtsR {
        DeltaCtsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - CTS"]
    #[inline(always)]
    pub fn cts(&self) -> CtsR {
        CtsR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "Modem Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`msr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MsrSpec;
impl crate::RegisterSpec for MsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msr::R`](R) reader structure"]
impl crate::Readable for MsrSpec {}
