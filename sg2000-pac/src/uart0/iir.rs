#[doc = "Register `IIR` reader"]
pub type R = crate::R<IirSpec>;
#[doc = "Interrupt ID Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IntStatus {
    #[doc = "0: Modem status"]
    ModemStatus = 0,
    #[doc = "1: No interrupt pending"]
    NoInterrupt = 1,
    #[doc = "2: Trasmit holding register empty"]
    Thrempty = 2,
    #[doc = "4: Received data available"]
    DataAvailable = 4,
    #[doc = "6: Receiver line status"]
    LineStatus = 6,
    #[doc = "7: Busy detect"]
    BusyDetect = 7,
    #[doc = "12: Character Timeout"]
    CharTimeout = 12,
}
impl From<IntStatus> for u8 {
    #[inline(always)]
    fn from(variant: IntStatus) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IntStatus {
    type Ux = u8;
}
impl crate::IsEnum for IntStatus {}
#[doc = "Field `INT_ID` reader - Interrupt ID Status"]
pub type IntIdR = crate::FieldReader<IntStatus>;
impl IntIdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<IntStatus> {
        match self.bits {
            0 => Some(IntStatus::ModemStatus),
            1 => Some(IntStatus::NoInterrupt),
            2 => Some(IntStatus::Thrempty),
            4 => Some(IntStatus::DataAvailable),
            6 => Some(IntStatus::LineStatus),
            7 => Some(IntStatus::BusyDetect),
            12 => Some(IntStatus::CharTimeout),
            _ => None,
        }
    }
    #[doc = "Modem status"]
    #[inline(always)]
    pub fn is_modem_status(&self) -> bool {
        *self == IntStatus::ModemStatus
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == IntStatus::NoInterrupt
    }
    #[doc = "Trasmit holding register empty"]
    #[inline(always)]
    pub fn is_thrempty(&self) -> bool {
        *self == IntStatus::Thrempty
    }
    #[doc = "Received data available"]
    #[inline(always)]
    pub fn is_data_available(&self) -> bool {
        *self == IntStatus::DataAvailable
    }
    #[doc = "Receiver line status"]
    #[inline(always)]
    pub fn is_line_status(&self) -> bool {
        *self == IntStatus::LineStatus
    }
    #[doc = "Busy detect"]
    #[inline(always)]
    pub fn is_busy_detect(&self) -> bool {
        *self == IntStatus::BusyDetect
    }
    #[doc = "Character Timeout"]
    #[inline(always)]
    pub fn is_char_timeout(&self) -> bool {
        *self == IntStatus::CharTimeout
    }
}
#[doc = "FIFOs Enabled Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FifoEn {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "3: `11`"]
    Enabled = 3,
}
impl From<FifoEn> for u8 {
    #[inline(always)]
    fn from(variant: FifoEn) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FifoEn {
    type Ux = u8;
}
impl crate::IsEnum for FifoEn {}
#[doc = "Field `FIFO_EN` reader - FIFOs Enabled Status"]
pub type FifoEnR = crate::FieldReader<FifoEn>;
impl FifoEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<FifoEn> {
        match self.bits {
            0 => Some(FifoEn::Disabled),
            3 => Some(FifoEn::Enabled),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FifoEn::Disabled
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FifoEn::Enabled
    }
}
impl R {
    #[doc = "Bits 0:3 - Interrupt ID Status"]
    #[inline(always)]
    pub fn int_id(&self) -> IntIdR {
        IntIdR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 6:7 - FIFOs Enabled Status"]
    #[inline(always)]
    pub fn fifo_en(&self) -> FifoEnR {
        FifoEnR::new(((self.bits >> 6) & 3) as u8)
    }
}
#[doc = "Interrupt Identification Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iir::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IirSpec;
impl crate::RegisterSpec for IirSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iir::R`](R) reader structure"]
impl crate::Readable for IirSpec {}
#[doc = "`reset()` method sets IIR to value 0x01"]
impl crate::Resettable for IirSpec {
    const RESET_VALUE: u32 = 0x01;
}
