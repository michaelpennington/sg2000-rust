#[doc = "Register `context[%s]` reader"]
pub type R = crate::R<ContextSpec>;
#[doc = "Register `context[%s]` writer"]
pub type W = crate::W<ContextSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Data For The Messages\n\nYou can [`read`](crate::Reg::read) this register and get [`context::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`context::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ContextSpec;
impl crate::RegisterSpec for ContextSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`context::R`](R) reader structure"]
impl crate::Readable for ContextSpec {}
#[doc = "`write(|w| ..)` method takes [`context::W`](W) writer structure"]
impl crate::Writable for ContextSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets context[%s] to value 0"]
impl crate::Resettable for ContextSpec {}
