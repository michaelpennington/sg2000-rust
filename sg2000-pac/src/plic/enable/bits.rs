#[doc = "Register `bits[%s]` reader"]
pub type R = crate::R<BitsSpec>;
#[doc = "Register `bits[%s]` writer"]
pub type W = crate::W<BitsSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Interrupt Enable Bits\n\nYou can [`read`](crate::Reg::read) this register and get [`bits::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bits::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BitsSpec;
impl crate::RegisterSpec for BitsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bits::R`](R) reader structure"]
impl crate::Readable for BitsSpec {}
#[doc = "`write(|w| ..)` method takes [`bits::W`](W) writer structure"]
impl crate::Writable for BitsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets bits[%s] to value 0"]
impl crate::Resettable for BitsSpec {}
