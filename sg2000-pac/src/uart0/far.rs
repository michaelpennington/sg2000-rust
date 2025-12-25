#[doc = "Register `FAR` reader"]
pub type R = crate::R<FarSpec>;
#[doc = "Register `FAR` writer"]
pub type W = crate::W<FarSpec>;
#[doc = "Field `FIFO_ACCESS` reader - Used to enable a FIFO access mode for testing."]
pub type FifoAccessR = crate::BitReader;
#[doc = "Field `FIFO_ACCESS` writer - Used to enable a FIFO access mode for testing."]
pub type FifoAccessW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Used to enable a FIFO access mode for testing."]
    #[inline(always)]
    pub fn fifo_access(&self) -> FifoAccessR {
        FifoAccessR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Used to enable a FIFO access mode for testing."]
    #[inline(always)]
    pub fn fifo_access(&mut self) -> FifoAccessW<'_, FarSpec> {
        FifoAccessW::new(self, 0)
    }
}
#[doc = "FIFO Access Register\n\nYou can [`read`](crate::Reg::read) this register and get [`far::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`far::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FarSpec;
impl crate::RegisterSpec for FarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`far::R`](R) reader structure"]
impl crate::Readable for FarSpec {}
#[doc = "`write(|w| ..)` method takes [`far::W`](W) writer structure"]
impl crate::Writable for FarSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FAR to value 0"]
impl crate::Resettable for FarSpec {}
