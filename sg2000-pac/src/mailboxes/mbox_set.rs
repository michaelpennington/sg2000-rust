#[doc = "Register `mbox_set` reader"]
pub type R = crate::R<MboxSetSpec>;
#[doc = "Register `mbox_set` writer"]
pub type W = crate::W<MboxSetSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "mbox_set\n\nYou can [`read`](crate::Reg::read) this register and get [`mbox_set::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mbox_set::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MboxSetSpec;
impl crate::RegisterSpec for MboxSetSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mbox_set::R`](R) reader structure"]
impl crate::Readable for MboxSetSpec {}
#[doc = "`write(|w| ..)` method takes [`mbox_set::W`](W) writer structure"]
impl crate::Writable for MboxSetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets mbox_set to value 0"]
impl crate::Resettable for MboxSetSpec {}
