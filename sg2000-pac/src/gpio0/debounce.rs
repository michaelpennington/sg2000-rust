#[doc = "Register `DEBOUNCE` reader"]
pub type R = crate::R<DebounceSpec>;
#[doc = "Register `DEBOUNCE` writer"]
pub type W = crate::W<DebounceSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Debounce Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`debounce::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debounce::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DebounceSpec;
impl crate::RegisterSpec for DebounceSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debounce::R`](R) reader structure"]
impl crate::Readable for DebounceSpec {}
#[doc = "`write(|w| ..)` method takes [`debounce::W`](W) writer structure"]
impl crate::Writable for DebounceSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets DEBOUNCE to value 0"]
impl crate::Resettable for DebounceSpec {}
