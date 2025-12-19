#[doc = "Register `EXT` reader"]
pub type R = crate::R<ExtSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "External Port Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ext::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExtSpec;
impl crate::RegisterSpec for ExtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ext::R`](R) reader structure"]
impl crate::Readable for ExtSpec {}
