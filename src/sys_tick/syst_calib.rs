#[doc = "Register `SYST_CALIB` reader"]
pub type R = crate::R<SystCalibSpec>;
#[doc = "Field `TENMS` reader - Reload value to use for 10ms timing"]
pub type TenmsR = crate::FieldReader<u32>;
#[doc = "Indicates whether the TENMS value is exact\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Skew {
    #[doc = "0: 10ms calibration value is exact"]
    ExactValue = 0,
    #[doc = "1: 10ms calibration value is inexact, because of the clock frequency"]
    InexactValue = 1,
}
impl From<Skew> for bool {
    #[inline(always)]
    fn from(variant: Skew) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SKEW` reader - Indicates whether the TENMS value is exact"]
pub type SkewR = crate::BitReader<Skew>;
impl SkewR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Skew {
        match self.bits {
            false => Skew::ExactValue,
            true => Skew::InexactValue,
        }
    }
    #[doc = "10ms calibration value is exact"]
    #[inline(always)]
    pub fn is_exact_value(&self) -> bool {
        *self == Skew::ExactValue
    }
    #[doc = "10ms calibration value is inexact, because of the clock frequency"]
    #[inline(always)]
    pub fn is_inexact_value(&self) -> bool {
        *self == Skew::InexactValue
    }
}
#[doc = "Indicates whether the device provides an alternative reference clock\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Noref {
    #[doc = "0: The alternative reference clock is provided"]
    ClockProvided = 0,
    #[doc = "1: The alternative reference clock is not provided"]
    ClockDisabled = 1,
}
impl From<Noref> for bool {
    #[inline(always)]
    fn from(variant: Noref) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NOREF` reader - Indicates whether the device provides an alternative reference clock"]
pub type NorefR = crate::BitReader<Noref>;
impl NorefR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Noref {
        match self.bits {
            false => Noref::ClockProvided,
            true => Noref::ClockDisabled,
        }
    }
    #[doc = "The alternative reference clock is provided"]
    #[inline(always)]
    pub fn is_clock_provided(&self) -> bool {
        *self == Noref::ClockProvided
    }
    #[doc = "The alternative reference clock is not provided"]
    #[inline(always)]
    pub fn is_clock_disabled(&self) -> bool {
        *self == Noref::ClockDisabled
    }
}
impl R {
    #[doc = "Bits 0:23 - Reload value to use for 10ms timing"]
    #[inline(always)]
    pub fn tenms(&self) -> TenmsR {
        TenmsR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 30 - Indicates whether the TENMS value is exact"]
    #[inline(always)]
    pub fn skew(&self) -> SkewR {
        SkewR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Indicates whether the device provides an alternative reference clock"]
    #[inline(always)]
    pub fn noref(&self) -> NorefR {
        NorefR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "SysTick Calibration Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`syst_calib::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SystCalibSpec;
impl crate::RegisterSpec for SystCalibSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syst_calib::R`](R) reader structure"]
impl crate::Readable for SystCalibSpec {}
#[doc = "`reset()` method sets SYST_CALIB to value 0x8000_0000"]
impl crate::Resettable for SystCalibSpec {
    const RESET_VALUE: u32 = 0x8000_0000;
}
