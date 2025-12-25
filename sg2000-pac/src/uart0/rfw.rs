#[doc = "Register `RFW` reader"]
pub type R = crate::R<RfwSpec>;
#[doc = "Register `RFW` writer"]
pub type W = crate::W<RfwSpec>;
#[doc = "Field `RX_FIFO_WRITE` reader - Receive FIFO Write Data"]
pub type RxFifoWriteR = crate::FieldReader;
#[doc = "Field `RX_FIFO_WRITE` writer - Receive FIFO Write Data"]
pub type RxFifoWriteW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RX_FIFO_PARITY_ERR` reader - Receive FIFO Parity Error"]
pub type RxFifoParityErrR = crate::BitReader;
#[doc = "Field `RX_FIFO_PARITY_ERR` writer - Receive FIFO Parity Error"]
pub type RxFifoParityErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_FIFO_FRAMING_ERR` reader - Receive FIFO Framing Error"]
pub type RxFifoFramingErrR = crate::BitReader;
#[doc = "Field `RX_FIFO_FRAMING_ERR` writer - Receive FIFO Framing Error"]
pub type RxFifoFramingErrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Receive FIFO Write Data"]
    #[inline(always)]
    pub fn rx_fifo_write(&self) -> RxFifoWriteR {
        RxFifoWriteR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Receive FIFO Parity Error"]
    #[inline(always)]
    pub fn rx_fifo_parity_err(&self) -> RxFifoParityErrR {
        RxFifoParityErrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Receive FIFO Framing Error"]
    #[inline(always)]
    pub fn rx_fifo_framing_err(&self) -> RxFifoFramingErrR {
        RxFifoFramingErrR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Receive FIFO Write Data"]
    #[inline(always)]
    pub fn rx_fifo_write(&mut self) -> RxFifoWriteW<'_, RfwSpec> {
        RxFifoWriteW::new(self, 0)
    }
    #[doc = "Bit 8 - Receive FIFO Parity Error"]
    #[inline(always)]
    pub fn rx_fifo_parity_err(&mut self) -> RxFifoParityErrW<'_, RfwSpec> {
        RxFifoParityErrW::new(self, 8)
    }
    #[doc = "Bit 9 - Receive FIFO Framing Error"]
    #[inline(always)]
    pub fn rx_fifo_framing_err(&mut self) -> RxFifoFramingErrW<'_, RfwSpec> {
        RxFifoFramingErrW::new(self, 9)
    }
}
#[doc = "Receive FIFO Write\n\nYou can [`read`](crate::Reg::read) this register and get [`rfw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfwSpec;
impl crate::RegisterSpec for RfwSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfw::R`](R) reader structure"]
impl crate::Readable for RfwSpec {}
#[doc = "`write(|w| ..)` method takes [`rfw::W`](W) writer structure"]
impl crate::Writable for RfwSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RFW to value 0"]
impl crate::Resettable for RfwSpec {}
