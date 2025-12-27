#[doc = "Register `PLIC_H0_MCLAIM` reader"]
pub type R = crate::R<PlicH0MclaimSpec>;
#[doc = "Register `PLIC_H0_MCLAIM` writer"]
pub type W = crate::W<PlicH0MclaimSpec>;
#[doc = "Field `CLAIM_ID` reader - "]
pub type ClaimIdR = crate::FieldReader<u32>;
#[doc = "Field `CLAIM_ID` writer - "]
pub type ClaimIdW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn claim_id(&self) -> ClaimIdR {
        ClaimIdR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn claim_id(&mut self) -> ClaimIdW<'_, PlicH0MclaimSpec> {
        ClaimIdW::new(self, 0)
    }
}
#[doc = "M-mode Interrupt Claim/Complete Register\n\nYou can [`read`](crate::Reg::read) this register and get [`plic_h0_mclaim::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plic_h0_mclaim::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PlicH0MclaimSpec;
impl crate::RegisterSpec for PlicH0MclaimSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`plic_h0_mclaim::R`](R) reader structure"]
impl crate::Readable for PlicH0MclaimSpec {}
#[doc = "`write(|w| ..)` method takes [`plic_h0_mclaim::W`](W) writer structure"]
impl crate::Writable for PlicH0MclaimSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PLIC_H0_MCLAIM to value 0"]
impl crate::Resettable for PlicH0MclaimSpec {}
