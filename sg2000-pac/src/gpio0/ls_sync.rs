#[doc = "Register `LS_SYNC` reader"]
pub type R = crate::R<LsSyncSpec>;
#[doc = "Register `LS_SYNC` writer"]
pub type W = crate::W<LsSyncSpec>;
#[doc = "Field `LS_SYNC` reader - Synchronize all level-sensitive interrupt to pclk_intr"]
pub type LsSyncR = crate::BitReader;
#[doc = "Field `LS_SYNC` writer - Synchronize all level-sensitive interrupt to pclk_intr"]
pub type LsSyncW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Synchronize all level-sensitive interrupt to pclk_intr"]
    #[inline(always)]
    pub fn ls_sync(&self) -> LsSyncR {
        LsSyncR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Synchronize all level-sensitive interrupt to pclk_intr"]
    #[inline(always)]
    pub fn ls_sync(&mut self) -> LsSyncW<'_, LsSyncSpec> {
        LsSyncW::new(self, 0)
    }
}
#[doc = "Level-Sensitive Synchronization Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ls_sync::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ls_sync::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LsSyncSpec;
impl crate::RegisterSpec for LsSyncSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ls_sync::R`](R) reader structure"]
impl crate::Readable for LsSyncSpec {}
#[doc = "`write(|w| ..)` method takes [`ls_sync::W`](W) writer structure"]
impl crate::Writable for LsSyncSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LS_SYNC to value 0"]
impl crate::Resettable for LsSyncSpec {}
