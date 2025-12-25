#[doc = "Register `SRT` reader"]
pub type R = crate::R<SrtSpec>;
#[doc = "Register `SRT` writer"]
pub type W = crate::W<SrtSpec>;
#[doc = "Field `RX_TRIG` reader - This is a shadow register for the RCVR trigger bits (FCR\\[7:6\\])"]
pub type RxTrigR = crate::FieldReader;
#[doc = "Field `RX_TRIG` writer - This is a shadow register for the RCVR trigger bits (FCR\\[7:6\\])"]
pub type RxTrigW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - This is a shadow register for the RCVR trigger bits (FCR\\[7:6\\])"]
    #[inline(always)]
    pub fn rx_trig(&self) -> RxTrigR {
        RxTrigR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - This is a shadow register for the RCVR trigger bits (FCR\\[7:6\\])"]
    #[inline(always)]
    pub fn rx_trig(&mut self) -> RxTrigW<'_, SrtSpec> {
        RxTrigW::new(self, 0)
    }
}
#[doc = "Shadow RCVR Trigger\n\nYou can [`read`](crate::Reg::read) this register and get [`srt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrtSpec;
impl crate::RegisterSpec for SrtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srt::R`](R) reader structure"]
impl crate::Readable for SrtSpec {}
#[doc = "`write(|w| ..)` method takes [`srt::W`](W) writer structure"]
impl crate::Writable for SrtSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SRT to value 0"]
impl crate::Resettable for SrtSpec {}
