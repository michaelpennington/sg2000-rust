#[doc = "Register `PLIC_IP[%s]` reader"]
pub type R = crate::R<PlicIpSpec>;
#[doc = "Register `PLIC_IP[%s]` writer"]
pub type W = crate::W<PlicIpSpec>;
#[doc = "Field `IP` reader - Interrupt Pending Bits"]
pub type IpR = crate::FieldReader<u32>;
#[doc = "Field `IP` writer - Interrupt Pending Bits"]
pub type IpW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Interrupt Pending Bits"]
    #[inline(always)]
    pub fn ip(&self) -> IpR {
        IpR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt Pending Bits"]
    #[inline(always)]
    pub fn ip(&mut self) -> IpW<'_, PlicIpSpec> {
        IpW::new(self, 0)
    }
}
#[doc = "Interrupt Pending Register\n\nYou can [`read`](crate::Reg::read) this register and get [`plic_ip::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plic_ip::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PlicIpSpec;
impl crate::RegisterSpec for PlicIpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`plic_ip::R`](R) reader structure"]
impl crate::Readable for PlicIpSpec {}
#[doc = "`write(|w| ..)` method takes [`plic_ip::W`](W) writer structure"]
impl crate::Writable for PlicIpSpec {
    type Safety = crate::Unsafe;
}
