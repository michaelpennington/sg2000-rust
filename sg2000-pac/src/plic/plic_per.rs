#[doc = "Register `PLIC_PER` reader"]
pub type R = crate::R<PlicPerSpec>;
#[doc = "Register `PLIC_PER` writer"]
pub type W = crate::W<PlicPerSpec>;
#[doc = "Field `S_PER` reader - S-mode Access Permission (0: M-mode only, 1: M/S modes)"]
pub type SPerR = crate::BitReader;
#[doc = "Field `S_PER` writer - S-mode Access Permission (0: M-mode only, 1: M/S modes)"]
pub type SPerW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - S-mode Access Permission (0: M-mode only, 1: M/S modes)"]
    #[inline(always)]
    pub fn s_per(&self) -> SPerR {
        SPerR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - S-mode Access Permission (0: M-mode only, 1: M/S modes)"]
    #[inline(always)]
    pub fn s_per(&mut self) -> SPerW<'_, PlicPerSpec> {
        SPerW::new(self, 0)
    }
}
#[doc = "PLIC Permission Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`plic_per::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plic_per::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PlicPerSpec;
impl crate::RegisterSpec for PlicPerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`plic_per::R`](R) reader structure"]
impl crate::Readable for PlicPerSpec {}
#[doc = "`write(|w| ..)` method takes [`plic_per::W`](W) writer structure"]
impl crate::Writable for PlicPerSpec {
    type Safety = crate::Unsafe;
}
