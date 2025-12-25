#[doc = "Register `MCR` reader"]
pub type R = crate::R<McrSpec>;
#[doc = "Register `MCR` writer"]
pub type W = crate::W<McrSpec>;
#[doc = "Field `REQ_SEND` reader - Request to Send. This is used to directly control the Request to Send (rts_n) output"]
pub type ReqSendR = crate::BitReader;
#[doc = "Field `REQ_SEND` writer - Request to Send. This is used to directly control the Request to Send (rts_n) output"]
pub type ReqSendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTO_EN` reader - Auto Flow Control Enable"]
pub type AutoEnR = crate::BitReader;
#[doc = "Field `AUTO_EN` writer - Auto Flow Control Enable"]
pub type AutoEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Request to Send. This is used to directly control the Request to Send (rts_n) output"]
    #[inline(always)]
    pub fn req_send(&self) -> ReqSendR {
        ReqSendR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - Auto Flow Control Enable"]
    #[inline(always)]
    pub fn auto_en(&self) -> AutoEnR {
        AutoEnR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Request to Send. This is used to directly control the Request to Send (rts_n) output"]
    #[inline(always)]
    pub fn req_send(&mut self) -> ReqSendW<'_, McrSpec> {
        ReqSendW::new(self, 1)
    }
    #[doc = "Bit 5 - Auto Flow Control Enable"]
    #[inline(always)]
    pub fn auto_en(&mut self) -> AutoEnW<'_, McrSpec> {
        AutoEnW::new(self, 5)
    }
}
#[doc = "Modem Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McrSpec;
impl crate::RegisterSpec for McrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcr::R`](R) reader structure"]
impl crate::Readable for McrSpec {}
#[doc = "`write(|w| ..)` method takes [`mcr::W`](W) writer structure"]
impl crate::Writable for McrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MCR to value 0"]
impl crate::Resettable for McrSpec {}
