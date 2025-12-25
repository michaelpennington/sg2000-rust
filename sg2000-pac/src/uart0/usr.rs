#[doc = "Register `USR` reader"]
pub type R = crate::R<UsrSpec>;
#[doc = "Field `BUSY` reader - "]
pub type BusyR = crate::BitReader;
#[doc = "Field `TX_FIFO_NOT_FULL` reader - Transmit FIFO Not Full"]
pub type TxFifoNotFullR = crate::BitReader;
#[doc = "Field `TX_FIFO_EMPTY` reader - Transmit FIFO Empty"]
pub type TxFifoEmptyR = crate::BitReader;
#[doc = "Field `RX_FIFO_NOT_EMPTY` reader - Receive FIFO Not EMPTY"]
pub type RxFifoNotEmptyR = crate::BitReader;
#[doc = "Field `RX_FIFO_FULL` reader - Receive FIFO Full"]
pub type RxFifoFullR = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit FIFO Not Full"]
    #[inline(always)]
    pub fn tx_fifo_not_full(&self) -> TxFifoNotFullR {
        TxFifoNotFullR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit FIFO Empty"]
    #[inline(always)]
    pub fn tx_fifo_empty(&self) -> TxFifoEmptyR {
        TxFifoEmptyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO Not EMPTY"]
    #[inline(always)]
    pub fn rx_fifo_not_empty(&self) -> RxFifoNotEmptyR {
        RxFifoNotEmptyR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO Full"]
    #[inline(always)]
    pub fn rx_fifo_full(&self) -> RxFifoFullR {
        RxFifoFullR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "UART Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsrSpec;
impl crate::RegisterSpec for UsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usr::R`](R) reader structure"]
impl crate::Readable for UsrSpec {}
