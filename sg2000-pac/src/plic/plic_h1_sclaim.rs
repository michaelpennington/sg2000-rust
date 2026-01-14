#[doc = "Register `PLIC_H1_SCLAIM` reader"]
pub type R = crate::R<PlicH1SclaimSpec>;
#[doc = "Register `PLIC_H1_SCLAIM` writer"]
pub type W = crate::W<PlicH1SclaimSpec>;
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
    pub fn claim_id(&mut self) -> ClaimIdW<'_, PlicH1SclaimSpec> {
        ClaimIdW::new(self, 0)
    }
}
#[doc = "Hart 1 S-mode Interrupt Claim/Complete Register\n\nYou can [`read`](crate::Reg::read) this register and get [`plic_h1_sclaim::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plic_h1_sclaim::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PlicH1SclaimSpec;
impl crate::RegisterSpec for PlicH1SclaimSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`plic_h1_sclaim::R`](R) reader structure"]
impl crate::Readable for PlicH1SclaimSpec {}
#[doc = "`write(|w| ..)` method takes [`plic_h1_sclaim::W`](W) writer structure"]
impl crate::Writable for PlicH1SclaimSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PLIC_H1_SCLAIM to value 0"]
impl crate::Resettable for PlicH1SclaimSpec {}
