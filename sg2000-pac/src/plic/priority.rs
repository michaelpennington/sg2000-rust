#[doc = "Register `priority[%s]` reader"]
pub type R = crate::R<PrioritySpec>;
#[doc = "Register `priority[%s]` writer"]
pub type W = crate::W<PrioritySpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Interrupt priority\n\nYou can [`read`](crate::Reg::read) this register and get [`priority::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrioritySpec;
impl crate::RegisterSpec for PrioritySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`priority::R`](R) reader structure"]
impl crate::Readable for PrioritySpec {}
#[doc = "`write(|w| ..)` method takes [`priority::W`](W) writer structure"]
impl crate::Writable for PrioritySpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets priority[%s] to value 0"]
impl crate::Resettable for PrioritySpec {}
