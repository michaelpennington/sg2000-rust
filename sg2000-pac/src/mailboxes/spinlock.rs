#[doc = "Register `spinlock[%s]` reader"]
pub type R = crate::R<SpinlockSpec>;
#[doc = "Register `spinlock[%s]` writer"]
pub type W = crate::W<SpinlockSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Spinlock\n\nYou can [`read`](crate::Reg::read) this register and get [`spinlock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spinlock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpinlockSpec;
impl crate::RegisterSpec for SpinlockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spinlock::R`](R) reader structure"]
impl crate::Readable for SpinlockSpec {}
#[doc = "`write(|w| ..)` method takes [`spinlock::W`](W) writer structure"]
impl crate::Writable for SpinlockSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets spinlock[%s] to value 0"]
impl crate::Resettable for SpinlockSpec {}
