#[doc = "Register `SRR` reader"]
pub type R = crate::R<SrrSpec>;
#[doc = "Register `SRR` writer"]
pub type W = crate::W<SrrSpec>;
#[doc = "Field `UART_RST` reader - UART Reset"]
pub type UartRstR = crate::BitReader;
#[doc = "Field `UART_RST` writer - UART Reset"]
pub type UartRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_FIFO_RST` reader - Receiver FIFO Reset"]
pub type RxFifoRstR = crate::BitReader;
#[doc = "Field `RX_FIFO_RST` writer - Receiver FIFO Reset"]
pub type RxFifoRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_FIFO_RST` reader - Transmitter FIFO Reset"]
pub type TxFifoRstR = crate::BitReader;
#[doc = "Field `TX_FIFO_RST` writer - Transmitter FIFO Reset"]
pub type TxFifoRstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - UART Reset"]
    #[inline(always)]
    pub fn uart_rst(&self) -> UartRstR {
        UartRstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receiver FIFO Reset"]
    #[inline(always)]
    pub fn rx_fifo_rst(&self) -> RxFifoRstR {
        RxFifoRstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmitter FIFO Reset"]
    #[inline(always)]
    pub fn tx_fifo_rst(&self) -> TxFifoRstR {
        TxFifoRstR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UART Reset"]
    #[inline(always)]
    pub fn uart_rst(&mut self) -> UartRstW<'_, SrrSpec> {
        UartRstW::new(self, 0)
    }
    #[doc = "Bit 1 - Receiver FIFO Reset"]
    #[inline(always)]
    pub fn rx_fifo_rst(&mut self) -> RxFifoRstW<'_, SrrSpec> {
        RxFifoRstW::new(self, 1)
    }
    #[doc = "Bit 2 - Transmitter FIFO Reset"]
    #[inline(always)]
    pub fn tx_fifo_rst(&mut self) -> TxFifoRstW<'_, SrrSpec> {
        TxFifoRstW::new(self, 2)
    }
}
#[doc = "Software Reset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`srr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrrSpec;
impl crate::RegisterSpec for SrrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srr::R`](R) reader structure"]
impl crate::Readable for SrrSpec {}
#[doc = "`write(|w| ..)` method takes [`srr::W`](W) writer structure"]
impl crate::Writable for SrrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SRR to value 0"]
impl crate::Resettable for SrrSpec {}
