#[doc = "Register `TFR` reader"]
pub type R = crate::R<TfrSpec>;
#[doc = "Register `TFR` writer"]
pub type W = crate::W<TfrSpec>;
#[doc = "Field `TX_FIFO_READ` reader - This bits are only valid when FIFO access mode is enabled."]
pub type TxFifoReadR = crate::FieldReader;
#[doc = "Field `TX_FIFO_READ` writer - This bits are only valid when FIFO access mode is enabled."]
pub type TxFifoReadW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - This bits are only valid when FIFO access mode is enabled."]
    #[inline(always)]
    pub fn tx_fifo_read(&self) -> TxFifoReadR {
        TxFifoReadR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - This bits are only valid when FIFO access mode is enabled."]
    #[inline(always)]
    pub fn tx_fifo_read(&mut self) -> TxFifoReadW<'_, TfrSpec> {
        TxFifoReadW::new(self, 0)
    }
}
#[doc = "Transmit FIFO Read\n\nYou can [`read`](crate::Reg::read) this register and get [`tfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TfrSpec;
impl crate::RegisterSpec for TfrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tfr::R`](R) reader structure"]
impl crate::Readable for TfrSpec {}
#[doc = "`write(|w| ..)` method takes [`tfr::W`](W) writer structure"]
impl crate::Writable for TfrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TFR to value 0"]
impl crate::Resettable for TfrSpec {}
