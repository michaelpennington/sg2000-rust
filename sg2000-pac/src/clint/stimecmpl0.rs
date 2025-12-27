#[doc = "Register `STIMECMPL0` reader"]
pub type R = crate::R<Stimecmpl0Spec>;
#[doc = "Register `STIMECMPL0` writer"]
pub type W = crate::W<Stimecmpl0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "S-mode Timer Compare Low\n\nYou can [`read`](crate::Reg::read) this register and get [`stimecmpl0::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stimecmpl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Stimecmpl0Spec;
impl crate::RegisterSpec for Stimecmpl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stimecmpl0::R`](R) reader structure"]
impl crate::Readable for Stimecmpl0Spec {}
#[doc = "`write(|w| ..)` method takes [`stimecmpl0::W`](W) writer structure"]
impl crate::Writable for Stimecmpl0Spec {
    type Safety = crate::Unsafe;
}
