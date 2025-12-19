#[doc = "Register `mtimecmp_low` reader"]
pub type R = crate::R<MtimecmpLowSpec>;
#[doc = "Register `mtimecmp_low` writer"]
pub type W = crate::W<MtimecmpLowSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Low Bits Of Mtimecmp\n\nYou can [`read`](crate::Reg::read) this register and get [`mtimecmp_low::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtimecmp_low::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MtimecmpLowSpec;
impl crate::RegisterSpec for MtimecmpLowSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtimecmp_low::R`](R) reader structure"]
impl crate::Readable for MtimecmpLowSpec {}
#[doc = "`write(|w| ..)` method takes [`mtimecmp_low::W`](W) writer structure"]
impl crate::Writable for MtimecmpLowSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets mtimecmp_low to value 0"]
impl crate::Resettable for MtimecmpLowSpec {}
