#[doc = "Register `PLIC_H0_STH` reader"]
pub type R = crate::R<PlicH0SthSpec>;
#[doc = "Register `PLIC_H0_STH` writer"]
pub type W = crate::W<PlicH0SthSpec>;
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
    pub fn threshold(&mut self) -> ThresholdW<'_, PlicH0SthSpec> {
        ThresholdW::new(self, 0)
    }
}
#[doc = "S-mode Interrupt Threshold Register\n\nYou can [`read`](crate::Reg::read) this register and get [`plic_h0_sth::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plic_h0_sth::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PlicH0SthSpec;
impl crate::RegisterSpec for PlicH0SthSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`plic_h0_sth::R`](R) reader structure"]
impl crate::Readable for PlicH0SthSpec {}
#[doc = "`write(|w| ..)` method takes [`plic_h0_sth::W`](W) writer structure"]
impl crate::Writable for PlicH0SthSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PLIC_H0_STH to value 0"]
impl crate::Resettable for PlicH0SthSpec {}
