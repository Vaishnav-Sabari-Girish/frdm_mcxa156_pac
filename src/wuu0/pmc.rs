#[doc = "Register `PMC` reader"]
pub type R = crate::R<PmcSpec>;
#[doc = "Register `PMC` writer"]
pub type W = crate::W<PmcSpec>;
#[doc = "Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reserved0 {
    #[doc = "0: Not supported"]
    LowPwrOnly = 0,
    #[doc = "1: Not supported"]
    AnyPwr = 1,
}
impl From<Reserved0> for bool {
    #[inline(always)]
    fn from(variant: Reserved0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Reserved0` reader - Reserved"]
pub type Reserved0R = crate::BitReader<Reserved0>;
impl Reserved0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Reserved0 {
        match self.bits {
            false => Reserved0::LowPwrOnly,
            true => Reserved0::AnyPwr,
        }
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_low_pwr_only(&self) -> bool {
        *self == Reserved0::LowPwrOnly
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_any_pwr(&self) -> bool {
        *self == Reserved0::AnyPwr
    }
}
#[doc = "Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reserved1 {
    #[doc = "0: Not supported"]
    LowPwrOnly = 0,
    #[doc = "1: Not supported"]
    AnyPwr = 1,
}
impl From<Reserved1> for bool {
    #[inline(always)]
    fn from(variant: Reserved1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::BitReader<Reserved1>;
impl Reserved1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Reserved1 {
        match self.bits {
            false => Reserved1::LowPwrOnly,
            true => Reserved1::AnyPwr,
        }
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_low_pwr_only(&self) -> bool {
        *self == Reserved1::LowPwrOnly
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_any_pwr(&self) -> bool {
        *self == Reserved1::AnyPwr
    }
}
#[doc = "Wake-up Pin Mode Configuration for WUU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wupmc2 {
    #[doc = "0: Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    LowPwrOnly = 0,
    #[doc = "1: Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    AnyPwr = 1,
}
impl From<Wupmc2> for bool {
    #[inline(always)]
    fn from(variant: Wupmc2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUPMC2` reader - Wake-up Pin Mode Configuration for WUU_Pn"]
pub type Wupmc2R = crate::BitReader<Wupmc2>;
impl Wupmc2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wupmc2 {
        match self.bits {
            false => Wupmc2::LowPwrOnly,
            true => Wupmc2::AnyPwr,
        }
    }
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn is_low_pwr_only(&self) -> bool {
        *self == Wupmc2::LowPwrOnly
    }
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn is_any_pwr(&self) -> bool {
        *self == Wupmc2::AnyPwr
    }
}
#[doc = "Field `WUPMC2` writer - Wake-up Pin Mode Configuration for WUU_Pn"]
pub type Wupmc2W<'a, REG> = crate::BitWriter<'a, REG, Wupmc2>;
impl<'a, REG> Wupmc2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn low_pwr_only(self) -> &'a mut crate::W<REG> {
        self.variant(Wupmc2::LowPwrOnly)
    }
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn any_pwr(self) -> &'a mut crate::W<REG> {
        self.variant(Wupmc2::AnyPwr)
    }
}
#[doc = "Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reserved3 {
    #[doc = "0: Not supported"]
    LowPwrOnly = 0,
    #[doc = "1: Not supported"]
    AnyPwr = 1,
}
impl From<Reserved3> for bool {
    #[inline(always)]
    fn from(variant: Reserved3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::BitReader<Reserved3>;
impl Reserved3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Reserved3 {
        match self.bits {
            false => Reserved3::LowPwrOnly,
            true => Reserved3::AnyPwr,
        }
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_low_pwr_only(&self) -> bool {
        *self == Reserved3::LowPwrOnly
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_any_pwr(&self) -> bool {
        *self == Reserved3::AnyPwr
    }
}
#[doc = "Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reserved4 {
    #[doc = "0: Not supported"]
    LowPwrOnly = 0,
    #[doc = "1: Not supported"]
    AnyPwr = 1,
}
impl From<Reserved4> for bool {
    #[inline(always)]
    fn from(variant: Reserved4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::BitReader<Reserved4>;
impl Reserved4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Reserved4 {
        match self.bits {
            false => Reserved4::LowPwrOnly,
            true => Reserved4::AnyPwr,
        }
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_low_pwr_only(&self) -> bool {
        *self == Reserved4::LowPwrOnly
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_any_pwr(&self) -> bool {
        *self == Reserved4::AnyPwr
    }
}
#[doc = "Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reserved5 {
    #[doc = "0: Not supported"]
    LowPwrOnly = 0,
    #[doc = "1: Not supported"]
    AnyPwr = 1,
}
impl From<Reserved5> for bool {
    #[inline(always)]
    fn from(variant: Reserved5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::BitReader<Reserved5>;
impl Reserved5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Reserved5 {
        match self.bits {
            false => Reserved5::LowPwrOnly,
            true => Reserved5::AnyPwr,
        }
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_low_pwr_only(&self) -> bool {
        *self == Reserved5::LowPwrOnly
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_any_pwr(&self) -> bool {
        *self == Reserved5::AnyPwr
    }
}
#[doc = "Wake-up Pin Mode Configuration for WUU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wupmc6 {
    #[doc = "0: Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    LowPwrOnly = 0,
    #[doc = "1: Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    AnyPwr = 1,
}
impl From<Wupmc6> for bool {
    #[inline(always)]
    fn from(variant: Wupmc6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUPMC6` reader - Wake-up Pin Mode Configuration for WUU_Pn"]
pub type Wupmc6R = crate::BitReader<Wupmc6>;
impl Wupmc6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wupmc6 {
        match self.bits {
            false => Wupmc6::LowPwrOnly,
            true => Wupmc6::AnyPwr,
        }
    }
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn is_low_pwr_only(&self) -> bool {
        *self == Wupmc6::LowPwrOnly
    }
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn is_any_pwr(&self) -> bool {
        *self == Wupmc6::AnyPwr
    }
}
#[doc = "Field `WUPMC6` writer - Wake-up Pin Mode Configuration for WUU_Pn"]
pub type Wupmc6W<'a, REG> = crate::BitWriter<'a, REG, Wupmc6>;
impl<'a, REG> Wupmc6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn low_pwr_only(self) -> &'a mut crate::W<REG> {
        self.variant(Wupmc6::LowPwrOnly)
    }
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn any_pwr(self) -> &'a mut crate::W<REG> {
        self.variant(Wupmc6::AnyPwr)
    }
}
#[doc = "Wake-up Pin Mode Configuration for WUU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wupmc7 {
    #[doc = "0: Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    LowPwrOnly = 0,
    #[doc = "1: Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    AnyPwr = 1,
}
impl From<Wupmc7> for bool {
    #[inline(always)]
    fn from(variant: Wupmc7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUPMC7` reader - Wake-up Pin Mode Configuration for WUU_Pn"]
pub type Wupmc7R = crate::BitReader<Wupmc7>;
impl Wupmc7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wupmc7 {
        match self.bits {
            false => Wupmc7::LowPwrOnly,
            true => Wupmc7::AnyPwr,
        }
    }
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn is_low_pwr_only(&self) -> bool {
        *self == Wupmc7::LowPwrOnly
    }
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn is_any_pwr(&self) -> bool {
        *self == Wupmc7::AnyPwr
    }
}
#[doc = "Field `WUPMC7` writer - Wake-up Pin Mode Configuration for WUU_Pn"]
pub type Wupmc7W<'a, REG> = crate::BitWriter<'a, REG, Wupmc7>;
impl<'a, REG> Wupmc7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn low_pwr_only(self) -> &'a mut crate::W<REG> {
        self.variant(Wupmc7::LowPwrOnly)
    }
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn any_pwr(self) -> &'a mut crate::W<REG> {
        self.variant(Wupmc7::AnyPwr)
    }
}
#[doc = "Wake-up Pin Mode Configuration for WUU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wupmc8 {
    #[doc = "0: Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    LowPwrOnly = 0,
    #[doc = "1: Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    AnyPwr = 1,
}
impl From<Wupmc8> for bool {
    #[inline(always)]
    fn from(variant: Wupmc8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUPMC8` reader - Wake-up Pin Mode Configuration for WUU_Pn"]
pub type Wupmc8R = crate::BitReader<Wupmc8>;
impl Wupmc8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wupmc8 {
        match self.bits {
            false => Wupmc8::LowPwrOnly,
            true => Wupmc8::AnyPwr,
        }
    }
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn is_low_pwr_only(&self) -> bool {
        *self == Wupmc8::LowPwrOnly
    }
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn is_any_pwr(&self) -> bool {
        *self == Wupmc8::AnyPwr
    }
}
#[doc = "Field `WUPMC8` writer - Wake-up Pin Mode Configuration for WUU_Pn"]
pub type Wupmc8W<'a, REG> = crate::BitWriter<'a, REG, Wupmc8>;
impl<'a, REG> Wupmc8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn low_pwr_only(self) -> &'a mut crate::W<REG> {
        self.variant(Wupmc8::LowPwrOnly)
    }
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn any_pwr(self) -> &'a mut crate::W<REG> {
        self.variant(Wupmc8::AnyPwr)
    }
}
#[doc = "Wake-up Pin Mode Configuration for WUU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wupmc9 {
    #[doc = "0: Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    LowPwrOnly = 0,
    #[doc = "1: Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    AnyPwr = 1,
}
impl From<Wupmc9> for bool {
    #[inline(always)]
    fn from(variant: Wupmc9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUPMC9` reader - Wake-up Pin Mode Configuration for WUU_Pn"]
pub type Wupmc9R = crate::BitReader<Wupmc9>;
impl Wupmc9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wupmc9 {
        match self.bits {
            false => Wupmc9::LowPwrOnly,
            true => Wupmc9::AnyPwr,
        }
    }
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn is_low_pwr_only(&self) -> bool {
        *self == Wupmc9::LowPwrOnly
    }
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn is_any_pwr(&self) -> bool {
        *self == Wupmc9::AnyPwr
    }
}
#[doc = "Field `WUPMC9` writer - Wake-up Pin Mode Configuration for WUU_Pn"]
pub type Wupmc9W<'a, REG> = crate::BitWriter<'a, REG, Wupmc9>;
impl<'a, REG> Wupmc9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn low_pwr_only(self) -> &'a mut crate::W<REG> {
        self.variant(Wupmc9::LowPwrOnly)
    }
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn any_pwr(self) -> &'a mut crate::W<REG> {
        self.variant(Wupmc9::AnyPwr)
    }
}
#[doc = "Wake-up Pin Mode Configuration for WUU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wupmc10 {
    #[doc = "0: Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    LowPwrOnly = 0,
    #[doc = "1: Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    AnyPwr = 1,
}
impl From<Wupmc10> for bool {
    #[inline(always)]
    fn from(variant: Wupmc10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUPMC10` reader - Wake-up Pin Mode Configuration for WUU_Pn"]
pub type Wupmc10R = crate::BitReader<Wupmc10>;
impl Wupmc10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wupmc10 {
        match self.bits {
            false => Wupmc10::LowPwrOnly,
            true => Wupmc10::AnyPwr,
        }
    }
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn is_low_pwr_only(&self) -> bool {
        *self == Wupmc10::LowPwrOnly
    }
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn is_any_pwr(&self) -> bool {
        *self == Wupmc10::AnyPwr
    }
}
#[doc = "Field `WUPMC10` writer - Wake-up Pin Mode Configuration for WUU_Pn"]
pub type Wupmc10W<'a, REG> = crate::BitWriter<'a, REG, Wupmc10>;
impl<'a, REG> Wupmc10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn low_pwr_only(self) -> &'a mut crate::W<REG> {
        self.variant(Wupmc10::LowPwrOnly)
    }
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn any_pwr(self) -> &'a mut crate::W<REG> {
        self.variant(Wupmc10::AnyPwr)
    }
}
#[doc = "Wake-up Pin Mode Configuration for WUU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wupmc11 {
    #[doc = "0: Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    LowPwrOnly = 0,
    #[doc = "1: Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    AnyPwr = 1,
}
impl From<Wupmc11> for bool {
    #[inline(always)]
    fn from(variant: Wupmc11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUPMC11` reader - Wake-up Pin Mode Configuration for WUU_Pn"]
pub type Wupmc11R = crate::BitReader<Wupmc11>;
impl Wupmc11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wupmc11 {
        match self.bits {
            false => Wupmc11::LowPwrOnly,
            true => Wupmc11::AnyPwr,
        }
    }
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn is_low_pwr_only(&self) -> bool {
        *self == Wupmc11::LowPwrOnly
    }
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn is_any_pwr(&self) -> bool {
        *self == Wupmc11::AnyPwr
    }
}
#[doc = "Field `WUPMC11` writer - Wake-up Pin Mode Configuration for WUU_Pn"]
pub type Wupmc11W<'a, REG> = crate::BitWriter<'a, REG, Wupmc11>;
impl<'a, REG> Wupmc11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn low_pwr_only(self) -> &'a mut crate::W<REG> {
        self.variant(Wupmc11::LowPwrOnly)
    }
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn any_pwr(self) -> &'a mut crate::W<REG> {
        self.variant(Wupmc11::AnyPwr)
    }
}
#[doc = "Wake-up Pin Mode Configuration for WUU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wupmc12 {
    #[doc = "0: Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    LowPwrOnly = 0,
    #[doc = "1: Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    AnyPwr = 1,
}
impl From<Wupmc12> for bool {
    #[inline(always)]
    fn from(variant: Wupmc12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUPMC12` reader - Wake-up Pin Mode Configuration for WUU_Pn"]
pub type Wupmc12R = crate::BitReader<Wupmc12>;
impl Wupmc12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wupmc12 {
        match self.bits {
            false => Wupmc12::LowPwrOnly,
            true => Wupmc12::AnyPwr,
        }
    }
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn is_low_pwr_only(&self) -> bool {
        *self == Wupmc12::LowPwrOnly
    }
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn is_any_pwr(&self) -> bool {
        *self == Wupmc12::AnyPwr
    }
}
#[doc = "Field `WUPMC12` writer - Wake-up Pin Mode Configuration for WUU_Pn"]
pub type Wupmc12W<'a, REG> = crate::BitWriter<'a, REG, Wupmc12>;
impl<'a, REG> Wupmc12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn low_pwr_only(self) -> &'a mut crate::W<REG> {
        self.variant(Wupmc12::LowPwrOnly)
    }
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn any_pwr(self) -> &'a mut crate::W<REG> {
        self.variant(Wupmc12::AnyPwr)
    }
}
#[doc = "Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reserved13 {
    #[doc = "0: Not supported"]
    LowPwrOnly = 0,
    #[doc = "1: Not supported"]
    AnyPwr = 1,
}
impl From<Reserved13> for bool {
    #[inline(always)]
    fn from(variant: Reserved13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Reserved13` reader - Reserved"]
pub type Reserved13R = crate::BitReader<Reserved13>;
impl Reserved13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Reserved13 {
        match self.bits {
            false => Reserved13::LowPwrOnly,
            true => Reserved13::AnyPwr,
        }
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_low_pwr_only(&self) -> bool {
        *self == Reserved13::LowPwrOnly
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_any_pwr(&self) -> bool {
        *self == Reserved13::AnyPwr
    }
}
#[doc = "Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reserved14 {
    #[doc = "0: Not supported"]
    LowPwrOnly = 0,
    #[doc = "1: Not supported"]
    AnyPwr = 1,
}
impl From<Reserved14> for bool {
    #[inline(always)]
    fn from(variant: Reserved14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Reserved14` reader - Reserved"]
pub type Reserved14R = crate::BitReader<Reserved14>;
impl Reserved14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Reserved14 {
        match self.bits {
            false => Reserved14::LowPwrOnly,
            true => Reserved14::AnyPwr,
        }
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_low_pwr_only(&self) -> bool {
        *self == Reserved14::LowPwrOnly
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_any_pwr(&self) -> bool {
        *self == Reserved14::AnyPwr
    }
}
#[doc = "Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reserved15 {
    #[doc = "0: Not supported"]
    LowPwrOnly = 0,
    #[doc = "1: Not supported"]
    AnyPwr = 1,
}
impl From<Reserved15> for bool {
    #[inline(always)]
    fn from(variant: Reserved15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Reserved15` reader - Reserved"]
pub type Reserved15R = crate::BitReader<Reserved15>;
impl Reserved15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Reserved15 {
        match self.bits {
            false => Reserved15::LowPwrOnly,
            true => Reserved15::AnyPwr,
        }
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_low_pwr_only(&self) -> bool {
        *self == Reserved15::LowPwrOnly
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_any_pwr(&self) -> bool {
        *self == Reserved15::AnyPwr
    }
}
#[doc = "Wake-up Pin Mode Configuration for WUU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wupmc16 {
    #[doc = "0: Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    LowPwrOnly = 0,
    #[doc = "1: Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    AnyPwr = 1,
}
impl From<Wupmc16> for bool {
    #[inline(always)]
    fn from(variant: Wupmc16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUPMC16` reader - Wake-up Pin Mode Configuration for WUU_Pn"]
pub type Wupmc16R = crate::BitReader<Wupmc16>;
impl Wupmc16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wupmc16 {
        match self.bits {
            false => Wupmc16::LowPwrOnly,
            true => Wupmc16::AnyPwr,
        }
    }
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn is_low_pwr_only(&self) -> bool {
        *self == Wupmc16::LowPwrOnly
    }
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn is_any_pwr(&self) -> bool {
        *self == Wupmc16::AnyPwr
    }
}
#[doc = "Field `WUPMC16` writer - Wake-up Pin Mode Configuration for WUU_Pn"]
pub type Wupmc16W<'a, REG> = crate::BitWriter<'a, REG, Wupmc16>;
impl<'a, REG> Wupmc16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn low_pwr_only(self) -> &'a mut crate::W<REG> {
        self.variant(Wupmc16::LowPwrOnly)
    }
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn any_pwr(self) -> &'a mut crate::W<REG> {
        self.variant(Wupmc16::AnyPwr)
    }
}
#[doc = "Wake-up Pin Mode Configuration for WUU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wupmc17 {
    #[doc = "0: Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    LowPwrOnly = 0,
    #[doc = "1: Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    AnyPwr = 1,
}
impl From<Wupmc17> for bool {
    #[inline(always)]
    fn from(variant: Wupmc17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUPMC17` reader - Wake-up Pin Mode Configuration for WUU_Pn"]
pub type Wupmc17R = crate::BitReader<Wupmc17>;
impl Wupmc17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wupmc17 {
        match self.bits {
            false => Wupmc17::LowPwrOnly,
            true => Wupmc17::AnyPwr,
        }
    }
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn is_low_pwr_only(&self) -> bool {
        *self == Wupmc17::LowPwrOnly
    }
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn is_any_pwr(&self) -> bool {
        *self == Wupmc17::AnyPwr
    }
}
#[doc = "Field `WUPMC17` writer - Wake-up Pin Mode Configuration for WUU_Pn"]
pub type Wupmc17W<'a, REG> = crate::BitWriter<'a, REG, Wupmc17>;
impl<'a, REG> Wupmc17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn low_pwr_only(self) -> &'a mut crate::W<REG> {
        self.variant(Wupmc17::LowPwrOnly)
    }
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn any_pwr(self) -> &'a mut crate::W<REG> {
        self.variant(Wupmc17::AnyPwr)
    }
}
#[doc = "Wake-up Pin Mode Configuration for WUU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wupmc18 {
    #[doc = "0: Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    LowPwrOnly = 0,
    #[doc = "1: Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    AnyPwr = 1,
}
impl From<Wupmc18> for bool {
    #[inline(always)]
    fn from(variant: Wupmc18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUPMC18` reader - Wake-up Pin Mode Configuration for WUU_Pn"]
pub type Wupmc18R = crate::BitReader<Wupmc18>;
impl Wupmc18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wupmc18 {
        match self.bits {
            false => Wupmc18::LowPwrOnly,
            true => Wupmc18::AnyPwr,
        }
    }
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn is_low_pwr_only(&self) -> bool {
        *self == Wupmc18::LowPwrOnly
    }
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn is_any_pwr(&self) -> bool {
        *self == Wupmc18::AnyPwr
    }
}
#[doc = "Field `WUPMC18` writer - Wake-up Pin Mode Configuration for WUU_Pn"]
pub type Wupmc18W<'a, REG> = crate::BitWriter<'a, REG, Wupmc18>;
impl<'a, REG> Wupmc18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn low_pwr_only(self) -> &'a mut crate::W<REG> {
        self.variant(Wupmc18::LowPwrOnly)
    }
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn any_pwr(self) -> &'a mut crate::W<REG> {
        self.variant(Wupmc18::AnyPwr)
    }
}
#[doc = "Wake-up Pin Mode Configuration for WUU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wupmc19 {
    #[doc = "0: Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    LowPwrOnly = 0,
    #[doc = "1: Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    AnyPwr = 1,
}
impl From<Wupmc19> for bool {
    #[inline(always)]
    fn from(variant: Wupmc19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUPMC19` reader - Wake-up Pin Mode Configuration for WUU_Pn"]
pub type Wupmc19R = crate::BitReader<Wupmc19>;
impl Wupmc19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wupmc19 {
        match self.bits {
            false => Wupmc19::LowPwrOnly,
            true => Wupmc19::AnyPwr,
        }
    }
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn is_low_pwr_only(&self) -> bool {
        *self == Wupmc19::LowPwrOnly
    }
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn is_any_pwr(&self) -> bool {
        *self == Wupmc19::AnyPwr
    }
}
#[doc = "Field `WUPMC19` writer - Wake-up Pin Mode Configuration for WUU_Pn"]
pub type Wupmc19W<'a, REG> = crate::BitWriter<'a, REG, Wupmc19>;
impl<'a, REG> Wupmc19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn low_pwr_only(self) -> &'a mut crate::W<REG> {
        self.variant(Wupmc19::LowPwrOnly)
    }
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn any_pwr(self) -> &'a mut crate::W<REG> {
        self.variant(Wupmc19::AnyPwr)
    }
}
#[doc = "Wake-up Pin Mode Configuration for WUU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wupmc20 {
    #[doc = "0: Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    LowPwrOnly = 0,
    #[doc = "1: Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    AnyPwr = 1,
}
impl From<Wupmc20> for bool {
    #[inline(always)]
    fn from(variant: Wupmc20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUPMC20` reader - Wake-up Pin Mode Configuration for WUU_Pn"]
pub type Wupmc20R = crate::BitReader<Wupmc20>;
impl Wupmc20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wupmc20 {
        match self.bits {
            false => Wupmc20::LowPwrOnly,
            true => Wupmc20::AnyPwr,
        }
    }
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn is_low_pwr_only(&self) -> bool {
        *self == Wupmc20::LowPwrOnly
    }
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn is_any_pwr(&self) -> bool {
        *self == Wupmc20::AnyPwr
    }
}
#[doc = "Field `WUPMC20` writer - Wake-up Pin Mode Configuration for WUU_Pn"]
pub type Wupmc20W<'a, REG> = crate::BitWriter<'a, REG, Wupmc20>;
impl<'a, REG> Wupmc20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn low_pwr_only(self) -> &'a mut crate::W<REG> {
        self.variant(Wupmc20::LowPwrOnly)
    }
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn any_pwr(self) -> &'a mut crate::W<REG> {
        self.variant(Wupmc20::AnyPwr)
    }
}
#[doc = "Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reserved21 {
    #[doc = "0: Not supported"]
    LowPwrOnly = 0,
    #[doc = "1: Not supported"]
    AnyPwr = 1,
}
impl From<Reserved21> for bool {
    #[inline(always)]
    fn from(variant: Reserved21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Reserved21` reader - Reserved"]
pub type Reserved21R = crate::BitReader<Reserved21>;
impl Reserved21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Reserved21 {
        match self.bits {
            false => Reserved21::LowPwrOnly,
            true => Reserved21::AnyPwr,
        }
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_low_pwr_only(&self) -> bool {
        *self == Reserved21::LowPwrOnly
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_any_pwr(&self) -> bool {
        *self == Reserved21::AnyPwr
    }
}
#[doc = "Wake-up Pin Mode Configuration for WUU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wupmc22 {
    #[doc = "0: Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    LowPwrOnly = 0,
    #[doc = "1: Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    AnyPwr = 1,
}
impl From<Wupmc22> for bool {
    #[inline(always)]
    fn from(variant: Wupmc22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUPMC22` reader - Wake-up Pin Mode Configuration for WUU_Pn"]
pub type Wupmc22R = crate::BitReader<Wupmc22>;
impl Wupmc22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wupmc22 {
        match self.bits {
            false => Wupmc22::LowPwrOnly,
            true => Wupmc22::AnyPwr,
        }
    }
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn is_low_pwr_only(&self) -> bool {
        *self == Wupmc22::LowPwrOnly
    }
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn is_any_pwr(&self) -> bool {
        *self == Wupmc22::AnyPwr
    }
}
#[doc = "Field `WUPMC22` writer - Wake-up Pin Mode Configuration for WUU_Pn"]
pub type Wupmc22W<'a, REG> = crate::BitWriter<'a, REG, Wupmc22>;
impl<'a, REG> Wupmc22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn low_pwr_only(self) -> &'a mut crate::W<REG> {
        self.variant(Wupmc22::LowPwrOnly)
    }
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn any_pwr(self) -> &'a mut crate::W<REG> {
        self.variant(Wupmc22::AnyPwr)
    }
}
#[doc = "Wake-up Pin Mode Configuration for WUU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wupmc23 {
    #[doc = "0: Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    LowPwrOnly = 0,
    #[doc = "1: Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    AnyPwr = 1,
}
impl From<Wupmc23> for bool {
    #[inline(always)]
    fn from(variant: Wupmc23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUPMC23` reader - Wake-up Pin Mode Configuration for WUU_Pn"]
pub type Wupmc23R = crate::BitReader<Wupmc23>;
impl Wupmc23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wupmc23 {
        match self.bits {
            false => Wupmc23::LowPwrOnly,
            true => Wupmc23::AnyPwr,
        }
    }
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn is_low_pwr_only(&self) -> bool {
        *self == Wupmc23::LowPwrOnly
    }
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn is_any_pwr(&self) -> bool {
        *self == Wupmc23::AnyPwr
    }
}
#[doc = "Field `WUPMC23` writer - Wake-up Pin Mode Configuration for WUU_Pn"]
pub type Wupmc23W<'a, REG> = crate::BitWriter<'a, REG, Wupmc23>;
impl<'a, REG> Wupmc23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn low_pwr_only(self) -> &'a mut crate::W<REG> {
        self.variant(Wupmc23::LowPwrOnly)
    }
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn any_pwr(self) -> &'a mut crate::W<REG> {
        self.variant(Wupmc23::AnyPwr)
    }
}
#[doc = "Wake-up Pin Mode Configuration for WUU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wupmc24 {
    #[doc = "0: Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    LowPwrOnly = 0,
    #[doc = "1: Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    AnyPwr = 1,
}
impl From<Wupmc24> for bool {
    #[inline(always)]
    fn from(variant: Wupmc24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUPMC24` reader - Wake-up Pin Mode Configuration for WUU_Pn"]
pub type Wupmc24R = crate::BitReader<Wupmc24>;
impl Wupmc24R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wupmc24 {
        match self.bits {
            false => Wupmc24::LowPwrOnly,
            true => Wupmc24::AnyPwr,
        }
    }
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn is_low_pwr_only(&self) -> bool {
        *self == Wupmc24::LowPwrOnly
    }
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn is_any_pwr(&self) -> bool {
        *self == Wupmc24::AnyPwr
    }
}
#[doc = "Field `WUPMC24` writer - Wake-up Pin Mode Configuration for WUU_Pn"]
pub type Wupmc24W<'a, REG> = crate::BitWriter<'a, REG, Wupmc24>;
impl<'a, REG> Wupmc24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn low_pwr_only(self) -> &'a mut crate::W<REG> {
        self.variant(Wupmc24::LowPwrOnly)
    }
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn any_pwr(self) -> &'a mut crate::W<REG> {
        self.variant(Wupmc24::AnyPwr)
    }
}
#[doc = "Wake-up Pin Mode Configuration for WUU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wupmc25 {
    #[doc = "0: Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    LowPwrOnly = 0,
    #[doc = "1: Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    AnyPwr = 1,
}
impl From<Wupmc25> for bool {
    #[inline(always)]
    fn from(variant: Wupmc25) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUPMC25` reader - Wake-up Pin Mode Configuration for WUU_Pn"]
pub type Wupmc25R = crate::BitReader<Wupmc25>;
impl Wupmc25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wupmc25 {
        match self.bits {
            false => Wupmc25::LowPwrOnly,
            true => Wupmc25::AnyPwr,
        }
    }
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn is_low_pwr_only(&self) -> bool {
        *self == Wupmc25::LowPwrOnly
    }
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn is_any_pwr(&self) -> bool {
        *self == Wupmc25::AnyPwr
    }
}
#[doc = "Field `WUPMC25` writer - Wake-up Pin Mode Configuration for WUU_Pn"]
pub type Wupmc25W<'a, REG> = crate::BitWriter<'a, REG, Wupmc25>;
impl<'a, REG> Wupmc25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn low_pwr_only(self) -> &'a mut crate::W<REG> {
        self.variant(Wupmc25::LowPwrOnly)
    }
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn any_pwr(self) -> &'a mut crate::W<REG> {
        self.variant(Wupmc25::AnyPwr)
    }
}
#[doc = "Wake-up Pin Mode Configuration for WUU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wupmc26 {
    #[doc = "0: Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    LowPwrOnly = 0,
    #[doc = "1: Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    AnyPwr = 1,
}
impl From<Wupmc26> for bool {
    #[inline(always)]
    fn from(variant: Wupmc26) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUPMC26` reader - Wake-up Pin Mode Configuration for WUU_Pn"]
pub type Wupmc26R = crate::BitReader<Wupmc26>;
impl Wupmc26R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wupmc26 {
        match self.bits {
            false => Wupmc26::LowPwrOnly,
            true => Wupmc26::AnyPwr,
        }
    }
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn is_low_pwr_only(&self) -> bool {
        *self == Wupmc26::LowPwrOnly
    }
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn is_any_pwr(&self) -> bool {
        *self == Wupmc26::AnyPwr
    }
}
#[doc = "Field `WUPMC26` writer - Wake-up Pin Mode Configuration for WUU_Pn"]
pub type Wupmc26W<'a, REG> = crate::BitWriter<'a, REG, Wupmc26>;
impl<'a, REG> Wupmc26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn low_pwr_only(self) -> &'a mut crate::W<REG> {
        self.variant(Wupmc26::LowPwrOnly)
    }
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn any_pwr(self) -> &'a mut crate::W<REG> {
        self.variant(Wupmc26::AnyPwr)
    }
}
#[doc = "Wake-up Pin Mode Configuration for WUU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wupmc27 {
    #[doc = "0: Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    LowPwrOnly = 0,
    #[doc = "1: Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    AnyPwr = 1,
}
impl From<Wupmc27> for bool {
    #[inline(always)]
    fn from(variant: Wupmc27) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUPMC27` reader - Wake-up Pin Mode Configuration for WUU_Pn"]
pub type Wupmc27R = crate::BitReader<Wupmc27>;
impl Wupmc27R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wupmc27 {
        match self.bits {
            false => Wupmc27::LowPwrOnly,
            true => Wupmc27::AnyPwr,
        }
    }
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn is_low_pwr_only(&self) -> bool {
        *self == Wupmc27::LowPwrOnly
    }
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn is_any_pwr(&self) -> bool {
        *self == Wupmc27::AnyPwr
    }
}
#[doc = "Field `WUPMC27` writer - Wake-up Pin Mode Configuration for WUU_Pn"]
pub type Wupmc27W<'a, REG> = crate::BitWriter<'a, REG, Wupmc27>;
impl<'a, REG> Wupmc27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn low_pwr_only(self) -> &'a mut crate::W<REG> {
        self.variant(Wupmc27::LowPwrOnly)
    }
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn any_pwr(self) -> &'a mut crate::W<REG> {
        self.variant(Wupmc27::AnyPwr)
    }
}
#[doc = "Wake-up Pin Mode Configuration for WUU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wupmc28 {
    #[doc = "0: Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    LowPwrOnly = 0,
    #[doc = "1: Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    AnyPwr = 1,
}
impl From<Wupmc28> for bool {
    #[inline(always)]
    fn from(variant: Wupmc28) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUPMC28` reader - Wake-up Pin Mode Configuration for WUU_Pn"]
pub type Wupmc28R = crate::BitReader<Wupmc28>;
impl Wupmc28R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wupmc28 {
        match self.bits {
            false => Wupmc28::LowPwrOnly,
            true => Wupmc28::AnyPwr,
        }
    }
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn is_low_pwr_only(&self) -> bool {
        *self == Wupmc28::LowPwrOnly
    }
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn is_any_pwr(&self) -> bool {
        *self == Wupmc28::AnyPwr
    }
}
#[doc = "Field `WUPMC28` writer - Wake-up Pin Mode Configuration for WUU_Pn"]
pub type Wupmc28W<'a, REG> = crate::BitWriter<'a, REG, Wupmc28>;
impl<'a, REG> Wupmc28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn low_pwr_only(self) -> &'a mut crate::W<REG> {
        self.variant(Wupmc28::LowPwrOnly)
    }
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn any_pwr(self) -> &'a mut crate::W<REG> {
        self.variant(Wupmc28::AnyPwr)
    }
}
#[doc = "Wake-up Pin Mode Configuration for WUU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wupmc29 {
    #[doc = "0: Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    LowPwrOnly = 0,
    #[doc = "1: Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    AnyPwr = 1,
}
impl From<Wupmc29> for bool {
    #[inline(always)]
    fn from(variant: Wupmc29) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUPMC29` reader - Wake-up Pin Mode Configuration for WUU_Pn"]
pub type Wupmc29R = crate::BitReader<Wupmc29>;
impl Wupmc29R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wupmc29 {
        match self.bits {
            false => Wupmc29::LowPwrOnly,
            true => Wupmc29::AnyPwr,
        }
    }
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn is_low_pwr_only(&self) -> bool {
        *self == Wupmc29::LowPwrOnly
    }
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn is_any_pwr(&self) -> bool {
        *self == Wupmc29::AnyPwr
    }
}
#[doc = "Field `WUPMC29` writer - Wake-up Pin Mode Configuration for WUU_Pn"]
pub type Wupmc29W<'a, REG> = crate::BitWriter<'a, REG, Wupmc29>;
impl<'a, REG> Wupmc29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn low_pwr_only(self) -> &'a mut crate::W<REG> {
        self.variant(Wupmc29::LowPwrOnly)
    }
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn any_pwr(self) -> &'a mut crate::W<REG> {
        self.variant(Wupmc29::AnyPwr)
    }
}
#[doc = "Wake-up Pin Mode Configuration for WUU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wupmc30 {
    #[doc = "0: Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    LowPwrOnly = 0,
    #[doc = "1: Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    AnyPwr = 1,
}
impl From<Wupmc30> for bool {
    #[inline(always)]
    fn from(variant: Wupmc30) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUPMC30` reader - Wake-up Pin Mode Configuration for WUU_Pn"]
pub type Wupmc30R = crate::BitReader<Wupmc30>;
impl Wupmc30R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wupmc30 {
        match self.bits {
            false => Wupmc30::LowPwrOnly,
            true => Wupmc30::AnyPwr,
        }
    }
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn is_low_pwr_only(&self) -> bool {
        *self == Wupmc30::LowPwrOnly
    }
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn is_any_pwr(&self) -> bool {
        *self == Wupmc30::AnyPwr
    }
}
#[doc = "Field `WUPMC30` writer - Wake-up Pin Mode Configuration for WUU_Pn"]
pub type Wupmc30W<'a, REG> = crate::BitWriter<'a, REG, Wupmc30>;
impl<'a, REG> Wupmc30W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn low_pwr_only(self) -> &'a mut crate::W<REG> {
        self.variant(Wupmc30::LowPwrOnly)
    }
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn any_pwr(self) -> &'a mut crate::W<REG> {
        self.variant(Wupmc30::AnyPwr)
    }
}
#[doc = "Wake-up Pin Mode Configuration for WUU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wupmc31 {
    #[doc = "0: Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    LowPwrOnly = 0,
    #[doc = "1: Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    AnyPwr = 1,
}
impl From<Wupmc31> for bool {
    #[inline(always)]
    fn from(variant: Wupmc31) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUPMC31` reader - Wake-up Pin Mode Configuration for WUU_Pn"]
pub type Wupmc31R = crate::BitReader<Wupmc31>;
impl Wupmc31R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wupmc31 {
        match self.bits {
            false => Wupmc31::LowPwrOnly,
            true => Wupmc31::AnyPwr,
        }
    }
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn is_low_pwr_only(&self) -> bool {
        *self == Wupmc31::LowPwrOnly
    }
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn is_any_pwr(&self) -> bool {
        *self == Wupmc31::AnyPwr
    }
}
#[doc = "Field `WUPMC31` writer - Wake-up Pin Mode Configuration for WUU_Pn"]
pub type Wupmc31W<'a, REG> = crate::BitWriter<'a, REG, Wupmc31>;
impl<'a, REG> Wupmc31W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn low_pwr_only(self) -> &'a mut crate::W<REG> {
        self.variant(Wupmc31::LowPwrOnly)
    }
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    #[inline(always)]
    pub fn any_pwr(self) -> &'a mut crate::W<REG> {
        self.variant(Wupmc31::AnyPwr)
    }
}
impl R {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wake-up Pin Mode Configuration for WUU_Pn"]
    #[inline(always)]
    pub fn wupmc2(&self) -> Wupmc2R {
        Wupmc2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Wake-up Pin Mode Configuration for WUU_Pn"]
    #[inline(always)]
    pub fn wupmc6(&self) -> Wupmc6R {
        Wupmc6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Wake-up Pin Mode Configuration for WUU_Pn"]
    #[inline(always)]
    pub fn wupmc7(&self) -> Wupmc7R {
        Wupmc7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Wake-up Pin Mode Configuration for WUU_Pn"]
    #[inline(always)]
    pub fn wupmc8(&self) -> Wupmc8R {
        Wupmc8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Wake-up Pin Mode Configuration for WUU_Pn"]
    #[inline(always)]
    pub fn wupmc9(&self) -> Wupmc9R {
        Wupmc9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Wake-up Pin Mode Configuration for WUU_Pn"]
    #[inline(always)]
    pub fn wupmc10(&self) -> Wupmc10R {
        Wupmc10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Wake-up Pin Mode Configuration for WUU_Pn"]
    #[inline(always)]
    pub fn wupmc11(&self) -> Wupmc11R {
        Wupmc11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Wake-up Pin Mode Configuration for WUU_Pn"]
    #[inline(always)]
    pub fn wupmc12(&self) -> Wupmc12R {
        Wupmc12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn reserved13(&self) -> Reserved13R {
        Reserved13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Reserved"]
    #[inline(always)]
    pub fn reserved14(&self) -> Reserved14R {
        Reserved14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Reserved"]
    #[inline(always)]
    pub fn reserved15(&self) -> Reserved15R {
        Reserved15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Wake-up Pin Mode Configuration for WUU_Pn"]
    #[inline(always)]
    pub fn wupmc16(&self) -> Wupmc16R {
        Wupmc16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Wake-up Pin Mode Configuration for WUU_Pn"]
    #[inline(always)]
    pub fn wupmc17(&self) -> Wupmc17R {
        Wupmc17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Wake-up Pin Mode Configuration for WUU_Pn"]
    #[inline(always)]
    pub fn wupmc18(&self) -> Wupmc18R {
        Wupmc18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Wake-up Pin Mode Configuration for WUU_Pn"]
    #[inline(always)]
    pub fn wupmc19(&self) -> Wupmc19R {
        Wupmc19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Wake-up Pin Mode Configuration for WUU_Pn"]
    #[inline(always)]
    pub fn wupmc20(&self) -> Wupmc20R {
        Wupmc20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Reserved"]
    #[inline(always)]
    pub fn reserved21(&self) -> Reserved21R {
        Reserved21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Wake-up Pin Mode Configuration for WUU_Pn"]
    #[inline(always)]
    pub fn wupmc22(&self) -> Wupmc22R {
        Wupmc22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Wake-up Pin Mode Configuration for WUU_Pn"]
    #[inline(always)]
    pub fn wupmc23(&self) -> Wupmc23R {
        Wupmc23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Wake-up Pin Mode Configuration for WUU_Pn"]
    #[inline(always)]
    pub fn wupmc24(&self) -> Wupmc24R {
        Wupmc24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Wake-up Pin Mode Configuration for WUU_Pn"]
    #[inline(always)]
    pub fn wupmc25(&self) -> Wupmc25R {
        Wupmc25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Wake-up Pin Mode Configuration for WUU_Pn"]
    #[inline(always)]
    pub fn wupmc26(&self) -> Wupmc26R {
        Wupmc26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Wake-up Pin Mode Configuration for WUU_Pn"]
    #[inline(always)]
    pub fn wupmc27(&self) -> Wupmc27R {
        Wupmc27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Wake-up Pin Mode Configuration for WUU_Pn"]
    #[inline(always)]
    pub fn wupmc28(&self) -> Wupmc28R {
        Wupmc28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Wake-up Pin Mode Configuration for WUU_Pn"]
    #[inline(always)]
    pub fn wupmc29(&self) -> Wupmc29R {
        Wupmc29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Wake-up Pin Mode Configuration for WUU_Pn"]
    #[inline(always)]
    pub fn wupmc30(&self) -> Wupmc30R {
        Wupmc30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Wake-up Pin Mode Configuration for WUU_Pn"]
    #[inline(always)]
    pub fn wupmc31(&self) -> Wupmc31R {
        Wupmc31R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Wake-up Pin Mode Configuration for WUU_Pn"]
    #[inline(always)]
    pub fn wupmc2(&mut self) -> Wupmc2W<'_, PmcSpec> {
        Wupmc2W::new(self, 2)
    }
    #[doc = "Bit 6 - Wake-up Pin Mode Configuration for WUU_Pn"]
    #[inline(always)]
    pub fn wupmc6(&mut self) -> Wupmc6W<'_, PmcSpec> {
        Wupmc6W::new(self, 6)
    }
    #[doc = "Bit 7 - Wake-up Pin Mode Configuration for WUU_Pn"]
    #[inline(always)]
    pub fn wupmc7(&mut self) -> Wupmc7W<'_, PmcSpec> {
        Wupmc7W::new(self, 7)
    }
    #[doc = "Bit 8 - Wake-up Pin Mode Configuration for WUU_Pn"]
    #[inline(always)]
    pub fn wupmc8(&mut self) -> Wupmc8W<'_, PmcSpec> {
        Wupmc8W::new(self, 8)
    }
    #[doc = "Bit 9 - Wake-up Pin Mode Configuration for WUU_Pn"]
    #[inline(always)]
    pub fn wupmc9(&mut self) -> Wupmc9W<'_, PmcSpec> {
        Wupmc9W::new(self, 9)
    }
    #[doc = "Bit 10 - Wake-up Pin Mode Configuration for WUU_Pn"]
    #[inline(always)]
    pub fn wupmc10(&mut self) -> Wupmc10W<'_, PmcSpec> {
        Wupmc10W::new(self, 10)
    }
    #[doc = "Bit 11 - Wake-up Pin Mode Configuration for WUU_Pn"]
    #[inline(always)]
    pub fn wupmc11(&mut self) -> Wupmc11W<'_, PmcSpec> {
        Wupmc11W::new(self, 11)
    }
    #[doc = "Bit 12 - Wake-up Pin Mode Configuration for WUU_Pn"]
    #[inline(always)]
    pub fn wupmc12(&mut self) -> Wupmc12W<'_, PmcSpec> {
        Wupmc12W::new(self, 12)
    }
    #[doc = "Bit 16 - Wake-up Pin Mode Configuration for WUU_Pn"]
    #[inline(always)]
    pub fn wupmc16(&mut self) -> Wupmc16W<'_, PmcSpec> {
        Wupmc16W::new(self, 16)
    }
    #[doc = "Bit 17 - Wake-up Pin Mode Configuration for WUU_Pn"]
    #[inline(always)]
    pub fn wupmc17(&mut self) -> Wupmc17W<'_, PmcSpec> {
        Wupmc17W::new(self, 17)
    }
    #[doc = "Bit 18 - Wake-up Pin Mode Configuration for WUU_Pn"]
    #[inline(always)]
    pub fn wupmc18(&mut self) -> Wupmc18W<'_, PmcSpec> {
        Wupmc18W::new(self, 18)
    }
    #[doc = "Bit 19 - Wake-up Pin Mode Configuration for WUU_Pn"]
    #[inline(always)]
    pub fn wupmc19(&mut self) -> Wupmc19W<'_, PmcSpec> {
        Wupmc19W::new(self, 19)
    }
    #[doc = "Bit 20 - Wake-up Pin Mode Configuration for WUU_Pn"]
    #[inline(always)]
    pub fn wupmc20(&mut self) -> Wupmc20W<'_, PmcSpec> {
        Wupmc20W::new(self, 20)
    }
    #[doc = "Bit 22 - Wake-up Pin Mode Configuration for WUU_Pn"]
    #[inline(always)]
    pub fn wupmc22(&mut self) -> Wupmc22W<'_, PmcSpec> {
        Wupmc22W::new(self, 22)
    }
    #[doc = "Bit 23 - Wake-up Pin Mode Configuration for WUU_Pn"]
    #[inline(always)]
    pub fn wupmc23(&mut self) -> Wupmc23W<'_, PmcSpec> {
        Wupmc23W::new(self, 23)
    }
    #[doc = "Bit 24 - Wake-up Pin Mode Configuration for WUU_Pn"]
    #[inline(always)]
    pub fn wupmc24(&mut self) -> Wupmc24W<'_, PmcSpec> {
        Wupmc24W::new(self, 24)
    }
    #[doc = "Bit 25 - Wake-up Pin Mode Configuration for WUU_Pn"]
    #[inline(always)]
    pub fn wupmc25(&mut self) -> Wupmc25W<'_, PmcSpec> {
        Wupmc25W::new(self, 25)
    }
    #[doc = "Bit 26 - Wake-up Pin Mode Configuration for WUU_Pn"]
    #[inline(always)]
    pub fn wupmc26(&mut self) -> Wupmc26W<'_, PmcSpec> {
        Wupmc26W::new(self, 26)
    }
    #[doc = "Bit 27 - Wake-up Pin Mode Configuration for WUU_Pn"]
    #[inline(always)]
    pub fn wupmc27(&mut self) -> Wupmc27W<'_, PmcSpec> {
        Wupmc27W::new(self, 27)
    }
    #[doc = "Bit 28 - Wake-up Pin Mode Configuration for WUU_Pn"]
    #[inline(always)]
    pub fn wupmc28(&mut self) -> Wupmc28W<'_, PmcSpec> {
        Wupmc28W::new(self, 28)
    }
    #[doc = "Bit 29 - Wake-up Pin Mode Configuration for WUU_Pn"]
    #[inline(always)]
    pub fn wupmc29(&mut self) -> Wupmc29W<'_, PmcSpec> {
        Wupmc29W::new(self, 29)
    }
    #[doc = "Bit 30 - Wake-up Pin Mode Configuration for WUU_Pn"]
    #[inline(always)]
    pub fn wupmc30(&mut self) -> Wupmc30W<'_, PmcSpec> {
        Wupmc30W::new(self, 30)
    }
    #[doc = "Bit 31 - Wake-up Pin Mode Configuration for WUU_Pn"]
    #[inline(always)]
    pub fn wupmc31(&mut self) -> Wupmc31W<'_, PmcSpec> {
        Wupmc31W::new(self, 31)
    }
}
#[doc = "Pin Mode Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`pmc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmcSpec;
impl crate::RegisterSpec for PmcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmc::R`](R) reader structure"]
impl crate::Readable for PmcSpec {}
#[doc = "`write(|w| ..)` method takes [`pmc::W`](W) writer structure"]
impl crate::Writable for PmcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PMC to value 0"]
impl crate::Resettable for PmcSpec {}
