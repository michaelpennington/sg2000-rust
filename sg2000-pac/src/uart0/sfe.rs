#[doc = "Register `SFE` reader"]
pub type R = crate::R<SfeSpec>;
#[doc = "Register `SFE` writer"]
pub type W = crate::W<SfeSpec>;
#[doc = "Field `FIFO_EN` reader - This is a shadow register for the FIFO enable bit (FCR\\[0\\])"]
pub type FifoEnR = crate::BitReader;
#[doc = "Field `FIFO_EN` writer - This is a shadow register for the FIFO enable bit (FCR\\[0\\])"]
pub type FifoEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This is a shadow register for the FIFO enable bit (FCR\\[0\\])"]
    #[inline(always)]
    pub fn fifo_en(&self) -> FifoEnR {
        FifoEnR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This is a shadow register for the FIFO enable bit (FCR\\[0\\])"]
    #[inline(always)]
    pub fn fifo_en(&mut self) -> FifoEnW<'_, SfeSpec> {
        FifoEnW::new(self, 0)
    }
}
#[doc = "Shadow FIFO Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`sfe::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfe::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfeSpec;
impl crate::RegisterSpec for SfeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfe::R`](R) reader structure"]
impl crate::Readable for SfeSpec {}
#[doc = "`write(|w| ..)` method takes [`sfe::W`](W) writer structure"]
impl crate::Writable for SfeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFE to value 0"]
impl crate::Resettable for SfeSpec {}
