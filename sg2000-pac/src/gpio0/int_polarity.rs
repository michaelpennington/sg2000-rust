#[doc = "Register `INT_POLARITY` reader"]
pub type R = crate::R<IntPolaritySpec>;
#[doc = "Register `INT_POLARITY` writer"]
pub type W = crate::W<IntPolaritySpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Interrupt Polarity Register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_polarity::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_polarity::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntPolaritySpec;
impl crate::RegisterSpec for IntPolaritySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_polarity::R`](R) reader structure"]
impl crate::Readable for IntPolaritySpec {}
#[doc = "`write(|w| ..)` method takes [`int_polarity::W`](W) writer structure"]
impl crate::Writable for IntPolaritySpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets INT_POLARITY to value 0"]
impl crate::Resettable for IntPolaritySpec {}
