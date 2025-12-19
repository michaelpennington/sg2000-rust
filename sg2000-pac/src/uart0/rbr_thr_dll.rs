#[doc = "Register `RBR_THR_DLL` reader"]
pub type R = crate::R<RbrThrDllSpec>;
#[doc = "Register `RBR_THR_DLL` writer"]
pub type W = crate::W<RbrThrDllSpec>;
#[doc = "Field `RBR_THR_DLL` reader - "]
pub type RbrThrDllR = crate::FieldReader;
#[doc = "Field `RBR_THR_DLL` writer - "]
pub type RbrThrDllW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn rbr_thr_dll(&self) -> RbrThrDllR {
        RbrThrDllR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn rbr_thr_dll(&mut self) -> RbrThrDllW<'_, RbrThrDllSpec> {
        RbrThrDllW::new(self, 0)
    }
}
#[doc = "Receive Buffer,Transmit Holding or Divisor Latch Low byte Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rbr_thr_dll::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rbr_thr_dll::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RbrThrDllSpec;
impl crate::RegisterSpec for RbrThrDllSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rbr_thr_dll::R`](R) reader structure"]
impl crate::Readable for RbrThrDllSpec {}
#[doc = "`write(|w| ..)` method takes [`rbr_thr_dll::W`](W) writer structure"]
impl crate::Writable for RbrThrDllSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RBR_THR_DLL to value 0"]
impl crate::Resettable for RbrThrDllSpec {}
