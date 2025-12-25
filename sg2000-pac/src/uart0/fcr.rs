#[doc = "Register `FCR` writer"]
pub type W = crate::W<FcrSpec>;
#[doc = "Field `FIFO_EN` writer - FIFO Enable"]
pub type FifoEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFIFO_RST` writer - Receiver FIFO Reset"]
pub type RxfifoRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFIFO_RST` writer - Transmit FIFO Reset"]
pub type TxfifoRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "DMA Mode Select"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmaMode {
    #[doc = "0: Single DMA data transfers"]
    Mode0 = 0,
    #[doc = "1: Multi DMA data transfers (continuous)"]
    Mode1 = 1,
}
impl From<DmaMode> for bool {
    #[inline(always)]
    fn from(variant: DmaMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA_MODE` writer - DMA Mode Select"]
pub type DmaModeW<'a, REG> = crate::BitWriter<'a, REG, DmaMode>;
impl<'a, REG> DmaModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Single DMA data transfers"]
    #[inline(always)]
    pub fn mode0(self) -> &'a mut crate::W<REG> {
        self.variant(DmaMode::Mode0)
    }
    #[doc = "Multi DMA data transfers (continuous)"]
    #[inline(always)]
    pub fn mode1(self) -> &'a mut crate::W<REG> {
        self.variant(DmaMode::Mode1)
    }
}
#[doc = "TX Empty Trigger Level"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TxEmptyTrig {
    #[doc = "0: FIFO empty"]
    Empty = 0,
    #[doc = "1: 2 characters in FIFO"]
    Chars2 = 1,
    #[doc = "2: FIFO 1/4 full"]
    Quarter = 2,
    #[doc = "3: FIFO 1/2 full"]
    Half = 3,
}
impl From<TxEmptyTrig> for u8 {
    #[inline(always)]
    fn from(variant: TxEmptyTrig) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TxEmptyTrig {
    type Ux = u8;
}
impl crate::IsEnum for TxEmptyTrig {}
#[doc = "Field `TX_EMPTY_TRIG` writer - TX Empty Trigger Level"]
pub type TxEmptyTrigW<'a, REG> = crate::FieldWriter<'a, REG, 2, TxEmptyTrig, crate::Safe>;
impl<'a, REG> TxEmptyTrigW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "FIFO empty"]
    #[inline(always)]
    pub fn empty(self) -> &'a mut crate::W<REG> {
        self.variant(TxEmptyTrig::Empty)
    }
    #[doc = "2 characters in FIFO"]
    #[inline(always)]
    pub fn chars2(self) -> &'a mut crate::W<REG> {
        self.variant(TxEmptyTrig::Chars2)
    }
    #[doc = "FIFO 1/4 full"]
    #[inline(always)]
    pub fn quarter(self) -> &'a mut crate::W<REG> {
        self.variant(TxEmptyTrig::Quarter)
    }
    #[doc = "FIFO 1/2 full"]
    #[inline(always)]
    pub fn half(self) -> &'a mut crate::W<REG> {
        self.variant(TxEmptyTrig::Half)
    }
}
#[doc = "RCVR Trigger Level"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RxTrig {
    #[doc = "0: 1 character in FIFO"]
    Char1 = 0,
    #[doc = "1: FIFO 1/4 full"]
    Quarter = 1,
    #[doc = "2: FIFO 1/2 full"]
    Half = 2,
    #[doc = "3: FIFO 2 less than full"]
    FullMinus2 = 3,
}
impl From<RxTrig> for u8 {
    #[inline(always)]
    fn from(variant: RxTrig) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RxTrig {
    type Ux = u8;
}
impl crate::IsEnum for RxTrig {}
#[doc = "Field `RX_TRIG` writer - RCVR Trigger Level"]
pub type RxTrigW<'a, REG> = crate::FieldWriter<'a, REG, 2, RxTrig, crate::Safe>;
impl<'a, REG> RxTrigW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 character in FIFO"]
    #[inline(always)]
    pub fn char1(self) -> &'a mut crate::W<REG> {
        self.variant(RxTrig::Char1)
    }
    #[doc = "FIFO 1/4 full"]
    #[inline(always)]
    pub fn quarter(self) -> &'a mut crate::W<REG> {
        self.variant(RxTrig::Quarter)
    }
    #[doc = "FIFO 1/2 full"]
    #[inline(always)]
    pub fn half(self) -> &'a mut crate::W<REG> {
        self.variant(RxTrig::Half)
    }
    #[doc = "FIFO 2 less than full"]
    #[inline(always)]
    pub fn full_minus2(self) -> &'a mut crate::W<REG> {
        self.variant(RxTrig::FullMinus2)
    }
}
impl W {
    #[doc = "Bit 0 - FIFO Enable"]
    #[inline(always)]
    pub fn fifo_en(&mut self) -> FifoEnW<'_, FcrSpec> {
        FifoEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Receiver FIFO Reset"]
    #[inline(always)]
    pub fn rxfifo_rst(&mut self) -> RxfifoRstW<'_, FcrSpec> {
        RxfifoRstW::new(self, 1)
    }
    #[doc = "Bit 2 - Transmit FIFO Reset"]
    #[inline(always)]
    pub fn txfifo_rst(&mut self) -> TxfifoRstW<'_, FcrSpec> {
        TxfifoRstW::new(self, 2)
    }
    #[doc = "Bit 3 - DMA Mode Select"]
    #[inline(always)]
    pub fn dma_mode(&mut self) -> DmaModeW<'_, FcrSpec> {
        DmaModeW::new(self, 3)
    }
    #[doc = "Bits 4:5 - TX Empty Trigger Level"]
    #[inline(always)]
    pub fn tx_empty_trig(&mut self) -> TxEmptyTrigW<'_, FcrSpec> {
        TxEmptyTrigW::new(self, 4)
    }
    #[doc = "Bits 6:7 - RCVR Trigger Level"]
    #[inline(always)]
    pub fn rx_trig(&mut self) -> RxTrigW<'_, FcrSpec> {
        RxTrigW::new(self, 6)
    }
}
#[doc = "FIFO Control Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcrSpec;
impl crate::RegisterSpec for FcrSpec {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [`fcr::W`](W) writer structure"]
impl crate::Writable for FcrSpec {
    type Safety = crate::Unsafe;
}
