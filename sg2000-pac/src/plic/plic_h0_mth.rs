#[doc = "Register `PLIC_H0_MTH` reader"]
pub type R = crate::R<PlicH0MthSpec>;
#[doc = "Register `PLIC_H0_MTH` writer"]
pub type W = crate::W<PlicH0MthSpec>;
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
    pub fn threshold(&mut self) -> ThresholdW<'_, PlicH0MthSpec> {
        ThresholdW::new(self, 0)
    }
}
#[doc = "M-mode Interrupt Threshold Register\n\nYou can [`read`](crate::Reg::read) this register and get [`plic_h0_mth::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plic_h0_mth::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PlicH0MthSpec;
impl crate::RegisterSpec for PlicH0MthSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`plic_h0_mth::R`](R) reader structure"]
impl crate::Readable for PlicH0MthSpec {}
#[doc = "`write(|w| ..)` method takes [`plic_h0_mth::W`](W) writer structure"]
impl crate::Writable for PlicH0MthSpec {
    type Safety = crate::Unsafe;
}
