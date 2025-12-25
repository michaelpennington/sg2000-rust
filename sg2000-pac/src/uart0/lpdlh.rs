#[doc = "Register `LPDLH` reader"]
pub type R = crate::R<LpdlhSpec>;
#[doc = "Register `LPDLH` writer"]
pub type W = crate::W<LpdlhSpec>;
#[doc = "Field `ECO_DIV_HIGH` reader - Low Power Divisor Latch (High byte) Register"]
pub type EcoDivHighR = crate::FieldReader;
#[doc = "Field `ECO_DIV_HIGH` writer - Low Power Divisor Latch (High byte) Register"]
pub type EcoDivHighW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Low Power Divisor Latch (High byte) Register"]
    #[inline(always)]
    pub fn eco_div_high(&self) -> EcoDivHighR {
        EcoDivHighR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Low Power Divisor Latch (High byte) Register"]
    #[inline(always)]
    pub fn eco_div_high(&mut self) -> EcoDivHighW<'_, LpdlhSpec> {
        EcoDivHighW::new(self, 0)
    }
}
#[doc = "Low Power Divisor Latch (High) Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lpdlh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpdlh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpdlhSpec;
impl crate::RegisterSpec for LpdlhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpdlh::R`](R) reader structure"]
impl crate::Readable for LpdlhSpec {}
#[doc = "`write(|w| ..)` method takes [`lpdlh::W`](W) writer structure"]
impl crate::Writable for LpdlhSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LPDLH to value 0"]
impl crate::Resettable for LpdlhSpec {}
