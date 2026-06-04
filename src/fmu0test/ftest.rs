#[doc = "Register `FTEST` reader"]
pub type R = crate::R<FtestSpec>;
#[doc = "Test Mode Entry Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tmectl {
    #[doc = "0: FTEST register always reads 0 and writes to FTEST are ignored"]
    Zz69 = 0,
    #[doc = "1: FTEST register is readable and can be written to enable writability of TME"]
    Zz70 = 1,
}
impl From<Tmectl> for bool {
    #[inline(always)]
    fn from(variant: Tmectl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMECTL` reader - Test Mode Entry Control"]
pub type TmectlR = crate::BitReader<Tmectl>;
impl TmectlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tmectl {
        match self.bits {
            false => Tmectl::Zz69,
            true => Tmectl::Zz70,
        }
    }
    #[doc = "FTEST register always reads 0 and writes to FTEST are ignored"]
    #[inline(always)]
    pub fn is_zz69(&self) -> bool {
        *self == Tmectl::Zz69
    }
    #[doc = "FTEST register is readable and can be written to enable writability of TME"]
    #[inline(always)]
    pub fn is_zz70(&self) -> bool {
        *self == Tmectl::Zz70
    }
}
#[doc = "Test Mode Entry Writable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tmewr {
    #[doc = "0: TME bit is not writable"]
    Zz67 = 0,
    #[doc = "1: TME bit is writable"]
    Zz68 = 1,
}
impl From<Tmewr> for bool {
    #[inline(always)]
    fn from(variant: Tmewr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMEWR` reader - Test Mode Entry Writable"]
pub type TmewrR = crate::BitReader<Tmewr>;
impl TmewrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tmewr {
        match self.bits {
            false => Tmewr::Zz67,
            true => Tmewr::Zz68,
        }
    }
    #[doc = "TME bit is not writable"]
    #[inline(always)]
    pub fn is_zz67(&self) -> bool {
        *self == Tmewr::Zz67
    }
    #[doc = "TME bit is writable"]
    #[inline(always)]
    pub fn is_zz68(&self) -> bool {
        *self == Tmewr::Zz68
    }
}
#[doc = "Test Mode Entry\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tme {
    #[doc = "0: Test mode entry not requested"]
    Zz65 = 0,
    #[doc = "1: Test mode entry requested"]
    Zz66 = 1,
}
impl From<Tme> for bool {
    #[inline(always)]
    fn from(variant: Tme) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TME` reader - Test Mode Entry"]
pub type TmeR = crate::BitReader<Tme>;
impl TmeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tme {
        match self.bits {
            false => Tme::Zz65,
            true => Tme::Zz66,
        }
    }
    #[doc = "Test mode entry not requested"]
    #[inline(always)]
    pub fn is_zz65(&self) -> bool {
        *self == Tme::Zz65
    }
    #[doc = "Test mode entry requested"]
    #[inline(always)]
    pub fn is_zz66(&self) -> bool {
        *self == Tme::Zz66
    }
}
#[doc = "Test Mode Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tmode {
    #[doc = "0: Test mode not active"]
    Zz63 = 0,
    #[doc = "1: Test mode active"]
    Zz64 = 1,
}
impl From<Tmode> for bool {
    #[inline(always)]
    fn from(variant: Tmode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMODE` reader - Test Mode Status"]
pub type TmodeR = crate::BitReader<Tmode>;
impl TmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tmode {
        match self.bits {
            false => Tmode::Zz63,
            true => Tmode::Zz64,
        }
    }
    #[doc = "Test mode not active"]
    #[inline(always)]
    pub fn is_zz63(&self) -> bool {
        *self == Tmode::Zz63
    }
    #[doc = "Test mode active"]
    #[inline(always)]
    pub fn is_zz64(&self) -> bool {
        *self == Tmode::Zz64
    }
}
#[doc = "Test Mode Entry Lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tmelock {
    #[doc = "0: FTEST register not locked from accepting writes"]
    Zz61 = 0,
    #[doc = "1: FTEST register locked from accepting writes"]
    Zz62 = 1,
}
impl From<Tmelock> for bool {
    #[inline(always)]
    fn from(variant: Tmelock) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMELOCK` reader - Test Mode Entry Lock"]
pub type TmelockR = crate::BitReader<Tmelock>;
impl TmelockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tmelock {
        match self.bits {
            false => Tmelock::Zz61,
            true => Tmelock::Zz62,
        }
    }
    #[doc = "FTEST register not locked from accepting writes"]
    #[inline(always)]
    pub fn is_zz61(&self) -> bool {
        *self == Tmelock::Zz61
    }
    #[doc = "FTEST register locked from accepting writes"]
    #[inline(always)]
    pub fn is_zz62(&self) -> bool {
        *self == Tmelock::Zz62
    }
}
impl R {
    #[doc = "Bit 0 - Test Mode Entry Control"]
    #[inline(always)]
    pub fn tmectl(&self) -> TmectlR {
        TmectlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Test Mode Entry Writable"]
    #[inline(always)]
    pub fn tmewr(&self) -> TmewrR {
        TmewrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Test Mode Entry"]
    #[inline(always)]
    pub fn tme(&self) -> TmeR {
        TmeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Test Mode Status"]
    #[inline(always)]
    pub fn tmode(&self) -> TmodeR {
        TmodeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Test Mode Entry Lock"]
    #[inline(always)]
    pub fn tmelock(&self) -> TmelockR {
        TmelockR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "Flash Test Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ftest::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FtestSpec;
impl crate::RegisterSpec for FtestSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ftest::R`](R) reader structure"]
impl crate::Readable for FtestSpec {}
#[doc = "`reset()` method sets FTEST to value 0x01"]
impl crate::Resettable for FtestSpec {
    const RESET_VALUE: u32 = 0x01;
}
