#[doc = "Register `SRTS` reader"]
pub type R = crate::R<SrtsSpec>;
#[doc = "Register `SRTS` writer"]
pub type W = crate::W<SrtsSpec>;
#[doc = "Field `REQ_SEND` reader - This is a shadow register for the RTS bit (MCR\\[1\\])"]
pub type ReqSendR = crate::BitReader;
#[doc = "Field `REQ_SEND` writer - This is a shadow register for the RTS bit (MCR\\[1\\])"]
pub type ReqSendW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This is a shadow register for the RTS bit (MCR\\[1\\])"]
    #[inline(always)]
    pub fn req_send(&self) -> ReqSendR {
        ReqSendR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This is a shadow register for the RTS bit (MCR\\[1\\])"]
    #[inline(always)]
    pub fn req_send(&mut self) -> ReqSendW<'_, SrtsSpec> {
        ReqSendW::new(self, 0)
    }
}
#[doc = "Shadow Request to Send\n\nYou can [`read`](crate::Reg::read) this register and get [`srts::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrtsSpec;
impl crate::RegisterSpec for SrtsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srts::R`](R) reader structure"]
impl crate::Readable for SrtsSpec {}
#[doc = "`write(|w| ..)` method takes [`srts::W`](W) writer structure"]
impl crate::Writable for SrtsSpec {
    type Safety = crate::Unsafe;
}
