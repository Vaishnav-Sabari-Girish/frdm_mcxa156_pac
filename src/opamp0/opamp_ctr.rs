#[doc = "Register `OPAMP_CTR` reader"]
pub type R = crate::R<OpampCtrSpec>;
#[doc = "Register `OPAMP_CTR` writer"]
pub type W = crate::W<OpampCtrSpec>;
#[doc = "OPAMP Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum En {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<En> for bool {
    #[inline(always)]
    fn from(variant: En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` reader - OPAMP Enable"]
pub type EnR = crate::BitReader<En>;
impl EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> En {
        match self.bits {
            false => En::Disable,
            true => En::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == En::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == En::Enable
    }
}
#[doc = "Field `EN` writer - OPAMP Enable"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG, En>;
impl<'a, REG> EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(En::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(En::Enable)
    }
}
#[doc = "Mode Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    #[doc = "0: High performance mode"]
    Low = 0,
    #[doc = "1: Low power mode"]
    High = 1,
}
impl From<Mode> for bool {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE` reader - Mode Selection"]
pub type ModeR = crate::BitReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode {
        match self.bits {
            false => Mode::Low,
            true => Mode::High,
        }
    }
    #[doc = "High performance mode"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Mode::Low
    }
    #[doc = "Low power mode"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Mode::High
    }
}
#[doc = "Field `MODE` writer - Mode Selection"]
pub type ModeW<'a, REG> = crate::BitWriter<'a, REG, Mode>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "High performance mode"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Low)
    }
    #[doc = "Low power mode"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::High)
    }
}
#[doc = "Bias Current Trim Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Biasc {
    #[doc = "0: Default"]
    Def = 0,
    #[doc = "1: Increase current"]
    Inc = 1,
    #[doc = "2: Decrease current"]
    Dec = 2,
    #[doc = "3: Further decrease current"]
    FurDec = 3,
}
impl From<Biasc> for u8 {
    #[inline(always)]
    fn from(variant: Biasc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Biasc {
    type Ux = u8;
}
impl crate::IsEnum for Biasc {}
#[doc = "Field `BIASC` reader - Bias Current Trim Selection"]
pub type BiascR = crate::FieldReader<Biasc>;
impl BiascR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Biasc {
        match self.bits {
            0 => Biasc::Def,
            1 => Biasc::Inc,
            2 => Biasc::Dec,
            3 => Biasc::FurDec,
            _ => unreachable!(),
        }
    }
    #[doc = "Default"]
    #[inline(always)]
    pub fn is_def(&self) -> bool {
        *self == Biasc::Def
    }
    #[doc = "Increase current"]
    #[inline(always)]
    pub fn is_inc(&self) -> bool {
        *self == Biasc::Inc
    }
    #[doc = "Decrease current"]
    #[inline(always)]
    pub fn is_dec(&self) -> bool {
        *self == Biasc::Dec
    }
    #[doc = "Further decrease current"]
    #[inline(always)]
    pub fn is_fur_dec(&self) -> bool {
        *self == Biasc::FurDec
    }
}
#[doc = "Field `BIASC` writer - Bias Current Trim Selection"]
pub type BiascW<'a, REG> = crate::FieldWriter<'a, REG, 2, Biasc, crate::Safe>;
impl<'a, REG> BiascW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Default"]
    #[inline(always)]
    pub fn def(self) -> &'a mut crate::W<REG> {
        self.variant(Biasc::Def)
    }
    #[doc = "Increase current"]
    #[inline(always)]
    pub fn inc(self) -> &'a mut crate::W<REG> {
        self.variant(Biasc::Inc)
    }
    #[doc = "Decrease current"]
    #[inline(always)]
    pub fn dec(self) -> &'a mut crate::W<REG> {
        self.variant(Biasc::Dec)
    }
    #[doc = "Further decrease current"]
    #[inline(always)]
    pub fn fur_dec(self) -> &'a mut crate::W<REG> {
        self.variant(Biasc::FurDec)
    }
}
#[doc = "Provide OPAMP rail to rail voltage selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Intref {
    #[doc = "0: Select OPAMP input rail to rail voltage from 0 to VDD_ANA"]
    Vdda2 = 0,
    #[doc = "1: Select OPAMP input rail to rail voltage from 0 to VDD_ANA-0.8V"]
    Vdda3v = 1,
    #[doc = "2: Select OPAMP input rail to rail voltage from 0.8V to VDD_ANA"]
    Vssa3v = 2,
    #[doc = "3: Not allowed"]
    Not = 3,
}
impl From<Intref> for u8 {
    #[inline(always)]
    fn from(variant: Intref) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Intref {
    type Ux = u8;
}
impl crate::IsEnum for Intref {}
#[doc = "Field `INTREF` reader - Provide OPAMP rail to rail voltage selection"]
pub type IntrefR = crate::FieldReader<Intref>;
impl IntrefR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Intref {
        match self.bits {
            0 => Intref::Vdda2,
            1 => Intref::Vdda3v,
            2 => Intref::Vssa3v,
            3 => Intref::Not,
            _ => unreachable!(),
        }
    }
    #[doc = "Select OPAMP input rail to rail voltage from 0 to VDD_ANA"]
    #[inline(always)]
    pub fn is_vdda2(&self) -> bool {
        *self == Intref::Vdda2
    }
    #[doc = "Select OPAMP input rail to rail voltage from 0 to VDD_ANA-0.8V"]
    #[inline(always)]
    pub fn is_vdda3v(&self) -> bool {
        *self == Intref::Vdda3v
    }
    #[doc = "Select OPAMP input rail to rail voltage from 0.8V to VDD_ANA"]
    #[inline(always)]
    pub fn is_vssa3v(&self) -> bool {
        *self == Intref::Vssa3v
    }
    #[doc = "Not allowed"]
    #[inline(always)]
    pub fn is_not(&self) -> bool {
        *self == Intref::Not
    }
}
#[doc = "Field `INTREF` writer - Provide OPAMP rail to rail voltage selection"]
pub type IntrefW<'a, REG> = crate::FieldWriter<'a, REG, 2, Intref, crate::Safe>;
impl<'a, REG> IntrefW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select OPAMP input rail to rail voltage from 0 to VDD_ANA"]
    #[inline(always)]
    pub fn vdda2(self) -> &'a mut crate::W<REG> {
        self.variant(Intref::Vdda2)
    }
    #[doc = "Select OPAMP input rail to rail voltage from 0 to VDD_ANA-0.8V"]
    #[inline(always)]
    pub fn vdda3v(self) -> &'a mut crate::W<REG> {
        self.variant(Intref::Vdda3v)
    }
    #[doc = "Select OPAMP input rail to rail voltage from 0.8V to VDD_ANA"]
    #[inline(always)]
    pub fn vssa3v(self) -> &'a mut crate::W<REG> {
        self.variant(Intref::Vssa3v)
    }
    #[doc = "Not allowed"]
    #[inline(always)]
    pub fn not(self) -> &'a mut crate::W<REG> {
        self.variant(Intref::Not)
    }
}
#[doc = "Trigger Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trigmd {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<Trigmd> for bool {
    #[inline(always)]
    fn from(variant: Trigmd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGMD` reader - Trigger Mode"]
pub type TrigmdR = crate::BitReader<Trigmd>;
impl TrigmdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trigmd {
        match self.bits {
            false => Trigmd::Disable,
            true => Trigmd::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Trigmd::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Trigmd::Enable
    }
}
#[doc = "Field `TRIGMD` writer - Trigger Mode"]
pub type TrigmdW<'a, REG> = crate::BitWriter<'a, REG, Trigmd>;
impl<'a, REG> TrigmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Trigmd::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Trigmd::Enable)
    }
}
#[doc = "Positive Input Channel Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Inpsel {
    #[doc = "0: When OPAMP is not in trigger mode, select positive input 0 (INP0)"]
    Inp0 = 0,
    #[doc = "1: When OPAMP is not in trigger mode, select positive input 1 (INP1)"]
    Inp1 = 1,
}
impl From<Inpsel> for bool {
    #[inline(always)]
    fn from(variant: Inpsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INPSEL` reader - Positive Input Channel Selection"]
pub type InpselR = crate::BitReader<Inpsel>;
impl InpselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Inpsel {
        match self.bits {
            false => Inpsel::Inp0,
            true => Inpsel::Inp1,
        }
    }
    #[doc = "When OPAMP is not in trigger mode, select positive input 0 (INP0)"]
    #[inline(always)]
    pub fn is_inp0(&self) -> bool {
        *self == Inpsel::Inp0
    }
    #[doc = "When OPAMP is not in trigger mode, select positive input 1 (INP1)"]
    #[inline(always)]
    pub fn is_inp1(&self) -> bool {
        *self == Inpsel::Inp1
    }
}
#[doc = "Field `INPSEL` writer - Positive Input Channel Selection"]
pub type InpselW<'a, REG> = crate::BitWriter<'a, REG, Inpsel>;
impl<'a, REG> InpselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "When OPAMP is not in trigger mode, select positive input 0 (INP0)"]
    #[inline(always)]
    pub fn inp0(self) -> &'a mut crate::W<REG> {
        self.variant(Inpsel::Inp0)
    }
    #[doc = "When OPAMP is not in trigger mode, select positive input 1 (INP1)"]
    #[inline(always)]
    pub fn inp1(self) -> &'a mut crate::W<REG> {
        self.variant(Inpsel::Inp1)
    }
}
#[doc = "Positive Input Connection Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Inpf {
    #[doc = "0: Positive input 0 (INP0)"]
    Inp0 = 0,
    #[doc = "1: Positive input 1 (INP1)"]
    Inp1 = 1,
}
impl From<Inpf> for bool {
    #[inline(always)]
    fn from(variant: Inpf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INPF` reader - Positive Input Connection Status"]
pub type InpfR = crate::BitReader<Inpf>;
impl InpfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Inpf {
        match self.bits {
            false => Inpf::Inp0,
            true => Inpf::Inp1,
        }
    }
    #[doc = "Positive input 0 (INP0)"]
    #[inline(always)]
    pub fn is_inp0(&self) -> bool {
        *self == Inpf::Inp0
    }
    #[doc = "Positive input 1 (INP1)"]
    #[inline(always)]
    pub fn is_inp1(&self) -> bool {
        *self == Inpf::Inp1
    }
}
#[doc = "Reference Buffer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bufen {
    #[doc = "0: Disables"]
    Disable = 0,
    #[doc = "1: Enables"]
    Enable = 1,
}
impl From<Bufen> for bool {
    #[inline(always)]
    fn from(variant: Bufen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUFEN` reader - Reference Buffer"]
pub type BufenR = crate::BitReader<Bufen>;
impl BufenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bufen {
        match self.bits {
            false => Bufen::Disable,
            true => Bufen::Enable,
        }
    }
    #[doc = "Disables"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Bufen::Disable
    }
    #[doc = "Enables"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Bufen::Enable
    }
}
#[doc = "Field `BUFEN` writer - Reference Buffer"]
pub type BufenW<'a, REG> = crate::BitWriter<'a, REG, Bufen>;
impl<'a, REG> BufenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Bufen::Disable)
    }
    #[doc = "Enables"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Bufen::Enable)
    }
}
#[doc = "Positive Reference Voltage Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pref {
    #[doc = "0: Input 0"]
    Val0 = 0,
    #[doc = "1: Input 1"]
    Val1 = 1,
    #[doc = "2: Input 2"]
    Val2 = 2,
    #[doc = "3: Input 3"]
    Val3 = 3,
}
impl From<Pref> for u8 {
    #[inline(always)]
    fn from(variant: Pref) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pref {
    type Ux = u8;
}
impl crate::IsEnum for Pref {}
#[doc = "Field `PREF` reader - Positive Reference Voltage Selection"]
pub type PrefR = crate::FieldReader<Pref>;
impl PrefR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pref {
        match self.bits {
            0 => Pref::Val0,
            1 => Pref::Val1,
            2 => Pref::Val2,
            3 => Pref::Val3,
            _ => unreachable!(),
        }
    }
    #[doc = "Input 0"]
    #[inline(always)]
    pub fn is_val0(&self) -> bool {
        *self == Pref::Val0
    }
    #[doc = "Input 1"]
    #[inline(always)]
    pub fn is_val1(&self) -> bool {
        *self == Pref::Val1
    }
    #[doc = "Input 2"]
    #[inline(always)]
    pub fn is_val2(&self) -> bool {
        *self == Pref::Val2
    }
    #[doc = "Input 3"]
    #[inline(always)]
    pub fn is_val3(&self) -> bool {
        *self == Pref::Val3
    }
}
#[doc = "Field `PREF` writer - Positive Reference Voltage Selection"]
pub type PrefW<'a, REG> = crate::FieldWriter<'a, REG, 2, Pref, crate::Safe>;
impl<'a, REG> PrefW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input 0"]
    #[inline(always)]
    pub fn val0(self) -> &'a mut crate::W<REG> {
        self.variant(Pref::Val0)
    }
    #[doc = "Input 1"]
    #[inline(always)]
    pub fn val1(self) -> &'a mut crate::W<REG> {
        self.variant(Pref::Val1)
    }
    #[doc = "Input 2"]
    #[inline(always)]
    pub fn val2(self) -> &'a mut crate::W<REG> {
        self.variant(Pref::Val2)
    }
    #[doc = "Input 3"]
    #[inline(always)]
    pub fn val3(self) -> &'a mut crate::W<REG> {
        self.variant(Pref::Val3)
    }
}
#[doc = "Measure Switch 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adcsw1 {
    #[doc = "0: Measure negative gain resistor ladder voltage switch off"]
    Off = 0,
    #[doc = "1: Measure negative gain resistor ladder voltage switch on"]
    On = 1,
}
impl From<Adcsw1> for bool {
    #[inline(always)]
    fn from(variant: Adcsw1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCSW1` reader - Measure Switch 1"]
pub type Adcsw1R = crate::BitReader<Adcsw1>;
impl Adcsw1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adcsw1 {
        match self.bits {
            false => Adcsw1::Off,
            true => Adcsw1::On,
        }
    }
    #[doc = "Measure negative gain resistor ladder voltage switch off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Adcsw1::Off
    }
    #[doc = "Measure negative gain resistor ladder voltage switch on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Adcsw1::On
    }
}
#[doc = "Field `ADCSW1` writer - Measure Switch 1"]
pub type Adcsw1W<'a, REG> = crate::BitWriter<'a, REG, Adcsw1>;
impl<'a, REG> Adcsw1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Measure negative gain resistor ladder voltage switch off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Adcsw1::Off)
    }
    #[doc = "Measure negative gain resistor ladder voltage switch on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Adcsw1::On)
    }
}
#[doc = "Measure Switch 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adcsw2 {
    #[doc = "0: Measure positive gain resistor ladder reference voltage switch off"]
    Disable = 0,
    #[doc = "1: Measure positive gain resistor ladder reference voltage switch on"]
    Enable = 1,
}
impl From<Adcsw2> for bool {
    #[inline(always)]
    fn from(variant: Adcsw2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCSW2` reader - Measure Switch 2"]
pub type Adcsw2R = crate::BitReader<Adcsw2>;
impl Adcsw2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adcsw2 {
        match self.bits {
            false => Adcsw2::Disable,
            true => Adcsw2::Enable,
        }
    }
    #[doc = "Measure positive gain resistor ladder reference voltage switch off"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Adcsw2::Disable
    }
    #[doc = "Measure positive gain resistor ladder reference voltage switch on"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Adcsw2::Enable
    }
}
#[doc = "Field `ADCSW2` writer - Measure Switch 2"]
pub type Adcsw2W<'a, REG> = crate::BitWriter<'a, REG, Adcsw2>;
impl<'a, REG> Adcsw2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Measure positive gain resistor ladder reference voltage switch off"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Adcsw2::Disable)
    }
    #[doc = "Measure positive gain resistor ladder reference voltage switch on"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Adcsw2::Enable)
    }
}
#[doc = "Output Switch\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outsw {
    #[doc = "0: OPAMP out to negative gain resistor ladder switch off"]
    Off = 0,
    #[doc = "1: OPAMP out to negative gain resistor ladder switch on"]
    On = 1,
}
impl From<Outsw> for bool {
    #[inline(always)]
    fn from(variant: Outsw) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OUTSW` reader - Output Switch"]
pub type OutswR = crate::BitReader<Outsw>;
impl OutswR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Outsw {
        match self.bits {
            false => Outsw::Off,
            true => Outsw::On,
        }
    }
    #[doc = "OPAMP out to negative gain resistor ladder switch off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Outsw::Off
    }
    #[doc = "OPAMP out to negative gain resistor ladder switch on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Outsw::On
    }
}
#[doc = "Field `OUTSW` writer - Output Switch"]
pub type OutswW<'a, REG> = crate::BitWriter<'a, REG, Outsw>;
impl<'a, REG> OutswW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "OPAMP out to negative gain resistor ladder switch off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Outsw::Off)
    }
    #[doc = "OPAMP out to negative gain resistor ladder switch on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Outsw::On)
    }
}
#[doc = "Positive PGA Selection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pgain {
    #[doc = "0: Positive input 1 (INP1)"]
    Inp1 = 0,
    #[doc = "1: Pgain=1"]
    G2 = 1,
    #[doc = "2: Pgain=2"]
    G3 = 2,
    #[doc = "3: Pgain=4"]
    G5 = 3,
    #[doc = "4: Pgain=8"]
    G9 = 4,
    #[doc = "5: Pgain=16"]
    G17 = 5,
    #[doc = "6: Pgain=33"]
    G34 = 6,
    #[doc = "7: Pgain=64"]
    G65 = 7,
}
impl From<Pgain> for u8 {
    #[inline(always)]
    fn from(variant: Pgain) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pgain {
    type Ux = u8;
}
impl crate::IsEnum for Pgain {}
#[doc = "Field `PGAIN` reader - Positive PGA Selection"]
pub type PgainR = crate::FieldReader<Pgain>;
impl PgainR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pgain {
        match self.bits {
            0 => Pgain::Inp1,
            1 => Pgain::G2,
            2 => Pgain::G3,
            3 => Pgain::G5,
            4 => Pgain::G9,
            5 => Pgain::G17,
            6 => Pgain::G34,
            7 => Pgain::G65,
            _ => unreachable!(),
        }
    }
    #[doc = "Positive input 1 (INP1)"]
    #[inline(always)]
    pub fn is_inp1(&self) -> bool {
        *self == Pgain::Inp1
    }
    #[doc = "Pgain=1"]
    #[inline(always)]
    pub fn is_g2(&self) -> bool {
        *self == Pgain::G2
    }
    #[doc = "Pgain=2"]
    #[inline(always)]
    pub fn is_g3(&self) -> bool {
        *self == Pgain::G3
    }
    #[doc = "Pgain=4"]
    #[inline(always)]
    pub fn is_g5(&self) -> bool {
        *self == Pgain::G5
    }
    #[doc = "Pgain=8"]
    #[inline(always)]
    pub fn is_g9(&self) -> bool {
        *self == Pgain::G9
    }
    #[doc = "Pgain=16"]
    #[inline(always)]
    pub fn is_g17(&self) -> bool {
        *self == Pgain::G17
    }
    #[doc = "Pgain=33"]
    #[inline(always)]
    pub fn is_g34(&self) -> bool {
        *self == Pgain::G34
    }
    #[doc = "Pgain=64"]
    #[inline(always)]
    pub fn is_g65(&self) -> bool {
        *self == Pgain::G65
    }
}
#[doc = "Field `PGAIN` writer - Positive PGA Selection"]
pub type PgainW<'a, REG> = crate::FieldWriter<'a, REG, 3, Pgain, crate::Safe>;
impl<'a, REG> PgainW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Positive input 1 (INP1)"]
    #[inline(always)]
    pub fn inp1(self) -> &'a mut crate::W<REG> {
        self.variant(Pgain::Inp1)
    }
    #[doc = "Pgain=1"]
    #[inline(always)]
    pub fn g2(self) -> &'a mut crate::W<REG> {
        self.variant(Pgain::G2)
    }
    #[doc = "Pgain=2"]
    #[inline(always)]
    pub fn g3(self) -> &'a mut crate::W<REG> {
        self.variant(Pgain::G3)
    }
    #[doc = "Pgain=4"]
    #[inline(always)]
    pub fn g5(self) -> &'a mut crate::W<REG> {
        self.variant(Pgain::G5)
    }
    #[doc = "Pgain=8"]
    #[inline(always)]
    pub fn g9(self) -> &'a mut crate::W<REG> {
        self.variant(Pgain::G9)
    }
    #[doc = "Pgain=16"]
    #[inline(always)]
    pub fn g17(self) -> &'a mut crate::W<REG> {
        self.variant(Pgain::G17)
    }
    #[doc = "Pgain=33"]
    #[inline(always)]
    pub fn g34(self) -> &'a mut crate::W<REG> {
        self.variant(Pgain::G34)
    }
    #[doc = "Pgain=64"]
    #[inline(always)]
    pub fn g65(self) -> &'a mut crate::W<REG> {
        self.variant(Pgain::G65)
    }
}
#[doc = "Negative PGA Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ngain {
    #[doc = "0: Buffer"]
    Buffer = 0,
    #[doc = "1: Ngain=1"]
    G1 = 1,
    #[doc = "2: Ngain=2"]
    G2 = 2,
    #[doc = "3: Ngain=4"]
    G4 = 3,
    #[doc = "4: Ngain=8"]
    G8 = 4,
    #[doc = "5: Ngain=16"]
    G16 = 5,
    #[doc = "6: Ngain=33"]
    G33 = 6,
    #[doc = "7: Ngain=64"]
    G64 = 7,
}
impl From<Ngain> for u8 {
    #[inline(always)]
    fn from(variant: Ngain) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ngain {
    type Ux = u8;
}
impl crate::IsEnum for Ngain {}
#[doc = "Field `NGAIN` reader - Negative PGA Selection"]
pub type NgainR = crate::FieldReader<Ngain>;
impl NgainR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ngain {
        match self.bits {
            0 => Ngain::Buffer,
            1 => Ngain::G1,
            2 => Ngain::G2,
            3 => Ngain::G4,
            4 => Ngain::G8,
            5 => Ngain::G16,
            6 => Ngain::G33,
            7 => Ngain::G64,
            _ => unreachable!(),
        }
    }
    #[doc = "Buffer"]
    #[inline(always)]
    pub fn is_buffer(&self) -> bool {
        *self == Ngain::Buffer
    }
    #[doc = "Ngain=1"]
    #[inline(always)]
    pub fn is_g1(&self) -> bool {
        *self == Ngain::G1
    }
    #[doc = "Ngain=2"]
    #[inline(always)]
    pub fn is_g2(&self) -> bool {
        *self == Ngain::G2
    }
    #[doc = "Ngain=4"]
    #[inline(always)]
    pub fn is_g4(&self) -> bool {
        *self == Ngain::G4
    }
    #[doc = "Ngain=8"]
    #[inline(always)]
    pub fn is_g8(&self) -> bool {
        *self == Ngain::G8
    }
    #[doc = "Ngain=16"]
    #[inline(always)]
    pub fn is_g16(&self) -> bool {
        *self == Ngain::G16
    }
    #[doc = "Ngain=33"]
    #[inline(always)]
    pub fn is_g33(&self) -> bool {
        *self == Ngain::G33
    }
    #[doc = "Ngain=64"]
    #[inline(always)]
    pub fn is_g64(&self) -> bool {
        *self == Ngain::G64
    }
}
#[doc = "Field `NGAIN` writer - Negative PGA Selection"]
pub type NgainW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ngain, crate::Safe>;
impl<'a, REG> NgainW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Buffer"]
    #[inline(always)]
    pub fn buffer(self) -> &'a mut crate::W<REG> {
        self.variant(Ngain::Buffer)
    }
    #[doc = "Ngain=1"]
    #[inline(always)]
    pub fn g1(self) -> &'a mut crate::W<REG> {
        self.variant(Ngain::G1)
    }
    #[doc = "Ngain=2"]
    #[inline(always)]
    pub fn g2(self) -> &'a mut crate::W<REG> {
        self.variant(Ngain::G2)
    }
    #[doc = "Ngain=4"]
    #[inline(always)]
    pub fn g4(self) -> &'a mut crate::W<REG> {
        self.variant(Ngain::G4)
    }
    #[doc = "Ngain=8"]
    #[inline(always)]
    pub fn g8(self) -> &'a mut crate::W<REG> {
        self.variant(Ngain::G8)
    }
    #[doc = "Ngain=16"]
    #[inline(always)]
    pub fn g16(self) -> &'a mut crate::W<REG> {
        self.variant(Ngain::G16)
    }
    #[doc = "Ngain=33"]
    #[inline(always)]
    pub fn g33(self) -> &'a mut crate::W<REG> {
        self.variant(Ngain::G33)
    }
    #[doc = "Ngain=64"]
    #[inline(always)]
    pub fn g64(self) -> &'a mut crate::W<REG> {
        self.variant(Ngain::G64)
    }
}
impl R {
    #[doc = "Bit 0 - OPAMP Enable"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mode Selection"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Bias Current Trim Selection"]
    #[inline(always)]
    pub fn biasc(&self) -> BiascR {
        BiascR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Provide OPAMP rail to rail voltage selection"]
    #[inline(always)]
    pub fn intref(&self) -> IntrefR {
        IntrefR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8 - Trigger Mode"]
    #[inline(always)]
    pub fn trigmd(&self) -> TrigmdR {
        TrigmdR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Positive Input Channel Selection"]
    #[inline(always)]
    pub fn inpsel(&self) -> InpselR {
        InpselR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Positive Input Connection Status"]
    #[inline(always)]
    pub fn inpf(&self) -> InpfR {
        InpfR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Reference Buffer"]
    #[inline(always)]
    pub fn bufen(&self) -> BufenR {
        BufenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - Positive Reference Voltage Selection"]
    #[inline(always)]
    pub fn pref(&self) -> PrefR {
        PrefR::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 20 - Measure Switch 1"]
    #[inline(always)]
    pub fn adcsw1(&self) -> Adcsw1R {
        Adcsw1R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Measure Switch 2"]
    #[inline(always)]
    pub fn adcsw2(&self) -> Adcsw2R {
        Adcsw2R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Output Switch"]
    #[inline(always)]
    pub fn outsw(&self) -> OutswR {
        OutswR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Positive PGA Selection"]
    #[inline(always)]
    pub fn pgain(&self) -> PgainR {
        PgainR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - Negative PGA Selection"]
    #[inline(always)]
    pub fn ngain(&self) -> NgainR {
        NgainR::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - OPAMP Enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, OpampCtrSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 1 - Mode Selection"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, OpampCtrSpec> {
        ModeW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Bias Current Trim Selection"]
    #[inline(always)]
    pub fn biasc(&mut self) -> BiascW<'_, OpampCtrSpec> {
        BiascW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Provide OPAMP rail to rail voltage selection"]
    #[inline(always)]
    pub fn intref(&mut self) -> IntrefW<'_, OpampCtrSpec> {
        IntrefW::new(self, 4)
    }
    #[doc = "Bit 8 - Trigger Mode"]
    #[inline(always)]
    pub fn trigmd(&mut self) -> TrigmdW<'_, OpampCtrSpec> {
        TrigmdW::new(self, 8)
    }
    #[doc = "Bit 9 - Positive Input Channel Selection"]
    #[inline(always)]
    pub fn inpsel(&mut self) -> InpselW<'_, OpampCtrSpec> {
        InpselW::new(self, 9)
    }
    #[doc = "Bit 16 - Reference Buffer"]
    #[inline(always)]
    pub fn bufen(&mut self) -> BufenW<'_, OpampCtrSpec> {
        BufenW::new(self, 16)
    }
    #[doc = "Bits 17:18 - Positive Reference Voltage Selection"]
    #[inline(always)]
    pub fn pref(&mut self) -> PrefW<'_, OpampCtrSpec> {
        PrefW::new(self, 17)
    }
    #[doc = "Bit 20 - Measure Switch 1"]
    #[inline(always)]
    pub fn adcsw1(&mut self) -> Adcsw1W<'_, OpampCtrSpec> {
        Adcsw1W::new(self, 20)
    }
    #[doc = "Bit 21 - Measure Switch 2"]
    #[inline(always)]
    pub fn adcsw2(&mut self) -> Adcsw2W<'_, OpampCtrSpec> {
        Adcsw2W::new(self, 21)
    }
    #[doc = "Bit 22 - Output Switch"]
    #[inline(always)]
    pub fn outsw(&mut self) -> OutswW<'_, OpampCtrSpec> {
        OutswW::new(self, 22)
    }
    #[doc = "Bits 24:26 - Positive PGA Selection"]
    #[inline(always)]
    pub fn pgain(&mut self) -> PgainW<'_, OpampCtrSpec> {
        PgainW::new(self, 24)
    }
    #[doc = "Bits 28:30 - Negative PGA Selection"]
    #[inline(always)]
    pub fn ngain(&mut self) -> NgainW<'_, OpampCtrSpec> {
        NgainW::new(self, 28)
    }
}
#[doc = "OPAMP Control\n\nYou can [`read`](crate::Reg::read) this register and get [`opamp_ctr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp_ctr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OpampCtrSpec;
impl crate::RegisterSpec for OpampCtrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`opamp_ctr::R`](R) reader structure"]
impl crate::Readable for OpampCtrSpec {}
#[doc = "`write(|w| ..)` method takes [`opamp_ctr::W`](W) writer structure"]
impl crate::Writable for OpampCtrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OPAMP_CTR to value 0x0100_0000"]
impl crate::Resettable for OpampCtrSpec {
    const RESET_VALUE: u32 = 0x0100_0000;
}
