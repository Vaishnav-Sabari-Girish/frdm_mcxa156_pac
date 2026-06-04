#[doc = "Register `UINT_CTL` reader"]
pub type R = crate::R<UintCtlSpec>;
#[doc = "Register `UINT_CTL` writer"]
pub type W = crate::W<UintCtlSpec>;
#[doc = "Set Fail On Exit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SetFail {
    #[doc = "0: FAIL flag should not be set on command exit (no failure detected)"]
    Zz165 = 0,
    #[doc = "1: FAIL flag should be set on command exit"]
    Zz166 = 1,
}
impl From<SetFail> for bool {
    #[inline(always)]
    fn from(variant: SetFail) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SET_FAIL` reader - Set Fail On Exit"]
pub type SetFailR = crate::BitReader<SetFail>;
impl SetFailR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SetFail {
        match self.bits {
            false => SetFail::Zz165,
            true => SetFail::Zz166,
        }
    }
    #[doc = "FAIL flag should not be set on command exit (no failure detected)"]
    #[inline(always)]
    pub fn is_zz165(&self) -> bool {
        *self == SetFail::Zz165
    }
    #[doc = "FAIL flag should be set on command exit"]
    #[inline(always)]
    pub fn is_zz166(&self) -> bool {
        *self == SetFail::Zz166
    }
}
#[doc = "Field `SET_FAIL` writer - Set Fail On Exit"]
pub type SetFailW<'a, REG> = crate::BitWriter<'a, REG, SetFail>;
impl<'a, REG> SetFailW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FAIL flag should not be set on command exit (no failure detected)"]
    #[inline(always)]
    pub fn zz165(self) -> &'a mut crate::W<REG> {
        self.variant(SetFail::Zz165)
    }
    #[doc = "FAIL flag should be set on command exit"]
    #[inline(always)]
    pub fn zz166(self) -> &'a mut crate::W<REG> {
        self.variant(SetFail::Zz166)
    }
}
#[doc = "Double-Bit ECC Fault Detect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dberr {
    #[doc = "0: No double-bit fault detected during UINT-driven read sequence"]
    Zz163 = 0,
    #[doc = "1: Double-bit fault detected during UINT-driven read sequence"]
    Zz164 = 1,
}
impl From<Dberr> for bool {
    #[inline(always)]
    fn from(variant: Dberr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBERR` reader - Double-Bit ECC Fault Detect"]
pub type DberrR = crate::BitReader<Dberr>;
impl DberrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dberr {
        match self.bits {
            false => Dberr::Zz163,
            true => Dberr::Zz164,
        }
    }
    #[doc = "No double-bit fault detected during UINT-driven read sequence"]
    #[inline(always)]
    pub fn is_zz163(&self) -> bool {
        *self == Dberr::Zz163
    }
    #[doc = "Double-bit fault detected during UINT-driven read sequence"]
    #[inline(always)]
    pub fn is_zz164(&self) -> bool {
        *self == Dberr::Zz164
    }
}
#[doc = "Field `DBERR` writer - Double-Bit ECC Fault Detect"]
pub type DberrW<'a, REG> = crate::BitWriter<'a, REG, Dberr>;
impl<'a, REG> DberrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No double-bit fault detected during UINT-driven read sequence"]
    #[inline(always)]
    pub fn zz163(self) -> &'a mut crate::W<REG> {
        self.variant(Dberr::Zz163)
    }
    #[doc = "Double-bit fault detected during UINT-driven read sequence"]
    #[inline(always)]
    pub fn zz164(self) -> &'a mut crate::W<REG> {
        self.variant(Dberr::Zz164)
    }
}
impl R {
    #[doc = "Bit 0 - Set Fail On Exit"]
    #[inline(always)]
    pub fn set_fail(&self) -> SetFailR {
        SetFailR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Double-Bit ECC Fault Detect"]
    #[inline(always)]
    pub fn dberr(&self) -> DberrR {
        DberrR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set Fail On Exit"]
    #[inline(always)]
    pub fn set_fail(&mut self) -> SetFailW<'_, UintCtlSpec> {
        SetFailW::new(self, 0)
    }
    #[doc = "Bit 1 - Double-Bit ECC Fault Detect"]
    #[inline(always)]
    pub fn dberr(&mut self) -> DberrW<'_, UintCtlSpec> {
        DberrW::new(self, 1)
    }
}
#[doc = "User Interface Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uint_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uint_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UintCtlSpec;
impl crate::RegisterSpec for UintCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uint_ctl::R`](R) reader structure"]
impl crate::Readable for UintCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`uint_ctl::W`](W) writer structure"]
impl crate::Writable for UintCtlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UINT_CTL to value 0"]
impl crate::Resettable for UintCtlSpec {}
