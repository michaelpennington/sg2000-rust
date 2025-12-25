#[doc = "Register `DMASA` reader"]
pub type R = crate::R<DmasaSpec>;
#[doc = "Register `DMASA` writer"]
pub type W = crate::W<DmasaSpec>;
#[doc = "Field `DMA_ACK` reader - This register is used to perform a DMA software acknowledge if a transfer needs to be terminated due to an error condition."]
pub type DmaAckR = crate::BitReader;
#[doc = "Field `DMA_ACK` writer - This register is used to perform a DMA software acknowledge if a transfer needs to be terminated due to an error condition."]
pub type DmaAckW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This register is used to perform a DMA software acknowledge if a transfer needs to be terminated due to an error condition."]
    #[inline(always)]
    pub fn dma_ack(&self) -> DmaAckR {
        DmaAckR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This register is used to perform a DMA software acknowledge if a transfer needs to be terminated due to an error condition."]
    #[inline(always)]
    pub fn dma_ack(&mut self) -> DmaAckW<'_, DmasaSpec> {
        DmaAckW::new(self, 0)
    }
}
#[doc = "DMA Software Acknowledge\n\nYou can [`read`](crate::Reg::read) this register and get [`dmasa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmasa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmasaSpec;
impl crate::RegisterSpec for DmasaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmasa::R`](R) reader structure"]
impl crate::Readable for DmasaSpec {}
#[doc = "`write(|w| ..)` method takes [`dmasa::W`](W) writer structure"]
impl crate::Writable for DmasaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMASA to value 0"]
impl crate::Resettable for DmasaSpec {}
