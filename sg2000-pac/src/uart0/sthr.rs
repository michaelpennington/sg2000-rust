#[doc = "Register `STHR` writer"]
pub type W = crate::W<SthrSpec>;
#[doc = "Field `TX_BUF` writer - Shadow Transmit Holding Register"]
pub type TxBufW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - Shadow Transmit Holding Register"]
    #[inline(always)]
    pub fn tx_buf(&mut self) -> TxBufW<'_, SthrSpec> {
        TxBufW::new(self, 0)
    }
}
#[doc = "Shadow Transmit Buffer Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sthr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SthrSpec;
impl crate::RegisterSpec for SthrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sthr::W`](W) writer structure"]
impl crate::Writable for SthrSpec {
    type Safety = crate::Unsafe;
}
