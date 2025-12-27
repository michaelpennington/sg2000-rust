#[doc = "Register `SSIP0` reader"]
pub type R = crate::R<Ssip0Spec>;
#[doc = "Register `SSIP0` writer"]
pub type W = crate::W<Ssip0Spec>;
#[doc = "Field `SSIP` reader - Supervisor Software Interrupt Pending"]
pub type SsipR = crate::BitReader;
#[doc = "Field `SSIP` writer - Supervisor Software Interrupt Pending"]
pub type SsipW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Supervisor Software Interrupt Pending"]
    #[inline(always)]
    pub fn ssip(&self) -> SsipR {
        SsipR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Supervisor Software Interrupt Pending"]
    #[inline(always)]
    pub fn ssip(&mut self) -> SsipW<'_, Ssip0Spec> {
        SsipW::new(self, 0)
    }
}
#[doc = "S-mode Software Interrupt Pending\n\nYou can [`read`](crate::Reg::read) this register and get [`ssip0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssip0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ssip0Spec;
impl crate::RegisterSpec for Ssip0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ssip0::R`](R) reader structure"]
impl crate::Readable for Ssip0Spec {}
#[doc = "`write(|w| ..)` method takes [`ssip0::W`](W) writer structure"]
impl crate::Writable for Ssip0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SSIP0 to value 0"]
impl crate::Resettable for Ssip0Spec {}
