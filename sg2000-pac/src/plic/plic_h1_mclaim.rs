#[doc = "Register `PLIC_H1_MCLAIM` reader"]
pub type R = crate::R<PlicH1MclaimSpec>;
#[doc = "Register `PLIC_H1_MCLAIM` writer"]
pub type W = crate::W<PlicH1MclaimSpec>;
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
    pub fn claim_id(&mut self) -> ClaimIdW<'_, PlicH1MclaimSpec> {
        ClaimIdW::new(self, 0)
    }
}
#[doc = "Hart 1 M-mode Interrupt Claim/Complete Register\n\nYou can [`read`](crate::Reg::read) this register and get [`plic_h1_mclaim::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plic_h1_mclaim::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PlicH1MclaimSpec;
impl crate::RegisterSpec for PlicH1MclaimSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`plic_h1_mclaim::R`](R) reader structure"]
impl crate::Readable for PlicH1MclaimSpec {}
#[doc = "`write(|w| ..)` method takes [`plic_h1_mclaim::W`](W) writer structure"]
impl crate::Writable for PlicH1MclaimSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PLIC_H1_MCLAIM to value 0"]
impl crate::Resettable for PlicH1MclaimSpec {}
