#[doc = "Register `PLIC_H1_STH` reader"]
pub type R = crate::R<PlicH1SthSpec>;
#[doc = "Register `PLIC_H1_STH` writer"]
pub type W = crate::W<PlicH1SthSpec>;
#[doc = "Field `THRESHOLD` reader - "]
pub type ThresholdR = crate::FieldReader<u32>;
#[doc = "Field `THRESHOLD` writer - "]
pub type ThresholdW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn threshold(&self) -> ThresholdR {
        ThresholdR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn threshold(&mut self) -> ThresholdW<'_, PlicH1SthSpec> {
        ThresholdW::new(self, 0)
    }
}
#[doc = "Hart 1 S-mode Interrupt Threshold Register\n\nYou can [`read`](crate::Reg::read) this register and get [`plic_h1_sth::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plic_h1_sth::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PlicH1SthSpec;
impl crate::RegisterSpec for PlicH1SthSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`plic_h1_sth::R`](R) reader structure"]
impl crate::Readable for PlicH1SthSpec {}
#[doc = "`write(|w| ..)` method takes [`plic_h1_sth::W`](W) writer structure"]
impl crate::Writable for PlicH1SthSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PLIC_H1_STH to value 0"]
impl crate::Resettable for PlicH1SthSpec {}
