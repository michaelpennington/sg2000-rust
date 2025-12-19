#[doc = "Register `INTMASK` reader"]
pub type R = crate::R<IntmaskSpec>;
#[doc = "Register `INTMASK` writer"]
pub type W = crate::W<IntmaskSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intmask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intmask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntmaskSpec;
impl crate::RegisterSpec for IntmaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intmask::R`](R) reader structure"]
impl crate::Readable for IntmaskSpec {}
#[doc = "`write(|w| ..)` method takes [`intmask::W`](W) writer structure"]
impl crate::Writable for IntmaskSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets INTMASK to value 0"]
impl crate::Resettable for IntmaskSpec {}
