#[doc = "Register `mtimecmp_high` reader"]
pub type R = crate::R<MtimecmpHighSpec>;
#[doc = "Register `mtimecmp_high` writer"]
pub type W = crate::W<MtimecmpHighSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Low Bits Of Mtimecmp\n\nYou can [`read`](crate::Reg::read) this register and get [`mtimecmp_high::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtimecmp_high::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MtimecmpHighSpec;
impl crate::RegisterSpec for MtimecmpHighSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtimecmp_high::R`](R) reader structure"]
impl crate::Readable for MtimecmpHighSpec {}
#[doc = "`write(|w| ..)` method takes [`mtimecmp_high::W`](W) writer structure"]
impl crate::Writable for MtimecmpHighSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets mtimecmp_high to value 0"]
impl crate::Resettable for MtimecmpHighSpec {}
