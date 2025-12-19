#[doc = "Register `cpu_mbox_en[%s]` reader"]
pub type R = crate::R<CpuMboxEnSpec>;
#[doc = "Register `cpu_mbox_en[%s]` writer"]
pub type W = crate::W<CpuMboxEnSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Mailbox Enable Bits\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_mbox_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_mbox_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpuMboxEnSpec;
impl crate::RegisterSpec for CpuMboxEnSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`cpu_mbox_en::R`](R) reader structure"]
impl crate::Readable for CpuMboxEnSpec {}
#[doc = "`write(|w| ..)` method takes [`cpu_mbox_en::W`](W) writer structure"]
impl crate::Writable for CpuMboxEnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets cpu_mbox_en[%s] to value 0"]
impl crate::Resettable for CpuMboxEnSpec {}
