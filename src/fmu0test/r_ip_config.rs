#[doc = "Register `R_IP_CONFIG` reader"]
pub type R = crate::R<RIpConfigSpec>;
#[doc = "Register `R_IP_CONFIG` writer"]
pub type W = crate::W<RIpConfigSpec>;
#[doc = "Block 0 Select Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ipsel0 {
    #[doc = "0: Unselect block 0"]
    Zz223 = 0,
    #[doc = "1: not used, reserved"]
    Zz224 = 1,
    #[doc = "2: Enable block 0 test, repair off (default)"]
    Zz225 = 2,
    #[doc = "3: Enable block 0 test, repair on"]
    Zz226 = 3,
}
impl From<Ipsel0> for u8 {
    #[inline(always)]
    fn from(variant: Ipsel0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ipsel0 {
    type Ux = u8;
}
impl crate::IsEnum for Ipsel0 {}
#[doc = "Field `IPSEL0` reader - Block 0 Select Control"]
pub type Ipsel0R = crate::FieldReader<Ipsel0>;
impl Ipsel0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ipsel0 {
        match self.bits {
            0 => Ipsel0::Zz223,
            1 => Ipsel0::Zz224,
            2 => Ipsel0::Zz225,
            3 => Ipsel0::Zz226,
            _ => unreachable!(),
        }
    }
    #[doc = "Unselect block 0"]
    #[inline(always)]
    pub fn is_zz223(&self) -> bool {
        *self == Ipsel0::Zz223
    }
    #[doc = "not used, reserved"]
    #[inline(always)]
    pub fn is_zz224(&self) -> bool {
        *self == Ipsel0::Zz224
    }
    #[doc = "Enable block 0 test, repair off (default)"]
    #[inline(always)]
    pub fn is_zz225(&self) -> bool {
        *self == Ipsel0::Zz225
    }
    #[doc = "Enable block 0 test, repair on"]
    #[inline(always)]
    pub fn is_zz226(&self) -> bool {
        *self == Ipsel0::Zz226
    }
}
#[doc = "Field `IPSEL0` writer - Block 0 Select Control"]
pub type Ipsel0W<'a, REG> = crate::FieldWriter<'a, REG, 2, Ipsel0, crate::Safe>;
impl<'a, REG> Ipsel0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Unselect block 0"]
    #[inline(always)]
    pub fn zz223(self) -> &'a mut crate::W<REG> {
        self.variant(Ipsel0::Zz223)
    }
    #[doc = "not used, reserved"]
    #[inline(always)]
    pub fn zz224(self) -> &'a mut crate::W<REG> {
        self.variant(Ipsel0::Zz224)
    }
    #[doc = "Enable block 0 test, repair off (default)"]
    #[inline(always)]
    pub fn zz225(self) -> &'a mut crate::W<REG> {
        self.variant(Ipsel0::Zz225)
    }
    #[doc = "Enable block 0 test, repair on"]
    #[inline(always)]
    pub fn zz226(self) -> &'a mut crate::W<REG> {
        self.variant(Ipsel0::Zz226)
    }
}
#[doc = "Block 1 Select Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ipsel1 {
    #[doc = "0: Unselect block 1"]
    Zz219 = 0,
    #[doc = "1: not used, reserved"]
    Zz220 = 1,
    #[doc = "2: Enable block 1 test, repair off (default)"]
    Zz221 = 2,
    #[doc = "3: Enable block 1 test, repair on"]
    Zz222 = 3,
}
impl From<Ipsel1> for u8 {
    #[inline(always)]
    fn from(variant: Ipsel1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ipsel1 {
    type Ux = u8;
}
impl crate::IsEnum for Ipsel1 {}
#[doc = "Field `IPSEL1` reader - Block 1 Select Control"]
pub type Ipsel1R = crate::FieldReader<Ipsel1>;
impl Ipsel1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ipsel1 {
        match self.bits {
            0 => Ipsel1::Zz219,
            1 => Ipsel1::Zz220,
            2 => Ipsel1::Zz221,
            3 => Ipsel1::Zz222,
            _ => unreachable!(),
        }
    }
    #[doc = "Unselect block 1"]
    #[inline(always)]
    pub fn is_zz219(&self) -> bool {
        *self == Ipsel1::Zz219
    }
    #[doc = "not used, reserved"]
    #[inline(always)]
    pub fn is_zz220(&self) -> bool {
        *self == Ipsel1::Zz220
    }
    #[doc = "Enable block 1 test, repair off (default)"]
    #[inline(always)]
    pub fn is_zz221(&self) -> bool {
        *self == Ipsel1::Zz221
    }
    #[doc = "Enable block 1 test, repair on"]
    #[inline(always)]
    pub fn is_zz222(&self) -> bool {
        *self == Ipsel1::Zz222
    }
}
#[doc = "Field `IPSEL1` writer - Block 1 Select Control"]
pub type Ipsel1W<'a, REG> = crate::FieldWriter<'a, REG, 2, Ipsel1, crate::Safe>;
impl<'a, REG> Ipsel1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Unselect block 1"]
    #[inline(always)]
    pub fn zz219(self) -> &'a mut crate::W<REG> {
        self.variant(Ipsel1::Zz219)
    }
    #[doc = "not used, reserved"]
    #[inline(always)]
    pub fn zz220(self) -> &'a mut crate::W<REG> {
        self.variant(Ipsel1::Zz220)
    }
    #[doc = "Enable block 1 test, repair off (default)"]
    #[inline(always)]
    pub fn zz221(self) -> &'a mut crate::W<REG> {
        self.variant(Ipsel1::Zz221)
    }
    #[doc = "Enable block 1 test, repair on"]
    #[inline(always)]
    pub fn zz222(self) -> &'a mut crate::W<REG> {
        self.variant(Ipsel1::Zz222)
    }
}
#[doc = "Field `BIST_CDIVL` reader - Clock Divide Scalar for Long Pulse"]
pub type BistCdivlR = crate::FieldReader;
#[doc = "Field `BIST_CDIVL` writer - Clock Divide Scalar for Long Pulse"]
pub type BistCdivlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CDIVS` reader - Number of clock cycles to generate short pulse"]
pub type CdivsR = crate::FieldReader;
#[doc = "Field `CDIVS` writer - Number of clock cycles to generate short pulse"]
pub type CdivsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BIST_TVFY` reader - Timer adjust for verify"]
pub type BistTvfyR = crate::FieldReader;
#[doc = "Field `BIST_TVFY` writer - Timer adjust for verify"]
pub type BistTvfyW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "BIST self-test control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tstctl {
    #[doc = "0: Default, disable both BIST self-test and MISR"]
    Zz215 = 0,
    #[doc = "1: Enable BIST self-test mode DOUT from macro will be forced to '0', and disable MISR."]
    Zz216 = 1,
    #[doc = "2: Enable MISR"]
    Zz217 = 2,
    #[doc = "3: Enable both BIST self-test mode and MISR"]
    Zz218 = 3,
}
impl From<Tstctl> for u8 {
    #[inline(always)]
    fn from(variant: Tstctl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tstctl {
    type Ux = u8;
}
impl crate::IsEnum for Tstctl {}
#[doc = "Field `TSTCTL` reader - BIST self-test control"]
pub type TstctlR = crate::FieldReader<Tstctl>;
impl TstctlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tstctl {
        match self.bits {
            0 => Tstctl::Zz215,
            1 => Tstctl::Zz216,
            2 => Tstctl::Zz217,
            3 => Tstctl::Zz218,
            _ => unreachable!(),
        }
    }
    #[doc = "Default, disable both BIST self-test and MISR"]
    #[inline(always)]
    pub fn is_zz215(&self) -> bool {
        *self == Tstctl::Zz215
    }
    #[doc = "Enable BIST self-test mode DOUT from macro will be forced to '0', and disable MISR."]
    #[inline(always)]
    pub fn is_zz216(&self) -> bool {
        *self == Tstctl::Zz216
    }
    #[doc = "Enable MISR"]
    #[inline(always)]
    pub fn is_zz217(&self) -> bool {
        *self == Tstctl::Zz217
    }
    #[doc = "Enable both BIST self-test mode and MISR"]
    #[inline(always)]
    pub fn is_zz218(&self) -> bool {
        *self == Tstctl::Zz218
    }
}
#[doc = "Field `TSTCTL` writer - BIST self-test control"]
pub type TstctlW<'a, REG> = crate::FieldWriter<'a, REG, 2, Tstctl, crate::Safe>;
impl<'a, REG> TstctlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Default, disable both BIST self-test and MISR"]
    #[inline(always)]
    pub fn zz215(self) -> &'a mut crate::W<REG> {
        self.variant(Tstctl::Zz215)
    }
    #[doc = "Enable BIST self-test mode DOUT from macro will be forced to '0', and disable MISR."]
    #[inline(always)]
    pub fn zz216(self) -> &'a mut crate::W<REG> {
        self.variant(Tstctl::Zz216)
    }
    #[doc = "Enable MISR"]
    #[inline(always)]
    pub fn zz217(self) -> &'a mut crate::W<REG> {
        self.variant(Tstctl::Zz217)
    }
    #[doc = "Enable both BIST self-test mode and MISR"]
    #[inline(always)]
    pub fn zz218(self) -> &'a mut crate::W<REG> {
        self.variant(Tstctl::Zz218)
    }
}
#[doc = "Debug feature control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dbgctl {
    #[doc = "0: Default"]
    Zz213 = 0,
    #[doc = "1: Enable debug feature to collect failure address and data."]
    Zz214 = 1,
}
impl From<Dbgctl> for bool {
    #[inline(always)]
    fn from(variant: Dbgctl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBGCTL` reader - Debug feature control"]
pub type DbgctlR = crate::BitReader<Dbgctl>;
impl DbgctlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dbgctl {
        match self.bits {
            false => Dbgctl::Zz213,
            true => Dbgctl::Zz214,
        }
    }
    #[doc = "Default"]
    #[inline(always)]
    pub fn is_zz213(&self) -> bool {
        *self == Dbgctl::Zz213
    }
    #[doc = "Enable debug feature to collect failure address and data."]
    #[inline(always)]
    pub fn is_zz214(&self) -> bool {
        *self == Dbgctl::Zz214
    }
}
#[doc = "Field `DBGCTL` writer - Debug feature control"]
pub type DbgctlW<'a, REG> = crate::BitWriter<'a, REG, Dbgctl>;
impl<'a, REG> DbgctlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Default"]
    #[inline(always)]
    pub fn zz213(self) -> &'a mut crate::W<REG> {
        self.variant(Dbgctl::Zz213)
    }
    #[doc = "Enable debug feature to collect failure address and data."]
    #[inline(always)]
    pub fn zz214(self) -> &'a mut crate::W<REG> {
        self.variant(Dbgctl::Zz214)
    }
}
#[doc = "Field `BIST_CLK_SEL` reader - BIST Clock Select"]
pub type BistClkSelR = crate::BitReader;
#[doc = "Field `BIST_CLK_SEL` writer - BIST Clock Select"]
pub type BistClkSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "SMWR DOUT Function Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Smwtst {
    #[doc = "0: Default"]
    Zz209 = 0,
    #[doc = "1: Enable SMWR self-test mode, DOUT from macro will be forced to all 0"]
    Zz210 = 1,
    #[doc = "2: Enable SMWR self-test mode, DOUT from macro will be forced to all 1"]
    Zz211 = 2,
}
impl From<Smwtst> for u8 {
    #[inline(always)]
    fn from(variant: Smwtst) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Smwtst {
    type Ux = u8;
}
impl crate::IsEnum for Smwtst {}
#[doc = "Field `SMWTST` reader - SMWR DOUT Function Control"]
pub type SmwtstR = crate::FieldReader<Smwtst>;
impl SmwtstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Smwtst> {
        match self.bits {
            0 => Some(Smwtst::Zz209),
            1 => Some(Smwtst::Zz210),
            2 => Some(Smwtst::Zz211),
            _ => None,
        }
    }
    #[doc = "Default"]
    #[inline(always)]
    pub fn is_zz209(&self) -> bool {
        *self == Smwtst::Zz209
    }
    #[doc = "Enable SMWR self-test mode, DOUT from macro will be forced to all 0"]
    #[inline(always)]
    pub fn is_zz210(&self) -> bool {
        *self == Smwtst::Zz210
    }
    #[doc = "Enable SMWR self-test mode, DOUT from macro will be forced to all 1"]
    #[inline(always)]
    pub fn is_zz211(&self) -> bool {
        *self == Smwtst::Zz211
    }
}
#[doc = "Field `SMWTST` writer - SMWR DOUT Function Control"]
pub type SmwtstW<'a, REG> = crate::FieldWriter<'a, REG, 2, Smwtst>;
impl<'a, REG> SmwtstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Default"]
    #[inline(always)]
    pub fn zz209(self) -> &'a mut crate::W<REG> {
        self.variant(Smwtst::Zz209)
    }
    #[doc = "Enable SMWR self-test mode, DOUT from macro will be forced to all 0"]
    #[inline(always)]
    pub fn zz210(self) -> &'a mut crate::W<REG> {
        self.variant(Smwtst::Zz210)
    }
    #[doc = "Enable SMWR self-test mode, DOUT from macro will be forced to all 1"]
    #[inline(always)]
    pub fn zz211(self) -> &'a mut crate::W<REG> {
        self.variant(Smwtst::Zz211)
    }
}
#[doc = "BIST ECC Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eccen {
    #[doc = "0: Default mode (no ECC encode or decode)"]
    Zz207 = 0,
    #[doc = "1: Enable ECC encode/decode"]
    Zz208 = 1,
}
impl From<Eccen> for bool {
    #[inline(always)]
    fn from(variant: Eccen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECCEN` reader - BIST ECC Control"]
pub type EccenR = crate::BitReader<Eccen>;
impl EccenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eccen {
        match self.bits {
            false => Eccen::Zz207,
            true => Eccen::Zz208,
        }
    }
    #[doc = "Default mode (no ECC encode or decode)"]
    #[inline(always)]
    pub fn is_zz207(&self) -> bool {
        *self == Eccen::Zz207
    }
    #[doc = "Enable ECC encode/decode"]
    #[inline(always)]
    pub fn is_zz208(&self) -> bool {
        *self == Eccen::Zz208
    }
}
#[doc = "Field `ECCEN` writer - BIST ECC Control"]
pub type EccenW<'a, REG> = crate::BitWriter<'a, REG, Eccen>;
impl<'a, REG> EccenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Default mode (no ECC encode or decode)"]
    #[inline(always)]
    pub fn zz207(self) -> &'a mut crate::W<REG> {
        self.variant(Eccen::Zz207)
    }
    #[doc = "Enable ECC encode/decode"]
    #[inline(always)]
    pub fn zz208(self) -> &'a mut crate::W<REG> {
        self.variant(Eccen::Zz208)
    }
}
impl R {
    #[doc = "Bits 0:1 - Block 0 Select Control"]
    #[inline(always)]
    pub fn ipsel0(&self) -> Ipsel0R {
        Ipsel0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Block 1 Select Control"]
    #[inline(always)]
    pub fn ipsel1(&self) -> Ipsel1R {
        Ipsel1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:11 - Clock Divide Scalar for Long Pulse"]
    #[inline(always)]
    pub fn bist_cdivl(&self) -> BistCdivlR {
        BistCdivlR::new(((self.bits >> 4) & 0xff) as u8)
    }
    #[doc = "Bits 12:14 - Number of clock cycles to generate short pulse"]
    #[inline(always)]
    pub fn cdivs(&self) -> CdivsR {
        CdivsR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:19 - Timer adjust for verify"]
    #[inline(always)]
    pub fn bist_tvfy(&self) -> BistTvfyR {
        BistTvfyR::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:21 - BIST self-test control"]
    #[inline(always)]
    pub fn tstctl(&self) -> TstctlR {
        TstctlR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - Debug feature control"]
    #[inline(always)]
    pub fn dbgctl(&self) -> DbgctlR {
        DbgctlR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - BIST Clock Select"]
    #[inline(always)]
    pub fn bist_clk_sel(&self) -> BistClkSelR {
        BistClkSelR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - SMWR DOUT Function Control"]
    #[inline(always)]
    pub fn smwtst(&self) -> SmwtstR {
        SmwtstR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - BIST ECC Control"]
    #[inline(always)]
    pub fn eccen(&self) -> EccenR {
        EccenR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Block 0 Select Control"]
    #[inline(always)]
    pub fn ipsel0(&mut self) -> Ipsel0W<'_, RIpConfigSpec> {
        Ipsel0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Block 1 Select Control"]
    #[inline(always)]
    pub fn ipsel1(&mut self) -> Ipsel1W<'_, RIpConfigSpec> {
        Ipsel1W::new(self, 2)
    }
    #[doc = "Bits 4:11 - Clock Divide Scalar for Long Pulse"]
    #[inline(always)]
    pub fn bist_cdivl(&mut self) -> BistCdivlW<'_, RIpConfigSpec> {
        BistCdivlW::new(self, 4)
    }
    #[doc = "Bits 12:14 - Number of clock cycles to generate short pulse"]
    #[inline(always)]
    pub fn cdivs(&mut self) -> CdivsW<'_, RIpConfigSpec> {
        CdivsW::new(self, 12)
    }
    #[doc = "Bits 15:19 - Timer adjust for verify"]
    #[inline(always)]
    pub fn bist_tvfy(&mut self) -> BistTvfyW<'_, RIpConfigSpec> {
        BistTvfyW::new(self, 15)
    }
    #[doc = "Bits 20:21 - BIST self-test control"]
    #[inline(always)]
    pub fn tstctl(&mut self) -> TstctlW<'_, RIpConfigSpec> {
        TstctlW::new(self, 20)
    }
    #[doc = "Bit 22 - Debug feature control"]
    #[inline(always)]
    pub fn dbgctl(&mut self) -> DbgctlW<'_, RIpConfigSpec> {
        DbgctlW::new(self, 22)
    }
    #[doc = "Bit 23 - BIST Clock Select"]
    #[inline(always)]
    pub fn bist_clk_sel(&mut self) -> BistClkSelW<'_, RIpConfigSpec> {
        BistClkSelW::new(self, 23)
    }
    #[doc = "Bits 24:25 - SMWR DOUT Function Control"]
    #[inline(always)]
    pub fn smwtst(&mut self) -> SmwtstW<'_, RIpConfigSpec> {
        SmwtstW::new(self, 24)
    }
    #[doc = "Bit 26 - BIST ECC Control"]
    #[inline(always)]
    pub fn eccen(&mut self) -> EccenW<'_, RIpConfigSpec> {
        EccenW::new(self, 26)
    }
}
#[doc = "BIST Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r_ip_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r_ip_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RIpConfigSpec;
impl crate::RegisterSpec for RIpConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r_ip_config::R`](R) reader structure"]
impl crate::Readable for RIpConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`r_ip_config::W`](W) writer structure"]
impl crate::Writable for RIpConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R_IP_CONFIG to value 0"]
impl crate::Resettable for RIpConfigSpec {}
