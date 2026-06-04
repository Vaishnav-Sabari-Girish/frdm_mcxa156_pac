#[doc = "Register `PE1` reader"]
pub type R = crate::R<Pe1Spec>;
#[doc = "Register `PE1` writer"]
pub type W = crate::W<Pe1Spec>;
#[doc = "Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Reserved0 {
    #[doc = "0: Not supported"]
    Disable = 0,
    #[doc = "1: Not supported"]
    EnRiseHi = 1,
    #[doc = "2: Not supported"]
    EnFallLo = 2,
    #[doc = "3: Not supported"]
    EnAny = 3,
}
impl From<Reserved0> for u8 {
    #[inline(always)]
    fn from(variant: Reserved0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Reserved0 {
    type Ux = u8;
}
impl crate::IsEnum for Reserved0 {}
#[doc = "Field `Reserved0` reader - Reserved"]
pub type Reserved0R = crate::FieldReader<Reserved0>;
impl Reserved0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Reserved0 {
        match self.bits {
            0 => Reserved0::Disable,
            1 => Reserved0::EnRiseHi,
            2 => Reserved0::EnFallLo,
            3 => Reserved0::EnAny,
            _ => unreachable!(),
        }
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Reserved0::Disable
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_en_rise_hi(&self) -> bool {
        *self == Reserved0::EnRiseHi
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_en_fall_lo(&self) -> bool {
        *self == Reserved0::EnFallLo
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_en_any(&self) -> bool {
        *self == Reserved0::EnAny
    }
}
#[doc = "Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Reserved1 {
    #[doc = "0: Not supported"]
    Disable = 0,
    #[doc = "1: Not supported"]
    EnRiseHi = 1,
    #[doc = "2: Not supported"]
    EnFallLo = 2,
    #[doc = "3: Not supported"]
    EnAny = 3,
}
impl From<Reserved1> for u8 {
    #[inline(always)]
    fn from(variant: Reserved1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Reserved1 {
    type Ux = u8;
}
impl crate::IsEnum for Reserved1 {}
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader<Reserved1>;
impl Reserved1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Reserved1 {
        match self.bits {
            0 => Reserved1::Disable,
            1 => Reserved1::EnRiseHi,
            2 => Reserved1::EnFallLo,
            3 => Reserved1::EnAny,
            _ => unreachable!(),
        }
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Reserved1::Disable
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_en_rise_hi(&self) -> bool {
        *self == Reserved1::EnRiseHi
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_en_fall_lo(&self) -> bool {
        *self == Reserved1::EnFallLo
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_en_any(&self) -> bool {
        *self == Reserved1::EnAny
    }
}
#[doc = "Wake-up Pin Enable for WUU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wupe2 {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable (detect on rising edge or high level)"]
    EnRiseHi = 1,
    #[doc = "2: Enable (detect on falling edge or low level)"]
    EnFallLo = 2,
    #[doc = "3: Enable (detect on any edge)"]
    EnAny = 3,
}
impl From<Wupe2> for u8 {
    #[inline(always)]
    fn from(variant: Wupe2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wupe2 {
    type Ux = u8;
}
impl crate::IsEnum for Wupe2 {}
#[doc = "Field `WUPE2` reader - Wake-up Pin Enable for WUU_Pn"]
pub type Wupe2R = crate::FieldReader<Wupe2>;
impl Wupe2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wupe2 {
        match self.bits {
            0 => Wupe2::Disable,
            1 => Wupe2::EnRiseHi,
            2 => Wupe2::EnFallLo,
            3 => Wupe2::EnAny,
            _ => unreachable!(),
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Wupe2::Disable
    }
    #[doc = "Enable (detect on rising edge or high level)"]
    #[inline(always)]
    pub fn is_en_rise_hi(&self) -> bool {
        *self == Wupe2::EnRiseHi
    }
    #[doc = "Enable (detect on falling edge or low level)"]
    #[inline(always)]
    pub fn is_en_fall_lo(&self) -> bool {
        *self == Wupe2::EnFallLo
    }
    #[doc = "Enable (detect on any edge)"]
    #[inline(always)]
    pub fn is_en_any(&self) -> bool {
        *self == Wupe2::EnAny
    }
}
#[doc = "Field `WUPE2` writer - Wake-up Pin Enable for WUU_Pn"]
pub type Wupe2W<'a, REG> = crate::FieldWriter<'a, REG, 2, Wupe2, crate::Safe>;
impl<'a, REG> Wupe2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe2::Disable)
    }
    #[doc = "Enable (detect on rising edge or high level)"]
    #[inline(always)]
    pub fn en_rise_hi(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe2::EnRiseHi)
    }
    #[doc = "Enable (detect on falling edge or low level)"]
    #[inline(always)]
    pub fn en_fall_lo(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe2::EnFallLo)
    }
    #[doc = "Enable (detect on any edge)"]
    #[inline(always)]
    pub fn en_any(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe2::EnAny)
    }
}
#[doc = "Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Reserved3 {
    #[doc = "0: Not supported"]
    Disable = 0,
    #[doc = "1: Not supported"]
    EnRiseHi = 1,
    #[doc = "2: Not supported"]
    EnFallLo = 2,
    #[doc = "3: Not supported"]
    EnAny = 3,
}
impl From<Reserved3> for u8 {
    #[inline(always)]
    fn from(variant: Reserved3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Reserved3 {
    type Ux = u8;
}
impl crate::IsEnum for Reserved3 {}
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::FieldReader<Reserved3>;
impl Reserved3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Reserved3 {
        match self.bits {
            0 => Reserved3::Disable,
            1 => Reserved3::EnRiseHi,
            2 => Reserved3::EnFallLo,
            3 => Reserved3::EnAny,
            _ => unreachable!(),
        }
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Reserved3::Disable
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_en_rise_hi(&self) -> bool {
        *self == Reserved3::EnRiseHi
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_en_fall_lo(&self) -> bool {
        *self == Reserved3::EnFallLo
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_en_any(&self) -> bool {
        *self == Reserved3::EnAny
    }
}
#[doc = "Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Reserved4 {
    #[doc = "0: Not supported"]
    Disable = 0,
    #[doc = "1: Not supported"]
    EnRiseHi = 1,
    #[doc = "2: Not supported"]
    EnFallLo = 2,
    #[doc = "3: Not supported"]
    EnAny = 3,
}
impl From<Reserved4> for u8 {
    #[inline(always)]
    fn from(variant: Reserved4) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Reserved4 {
    type Ux = u8;
}
impl crate::IsEnum for Reserved4 {}
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::FieldReader<Reserved4>;
impl Reserved4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Reserved4 {
        match self.bits {
            0 => Reserved4::Disable,
            1 => Reserved4::EnRiseHi,
            2 => Reserved4::EnFallLo,
            3 => Reserved4::EnAny,
            _ => unreachable!(),
        }
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Reserved4::Disable
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_en_rise_hi(&self) -> bool {
        *self == Reserved4::EnRiseHi
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_en_fall_lo(&self) -> bool {
        *self == Reserved4::EnFallLo
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_en_any(&self) -> bool {
        *self == Reserved4::EnAny
    }
}
#[doc = "Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Reserved5 {
    #[doc = "0: Not supported"]
    Disable = 0,
    #[doc = "1: Not supported"]
    EnRiseHi = 1,
    #[doc = "2: Not supported"]
    EnFallLo = 2,
    #[doc = "3: Not supported"]
    EnAny = 3,
}
impl From<Reserved5> for u8 {
    #[inline(always)]
    fn from(variant: Reserved5) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Reserved5 {
    type Ux = u8;
}
impl crate::IsEnum for Reserved5 {}
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::FieldReader<Reserved5>;
impl Reserved5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Reserved5 {
        match self.bits {
            0 => Reserved5::Disable,
            1 => Reserved5::EnRiseHi,
            2 => Reserved5::EnFallLo,
            3 => Reserved5::EnAny,
            _ => unreachable!(),
        }
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Reserved5::Disable
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_en_rise_hi(&self) -> bool {
        *self == Reserved5::EnRiseHi
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_en_fall_lo(&self) -> bool {
        *self == Reserved5::EnFallLo
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_en_any(&self) -> bool {
        *self == Reserved5::EnAny
    }
}
#[doc = "Wake-up Pin Enable for WUU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wupe6 {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable (detect on rising edge or high level)"]
    EnRiseHi = 1,
    #[doc = "2: Enable (detect on falling edge or low level)"]
    EnFallLo = 2,
    #[doc = "3: Enable (detect on any edge)"]
    EnAny = 3,
}
impl From<Wupe6> for u8 {
    #[inline(always)]
    fn from(variant: Wupe6) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wupe6 {
    type Ux = u8;
}
impl crate::IsEnum for Wupe6 {}
#[doc = "Field `WUPE6` reader - Wake-up Pin Enable for WUU_Pn"]
pub type Wupe6R = crate::FieldReader<Wupe6>;
impl Wupe6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wupe6 {
        match self.bits {
            0 => Wupe6::Disable,
            1 => Wupe6::EnRiseHi,
            2 => Wupe6::EnFallLo,
            3 => Wupe6::EnAny,
            _ => unreachable!(),
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Wupe6::Disable
    }
    #[doc = "Enable (detect on rising edge or high level)"]
    #[inline(always)]
    pub fn is_en_rise_hi(&self) -> bool {
        *self == Wupe6::EnRiseHi
    }
    #[doc = "Enable (detect on falling edge or low level)"]
    #[inline(always)]
    pub fn is_en_fall_lo(&self) -> bool {
        *self == Wupe6::EnFallLo
    }
    #[doc = "Enable (detect on any edge)"]
    #[inline(always)]
    pub fn is_en_any(&self) -> bool {
        *self == Wupe6::EnAny
    }
}
#[doc = "Field `WUPE6` writer - Wake-up Pin Enable for WUU_Pn"]
pub type Wupe6W<'a, REG> = crate::FieldWriter<'a, REG, 2, Wupe6, crate::Safe>;
impl<'a, REG> Wupe6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe6::Disable)
    }
    #[doc = "Enable (detect on rising edge or high level)"]
    #[inline(always)]
    pub fn en_rise_hi(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe6::EnRiseHi)
    }
    #[doc = "Enable (detect on falling edge or low level)"]
    #[inline(always)]
    pub fn en_fall_lo(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe6::EnFallLo)
    }
    #[doc = "Enable (detect on any edge)"]
    #[inline(always)]
    pub fn en_any(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe6::EnAny)
    }
}
#[doc = "Wake-up Pin Enable for WUU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wupe7 {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable (detect on rising edge or high level)"]
    EnRiseHi = 1,
    #[doc = "2: Enable (detect on falling edge or low level)"]
    EnFallLo = 2,
    #[doc = "3: Enable (detect on any edge)"]
    EnAny = 3,
}
impl From<Wupe7> for u8 {
    #[inline(always)]
    fn from(variant: Wupe7) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wupe7 {
    type Ux = u8;
}
impl crate::IsEnum for Wupe7 {}
#[doc = "Field `WUPE7` reader - Wake-up Pin Enable for WUU_Pn"]
pub type Wupe7R = crate::FieldReader<Wupe7>;
impl Wupe7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wupe7 {
        match self.bits {
            0 => Wupe7::Disable,
            1 => Wupe7::EnRiseHi,
            2 => Wupe7::EnFallLo,
            3 => Wupe7::EnAny,
            _ => unreachable!(),
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Wupe7::Disable
    }
    #[doc = "Enable (detect on rising edge or high level)"]
    #[inline(always)]
    pub fn is_en_rise_hi(&self) -> bool {
        *self == Wupe7::EnRiseHi
    }
    #[doc = "Enable (detect on falling edge or low level)"]
    #[inline(always)]
    pub fn is_en_fall_lo(&self) -> bool {
        *self == Wupe7::EnFallLo
    }
    #[doc = "Enable (detect on any edge)"]
    #[inline(always)]
    pub fn is_en_any(&self) -> bool {
        *self == Wupe7::EnAny
    }
}
#[doc = "Field `WUPE7` writer - Wake-up Pin Enable for WUU_Pn"]
pub type Wupe7W<'a, REG> = crate::FieldWriter<'a, REG, 2, Wupe7, crate::Safe>;
impl<'a, REG> Wupe7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe7::Disable)
    }
    #[doc = "Enable (detect on rising edge or high level)"]
    #[inline(always)]
    pub fn en_rise_hi(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe7::EnRiseHi)
    }
    #[doc = "Enable (detect on falling edge or low level)"]
    #[inline(always)]
    pub fn en_fall_lo(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe7::EnFallLo)
    }
    #[doc = "Enable (detect on any edge)"]
    #[inline(always)]
    pub fn en_any(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe7::EnAny)
    }
}
#[doc = "Wake-up Pin Enable for WUU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wupe8 {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable (detect on rising edge or high level)"]
    EnRiseHi = 1,
    #[doc = "2: Enable (detect on falling edge or low level)"]
    EnFallLo = 2,
    #[doc = "3: Enable (detect on any edge)"]
    EnAny = 3,
}
impl From<Wupe8> for u8 {
    #[inline(always)]
    fn from(variant: Wupe8) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wupe8 {
    type Ux = u8;
}
impl crate::IsEnum for Wupe8 {}
#[doc = "Field `WUPE8` reader - Wake-up Pin Enable for WUU_Pn"]
pub type Wupe8R = crate::FieldReader<Wupe8>;
impl Wupe8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wupe8 {
        match self.bits {
            0 => Wupe8::Disable,
            1 => Wupe8::EnRiseHi,
            2 => Wupe8::EnFallLo,
            3 => Wupe8::EnAny,
            _ => unreachable!(),
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Wupe8::Disable
    }
    #[doc = "Enable (detect on rising edge or high level)"]
    #[inline(always)]
    pub fn is_en_rise_hi(&self) -> bool {
        *self == Wupe8::EnRiseHi
    }
    #[doc = "Enable (detect on falling edge or low level)"]
    #[inline(always)]
    pub fn is_en_fall_lo(&self) -> bool {
        *self == Wupe8::EnFallLo
    }
    #[doc = "Enable (detect on any edge)"]
    #[inline(always)]
    pub fn is_en_any(&self) -> bool {
        *self == Wupe8::EnAny
    }
}
#[doc = "Field `WUPE8` writer - Wake-up Pin Enable for WUU_Pn"]
pub type Wupe8W<'a, REG> = crate::FieldWriter<'a, REG, 2, Wupe8, crate::Safe>;
impl<'a, REG> Wupe8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe8::Disable)
    }
    #[doc = "Enable (detect on rising edge or high level)"]
    #[inline(always)]
    pub fn en_rise_hi(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe8::EnRiseHi)
    }
    #[doc = "Enable (detect on falling edge or low level)"]
    #[inline(always)]
    pub fn en_fall_lo(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe8::EnFallLo)
    }
    #[doc = "Enable (detect on any edge)"]
    #[inline(always)]
    pub fn en_any(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe8::EnAny)
    }
}
#[doc = "Wake-up Pin Enable for WUU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wupe9 {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable (detect on rising edge or high level)"]
    EnRiseHi = 1,
    #[doc = "2: Enable (detect on falling edge or low level)"]
    EnFallLo = 2,
    #[doc = "3: Enable (detect on any edge)"]
    EnAny = 3,
}
impl From<Wupe9> for u8 {
    #[inline(always)]
    fn from(variant: Wupe9) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wupe9 {
    type Ux = u8;
}
impl crate::IsEnum for Wupe9 {}
#[doc = "Field `WUPE9` reader - Wake-up Pin Enable for WUU_Pn"]
pub type Wupe9R = crate::FieldReader<Wupe9>;
impl Wupe9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wupe9 {
        match self.bits {
            0 => Wupe9::Disable,
            1 => Wupe9::EnRiseHi,
            2 => Wupe9::EnFallLo,
            3 => Wupe9::EnAny,
            _ => unreachable!(),
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Wupe9::Disable
    }
    #[doc = "Enable (detect on rising edge or high level)"]
    #[inline(always)]
    pub fn is_en_rise_hi(&self) -> bool {
        *self == Wupe9::EnRiseHi
    }
    #[doc = "Enable (detect on falling edge or low level)"]
    #[inline(always)]
    pub fn is_en_fall_lo(&self) -> bool {
        *self == Wupe9::EnFallLo
    }
    #[doc = "Enable (detect on any edge)"]
    #[inline(always)]
    pub fn is_en_any(&self) -> bool {
        *self == Wupe9::EnAny
    }
}
#[doc = "Field `WUPE9` writer - Wake-up Pin Enable for WUU_Pn"]
pub type Wupe9W<'a, REG> = crate::FieldWriter<'a, REG, 2, Wupe9, crate::Safe>;
impl<'a, REG> Wupe9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe9::Disable)
    }
    #[doc = "Enable (detect on rising edge or high level)"]
    #[inline(always)]
    pub fn en_rise_hi(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe9::EnRiseHi)
    }
    #[doc = "Enable (detect on falling edge or low level)"]
    #[inline(always)]
    pub fn en_fall_lo(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe9::EnFallLo)
    }
    #[doc = "Enable (detect on any edge)"]
    #[inline(always)]
    pub fn en_any(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe9::EnAny)
    }
}
#[doc = "Wake-up Pin Enable for WUU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wupe10 {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable (detect on rising edge or high level)"]
    EnRiseHi = 1,
    #[doc = "2: Enable (detect on falling edge or low level)"]
    EnFallLo = 2,
    #[doc = "3: Enable (detect on any edge)"]
    EnAny = 3,
}
impl From<Wupe10> for u8 {
    #[inline(always)]
    fn from(variant: Wupe10) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wupe10 {
    type Ux = u8;
}
impl crate::IsEnum for Wupe10 {}
#[doc = "Field `WUPE10` reader - Wake-up Pin Enable for WUU_Pn"]
pub type Wupe10R = crate::FieldReader<Wupe10>;
impl Wupe10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wupe10 {
        match self.bits {
            0 => Wupe10::Disable,
            1 => Wupe10::EnRiseHi,
            2 => Wupe10::EnFallLo,
            3 => Wupe10::EnAny,
            _ => unreachable!(),
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Wupe10::Disable
    }
    #[doc = "Enable (detect on rising edge or high level)"]
    #[inline(always)]
    pub fn is_en_rise_hi(&self) -> bool {
        *self == Wupe10::EnRiseHi
    }
    #[doc = "Enable (detect on falling edge or low level)"]
    #[inline(always)]
    pub fn is_en_fall_lo(&self) -> bool {
        *self == Wupe10::EnFallLo
    }
    #[doc = "Enable (detect on any edge)"]
    #[inline(always)]
    pub fn is_en_any(&self) -> bool {
        *self == Wupe10::EnAny
    }
}
#[doc = "Field `WUPE10` writer - Wake-up Pin Enable for WUU_Pn"]
pub type Wupe10W<'a, REG> = crate::FieldWriter<'a, REG, 2, Wupe10, crate::Safe>;
impl<'a, REG> Wupe10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe10::Disable)
    }
    #[doc = "Enable (detect on rising edge or high level)"]
    #[inline(always)]
    pub fn en_rise_hi(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe10::EnRiseHi)
    }
    #[doc = "Enable (detect on falling edge or low level)"]
    #[inline(always)]
    pub fn en_fall_lo(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe10::EnFallLo)
    }
    #[doc = "Enable (detect on any edge)"]
    #[inline(always)]
    pub fn en_any(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe10::EnAny)
    }
}
#[doc = "Wake-up Pin Enable for WUU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wupe11 {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable (detect on rising edge or high level)"]
    EnRiseHi = 1,
    #[doc = "2: Enable (detect on falling edge or low level)"]
    EnFallLo = 2,
    #[doc = "3: Enable (detect on any edge)"]
    EnAny = 3,
}
impl From<Wupe11> for u8 {
    #[inline(always)]
    fn from(variant: Wupe11) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wupe11 {
    type Ux = u8;
}
impl crate::IsEnum for Wupe11 {}
#[doc = "Field `WUPE11` reader - Wake-up Pin Enable for WUU_Pn"]
pub type Wupe11R = crate::FieldReader<Wupe11>;
impl Wupe11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wupe11 {
        match self.bits {
            0 => Wupe11::Disable,
            1 => Wupe11::EnRiseHi,
            2 => Wupe11::EnFallLo,
            3 => Wupe11::EnAny,
            _ => unreachable!(),
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Wupe11::Disable
    }
    #[doc = "Enable (detect on rising edge or high level)"]
    #[inline(always)]
    pub fn is_en_rise_hi(&self) -> bool {
        *self == Wupe11::EnRiseHi
    }
    #[doc = "Enable (detect on falling edge or low level)"]
    #[inline(always)]
    pub fn is_en_fall_lo(&self) -> bool {
        *self == Wupe11::EnFallLo
    }
    #[doc = "Enable (detect on any edge)"]
    #[inline(always)]
    pub fn is_en_any(&self) -> bool {
        *self == Wupe11::EnAny
    }
}
#[doc = "Field `WUPE11` writer - Wake-up Pin Enable for WUU_Pn"]
pub type Wupe11W<'a, REG> = crate::FieldWriter<'a, REG, 2, Wupe11, crate::Safe>;
impl<'a, REG> Wupe11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe11::Disable)
    }
    #[doc = "Enable (detect on rising edge or high level)"]
    #[inline(always)]
    pub fn en_rise_hi(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe11::EnRiseHi)
    }
    #[doc = "Enable (detect on falling edge or low level)"]
    #[inline(always)]
    pub fn en_fall_lo(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe11::EnFallLo)
    }
    #[doc = "Enable (detect on any edge)"]
    #[inline(always)]
    pub fn en_any(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe11::EnAny)
    }
}
#[doc = "Wake-up Pin Enable for WUU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wupe12 {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable (detect on rising edge or high level)"]
    EnRiseHi = 1,
    #[doc = "2: Enable (detect on falling edge or low level)"]
    EnFallLo = 2,
    #[doc = "3: Enable (detect on any edge)"]
    EnAny = 3,
}
impl From<Wupe12> for u8 {
    #[inline(always)]
    fn from(variant: Wupe12) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wupe12 {
    type Ux = u8;
}
impl crate::IsEnum for Wupe12 {}
#[doc = "Field `WUPE12` reader - Wake-up Pin Enable for WUU_Pn"]
pub type Wupe12R = crate::FieldReader<Wupe12>;
impl Wupe12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wupe12 {
        match self.bits {
            0 => Wupe12::Disable,
            1 => Wupe12::EnRiseHi,
            2 => Wupe12::EnFallLo,
            3 => Wupe12::EnAny,
            _ => unreachable!(),
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Wupe12::Disable
    }
    #[doc = "Enable (detect on rising edge or high level)"]
    #[inline(always)]
    pub fn is_en_rise_hi(&self) -> bool {
        *self == Wupe12::EnRiseHi
    }
    #[doc = "Enable (detect on falling edge or low level)"]
    #[inline(always)]
    pub fn is_en_fall_lo(&self) -> bool {
        *self == Wupe12::EnFallLo
    }
    #[doc = "Enable (detect on any edge)"]
    #[inline(always)]
    pub fn is_en_any(&self) -> bool {
        *self == Wupe12::EnAny
    }
}
#[doc = "Field `WUPE12` writer - Wake-up Pin Enable for WUU_Pn"]
pub type Wupe12W<'a, REG> = crate::FieldWriter<'a, REG, 2, Wupe12, crate::Safe>;
impl<'a, REG> Wupe12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe12::Disable)
    }
    #[doc = "Enable (detect on rising edge or high level)"]
    #[inline(always)]
    pub fn en_rise_hi(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe12::EnRiseHi)
    }
    #[doc = "Enable (detect on falling edge or low level)"]
    #[inline(always)]
    pub fn en_fall_lo(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe12::EnFallLo)
    }
    #[doc = "Enable (detect on any edge)"]
    #[inline(always)]
    pub fn en_any(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe12::EnAny)
    }
}
#[doc = "Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Reserved13 {
    #[doc = "0: Not supported"]
    Disable = 0,
    #[doc = "1: Not supported"]
    EnRiseHi = 1,
    #[doc = "2: Not supported"]
    EnFallLo = 2,
    #[doc = "3: Not supported"]
    EnAny = 3,
}
impl From<Reserved13> for u8 {
    #[inline(always)]
    fn from(variant: Reserved13) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Reserved13 {
    type Ux = u8;
}
impl crate::IsEnum for Reserved13 {}
#[doc = "Field `Reserved13` reader - Reserved"]
pub type Reserved13R = crate::FieldReader<Reserved13>;
impl Reserved13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Reserved13 {
        match self.bits {
            0 => Reserved13::Disable,
            1 => Reserved13::EnRiseHi,
            2 => Reserved13::EnFallLo,
            3 => Reserved13::EnAny,
            _ => unreachable!(),
        }
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Reserved13::Disable
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_en_rise_hi(&self) -> bool {
        *self == Reserved13::EnRiseHi
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_en_fall_lo(&self) -> bool {
        *self == Reserved13::EnFallLo
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_en_any(&self) -> bool {
        *self == Reserved13::EnAny
    }
}
#[doc = "Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Reserved14 {
    #[doc = "0: Not supported"]
    Disable = 0,
    #[doc = "1: Not supported"]
    EnRiseHi = 1,
    #[doc = "2: Not supported"]
    EnFallLo = 2,
    #[doc = "3: Not supported"]
    EnAny = 3,
}
impl From<Reserved14> for u8 {
    #[inline(always)]
    fn from(variant: Reserved14) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Reserved14 {
    type Ux = u8;
}
impl crate::IsEnum for Reserved14 {}
#[doc = "Field `Reserved14` reader - Reserved"]
pub type Reserved14R = crate::FieldReader<Reserved14>;
impl Reserved14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Reserved14 {
        match self.bits {
            0 => Reserved14::Disable,
            1 => Reserved14::EnRiseHi,
            2 => Reserved14::EnFallLo,
            3 => Reserved14::EnAny,
            _ => unreachable!(),
        }
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Reserved14::Disable
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_en_rise_hi(&self) -> bool {
        *self == Reserved14::EnRiseHi
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_en_fall_lo(&self) -> bool {
        *self == Reserved14::EnFallLo
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_en_any(&self) -> bool {
        *self == Reserved14::EnAny
    }
}
#[doc = "Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Reserved15 {
    #[doc = "0: Not supported"]
    Disable = 0,
    #[doc = "1: Not supported"]
    EnRiseHi = 1,
    #[doc = "2: Not supported"]
    EnFallLo = 2,
    #[doc = "3: Not supported"]
    EnAny = 3,
}
impl From<Reserved15> for u8 {
    #[inline(always)]
    fn from(variant: Reserved15) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Reserved15 {
    type Ux = u8;
}
impl crate::IsEnum for Reserved15 {}
#[doc = "Field `Reserved15` reader - Reserved"]
pub type Reserved15R = crate::FieldReader<Reserved15>;
impl Reserved15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Reserved15 {
        match self.bits {
            0 => Reserved15::Disable,
            1 => Reserved15::EnRiseHi,
            2 => Reserved15::EnFallLo,
            3 => Reserved15::EnAny,
            _ => unreachable!(),
        }
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Reserved15::Disable
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_en_rise_hi(&self) -> bool {
        *self == Reserved15::EnRiseHi
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_en_fall_lo(&self) -> bool {
        *self == Reserved15::EnFallLo
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_en_any(&self) -> bool {
        *self == Reserved15::EnAny
    }
}
impl R {
    #[doc = "Bits 0:1 - Reserved"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Wake-up Pin Enable for WUU_Pn"]
    #[inline(always)]
    pub fn wupe2(&self) -> Wupe2R {
        Wupe2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Wake-up Pin Enable for WUU_Pn"]
    #[inline(always)]
    pub fn wupe6(&self) -> Wupe6R {
        Wupe6R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Wake-up Pin Enable for WUU_Pn"]
    #[inline(always)]
    pub fn wupe7(&self) -> Wupe7R {
        Wupe7R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Wake-up Pin Enable for WUU_Pn"]
    #[inline(always)]
    pub fn wupe8(&self) -> Wupe8R {
        Wupe8R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Wake-up Pin Enable for WUU_Pn"]
    #[inline(always)]
    pub fn wupe9(&self) -> Wupe9R {
        Wupe9R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Wake-up Pin Enable for WUU_Pn"]
    #[inline(always)]
    pub fn wupe10(&self) -> Wupe10R {
        Wupe10R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Wake-up Pin Enable for WUU_Pn"]
    #[inline(always)]
    pub fn wupe11(&self) -> Wupe11R {
        Wupe11R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Wake-up Pin Enable for WUU_Pn"]
    #[inline(always)]
    pub fn wupe12(&self) -> Wupe12R {
        Wupe12R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Reserved"]
    #[inline(always)]
    pub fn reserved13(&self) -> Reserved13R {
        Reserved13R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Reserved"]
    #[inline(always)]
    pub fn reserved14(&self) -> Reserved14R {
        Reserved14R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Reserved"]
    #[inline(always)]
    pub fn reserved15(&self) -> Reserved15R {
        Reserved15R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 4:5 - Wake-up Pin Enable for WUU_Pn"]
    #[inline(always)]
    pub fn wupe2(&mut self) -> Wupe2W<'_, Pe1Spec> {
        Wupe2W::new(self, 4)
    }
    #[doc = "Bits 12:13 - Wake-up Pin Enable for WUU_Pn"]
    #[inline(always)]
    pub fn wupe6(&mut self) -> Wupe6W<'_, Pe1Spec> {
        Wupe6W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Wake-up Pin Enable for WUU_Pn"]
    #[inline(always)]
    pub fn wupe7(&mut self) -> Wupe7W<'_, Pe1Spec> {
        Wupe7W::new(self, 14)
    }
    #[doc = "Bits 16:17 - Wake-up Pin Enable for WUU_Pn"]
    #[inline(always)]
    pub fn wupe8(&mut self) -> Wupe8W<'_, Pe1Spec> {
        Wupe8W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Wake-up Pin Enable for WUU_Pn"]
    #[inline(always)]
    pub fn wupe9(&mut self) -> Wupe9W<'_, Pe1Spec> {
        Wupe9W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Wake-up Pin Enable for WUU_Pn"]
    #[inline(always)]
    pub fn wupe10(&mut self) -> Wupe10W<'_, Pe1Spec> {
        Wupe10W::new(self, 20)
    }
    #[doc = "Bits 22:23 - Wake-up Pin Enable for WUU_Pn"]
    #[inline(always)]
    pub fn wupe11(&mut self) -> Wupe11W<'_, Pe1Spec> {
        Wupe11W::new(self, 22)
    }
    #[doc = "Bits 24:25 - Wake-up Pin Enable for WUU_Pn"]
    #[inline(always)]
    pub fn wupe12(&mut self) -> Wupe12W<'_, Pe1Spec> {
        Wupe12W::new(self, 24)
    }
}
#[doc = "Pin Enable 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pe1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pe1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pe1Spec;
impl crate::RegisterSpec for Pe1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pe1::R`](R) reader structure"]
impl crate::Readable for Pe1Spec {}
#[doc = "`write(|w| ..)` method takes [`pe1::W`](W) writer structure"]
impl crate::Writable for Pe1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PE1 to value 0"]
impl crate::Resettable for Pe1Spec {}
