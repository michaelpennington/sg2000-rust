#[doc = "Register `LPDLL` reader"]
pub type R = crate::R<LpdllSpec>;
#[doc = "Register `LPDLL` writer"]
pub type W = crate::W<LpdllSpec>;
#[doc = "Field `ECO_DIV_LOW` reader - Low Power Divisor Latch (Low byte) Register"]
pub type EcoDivLowR = crate::FieldReader;
#[doc = "Field `ECO_DIV_LOW` writer - Low Power Divisor Latch (Low byte) Register"]
pub type EcoDivLowW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Low Power Divisor Latch (Low byte) Register"]
    #[inline(always)]
    pub fn eco_div_low(&self) -> EcoDivLowR {
        EcoDivLowR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Low Power Divisor Latch (Low byte) Register"]
    #[inline(always)]
    pub fn eco_div_low(&mut self) -> EcoDivLowW<'_, LpdllSpec> {
        EcoDivLowW::new(self, 0)
    }
}
#[doc = "Low Power Divisor Latch (Low) Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lpdll::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpdll::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpdllSpec;
impl crate::RegisterSpec for LpdllSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpdll::R`](R) reader structure"]
impl crate::Readable for LpdllSpec {}
#[doc = "`write(|w| ..)` method takes [`lpdll::W`](W) writer structure"]
impl crate::Writable for LpdllSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LPDLL to value 0"]
impl crate::Resettable for LpdllSpec {}
