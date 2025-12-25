#[doc = "Register `HTX` reader"]
pub type R = crate::R<HtxSpec>;
#[doc = "Register `HTX` writer"]
pub type W = crate::W<HtxSpec>;
#[doc = "Field `HALT_TX` reader - This register is used to halt transmissions for testing."]
pub type HaltTxR = crate::BitReader;
#[doc = "Field `HALT_TX` writer - This register is used to halt transmissions for testing."]
pub type HaltTxW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This register is used to halt transmissions for testing."]
    #[inline(always)]
    pub fn halt_tx(&self) -> HaltTxR {
        HaltTxR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This register is used to halt transmissions for testing."]
    #[inline(always)]
    pub fn halt_tx(&mut self) -> HaltTxW<'_, HtxSpec> {
        HaltTxW::new(self, 0)
    }
}
#[doc = "Halt TX\n\nYou can [`read`](crate::Reg::read) this register and get [`htx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`htx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HtxSpec;
impl crate::RegisterSpec for HtxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`htx::R`](R) reader structure"]
impl crate::Readable for HtxSpec {}
#[doc = "`write(|w| ..)` method takes [`htx::W`](W) writer structure"]
impl crate::Writable for HtxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HTX to value 0"]
impl crate::Resettable for HtxSpec {}
