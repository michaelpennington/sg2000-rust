#[doc = "Register `TFL` reader"]
pub type R = crate::R<TflSpec>;
#[doc = "Field `TX_FIFO_LVL` reader - This indicates the number of data entries in the transmit FIFO"]
pub type TxFifoLvlR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - This indicates the number of data entries in the transmit FIFO"]
    #[inline(always)]
    pub fn tx_fifo_lvl(&self) -> TxFifoLvlR {
        TxFifoLvlR::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "Transmit FIFO Level\n\nYou can [`read`](crate::Reg::read) this register and get [`tfl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TflSpec;
impl crate::RegisterSpec for TflSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tfl::R`](R) reader structure"]
impl crate::Readable for TflSpec {}
