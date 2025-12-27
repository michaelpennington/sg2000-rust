#[doc = "Register `MTIMECMPH0` reader"]
pub type R = crate::R<Mtimecmph0Spec>;
#[doc = "Register `MTIMECMPH0` writer"]
pub type W = crate::W<Mtimecmph0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "M-mode Timer Compare High\n\nYou can [`read`](crate::Reg::read) this register and get [`mtimecmph0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtimecmph0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mtimecmph0Spec;
impl crate::RegisterSpec for Mtimecmph0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtimecmph0::R`](R) reader structure"]
impl crate::Readable for Mtimecmph0Spec {}
#[doc = "`write(|w| ..)` method takes [`mtimecmph0::W`](W) writer structure"]
impl crate::Writable for Mtimecmph0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MTIMECMPH0 to value 0xffff_ffff"]
impl crate::Resettable for Mtimecmph0Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
