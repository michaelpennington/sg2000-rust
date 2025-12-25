#[doc = "Register `SRBR` reader"]
pub type R = crate::R<SrbrSpec>;
#[doc = "Field `RX_BUF` reader - Shadow Receive Buffer Register"]
pub type RxBufR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Shadow Receive Buffer Register"]
    #[inline(always)]
    pub fn rx_buf(&self) -> RxBufR {
        RxBufR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Shadow Receive Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`srbr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrbrSpec;
impl crate::RegisterSpec for SrbrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srbr::R`](R) reader structure"]
impl crate::Readable for SrbrSpec {}
#[doc = "`reset()` method sets SRBR to value 0"]
impl crate::Resettable for SrbrSpec {}
