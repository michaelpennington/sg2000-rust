#[doc = "Register `priority_threshold[%s]` reader"]
pub type R = crate::R<PriorityThresholdSpec>;
#[doc = "Register `priority_threshold[%s]` writer"]
pub type W = crate::W<PriorityThresholdSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Interrupt Priority Threshold\n\nYou can [`read`](crate::Reg::read) this register and get [`priority_threshold::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority_threshold::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PriorityThresholdSpec;
impl crate::RegisterSpec for PriorityThresholdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`priority_threshold::R`](R) reader structure"]
impl crate::Readable for PriorityThresholdSpec {}
#[doc = "`write(|w| ..)` method takes [`priority_threshold::W`](W) writer structure"]
impl crate::Writable for PriorityThresholdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets priority_threshold[%s] to value 0"]
impl crate::Resettable for PriorityThresholdSpec {}
