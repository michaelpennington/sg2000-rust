#[doc = "Register `MTIMECMPL0` reader"]
pub type R = crate::R<Mtimecmpl0Spec>;
#[doc = "Register `MTIMECMPL0` writer"]
pub type W = crate::W<Mtimecmpl0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "M-mode Timer Compare Low\n\nYou can [`read`](crate::Reg::read) this register and get [`mtimecmpl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtimecmpl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mtimecmpl0Spec;
impl crate::RegisterSpec for Mtimecmpl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtimecmpl0::R`](R) reader structure"]
impl crate::Readable for Mtimecmpl0Spec {}
#[doc = "`write(|w| ..)` method takes [`mtimecmpl0::W`](W) writer structure"]
impl crate::Writable for Mtimecmpl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MTIMECMPL0 to value 0xffff_ffff"]
impl crate::Resettable for Mtimecmpl0Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
