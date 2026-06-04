#[doc = "Register `FCTRL` reader"]
pub type R = crate::R<FctrlSpec>;
#[doc = "Register `FCTRL` writer"]
pub type W = crate::W<FctrlSpec>;
#[doc = "Read Wait-State Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rwsc {
    #[doc = "0: no additional wait-states are added (single cycle access)"]
    Zz45 = 0,
    #[doc = "1: 1 additional wait-state is added"]
    Zz46 = 1,
    #[doc = "2: 2 additional wait-states are added"]
    Zz47 = 2,
    #[doc = "3: 3 additional wait-states are added"]
    Zz48 = 3,
    #[doc = "4: 4 additional wait-states are added"]
    Zz49 = 4,
    #[doc = "5: 5 additional wait-states are added"]
    Zz50 = 5,
    #[doc = "6: 6 additional wait-states are added"]
    Zz51 = 6,
    #[doc = "7: 7 additional wait-states are added"]
    Zz52 = 7,
    #[doc = "8: 8 additional wait-states are added"]
    Zz53 = 8,
    #[doc = "9: 9 additional wait-states are added"]
    Zz54 = 9,
    #[doc = "10: 10 additional wait-states are added"]
    Zz55 = 10,
    #[doc = "11: 11 additional wait-states are added"]
    Zz56 = 11,
    #[doc = "12: 12 additional wait-states are added"]
    Zz57 = 12,
    #[doc = "13: 13 additional wait-states are added"]
    Zz58 = 13,
    #[doc = "14: 14 additional wait-states are added"]
    Zz59 = 14,
    #[doc = "15: 15 additional wait-states are added"]
    Zz60 = 15,
}
impl From<Rwsc> for u8 {
    #[inline(always)]
    fn from(variant: Rwsc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rwsc {
    type Ux = u8;
}
impl crate::IsEnum for Rwsc {}
#[doc = "Field `RWSC` reader - Read Wait-State Control"]
pub type RwscR = crate::FieldReader<Rwsc>;
impl RwscR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rwsc {
        match self.bits {
            0 => Rwsc::Zz45,
            1 => Rwsc::Zz46,
            2 => Rwsc::Zz47,
            3 => Rwsc::Zz48,
            4 => Rwsc::Zz49,
            5 => Rwsc::Zz50,
            6 => Rwsc::Zz51,
            7 => Rwsc::Zz52,
            8 => Rwsc::Zz53,
            9 => Rwsc::Zz54,
            10 => Rwsc::Zz55,
            11 => Rwsc::Zz56,
            12 => Rwsc::Zz57,
            13 => Rwsc::Zz58,
            14 => Rwsc::Zz59,
            15 => Rwsc::Zz60,
            _ => unreachable!(),
        }
    }
    #[doc = "no additional wait-states are added (single cycle access)"]
    #[inline(always)]
    pub fn is_zz45(&self) -> bool {
        *self == Rwsc::Zz45
    }
    #[doc = "1 additional wait-state is added"]
    #[inline(always)]
    pub fn is_zz46(&self) -> bool {
        *self == Rwsc::Zz46
    }
    #[doc = "2 additional wait-states are added"]
    #[inline(always)]
    pub fn is_zz47(&self) -> bool {
        *self == Rwsc::Zz47
    }
    #[doc = "3 additional wait-states are added"]
    #[inline(always)]
    pub fn is_zz48(&self) -> bool {
        *self == Rwsc::Zz48
    }
    #[doc = "4 additional wait-states are added"]
    #[inline(always)]
    pub fn is_zz49(&self) -> bool {
        *self == Rwsc::Zz49
    }
    #[doc = "5 additional wait-states are added"]
    #[inline(always)]
    pub fn is_zz50(&self) -> bool {
        *self == Rwsc::Zz50
    }
    #[doc = "6 additional wait-states are added"]
    #[inline(always)]
    pub fn is_zz51(&self) -> bool {
        *self == Rwsc::Zz51
    }
    #[doc = "7 additional wait-states are added"]
    #[inline(always)]
    pub fn is_zz52(&self) -> bool {
        *self == Rwsc::Zz52
    }
    #[doc = "8 additional wait-states are added"]
    #[inline(always)]
    pub fn is_zz53(&self) -> bool {
        *self == Rwsc::Zz53
    }
    #[doc = "9 additional wait-states are added"]
    #[inline(always)]
    pub fn is_zz54(&self) -> bool {
        *self == Rwsc::Zz54
    }
    #[doc = "10 additional wait-states are added"]
    #[inline(always)]
    pub fn is_zz55(&self) -> bool {
        *self == Rwsc::Zz55
    }
    #[doc = "11 additional wait-states are added"]
    #[inline(always)]
    pub fn is_zz56(&self) -> bool {
        *self == Rwsc::Zz56
    }
    #[doc = "12 additional wait-states are added"]
    #[inline(always)]
    pub fn is_zz57(&self) -> bool {
        *self == Rwsc::Zz57
    }
    #[doc = "13 additional wait-states are added"]
    #[inline(always)]
    pub fn is_zz58(&self) -> bool {
        *self == Rwsc::Zz58
    }
    #[doc = "14 additional wait-states are added"]
    #[inline(always)]
    pub fn is_zz59(&self) -> bool {
        *self == Rwsc::Zz59
    }
    #[doc = "15 additional wait-states are added"]
    #[inline(always)]
    pub fn is_zz60(&self) -> bool {
        *self == Rwsc::Zz60
    }
}
#[doc = "Field `RWSC` writer - Read Wait-State Control"]
pub type RwscW<'a, REG> = crate::FieldWriter<'a, REG, 4, Rwsc, crate::Safe>;
impl<'a, REG> RwscW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "no additional wait-states are added (single cycle access)"]
    #[inline(always)]
    pub fn zz45(self) -> &'a mut crate::W<REG> {
        self.variant(Rwsc::Zz45)
    }
    #[doc = "1 additional wait-state is added"]
    #[inline(always)]
    pub fn zz46(self) -> &'a mut crate::W<REG> {
        self.variant(Rwsc::Zz46)
    }
    #[doc = "2 additional wait-states are added"]
    #[inline(always)]
    pub fn zz47(self) -> &'a mut crate::W<REG> {
        self.variant(Rwsc::Zz47)
    }
    #[doc = "3 additional wait-states are added"]
    #[inline(always)]
    pub fn zz48(self) -> &'a mut crate::W<REG> {
        self.variant(Rwsc::Zz48)
    }
    #[doc = "4 additional wait-states are added"]
    #[inline(always)]
    pub fn zz49(self) -> &'a mut crate::W<REG> {
        self.variant(Rwsc::Zz49)
    }
    #[doc = "5 additional wait-states are added"]
    #[inline(always)]
    pub fn zz50(self) -> &'a mut crate::W<REG> {
        self.variant(Rwsc::Zz50)
    }
    #[doc = "6 additional wait-states are added"]
    #[inline(always)]
    pub fn zz51(self) -> &'a mut crate::W<REG> {
        self.variant(Rwsc::Zz51)
    }
    #[doc = "7 additional wait-states are added"]
    #[inline(always)]
    pub fn zz52(self) -> &'a mut crate::W<REG> {
        self.variant(Rwsc::Zz52)
    }
    #[doc = "8 additional wait-states are added"]
    #[inline(always)]
    pub fn zz53(self) -> &'a mut crate::W<REG> {
        self.variant(Rwsc::Zz53)
    }
    #[doc = "9 additional wait-states are added"]
    #[inline(always)]
    pub fn zz54(self) -> &'a mut crate::W<REG> {
        self.variant(Rwsc::Zz54)
    }
    #[doc = "10 additional wait-states are added"]
    #[inline(always)]
    pub fn zz55(self) -> &'a mut crate::W<REG> {
        self.variant(Rwsc::Zz55)
    }
    #[doc = "11 additional wait-states are added"]
    #[inline(always)]
    pub fn zz56(self) -> &'a mut crate::W<REG> {
        self.variant(Rwsc::Zz56)
    }
    #[doc = "12 additional wait-states are added"]
    #[inline(always)]
    pub fn zz57(self) -> &'a mut crate::W<REG> {
        self.variant(Rwsc::Zz57)
    }
    #[doc = "13 additional wait-states are added"]
    #[inline(always)]
    pub fn zz58(self) -> &'a mut crate::W<REG> {
        self.variant(Rwsc::Zz58)
    }
    #[doc = "14 additional wait-states are added"]
    #[inline(always)]
    pub fn zz59(self) -> &'a mut crate::W<REG> {
        self.variant(Rwsc::Zz59)
    }
    #[doc = "15 additional wait-states are added"]
    #[inline(always)]
    pub fn zz60(self) -> &'a mut crate::W<REG> {
        self.variant(Rwsc::Zz60)
    }
}
#[doc = "Low Speed Active Mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lsactive {
    #[doc = "0: Full speed active mode requested"]
    Zz43 = 0,
    #[doc = "1: Low speed active mode requested"]
    Zz44 = 1,
}
impl From<Lsactive> for bool {
    #[inline(always)]
    fn from(variant: Lsactive) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSACTIVE` reader - Low Speed Active Mode"]
pub type LsactiveR = crate::BitReader<Lsactive>;
impl LsactiveR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lsactive {
        match self.bits {
            false => Lsactive::Zz43,
            true => Lsactive::Zz44,
        }
    }
    #[doc = "Full speed active mode requested"]
    #[inline(always)]
    pub fn is_zz43(&self) -> bool {
        *self == Lsactive::Zz43
    }
    #[doc = "Low speed active mode requested"]
    #[inline(always)]
    pub fn is_zz44(&self) -> bool {
        *self == Lsactive::Zz44
    }
}
#[doc = "Field `LSACTIVE` writer - Low Speed Active Mode"]
pub type LsactiveW<'a, REG> = crate::BitWriter<'a, REG, Lsactive>;
impl<'a, REG> LsactiveW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Full speed active mode requested"]
    #[inline(always)]
    pub fn zz43(self) -> &'a mut crate::W<REG> {
        self.variant(Lsactive::Zz43)
    }
    #[doc = "Low speed active mode requested"]
    #[inline(always)]
    pub fn zz44(self) -> &'a mut crate::W<REG> {
        self.variant(Lsactive::Zz44)
    }
}
#[doc = "Force Double Bit Fault Detect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fdfd {
    #[doc = "0: FSTAT\\[DFDIF\\] sets only if a double bit fault is detected during a valid flash read access from the FMC"]
    Zz41 = 0,
    #[doc = "1: FSTAT\\[DFDIF\\] sets during any valid flash read access from the FMC; an interrupt request is generated if the DFDIE bit is set"]
    Zz42 = 1,
}
impl From<Fdfd> for bool {
    #[inline(always)]
    fn from(variant: Fdfd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FDFD` reader - Force Double Bit Fault Detect"]
pub type FdfdR = crate::BitReader<Fdfd>;
impl FdfdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fdfd {
        match self.bits {
            false => Fdfd::Zz41,
            true => Fdfd::Zz42,
        }
    }
    #[doc = "FSTAT\\[DFDIF\\] sets only if a double bit fault is detected during a valid flash read access from the FMC"]
    #[inline(always)]
    pub fn is_zz41(&self) -> bool {
        *self == Fdfd::Zz41
    }
    #[doc = "FSTAT\\[DFDIF\\] sets during any valid flash read access from the FMC; an interrupt request is generated if the DFDIE bit is set"]
    #[inline(always)]
    pub fn is_zz42(&self) -> bool {
        *self == Fdfd::Zz42
    }
}
#[doc = "Abort Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Abtreq {
    #[doc = "0: No request to abort a command write sequence"]
    Zz39 = 0,
    #[doc = "1: Request to abort a command write sequence"]
    Zz40 = 1,
}
impl From<Abtreq> for bool {
    #[inline(always)]
    fn from(variant: Abtreq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABTREQ` reader - Abort Request"]
pub type AbtreqR = crate::BitReader<Abtreq>;
impl AbtreqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Abtreq {
        match self.bits {
            false => Abtreq::Zz39,
            true => Abtreq::Zz40,
        }
    }
    #[doc = "No request to abort a command write sequence"]
    #[inline(always)]
    pub fn is_zz39(&self) -> bool {
        *self == Abtreq::Zz39
    }
    #[doc = "Request to abort a command write sequence"]
    #[inline(always)]
    pub fn is_zz40(&self) -> bool {
        *self == Abtreq::Zz40
    }
}
impl R {
    #[doc = "Bits 0:3 - Read Wait-State Control"]
    #[inline(always)]
    pub fn rwsc(&self) -> RwscR {
        RwscR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Low Speed Active Mode"]
    #[inline(always)]
    pub fn lsactive(&self) -> LsactiveR {
        LsactiveR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Force Double Bit Fault Detect"]
    #[inline(always)]
    pub fn fdfd(&self) -> FdfdR {
        FdfdR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Abort Request"]
    #[inline(always)]
    pub fn abtreq(&self) -> AbtreqR {
        AbtreqR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Read Wait-State Control"]
    #[inline(always)]
    pub fn rwsc(&mut self) -> RwscW<'_, FctrlSpec> {
        RwscW::new(self, 0)
    }
    #[doc = "Bit 8 - Low Speed Active Mode"]
    #[inline(always)]
    pub fn lsactive(&mut self) -> LsactiveW<'_, FctrlSpec> {
        LsactiveW::new(self, 8)
    }
}
#[doc = "Flash Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FctrlSpec;
impl crate::RegisterSpec for FctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fctrl::R`](R) reader structure"]
impl crate::Readable for FctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`fctrl::W`](W) writer structure"]
impl crate::Writable for FctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FCTRL to value 0x0100"]
impl crate::Resettable for FctrlSpec {
    const RESET_VALUE: u32 = 0x0100;
}
