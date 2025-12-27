#[doc = "Register `STIMECMPH0` reader"]
pub type R = crate::R<Stimecmph0Spec>;
#[doc = "Register `STIMECMPH0` writer"]
pub type W = crate::W<Stimecmph0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "S-mode Timer Compare High\n\nYou can [`read`](crate::Reg::read) this register and get [`stimecmph0::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stimecmph0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Stimecmph0Spec;
impl crate::RegisterSpec for Stimecmph0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stimecmph0::R`](R) reader structure"]
impl crate::Readable for Stimecmph0Spec {}
#[doc = "`write(|w| ..)` method takes [`stimecmph0::W`](W) writer structure"]
impl crate::Writable for Stimecmph0Spec {
    type Safety = crate::Unsafe;
}
