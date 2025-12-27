#[doc = "Register `MSIP0` reader"]
pub type R = crate::R<Msip0Spec>;
#[doc = "Register `MSIP0` writer"]
pub type W = crate::W<Msip0Spec>;
#[doc = "Field `MSIP` reader - Machine Software Interrupt Pending"]
pub type MsipR = crate::BitReader;
#[doc = "Field `MSIP` writer - Machine Software Interrupt Pending"]
pub type MsipW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Machine Software Interrupt Pending"]
    #[inline(always)]
    pub fn msip(&self) -> MsipR {
        MsipR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Machine Software Interrupt Pending"]
    #[inline(always)]
    pub fn msip(&mut self) -> MsipW<'_, Msip0Spec> {
        MsipW::new(self, 0)
    }
}
#[doc = "M-mode Software Interrupt Pending\n\nYou can [`read`](crate::Reg::read) this register and get [`msip0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`msip0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Msip0Spec;
impl crate::RegisterSpec for Msip0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msip0::R`](R) reader structure"]
impl crate::Readable for Msip0Spec {}
#[doc = "`write(|w| ..)` method takes [`msip0::W`](W) writer structure"]
impl crate::Writable for Msip0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MSIP0 to value 0"]
impl crate::Resettable for Msip0Spec {}
