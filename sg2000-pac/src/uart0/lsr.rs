#[doc = "Register `LSR` reader"]
pub type R = crate::R<LsrSpec>;
#[doc = "Field `Data Ready Bit` reader - "]
pub type DataReadyBitR = crate::BitReader;
#[doc = "Field `Overrun Error Bit` reader - "]
pub type OverrunErrorBitR = crate::BitReader;
#[doc = "Field `Parity Error Bit` reader - "]
pub type ParityErrorBitR = crate::BitReader;
#[doc = "Field `Framing Error Bit` reader - "]
pub type FramingErrorBitR = crate::BitReader;
#[doc = "Field `Break Interrupt Bit` reader - "]
pub type BreakInterruptBitR = crate::BitReader;
#[doc = "Field `Transmit Holding Register Empty Bit` reader - "]
pub type TransmitHoldingRegisterEmptyBitR = crate::BitReader;
#[doc = "Field `Transmitter Empty Bit` reader - "]
pub type TransmitterEmptyBitR = crate::BitReader;
#[doc = "Field `Receiver FIFO Error Bit` reader - "]
pub type ReceiverFifoerrorBitR = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn data_ready_bit(&self) -> DataReadyBitR {
        DataReadyBitR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn overrun_error_bit(&self) -> OverrunErrorBitR {
        OverrunErrorBitR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn parity_error_bit(&self) -> ParityErrorBitR {
        ParityErrorBitR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn framing_error_bit(&self) -> FramingErrorBitR {
        FramingErrorBitR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn break_interrupt_bit(&self) -> BreakInterruptBitR {
        BreakInterruptBitR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn transmit_holding_register_empty_bit(&self) -> TransmitHoldingRegisterEmptyBitR {
        TransmitHoldingRegisterEmptyBitR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn transmitter_empty_bit(&self) -> TransmitterEmptyBitR {
        TransmitterEmptyBitR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn receiver_fifoerror_bit(&self) -> ReceiverFifoerrorBitR {
        ReceiverFifoerrorBitR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Line Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LsrSpec;
impl crate::RegisterSpec for LsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lsr::R`](R) reader structure"]
impl crate::Readable for LsrSpec {}
