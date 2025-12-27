#[doc = "Register `PLIC_PRIO[%s]` reader"]
pub type R = crate::R<PlicPrioSpec>;
#[doc = "Register `PLIC_PRIO[%s]` writer"]
pub type W = crate::W<PlicPrioSpec>;
#[doc = "Field `PRIO` reader - Interrupt Priority (0-31)"]
pub type PrioR = crate::FieldReader;
#[doc = "Field `PRIO` writer - Interrupt Priority (0-31)"]
pub type PrioW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Interrupt Priority (0-31)"]
    #[inline(always)]
    pub fn prio(&self) -> PrioR {
        PrioR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Interrupt Priority (0-31)"]
    #[inline(always)]
    pub fn prio(&mut self) -> PrioW<'_, PlicPrioSpec> {
        PrioW::new(self, 0)
    }
}
#[doc = "Interrupt Priority Register (1-1023)\n\nYou can [`read`](crate::Reg::read) this register and get [`plic_prio::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plic_prio::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PlicPrioSpec;
impl crate::RegisterSpec for PlicPrioSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`plic_prio::R`](R) reader structure"]
impl crate::Readable for PlicPrioSpec {}
#[doc = "`write(|w| ..)` method takes [`plic_prio::W`](W) writer structure"]
impl crate::Writable for PlicPrioSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PLIC_PRIO[%s] to value 0"]
impl crate::Resettable for PlicPrioSpec {}
