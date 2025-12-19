#[doc = "Register `DDR` reader"]
pub type R = crate::R<DdrSpec>;
#[doc = "Register `DDR` writer"]
pub type W = crate::W<DdrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Data Direction Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrSpec;
impl crate::RegisterSpec for DdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr::R`](R) reader structure"]
impl crate::Readable for DdrSpec {}
#[doc = "`write(|w| ..)` method takes [`ddr::W`](W) writer structure"]
impl crate::Writable for DdrSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets DDR to value 0"]
impl crate::Resettable for DdrSpec {}
