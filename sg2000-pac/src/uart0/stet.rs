#[doc = "Register `STET` reader"]
pub type R = crate::R<StetSpec>;
#[doc = "Register `STET` writer"]
pub type W = crate::W<StetSpec>;
#[doc = "Field `TX_EMPTY_TRIG` reader - This is a shadow register for the TX empty trigger bits (FCR\\[5:4\\])"]
pub type TxEmptyTrigR = crate::FieldReader;
#[doc = "Field `TX_EMPTY_TRIG` writer - This is a shadow register for the TX empty trigger bits (FCR\\[5:4\\])"]
pub type TxEmptyTrigW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - This is a shadow register for the TX empty trigger bits (FCR\\[5:4\\])"]
    #[inline(always)]
    pub fn tx_empty_trig(&self) -> TxEmptyTrigR {
        TxEmptyTrigR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - This is a shadow register for the TX empty trigger bits (FCR\\[5:4\\])"]
    #[inline(always)]
    pub fn tx_empty_trig(&mut self) -> TxEmptyTrigW<'_, StetSpec> {
        TxEmptyTrigW::new(self, 0)
    }
}
#[doc = "Shadow TX Empty Trigger\n\nYou can [`read`](crate::Reg::read) this register and get [`stet::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stet::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StetSpec;
impl crate::RegisterSpec for StetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stet::R`](R) reader structure"]
impl crate::Readable for StetSpec {}
#[doc = "`write(|w| ..)` method takes [`stet::W`](W) writer structure"]
impl crate::Writable for StetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STET to value 0"]
impl crate::Resettable for StetSpec {}
