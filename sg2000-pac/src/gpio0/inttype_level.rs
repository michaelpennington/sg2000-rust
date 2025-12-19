#[doc = "Register `INTTYPE_LEVEL` reader"]
pub type R = crate::R<InttypeLevelSpec>;
#[doc = "Register `INTTYPE_LEVEL` writer"]
pub type W = crate::W<InttypeLevelSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Interrupt Level Register\n\nYou can [`read`](crate::Reg::read) this register and get [`inttype_level::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inttype_level::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InttypeLevelSpec;
impl crate::RegisterSpec for InttypeLevelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inttype_level::R`](R) reader structure"]
impl crate::Readable for InttypeLevelSpec {}
#[doc = "`write(|w| ..)` method takes [`inttype_level::W`](W) writer structure"]
impl crate::Writable for InttypeLevelSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets INTTYPE_LEVEL to value 0"]
impl crate::Resettable for InttypeLevelSpec {}
