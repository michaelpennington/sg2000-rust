#[doc = "Register `cpu_mbox_status[%s]` reader"]
pub type R = crate::R<CpuMboxStatusSpec>;
#[doc = "Register `cpu_mbox_status[%s]` writer"]
pub type W = crate::W<CpuMboxStatusSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "cpu_mbox_status\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_mbox_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_mbox_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpuMboxStatusSpec;
impl crate::RegisterSpec for CpuMboxStatusSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`cpu_mbox_status::R`](R) reader structure"]
impl crate::Readable for CpuMboxStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`cpu_mbox_status::W`](W) writer structure"]
impl crate::Writable for CpuMboxStatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets cpu_mbox_status[%s] to value 0"]
impl crate::Resettable for CpuMboxStatusSpec {}
