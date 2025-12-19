#[doc = "Register `mbox_status` reader"]
pub type R = crate::R<MboxStatusSpec>;
#[doc = "Register `mbox_status` writer"]
pub type W = crate::W<MboxStatusSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "mbox_status\n\nYou can [`read`](crate::Reg::read) this register and get [`mbox_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mbox_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MboxStatusSpec;
impl crate::RegisterSpec for MboxStatusSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mbox_status::R`](R) reader structure"]
impl crate::Readable for MboxStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`mbox_status::W`](W) writer structure"]
impl crate::Writable for MboxStatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets mbox_status to value 0"]
impl crate::Resettable for MboxStatusSpec {}
