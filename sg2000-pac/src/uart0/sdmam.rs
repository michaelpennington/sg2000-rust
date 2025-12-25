#[doc = "Register `SDMAM` reader"]
pub type R = crate::R<SdmamSpec>;
#[doc = "Register `SDMAM` writer"]
pub type W = crate::W<SdmamSpec>;
#[doc = "Field `DMA_MODE` reader - This is a shadow register for the DMA mode bit (FCR\\[3\\])"]
pub type DmaModeR = crate::BitReader;
#[doc = "Field `DMA_MODE` writer - This is a shadow register for the DMA mode bit (FCR\\[3\\])"]
pub type DmaModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This is a shadow register for the DMA mode bit (FCR\\[3\\])"]
    #[inline(always)]
    pub fn dma_mode(&self) -> DmaModeR {
        DmaModeR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This is a shadow register for the DMA mode bit (FCR\\[3\\])"]
    #[inline(always)]
    pub fn dma_mode(&mut self) -> DmaModeW<'_, SdmamSpec> {
        DmaModeW::new(self, 0)
    }
}
#[doc = "Shadow DMA Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`sdmam::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdmam::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdmamSpec;
impl crate::RegisterSpec for SdmamSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdmam::R`](R) reader structure"]
impl crate::Readable for SdmamSpec {}
#[doc = "`write(|w| ..)` method takes [`sdmam::W`](W) writer structure"]
impl crate::Writable for SdmamSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SDMAM to value 0"]
impl crate::Resettable for SdmamSpec {}
