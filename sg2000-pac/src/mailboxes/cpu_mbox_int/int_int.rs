#[doc = "Register `int_int` reader"]
pub type R = crate::R<IntIntSpec>;
#[doc = "Register `int_int` writer"]
pub type W = crate::W<IntIntSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`int_int::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_int::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntIntSpec;
impl crate::RegisterSpec for IntIntSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`int_int::R`](R) reader structure"]
impl crate::Readable for IntIntSpec {}
#[doc = "`write(|w| ..)` method takes [`int_int::W`](W) writer structure"]
impl crate::Writable for IntIntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets int_int to value 0"]
impl crate::Resettable for IntIntSpec {}
