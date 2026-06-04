#[doc = "Register `MRCC_GLB_ACC1` reader"]
pub type R = crate::R<MrccGlbAcc1Spec>;
#[doc = "Register `MRCC_GLB_ACC1` writer"]
pub type W = crate::W<MrccGlbAcc1Spec>;
#[doc = "OSTIMER0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ostimer0 {
    #[doc = "0: Automatic clock gating is disabled"]
    Disabled = 0,
    #[doc = "1: Automatic clock gating is enabled"]
    Enabled = 1,
}
impl From<Ostimer0> for bool {
    #[inline(always)]
    fn from(variant: Ostimer0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSTIMER0` reader - OSTIMER0"]
pub type Ostimer0R = crate::BitReader<Ostimer0>;
impl Ostimer0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ostimer0 {
        match self.bits {
            false => Ostimer0::Disabled,
            true => Ostimer0::Enabled,
        }
    }
    #[doc = "Automatic clock gating is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ostimer0::Disabled
    }
    #[doc = "Automatic clock gating is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ostimer0::Enabled
    }
}
#[doc = "Field `OSTIMER0` writer - OSTIMER0"]
pub type Ostimer0W<'a, REG> = crate::BitWriter<'a, REG, Ostimer0>;
impl<'a, REG> Ostimer0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Automatic clock gating is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ostimer0::Disabled)
    }
    #[doc = "Automatic clock gating is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ostimer0::Enabled)
    }
}
#[doc = "ADC0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc0 {
    #[doc = "0: Automatic clock gating is disabled"]
    Disabled = 0,
    #[doc = "1: Automatic clock gating is enabled"]
    Enabled = 1,
}
impl From<Adc0> for bool {
    #[inline(always)]
    fn from(variant: Adc0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC0` reader - ADC0"]
pub type Adc0R = crate::BitReader<Adc0>;
impl Adc0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc0 {
        match self.bits {
            false => Adc0::Disabled,
            true => Adc0::Enabled,
        }
    }
    #[doc = "Automatic clock gating is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Adc0::Disabled
    }
    #[doc = "Automatic clock gating is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Adc0::Enabled
    }
}
#[doc = "Field `ADC0` writer - ADC0"]
pub type Adc0W<'a, REG> = crate::BitWriter<'a, REG, Adc0>;
impl<'a, REG> Adc0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Automatic clock gating is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Adc0::Disabled)
    }
    #[doc = "Automatic clock gating is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Adc0::Enabled)
    }
}
#[doc = "ADC1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc1 {
    #[doc = "0: Automatic clock gating is disabled"]
    Disabled = 0,
    #[doc = "1: Automatic clock gating is enabled"]
    Enabled = 1,
}
impl From<Adc1> for bool {
    #[inline(always)]
    fn from(variant: Adc1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC1` reader - ADC1"]
pub type Adc1R = crate::BitReader<Adc1>;
impl Adc1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc1 {
        match self.bits {
            false => Adc1::Disabled,
            true => Adc1::Enabled,
        }
    }
    #[doc = "Automatic clock gating is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Adc1::Disabled
    }
    #[doc = "Automatic clock gating is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Adc1::Enabled
    }
}
#[doc = "Field `ADC1` writer - ADC1"]
pub type Adc1W<'a, REG> = crate::BitWriter<'a, REG, Adc1>;
impl<'a, REG> Adc1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Automatic clock gating is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Adc1::Disabled)
    }
    #[doc = "Automatic clock gating is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Adc1::Enabled)
    }
}
#[doc = "CMP0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmp0 {
    #[doc = "0: Automatic clock gating is disabled"]
    Disabled = 0,
    #[doc = "1: Automatic clock gating is enabled"]
    Enabled = 1,
}
impl From<Cmp0> for bool {
    #[inline(always)]
    fn from(variant: Cmp0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMP0` reader - CMP0"]
pub type Cmp0R = crate::BitReader<Cmp0>;
impl Cmp0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmp0 {
        match self.bits {
            false => Cmp0::Disabled,
            true => Cmp0::Enabled,
        }
    }
    #[doc = "Automatic clock gating is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Cmp0::Disabled
    }
    #[doc = "Automatic clock gating is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Cmp0::Enabled
    }
}
#[doc = "Field `CMP0` writer - CMP0"]
pub type Cmp0W<'a, REG> = crate::BitWriter<'a, REG, Cmp0>;
impl<'a, REG> Cmp0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Automatic clock gating is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cmp0::Disabled)
    }
    #[doc = "Automatic clock gating is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cmp0::Enabled)
    }
}
#[doc = "CMP1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmp1 {
    #[doc = "0: Automatic clock gating is disabled"]
    Disabled = 0,
    #[doc = "1: Automatic clock gating is enabled"]
    Enabled = 1,
}
impl From<Cmp1> for bool {
    #[inline(always)]
    fn from(variant: Cmp1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMP1` reader - CMP1"]
pub type Cmp1R = crate::BitReader<Cmp1>;
impl Cmp1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmp1 {
        match self.bits {
            false => Cmp1::Disabled,
            true => Cmp1::Enabled,
        }
    }
    #[doc = "Automatic clock gating is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Cmp1::Disabled
    }
    #[doc = "Automatic clock gating is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Cmp1::Enabled
    }
}
#[doc = "Field `CMP1` writer - CMP1"]
pub type Cmp1W<'a, REG> = crate::BitWriter<'a, REG, Cmp1>;
impl<'a, REG> Cmp1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Automatic clock gating is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cmp1::Disabled)
    }
    #[doc = "Automatic clock gating is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cmp1::Enabled)
    }
}
#[doc = "DAC0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dac0 {
    #[doc = "0: Automatic clock gating is disabled"]
    Disabled = 0,
    #[doc = "1: Automatic clock gating is enabled"]
    Enabled = 1,
}
impl From<Dac0> for bool {
    #[inline(always)]
    fn from(variant: Dac0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAC0` reader - DAC0"]
pub type Dac0R = crate::BitReader<Dac0>;
impl Dac0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dac0 {
        match self.bits {
            false => Dac0::Disabled,
            true => Dac0::Enabled,
        }
    }
    #[doc = "Automatic clock gating is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dac0::Disabled
    }
    #[doc = "Automatic clock gating is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dac0::Enabled
    }
}
#[doc = "Field `DAC0` writer - DAC0"]
pub type Dac0W<'a, REG> = crate::BitWriter<'a, REG, Dac0>;
impl<'a, REG> Dac0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Automatic clock gating is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dac0::Disabled)
    }
    #[doc = "Automatic clock gating is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dac0::Enabled)
    }
}
#[doc = "OPAMP0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Opamp0 {
    #[doc = "0: Automatic clock gating is disabled"]
    Disabled = 0,
    #[doc = "1: Automatic clock gating is enabled"]
    Enabled = 1,
}
impl From<Opamp0> for bool {
    #[inline(always)]
    fn from(variant: Opamp0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPAMP0` reader - OPAMP0"]
pub type Opamp0R = crate::BitReader<Opamp0>;
impl Opamp0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Opamp0 {
        match self.bits {
            false => Opamp0::Disabled,
            true => Opamp0::Enabled,
        }
    }
    #[doc = "Automatic clock gating is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Opamp0::Disabled
    }
    #[doc = "Automatic clock gating is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Opamp0::Enabled
    }
}
#[doc = "Field `OPAMP0` writer - OPAMP0"]
pub type Opamp0W<'a, REG> = crate::BitWriter<'a, REG, Opamp0>;
impl<'a, REG> Opamp0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Automatic clock gating is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Opamp0::Disabled)
    }
    #[doc = "Automatic clock gating is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Opamp0::Enabled)
    }
}
#[doc = "PORT0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Port0 {
    #[doc = "0: Automatic clock gating is disabled"]
    Disabled = 0,
    #[doc = "1: Automatic clock gating is enabled"]
    Enabled = 1,
}
impl From<Port0> for bool {
    #[inline(always)]
    fn from(variant: Port0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORT0` reader - PORT0"]
pub type Port0R = crate::BitReader<Port0>;
impl Port0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Port0 {
        match self.bits {
            false => Port0::Disabled,
            true => Port0::Enabled,
        }
    }
    #[doc = "Automatic clock gating is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Port0::Disabled
    }
    #[doc = "Automatic clock gating is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Port0::Enabled
    }
}
#[doc = "Field `PORT0` writer - PORT0"]
pub type Port0W<'a, REG> = crate::BitWriter<'a, REG, Port0>;
impl<'a, REG> Port0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Automatic clock gating is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Port0::Disabled)
    }
    #[doc = "Automatic clock gating is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Port0::Enabled)
    }
}
#[doc = "PORT1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Port1 {
    #[doc = "0: Automatic clock gating is disabled"]
    Disabled = 0,
    #[doc = "1: Automatic clock gating is enabled"]
    Enabled = 1,
}
impl From<Port1> for bool {
    #[inline(always)]
    fn from(variant: Port1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORT1` reader - PORT1"]
pub type Port1R = crate::BitReader<Port1>;
impl Port1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Port1 {
        match self.bits {
            false => Port1::Disabled,
            true => Port1::Enabled,
        }
    }
    #[doc = "Automatic clock gating is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Port1::Disabled
    }
    #[doc = "Automatic clock gating is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Port1::Enabled
    }
}
#[doc = "Field `PORT1` writer - PORT1"]
pub type Port1W<'a, REG> = crate::BitWriter<'a, REG, Port1>;
impl<'a, REG> Port1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Automatic clock gating is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Port1::Disabled)
    }
    #[doc = "Automatic clock gating is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Port1::Enabled)
    }
}
#[doc = "PORT2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Port2 {
    #[doc = "0: Automatic clock gating is disabled"]
    Disabled = 0,
    #[doc = "1: Automatic clock gating is enabled"]
    Enabled = 1,
}
impl From<Port2> for bool {
    #[inline(always)]
    fn from(variant: Port2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORT2` reader - PORT2"]
pub type Port2R = crate::BitReader<Port2>;
impl Port2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Port2 {
        match self.bits {
            false => Port2::Disabled,
            true => Port2::Enabled,
        }
    }
    #[doc = "Automatic clock gating is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Port2::Disabled
    }
    #[doc = "Automatic clock gating is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Port2::Enabled
    }
}
#[doc = "Field `PORT2` writer - PORT2"]
pub type Port2W<'a, REG> = crate::BitWriter<'a, REG, Port2>;
impl<'a, REG> Port2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Automatic clock gating is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Port2::Disabled)
    }
    #[doc = "Automatic clock gating is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Port2::Enabled)
    }
}
#[doc = "PORT3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Port3 {
    #[doc = "0: Automatic clock gating is disabled"]
    Disabled = 0,
    #[doc = "1: Automatic clock gating is enabled"]
    Enabled = 1,
}
impl From<Port3> for bool {
    #[inline(always)]
    fn from(variant: Port3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORT3` reader - PORT3"]
pub type Port3R = crate::BitReader<Port3>;
impl Port3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Port3 {
        match self.bits {
            false => Port3::Disabled,
            true => Port3::Enabled,
        }
    }
    #[doc = "Automatic clock gating is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Port3::Disabled
    }
    #[doc = "Automatic clock gating is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Port3::Enabled
    }
}
#[doc = "Field `PORT3` writer - PORT3"]
pub type Port3W<'a, REG> = crate::BitWriter<'a, REG, Port3>;
impl<'a, REG> Port3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Automatic clock gating is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Port3::Disabled)
    }
    #[doc = "Automatic clock gating is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Port3::Enabled)
    }
}
#[doc = "PORT4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Port4 {
    #[doc = "0: Automatic clock gating is disabled"]
    Disabled = 0,
    #[doc = "1: Automatic clock gating is enabled"]
    Enabled = 1,
}
impl From<Port4> for bool {
    #[inline(always)]
    fn from(variant: Port4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORT4` reader - PORT4"]
pub type Port4R = crate::BitReader<Port4>;
impl Port4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Port4 {
        match self.bits {
            false => Port4::Disabled,
            true => Port4::Enabled,
        }
    }
    #[doc = "Automatic clock gating is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Port4::Disabled
    }
    #[doc = "Automatic clock gating is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Port4::Enabled
    }
}
#[doc = "Field `PORT4` writer - PORT4"]
pub type Port4W<'a, REG> = crate::BitWriter<'a, REG, Port4>;
impl<'a, REG> Port4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Automatic clock gating is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Port4::Disabled)
    }
    #[doc = "Automatic clock gating is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Port4::Enabled)
    }
}
#[doc = "FLEXCAN0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcan0 {
    #[doc = "0: Automatic clock gating is disabled"]
    Disabled = 0,
    #[doc = "1: Automatic clock gating is enabled"]
    Enabled = 1,
}
impl From<Flexcan0> for bool {
    #[inline(always)]
    fn from(variant: Flexcan0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCAN0` reader - FLEXCAN0"]
pub type Flexcan0R = crate::BitReader<Flexcan0>;
impl Flexcan0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flexcan0 {
        match self.bits {
            false => Flexcan0::Disabled,
            true => Flexcan0::Enabled,
        }
    }
    #[doc = "Automatic clock gating is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Flexcan0::Disabled
    }
    #[doc = "Automatic clock gating is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Flexcan0::Enabled
    }
}
#[doc = "Field `FLEXCAN0` writer - FLEXCAN0"]
pub type Flexcan0W<'a, REG> = crate::BitWriter<'a, REG, Flexcan0>;
impl<'a, REG> Flexcan0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Automatic clock gating is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcan0::Disabled)
    }
    #[doc = "Automatic clock gating is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcan0::Enabled)
    }
}
#[doc = "LPI2C2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lpi2c2 {
    #[doc = "0: Automatic clock gating is disabled"]
    Disabled = 0,
    #[doc = "1: Automatic clock gating is enabled"]
    Enabled = 1,
}
impl From<Lpi2c2> for bool {
    #[inline(always)]
    fn from(variant: Lpi2c2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPI2C2` reader - LPI2C2"]
pub type Lpi2c2R = crate::BitReader<Lpi2c2>;
impl Lpi2c2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpi2c2 {
        match self.bits {
            false => Lpi2c2::Disabled,
            true => Lpi2c2::Enabled,
        }
    }
    #[doc = "Automatic clock gating is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Lpi2c2::Disabled
    }
    #[doc = "Automatic clock gating is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Lpi2c2::Enabled
    }
}
#[doc = "Field `LPI2C2` writer - LPI2C2"]
pub type Lpi2c2W<'a, REG> = crate::BitWriter<'a, REG, Lpi2c2>;
impl<'a, REG> Lpi2c2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Automatic clock gating is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Lpi2c2::Disabled)
    }
    #[doc = "Automatic clock gating is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Lpi2c2::Enabled)
    }
}
#[doc = "LPI2C3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lpi2c3 {
    #[doc = "0: Automatic clock gating is disabled"]
    Disabled = 0,
    #[doc = "1: Automatic clock gating is enabled"]
    Enabled = 1,
}
impl From<Lpi2c3> for bool {
    #[inline(always)]
    fn from(variant: Lpi2c3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPI2C3` reader - LPI2C3"]
pub type Lpi2c3R = crate::BitReader<Lpi2c3>;
impl Lpi2c3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpi2c3 {
        match self.bits {
            false => Lpi2c3::Disabled,
            true => Lpi2c3::Enabled,
        }
    }
    #[doc = "Automatic clock gating is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Lpi2c3::Disabled
    }
    #[doc = "Automatic clock gating is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Lpi2c3::Enabled
    }
}
#[doc = "Field `LPI2C3` writer - LPI2C3"]
pub type Lpi2c3W<'a, REG> = crate::BitWriter<'a, REG, Lpi2c3>;
impl<'a, REG> Lpi2c3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Automatic clock gating is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Lpi2c3::Disabled)
    }
    #[doc = "Automatic clock gating is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Lpi2c3::Enabled)
    }
}
#[doc = "RAMA\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rama {
    #[doc = "0: Automatic clock gating is disabled"]
    Disabled = 0,
    #[doc = "1: Automatic clock gating is enabled"]
    Enabled = 1,
}
impl From<Rama> for bool {
    #[inline(always)]
    fn from(variant: Rama) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAMA` reader - RAMA"]
pub type RamaR = crate::BitReader<Rama>;
impl RamaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rama {
        match self.bits {
            false => Rama::Disabled,
            true => Rama::Enabled,
        }
    }
    #[doc = "Automatic clock gating is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rama::Disabled
    }
    #[doc = "Automatic clock gating is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rama::Enabled
    }
}
#[doc = "Field `RAMA` writer - RAMA"]
pub type RamaW<'a, REG> = crate::BitWriter<'a, REG, Rama>;
impl<'a, REG> RamaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Automatic clock gating is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rama::Disabled)
    }
    #[doc = "Automatic clock gating is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rama::Enabled)
    }
}
#[doc = "RAMB\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ramb {
    #[doc = "0: Automatic clock gating is disabled"]
    Disabled = 0,
    #[doc = "1: Automatic clock gating is enabled"]
    Enabled = 1,
}
impl From<Ramb> for bool {
    #[inline(always)]
    fn from(variant: Ramb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAMB` reader - RAMB"]
pub type RambR = crate::BitReader<Ramb>;
impl RambR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ramb {
        match self.bits {
            false => Ramb::Disabled,
            true => Ramb::Enabled,
        }
    }
    #[doc = "Automatic clock gating is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ramb::Disabled
    }
    #[doc = "Automatic clock gating is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ramb::Enabled
    }
}
#[doc = "Field `RAMB` writer - RAMB"]
pub type RambW<'a, REG> = crate::BitWriter<'a, REG, Ramb>;
impl<'a, REG> RambW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Automatic clock gating is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ramb::Disabled)
    }
    #[doc = "Automatic clock gating is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ramb::Enabled)
    }
}
#[doc = "GPIO0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio0 {
    #[doc = "0: Automatic clock gating is disabled"]
    Disabled = 0,
    #[doc = "1: Automatic clock gating is enabled"]
    Enabled = 1,
}
impl From<Gpio0> for bool {
    #[inline(always)]
    fn from(variant: Gpio0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO0` reader - GPIO0"]
pub type Gpio0R = crate::BitReader<Gpio0>;
impl Gpio0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio0 {
        match self.bits {
            false => Gpio0::Disabled,
            true => Gpio0::Enabled,
        }
    }
    #[doc = "Automatic clock gating is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio0::Disabled
    }
    #[doc = "Automatic clock gating is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio0::Enabled
    }
}
#[doc = "Field `GPIO0` writer - GPIO0"]
pub type Gpio0W<'a, REG> = crate::BitWriter<'a, REG, Gpio0>;
impl<'a, REG> Gpio0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Automatic clock gating is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0::Disabled)
    }
    #[doc = "Automatic clock gating is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0::Enabled)
    }
}
#[doc = "GPIO1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio1 {
    #[doc = "0: Automatic clock gating is disabled"]
    Disabled = 0,
    #[doc = "1: Automatic clock gating is enabled"]
    Enabled = 1,
}
impl From<Gpio1> for bool {
    #[inline(always)]
    fn from(variant: Gpio1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO1` reader - GPIO1"]
pub type Gpio1R = crate::BitReader<Gpio1>;
impl Gpio1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio1 {
        match self.bits {
            false => Gpio1::Disabled,
            true => Gpio1::Enabled,
        }
    }
    #[doc = "Automatic clock gating is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio1::Disabled
    }
    #[doc = "Automatic clock gating is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio1::Enabled
    }
}
#[doc = "Field `GPIO1` writer - GPIO1"]
pub type Gpio1W<'a, REG> = crate::BitWriter<'a, REG, Gpio1>;
impl<'a, REG> Gpio1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Automatic clock gating is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1::Disabled)
    }
    #[doc = "Automatic clock gating is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1::Enabled)
    }
}
#[doc = "GPIO2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio2 {
    #[doc = "0: Automatic clock gating is disabled"]
    Disabled = 0,
    #[doc = "1: Automatic clock gating is enabled"]
    Enabled = 1,
}
impl From<Gpio2> for bool {
    #[inline(always)]
    fn from(variant: Gpio2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO2` reader - GPIO2"]
pub type Gpio2R = crate::BitReader<Gpio2>;
impl Gpio2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2 {
        match self.bits {
            false => Gpio2::Disabled,
            true => Gpio2::Enabled,
        }
    }
    #[doc = "Automatic clock gating is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio2::Disabled
    }
    #[doc = "Automatic clock gating is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio2::Enabled
    }
}
#[doc = "Field `GPIO2` writer - GPIO2"]
pub type Gpio2W<'a, REG> = crate::BitWriter<'a, REG, Gpio2>;
impl<'a, REG> Gpio2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Automatic clock gating is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2::Disabled)
    }
    #[doc = "Automatic clock gating is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2::Enabled)
    }
}
#[doc = "GPIO3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio3 {
    #[doc = "0: Automatic clock gating is disabled"]
    Disabled = 0,
    #[doc = "1: Automatic clock gating is enabled"]
    Enabled = 1,
}
impl From<Gpio3> for bool {
    #[inline(always)]
    fn from(variant: Gpio3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO3` reader - GPIO3"]
pub type Gpio3R = crate::BitReader<Gpio3>;
impl Gpio3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio3 {
        match self.bits {
            false => Gpio3::Disabled,
            true => Gpio3::Enabled,
        }
    }
    #[doc = "Automatic clock gating is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio3::Disabled
    }
    #[doc = "Automatic clock gating is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio3::Enabled
    }
}
#[doc = "Field `GPIO3` writer - GPIO3"]
pub type Gpio3W<'a, REG> = crate::BitWriter<'a, REG, Gpio3>;
impl<'a, REG> Gpio3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Automatic clock gating is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3::Disabled)
    }
    #[doc = "Automatic clock gating is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3::Enabled)
    }
}
#[doc = "GPIO4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio4 {
    #[doc = "0: Automatic clock gating is disabled"]
    Disabled = 0,
    #[doc = "1: Automatic clock gating is enabled"]
    Enabled = 1,
}
impl From<Gpio4> for bool {
    #[inline(always)]
    fn from(variant: Gpio4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO4` reader - GPIO4"]
pub type Gpio4R = crate::BitReader<Gpio4>;
impl Gpio4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio4 {
        match self.bits {
            false => Gpio4::Disabled,
            true => Gpio4::Enabled,
        }
    }
    #[doc = "Automatic clock gating is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio4::Disabled
    }
    #[doc = "Automatic clock gating is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio4::Enabled
    }
}
#[doc = "Field `GPIO4` writer - GPIO4"]
pub type Gpio4W<'a, REG> = crate::BitWriter<'a, REG, Gpio4>;
impl<'a, REG> Gpio4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Automatic clock gating is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4::Disabled)
    }
    #[doc = "Automatic clock gating is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4::Enabled)
    }
}
#[doc = "ROMC\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Romc {
    #[doc = "0: Automatic clock gating is disabled"]
    Disabled = 0,
    #[doc = "1: Automatic clock gating is enabled"]
    Enabled = 1,
}
impl From<Romc> for bool {
    #[inline(always)]
    fn from(variant: Romc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ROMC` reader - ROMC"]
pub type RomcR = crate::BitReader<Romc>;
impl RomcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Romc {
        match self.bits {
            false => Romc::Disabled,
            true => Romc::Enabled,
        }
    }
    #[doc = "Automatic clock gating is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Romc::Disabled
    }
    #[doc = "Automatic clock gating is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Romc::Enabled
    }
}
#[doc = "Field `ROMC` writer - ROMC"]
pub type RomcW<'a, REG> = crate::BitWriter<'a, REG, Romc>;
impl<'a, REG> RomcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Automatic clock gating is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Romc::Disabled)
    }
    #[doc = "Automatic clock gating is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Romc::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - OSTIMER0"]
    #[inline(always)]
    pub fn ostimer0(&self) -> Ostimer0R {
        Ostimer0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC0"]
    #[inline(always)]
    pub fn adc0(&self) -> Adc0R {
        Adc0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC1"]
    #[inline(always)]
    pub fn adc1(&self) -> Adc1R {
        Adc1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CMP0"]
    #[inline(always)]
    pub fn cmp0(&self) -> Cmp0R {
        Cmp0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CMP1"]
    #[inline(always)]
    pub fn cmp1(&self) -> Cmp1R {
        Cmp1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DAC0"]
    #[inline(always)]
    pub fn dac0(&self) -> Dac0R {
        Dac0R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - OPAMP0"]
    #[inline(always)]
    pub fn opamp0(&self) -> Opamp0R {
        Opamp0R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PORT0"]
    #[inline(always)]
    pub fn port0(&self) -> Port0R {
        Port0R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PORT1"]
    #[inline(always)]
    pub fn port1(&self) -> Port1R {
        Port1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PORT2"]
    #[inline(always)]
    pub fn port2(&self) -> Port2R {
        Port2R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PORT3"]
    #[inline(always)]
    pub fn port3(&self) -> Port3R {
        Port3R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PORT4"]
    #[inline(always)]
    pub fn port4(&self) -> Port4R {
        Port4R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - FLEXCAN0"]
    #[inline(always)]
    pub fn flexcan0(&self) -> Flexcan0R {
        Flexcan0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - LPI2C2"]
    #[inline(always)]
    pub fn lpi2c2(&self) -> Lpi2c2R {
        Lpi2c2R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - LPI2C3"]
    #[inline(always)]
    pub fn lpi2c3(&self) -> Lpi2c3R {
        Lpi2c3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 18 - RAMA"]
    #[inline(always)]
    pub fn rama(&self) -> RamaR {
        RamaR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - RAMB"]
    #[inline(always)]
    pub fn ramb(&self) -> RambR {
        RambR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - GPIO0"]
    #[inline(always)]
    pub fn gpio0(&self) -> Gpio0R {
        Gpio0R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - GPIO1"]
    #[inline(always)]
    pub fn gpio1(&self) -> Gpio1R {
        Gpio1R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - GPIO2"]
    #[inline(always)]
    pub fn gpio2(&self) -> Gpio2R {
        Gpio2R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GPIO3"]
    #[inline(always)]
    pub fn gpio3(&self) -> Gpio3R {
        Gpio3R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - GPIO4"]
    #[inline(always)]
    pub fn gpio4(&self) -> Gpio4R {
        Gpio4R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - ROMC"]
    #[inline(always)]
    pub fn romc(&self) -> RomcR {
        RomcR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OSTIMER0"]
    #[inline(always)]
    pub fn ostimer0(&mut self) -> Ostimer0W<'_, MrccGlbAcc1Spec> {
        Ostimer0W::new(self, 0)
    }
    #[doc = "Bit 1 - ADC0"]
    #[inline(always)]
    pub fn adc0(&mut self) -> Adc0W<'_, MrccGlbAcc1Spec> {
        Adc0W::new(self, 1)
    }
    #[doc = "Bit 2 - ADC1"]
    #[inline(always)]
    pub fn adc1(&mut self) -> Adc1W<'_, MrccGlbAcc1Spec> {
        Adc1W::new(self, 2)
    }
    #[doc = "Bit 3 - CMP0"]
    #[inline(always)]
    pub fn cmp0(&mut self) -> Cmp0W<'_, MrccGlbAcc1Spec> {
        Cmp0W::new(self, 3)
    }
    #[doc = "Bit 4 - CMP1"]
    #[inline(always)]
    pub fn cmp1(&mut self) -> Cmp1W<'_, MrccGlbAcc1Spec> {
        Cmp1W::new(self, 4)
    }
    #[doc = "Bit 5 - DAC0"]
    #[inline(always)]
    pub fn dac0(&mut self) -> Dac0W<'_, MrccGlbAcc1Spec> {
        Dac0W::new(self, 5)
    }
    #[doc = "Bit 6 - OPAMP0"]
    #[inline(always)]
    pub fn opamp0(&mut self) -> Opamp0W<'_, MrccGlbAcc1Spec> {
        Opamp0W::new(self, 6)
    }
    #[doc = "Bit 7 - PORT0"]
    #[inline(always)]
    pub fn port0(&mut self) -> Port0W<'_, MrccGlbAcc1Spec> {
        Port0W::new(self, 7)
    }
    #[doc = "Bit 8 - PORT1"]
    #[inline(always)]
    pub fn port1(&mut self) -> Port1W<'_, MrccGlbAcc1Spec> {
        Port1W::new(self, 8)
    }
    #[doc = "Bit 9 - PORT2"]
    #[inline(always)]
    pub fn port2(&mut self) -> Port2W<'_, MrccGlbAcc1Spec> {
        Port2W::new(self, 9)
    }
    #[doc = "Bit 10 - PORT3"]
    #[inline(always)]
    pub fn port3(&mut self) -> Port3W<'_, MrccGlbAcc1Spec> {
        Port3W::new(self, 10)
    }
    #[doc = "Bit 11 - PORT4"]
    #[inline(always)]
    pub fn port4(&mut self) -> Port4W<'_, MrccGlbAcc1Spec> {
        Port4W::new(self, 11)
    }
    #[doc = "Bit 12 - FLEXCAN0"]
    #[inline(always)]
    pub fn flexcan0(&mut self) -> Flexcan0W<'_, MrccGlbAcc1Spec> {
        Flexcan0W::new(self, 12)
    }
    #[doc = "Bit 13 - LPI2C2"]
    #[inline(always)]
    pub fn lpi2c2(&mut self) -> Lpi2c2W<'_, MrccGlbAcc1Spec> {
        Lpi2c2W::new(self, 13)
    }
    #[doc = "Bit 14 - LPI2C3"]
    #[inline(always)]
    pub fn lpi2c3(&mut self) -> Lpi2c3W<'_, MrccGlbAcc1Spec> {
        Lpi2c3W::new(self, 14)
    }
    #[doc = "Bit 18 - RAMA"]
    #[inline(always)]
    pub fn rama(&mut self) -> RamaW<'_, MrccGlbAcc1Spec> {
        RamaW::new(self, 18)
    }
    #[doc = "Bit 19 - RAMB"]
    #[inline(always)]
    pub fn ramb(&mut self) -> RambW<'_, MrccGlbAcc1Spec> {
        RambW::new(self, 19)
    }
    #[doc = "Bit 20 - GPIO0"]
    #[inline(always)]
    pub fn gpio0(&mut self) -> Gpio0W<'_, MrccGlbAcc1Spec> {
        Gpio0W::new(self, 20)
    }
    #[doc = "Bit 21 - GPIO1"]
    #[inline(always)]
    pub fn gpio1(&mut self) -> Gpio1W<'_, MrccGlbAcc1Spec> {
        Gpio1W::new(self, 21)
    }
    #[doc = "Bit 22 - GPIO2"]
    #[inline(always)]
    pub fn gpio2(&mut self) -> Gpio2W<'_, MrccGlbAcc1Spec> {
        Gpio2W::new(self, 22)
    }
    #[doc = "Bit 23 - GPIO3"]
    #[inline(always)]
    pub fn gpio3(&mut self) -> Gpio3W<'_, MrccGlbAcc1Spec> {
        Gpio3W::new(self, 23)
    }
    #[doc = "Bit 24 - GPIO4"]
    #[inline(always)]
    pub fn gpio4(&mut self) -> Gpio4W<'_, MrccGlbAcc1Spec> {
        Gpio4W::new(self, 24)
    }
    #[doc = "Bit 25 - ROMC"]
    #[inline(always)]
    pub fn romc(&mut self) -> RomcW<'_, MrccGlbAcc1Spec> {
        RomcW::new(self, 25)
    }
}
#[doc = "Control Automatic Clock Gating 1\n\nYou can [`read`](crate::Reg::read) this register and get [`mrcc_glb_acc1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mrcc_glb_acc1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MrccGlbAcc1Spec;
impl crate::RegisterSpec for MrccGlbAcc1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mrcc_glb_acc1::R`](R) reader structure"]
impl crate::Readable for MrccGlbAcc1Spec {}
#[doc = "`write(|w| ..)` method takes [`mrcc_glb_acc1::W`](W) writer structure"]
impl crate::Writable for MrccGlbAcc1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MRCC_GLB_ACC1 to value 0x020c_0000"]
impl crate::Resettable for MrccGlbAcc1Spec {
    const RESET_VALUE: u32 = 0x020c_0000;
}
