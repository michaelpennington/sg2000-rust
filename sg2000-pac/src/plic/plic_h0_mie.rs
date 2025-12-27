#[doc = "Register `PLIC_H0_MIE[%s]` reader"]
pub type R = crate::R<PlicH0MieSpec>;
#[doc = "Register `PLIC_H0_MIE[%s]` writer"]
pub type W = crate::W<PlicH0MieSpec>;
#[doc = "Field `IE` reader - Interrupt Enable Bits"]
pub type IeR = crate::FieldReader<u32>;
#[doc = "Field `IE` writer - Interrupt Enable Bits"]
pub type IeW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Interrupt Enable Bits"]
    #[inline(always)]
    pub fn ie(&self) -> IeR {
        IeR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt Enable Bits"]
    #[inline(always)]
    pub fn ie(&mut self) -> IeW<'_, PlicH0MieSpec> {
        IeW::new(self, 0)
    }
}
#[doc = "M-mode Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`plic_h0_mie::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plic_h0_mie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PlicH0MieSpec;
impl crate::RegisterSpec for PlicH0MieSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`plic_h0_mie::R`](R) reader structure"]
impl crate::Readable for PlicH0MieSpec {}
#[doc = "`write(|w| ..)` method takes [`plic_h0_mie::W`](W) writer structure"]
impl crate::Writable for PlicH0MieSpec {
    type Safety = crate::Unsafe;
}
