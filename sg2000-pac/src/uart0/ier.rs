#[doc = "Register `IER` reader"]
pub type R = crate::R<IerSpec>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IerSpec>;
#[doc = "Field `RX_DATA` reader - Enable Received Data Enable Interrupt"]
pub type RxDataR = crate::BitReader;
#[doc = "Field `RX_DATA` writer - Enable Received Data Enable Interrupt"]
pub type RxDataW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_EMPTY` reader - Enable Transmit Holding Register Empty Interrupt"]
pub type TxEmptyR = crate::BitReader;
#[doc = "Field `TX_EMPTY` writer - Enable Transmit Holding Register Empty Interrupt"]
pub type TxEmptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_STATUS` reader - Enable Receiver Line Status Interrupt"]
pub type RxStatusR = crate::BitReader;
#[doc = "Field `RX_STATUS` writer - Enable Receiver Line Status Interrupt"]
pub type RxStatusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODEM_STATUS` reader - Enable Modem Status Interrupt"]
pub type ModemStatusR = crate::BitReader;
#[doc = "Field `MODEM_STATUS` writer - Enable Modem Status Interrupt"]
pub type ModemStatusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THRE_MODE` reader - Enable Programmable THRE Interrupt Mode"]
pub type ThreModeR = crate::BitReader;
#[doc = "Field `THRE_MODE` writer - Enable Programmable THRE Interrupt Mode"]
pub type ThreModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable Received Data Enable Interrupt"]
    #[inline(always)]
    pub fn rx_data(&self) -> RxDataR {
        RxDataR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Transmit Holding Register Empty Interrupt"]
    #[inline(always)]
    pub fn tx_empty(&self) -> TxEmptyR {
        TxEmptyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Receiver Line Status Interrupt"]
    #[inline(always)]
    pub fn rx_status(&self) -> RxStatusR {
        RxStatusR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Modem Status Interrupt"]
    #[inline(always)]
    pub fn modem_status(&self) -> ModemStatusR {
        ModemStatusR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Programmable THRE Interrupt Mode"]
    #[inline(always)]
    pub fn thre_mode(&self) -> ThreModeR {
        ThreModeR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Received Data Enable Interrupt"]
    #[inline(always)]
    pub fn rx_data(&mut self) -> RxDataW<'_, IerSpec> {
        RxDataW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Transmit Holding Register Empty Interrupt"]
    #[inline(always)]
    pub fn tx_empty(&mut self) -> TxEmptyW<'_, IerSpec> {
        TxEmptyW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Receiver Line Status Interrupt"]
    #[inline(always)]
    pub fn rx_status(&mut self) -> RxStatusW<'_, IerSpec> {
        RxStatusW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Modem Status Interrupt"]
    #[inline(always)]
    pub fn modem_status(&mut self) -> ModemStatusW<'_, IerSpec> {
        ModemStatusW::new(self, 3)
    }
    #[doc = "Bit 7 - Enable Programmable THRE Interrupt Mode"]
    #[inline(always)]
    pub fn thre_mode(&mut self) -> ThreModeW<'_, IerSpec> {
        ThreModeW::new(self, 7)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IerSpec;
impl crate::RegisterSpec for IerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier::R`](R) reader structure"]
impl crate::Readable for IerSpec {}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IerSpec {}
