#[doc = "Register `SBCR` reader"]
pub type R = crate::R<SbcrSpec>;
#[doc = "Register `SBCR` writer"]
pub type W = crate::W<SbcrSpec>;
#[doc = "Field `BREAK_CNT` reader - This is a shadow register for the Break bit (LCR\\[6\\])."]
pub type BreakCntR = crate::BitReader;
#[doc = "Field `BREAK_CNT` writer - This is a shadow register for the Break bit (LCR\\[6\\])."]
pub type BreakCntW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This is a shadow register for the Break bit (LCR\\[6\\])."]
    #[inline(always)]
    pub fn break_cnt(&self) -> BreakCntR {
        BreakCntR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This is a shadow register for the Break bit (LCR\\[6\\])."]
    #[inline(always)]
    pub fn break_cnt(&mut self) -> BreakCntW<'_, SbcrSpec> {
        BreakCntW::new(self, 0)
    }
}
#[doc = "Shadow Break Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sbcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SbcrSpec;
impl crate::RegisterSpec for SbcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sbcr::R`](R) reader structure"]
impl crate::Readable for SbcrSpec {}
#[doc = "`write(|w| ..)` method takes [`sbcr::W`](W) writer structure"]
impl crate::Writable for SbcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SBCR to value 0"]
impl crate::Resettable for SbcrSpec {}
