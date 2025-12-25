#[doc = "Register `LSR` reader"]
pub type R = crate::R<LsrSpec>;
#[doc = "Field `DATA_READY` reader - Data Ready Bit"]
pub type DataReadyR = crate::BitReader;
#[doc = "Field `OVERRUN_ERR` reader - Overrun Error Bit"]
pub type OverrunErrR = crate::BitReader;
#[doc = "Field `PARITY_ERR` reader - Parity Error Bit"]
pub type ParityErrR = crate::BitReader;
#[doc = "Field `FRAMING_ERR` reader - Framing Error Bit"]
pub type FramingErrR = crate::BitReader;
#[doc = "Field `BREAK_INT` reader - Break Interrupt Bit"]
pub type BreakIntR = crate::BitReader;
#[doc = "Field `TX_HOLD_EMPTY` reader - Transmit Holding Register Empty Bit"]
pub type TxHoldEmptyR = crate::BitReader;
#[doc = "Field `TX_EMPTY` reader - Transmitter Empty Bit"]
pub type TxEmptyR = crate::BitReader;
#[doc = "Field `RX_FIFO_ERR` reader - Receiver FIFO Error Bit"]
pub type RxFifoErrR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Data Ready Bit"]
    #[inline(always)]
    pub fn data_ready(&self) -> DataReadyR {
        DataReadyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Overrun Error Bit"]
    #[inline(always)]
    pub fn overrun_err(&self) -> OverrunErrR {
        OverrunErrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Parity Error Bit"]
    #[inline(always)]
    pub fn parity_err(&self) -> ParityErrR {
        ParityErrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Framing Error Bit"]
    #[inline(always)]
    pub fn framing_err(&self) -> FramingErrR {
        FramingErrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Break Interrupt Bit"]
    #[inline(always)]
    pub fn break_int(&self) -> BreakIntR {
        BreakIntR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit Holding Register Empty Bit"]
    #[inline(always)]
    pub fn tx_hold_empty(&self) -> TxHoldEmptyR {
        TxHoldEmptyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmitter Empty Bit"]
    #[inline(always)]
    pub fn tx_empty(&self) -> TxEmptyR {
        TxEmptyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receiver FIFO Error Bit"]
    #[inline(always)]
    pub fn rx_fifo_err(&self) -> RxFifoErrR {
        RxFifoErrR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Line Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LsrSpec;
impl crate::RegisterSpec for LsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lsr::R`](R) reader structure"]
impl crate::Readable for LsrSpec {}
