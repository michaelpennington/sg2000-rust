#[doc = "Register `RAW_INTSTATUS` reader"]
pub type R = crate::R<RawIntstatusSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Raw (pre-masking) Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`raw_intstatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RawIntstatusSpec;
impl crate::RegisterSpec for RawIntstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`raw_intstatus::R`](R) reader structure"]
impl crate::Readable for RawIntstatusSpec {}
