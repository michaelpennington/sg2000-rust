#[doc = "Register `LCR` reader"]
pub type R = crate::R<LcrSpec>;
#[doc = "Register `LCR` writer"]
pub type W = crate::W<LcrSpec>;
#[doc = "Data length selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DataLen {
    #[doc = "0: Five data bits"]
    Five = 0,
    #[doc = "1: Six data bits"]
    Six = 1,
    #[doc = "2: Seven data bits"]
    Seven = 2,
    #[doc = "3: Eight data bits"]
    Eight = 3,
}
impl From<DataLen> for u8 {
    #[inline(always)]
    fn from(variant: DataLen) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DataLen {
    type Ux = u8;
}
impl crate::IsEnum for DataLen {}
#[doc = "Field `DATA_LEN` reader - Data length selection"]
pub type DataLenR = crate::FieldReader<DataLen>;
impl DataLenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DataLen {
        match self.bits {
            0 => DataLen::Five,
            1 => DataLen::Six,
            2 => DataLen::Seven,
            3 => DataLen::Eight,
            _ => unreachable!(),
        }
    }
    #[doc = "Five data bits"]
    #[inline(always)]
    pub fn is_five(&self) -> bool {
        *self == DataLen::Five
    }
    #[doc = "Six data bits"]
    #[inline(always)]
    pub fn is_six(&self) -> bool {
        *self == DataLen::Six
    }
    #[doc = "Seven data bits"]
    #[inline(always)]
    pub fn is_seven(&self) -> bool {
        *self == DataLen::Seven
    }
    #[doc = "Eight data bits"]
    #[inline(always)]
    pub fn is_eight(&self) -> bool {
        *self == DataLen::Eight
    }
}
#[doc = "Field `DATA_LEN` writer - Data length selection"]
pub type DataLenW<'a, REG> = crate::FieldWriter<'a, REG, 2, DataLen, crate::Safe>;
impl<'a, REG> DataLenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Five data bits"]
    #[inline(always)]
    pub fn five(self) -> &'a mut crate::W<REG> {
        self.variant(DataLen::Five)
    }
    #[doc = "Six data bits"]
    #[inline(always)]
    pub fn six(self) -> &'a mut crate::W<REG> {
        self.variant(DataLen::Six)
    }
    #[doc = "Seven data bits"]
    #[inline(always)]
    pub fn seven(self) -> &'a mut crate::W<REG> {
        self.variant(DataLen::Seven)
    }
    #[doc = "Eight data bits"]
    #[inline(always)]
    pub fn eight(self) -> &'a mut crate::W<REG> {
        self.variant(DataLen::Eight)
    }
}
#[doc = "Number of stop bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StopBits {
    #[doc = "0: One stop bit"]
    One = 0,
    #[doc = "1: Two stop bits if DATA_LEN is not 0, else 1.5 bits"]
    Two = 1,
}
impl From<StopBits> for bool {
    #[inline(always)]
    fn from(variant: StopBits) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOP_BITS` reader - Number of stop bits."]
pub type StopBitsR = crate::BitReader<StopBits>;
impl StopBitsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> StopBits {
        match self.bits {
            false => StopBits::One,
            true => StopBits::Two,
        }
    }
    #[doc = "One stop bit"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == StopBits::One
    }
    #[doc = "Two stop bits if DATA_LEN is not 0, else 1.5 bits"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == StopBits::Two
    }
}
#[doc = "Field `STOP_BITS` writer - Number of stop bits."]
pub type StopBitsW<'a, REG> = crate::BitWriter<'a, REG, StopBits>;
impl<'a, REG> StopBitsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "One stop bit"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(StopBits::One)
    }
    #[doc = "Two stop bits if DATA_LEN is not 0, else 1.5 bits"]
    #[inline(always)]
    pub fn two(self) -> &'a mut crate::W<REG> {
        self.variant(StopBits::Two)
    }
}
#[doc = "Field `PARITY_EN` reader - Parity enable"]
pub type ParityEnR = crate::BitReader;
#[doc = "Field `PARITY_EN` writer - Parity enable"]
pub type ParityEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAR_EVEN` reader - Even parity select"]
pub type ParEvenR = crate::BitReader;
#[doc = "Field `PAR_EVEN` writer - Even parity select"]
pub type ParEvenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STICK_PAR` reader - Stick parity"]
pub type StickParR = crate::BitReader;
#[doc = "Field `STICK_PAR` writer - Stick parity"]
pub type StickParW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BREAK_CNT` reader - Break control bit"]
pub type BreakCntR = crate::BitReader;
#[doc = "Field `BREAK_CNT` writer - Break control bit"]
pub type BreakCntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIV_LATCH` reader - Divisor Latch Access Bit"]
pub type DivLatchR = crate::BitReader;
#[doc = "Field `DIV_LATCH` writer - Divisor Latch Access Bit"]
pub type DivLatchW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Data length selection"]
    #[inline(always)]
    pub fn data_len(&self) -> DataLenR {
        DataLenR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Number of stop bits."]
    #[inline(always)]
    pub fn stop_bits(&self) -> StopBitsR {
        StopBitsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Parity enable"]
    #[inline(always)]
    pub fn parity_en(&self) -> ParityEnR {
        ParityEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Even parity select"]
    #[inline(always)]
    pub fn par_even(&self) -> ParEvenR {
        ParEvenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Stick parity"]
    #[inline(always)]
    pub fn stick_par(&self) -> StickParR {
        StickParR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Break control bit"]
    #[inline(always)]
    pub fn break_cnt(&self) -> BreakCntR {
        BreakCntR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Divisor Latch Access Bit"]
    #[inline(always)]
    pub fn div_latch(&self) -> DivLatchR {
        DivLatchR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Data length selection"]
    #[inline(always)]
    pub fn data_len(&mut self) -> DataLenW<'_, LcrSpec> {
        DataLenW::new(self, 0)
    }
    #[doc = "Bit 2 - Number of stop bits."]
    #[inline(always)]
    pub fn stop_bits(&mut self) -> StopBitsW<'_, LcrSpec> {
        StopBitsW::new(self, 2)
    }
    #[doc = "Bit 3 - Parity enable"]
    #[inline(always)]
    pub fn parity_en(&mut self) -> ParityEnW<'_, LcrSpec> {
        ParityEnW::new(self, 3)
    }
    #[doc = "Bit 4 - Even parity select"]
    #[inline(always)]
    pub fn par_even(&mut self) -> ParEvenW<'_, LcrSpec> {
        ParEvenW::new(self, 4)
    }
    #[doc = "Bit 5 - Stick parity"]
    #[inline(always)]
    pub fn stick_par(&mut self) -> StickParW<'_, LcrSpec> {
        StickParW::new(self, 5)
    }
    #[doc = "Bit 6 - Break control bit"]
    #[inline(always)]
    pub fn break_cnt(&mut self) -> BreakCntW<'_, LcrSpec> {
        BreakCntW::new(self, 6)
    }
    #[doc = "Bit 7 - Divisor Latch Access Bit"]
    #[inline(always)]
    pub fn div_latch(&mut self) -> DivLatchW<'_, LcrSpec> {
        DivLatchW::new(self, 7)
    }
}
#[doc = "Line Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcrSpec;
impl crate::RegisterSpec for LcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcr::R`](R) reader structure"]
impl crate::Readable for LcrSpec {}
#[doc = "`write(|w| ..)` method takes [`lcr::W`](W) writer structure"]
impl crate::Writable for LcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCR to value 0"]
impl crate::Resettable for LcrSpec {}
