#[doc = "Register `RFL` reader"]
pub type R = crate::R<RflSpec>;
#[doc = "Field `RX_FIFO_LVL` reader - This indicates the number of data entries in the receive FIFO"]
pub type RxFifoLvlR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - This indicates the number of data entries in the receive FIFO"]
    #[inline(always)]
    pub fn rx_fifo_lvl(&self) -> RxFifoLvlR {
        RxFifoLvlR::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "Receive FIFO Level\n\nYou can [`read`](crate::Reg::read) this register and get [`rfl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RflSpec;
impl crate::RegisterSpec for RflSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfl::R`](R) reader structure"]
impl crate::Readable for RflSpec {}
