#[doc = "Register `FCNFG` reader"]
pub type R = crate::R<FcnfgSpec>;
#[doc = "Register `FCNFG` writer"]
pub type W = crate::W<FcnfgSpec>;
#[doc = "Command Complete Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccie {
    #[doc = "0: Command complete interrupt disabled"]
    Zz37 = 0,
    #[doc = "1: Command complete interrupt enabled. An interrupt request is generated whenever the FSTAT\\[CCIF\\] flag is set."]
    Zz38 = 1,
}
impl From<Ccie> for bool {
    #[inline(always)]
    fn from(variant: Ccie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCIE` reader - Command Complete Interrupt Enable"]
pub type CcieR = crate::BitReader<Ccie>;
impl CcieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccie {
        match self.bits {
            false => Ccie::Zz37,
            true => Ccie::Zz38,
        }
    }
    #[doc = "Command complete interrupt disabled"]
    #[inline(always)]
    pub fn is_zz37(&self) -> bool {
        *self == Ccie::Zz37
    }
    #[doc = "Command complete interrupt enabled. An interrupt request is generated whenever the FSTAT\\[CCIF\\] flag is set."]
    #[inline(always)]
    pub fn is_zz38(&self) -> bool {
        *self == Ccie::Zz38
    }
}
#[doc = "Mass Erase (Erase All) Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ersreq {
    #[doc = "0: No request or request complete"]
    Zz35 = 0,
    #[doc = "1: Request to run the Mass Erase operation"]
    Zz36 = 1,
}
impl From<Ersreq> for bool {
    #[inline(always)]
    fn from(variant: Ersreq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERSREQ` reader - Mass Erase (Erase All) Request"]
pub type ErsreqR = crate::BitReader<Ersreq>;
impl ErsreqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ersreq {
        match self.bits {
            false => Ersreq::Zz35,
            true => Ersreq::Zz36,
        }
    }
    #[doc = "No request or request complete"]
    #[inline(always)]
    pub fn is_zz35(&self) -> bool {
        *self == Ersreq::Zz35
    }
    #[doc = "Request to run the Mass Erase operation"]
    #[inline(always)]
    pub fn is_zz36(&self) -> bool {
        *self == Ersreq::Zz36
    }
}
#[doc = "Double Bit Fault Detect Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dfdie {
    #[doc = "0: Double bit fault detect interrupt disabled"]
    Zz33 = 0,
    #[doc = "1: Double bit fault detect interrupt enabled; an interrupt request is generated whenever the FSTAT\\[DFDIF\\] flag is set"]
    Zz34 = 1,
}
impl From<Dfdie> for bool {
    #[inline(always)]
    fn from(variant: Dfdie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DFDIE` reader - Double Bit Fault Detect Interrupt Enable"]
pub type DfdieR = crate::BitReader<Dfdie>;
impl DfdieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dfdie {
        match self.bits {
            false => Dfdie::Zz33,
            true => Dfdie::Zz34,
        }
    }
    #[doc = "Double bit fault detect interrupt disabled"]
    #[inline(always)]
    pub fn is_zz33(&self) -> bool {
        *self == Dfdie::Zz33
    }
    #[doc = "Double bit fault detect interrupt enabled; an interrupt request is generated whenever the FSTAT\\[DFDIF\\] flag is set"]
    #[inline(always)]
    pub fn is_zz34(&self) -> bool {
        *self == Dfdie::Zz34
    }
}
#[doc = "Erase IFR Sector Enable - Block 0\n\nValue on reset: 14"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ersien0 {
    #[doc = "0: Block 0 IFR Sector X is protected from erase by ERSSCR command"]
    Zz31 = 0,
    #[doc = "1: Block 0 IFR Sector X is not protected from erase by ERSSCR command"]
    Zz32 = 1,
}
impl From<Ersien0> for u8 {
    #[inline(always)]
    fn from(variant: Ersien0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ersien0 {
    type Ux = u8;
}
impl crate::IsEnum for Ersien0 {}
#[doc = "Field `ERSIEN0` reader - Erase IFR Sector Enable - Block 0"]
pub type Ersien0R = crate::FieldReader<Ersien0>;
impl Ersien0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ersien0> {
        match self.bits {
            0 => Some(Ersien0::Zz31),
            1 => Some(Ersien0::Zz32),
            _ => None,
        }
    }
    #[doc = "Block 0 IFR Sector X is protected from erase by ERSSCR command"]
    #[inline(always)]
    pub fn is_zz31(&self) -> bool {
        *self == Ersien0::Zz31
    }
    #[doc = "Block 0 IFR Sector X is not protected from erase by ERSSCR command"]
    #[inline(always)]
    pub fn is_zz32(&self) -> bool {
        *self == Ersien0::Zz32
    }
}
#[doc = "Field `ERSIEN0` writer - Erase IFR Sector Enable - Block 0"]
pub type Ersien0W<'a, REG> = crate::FieldWriter<'a, REG, 4, Ersien0>;
impl<'a, REG> Ersien0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Block 0 IFR Sector X is protected from erase by ERSSCR command"]
    #[inline(always)]
    pub fn zz31(self) -> &'a mut crate::W<REG> {
        self.variant(Ersien0::Zz31)
    }
    #[doc = "Block 0 IFR Sector X is not protected from erase by ERSSCR command"]
    #[inline(always)]
    pub fn zz32(self) -> &'a mut crate::W<REG> {
        self.variant(Ersien0::Zz32)
    }
}
#[doc = "Erase IFR Sector Enable - Block 1 (for dual block configs)\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ersien1 {
    #[doc = "0: Block 1 IFR Sector X is protected from erase by ERSSCR command"]
    Zz29 = 0,
    #[doc = "1: Block 1 IFR Sector X is not protected from erase by ERSSCR command"]
    Zz30 = 1,
}
impl From<Ersien1> for u8 {
    #[inline(always)]
    fn from(variant: Ersien1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ersien1 {
    type Ux = u8;
}
impl crate::IsEnum for Ersien1 {}
#[doc = "Field `ERSIEN1` reader - Erase IFR Sector Enable - Block 1 (for dual block configs)"]
pub type Ersien1R = crate::FieldReader<Ersien1>;
impl Ersien1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ersien1> {
        match self.bits {
            0 => Some(Ersien1::Zz29),
            1 => Some(Ersien1::Zz30),
            _ => None,
        }
    }
    #[doc = "Block 1 IFR Sector X is protected from erase by ERSSCR command"]
    #[inline(always)]
    pub fn is_zz29(&self) -> bool {
        *self == Ersien1::Zz29
    }
    #[doc = "Block 1 IFR Sector X is not protected from erase by ERSSCR command"]
    #[inline(always)]
    pub fn is_zz30(&self) -> bool {
        *self == Ersien1::Zz30
    }
}
#[doc = "Field `ERSIEN1` writer - Erase IFR Sector Enable - Block 1 (for dual block configs)"]
pub type Ersien1W<'a, REG> = crate::FieldWriter<'a, REG, 4, Ersien1>;
impl<'a, REG> Ersien1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Block 1 IFR Sector X is protected from erase by ERSSCR command"]
    #[inline(always)]
    pub fn zz29(self) -> &'a mut crate::W<REG> {
        self.variant(Ersien1::Zz29)
    }
    #[doc = "Block 1 IFR Sector X is not protected from erase by ERSSCR command"]
    #[inline(always)]
    pub fn zz30(self) -> &'a mut crate::W<REG> {
        self.variant(Ersien1::Zz30)
    }
}
impl R {
    #[doc = "Bit 7 - Command Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ccie(&self) -> CcieR {
        CcieR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Mass Erase (Erase All) Request"]
    #[inline(always)]
    pub fn ersreq(&self) -> ErsreqR {
        ErsreqR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Double Bit Fault Detect Interrupt Enable"]
    #[inline(always)]
    pub fn dfdie(&self) -> DfdieR {
        DfdieR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Erase IFR Sector Enable - Block 0"]
    #[inline(always)]
    pub fn ersien0(&self) -> Ersien0R {
        Ersien0R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Erase IFR Sector Enable - Block 1 (for dual block configs)"]
    #[inline(always)]
    pub fn ersien1(&self) -> Ersien1R {
        Ersien1R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:27 - Erase IFR Sector Enable - Block 0"]
    #[inline(always)]
    pub fn ersien0(&mut self) -> Ersien0W<'_, FcnfgSpec> {
        Ersien0W::new(self, 24)
    }
    #[doc = "Bits 28:31 - Erase IFR Sector Enable - Block 1 (for dual block configs)"]
    #[inline(always)]
    pub fn ersien1(&mut self) -> Ersien1W<'_, FcnfgSpec> {
        Ersien1W::new(self, 28)
    }
}
#[doc = "Flash Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fcnfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcnfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcnfgSpec;
impl crate::RegisterSpec for FcnfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fcnfg::R`](R) reader structure"]
impl crate::Readable for FcnfgSpec {}
#[doc = "`write(|w| ..)` method takes [`fcnfg::W`](W) writer structure"]
impl crate::Writable for FcnfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FCNFG to value 0xfe00_0000"]
impl crate::Resettable for FcnfgSpec {
    const RESET_VALUE: u32 = 0xfe00_0000;
}
