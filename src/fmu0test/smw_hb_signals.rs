#[doc = "Register `SMW_HB_SIGNALS` reader"]
pub type R = crate::R<SmwHbSignalsSpec>;
#[doc = "Register `SMW_HB_SIGNALS` writer"]
pub type W = crate::W<SmwHbSignalsSpec>;
#[doc = "SMW Region Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SmwArray {
    #[doc = "0: Main array"]
    Zz425 = 0,
    #[doc = "1: IFR space only or main (and REDEN space) with IFR space for mass erase"]
    Zz426 = 1,
    #[doc = "2: IFR1 space"]
    Zz427 = 2,
    #[doc = "4: REDEN space"]
    Zz428 = 4,
}
impl From<SmwArray> for u8 {
    #[inline(always)]
    fn from(variant: SmwArray) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SmwArray {
    type Ux = u8;
}
impl crate::IsEnum for SmwArray {}
#[doc = "Field `SMW_ARRAY` reader - SMW Region Select"]
pub type SmwArrayR = crate::FieldReader<SmwArray>;
impl SmwArrayR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SmwArray> {
        match self.bits {
            0 => Some(SmwArray::Zz425),
            1 => Some(SmwArray::Zz426),
            2 => Some(SmwArray::Zz427),
            4 => Some(SmwArray::Zz428),
            _ => None,
        }
    }
    #[doc = "Main array"]
    #[inline(always)]
    pub fn is_zz425(&self) -> bool {
        *self == SmwArray::Zz425
    }
    #[doc = "IFR space only or main (and REDEN space) with IFR space for mass erase"]
    #[inline(always)]
    pub fn is_zz426(&self) -> bool {
        *self == SmwArray::Zz426
    }
    #[doc = "IFR1 space"]
    #[inline(always)]
    pub fn is_zz427(&self) -> bool {
        *self == SmwArray::Zz427
    }
    #[doc = "REDEN space"]
    #[inline(always)]
    pub fn is_zz428(&self) -> bool {
        *self == SmwArray::Zz428
    }
}
#[doc = "Field `SMW_ARRAY` writer - SMW Region Select"]
pub type SmwArrayW<'a, REG> = crate::FieldWriter<'a, REG, 3, SmwArray>;
impl<'a, REG> SmwArrayW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Main array"]
    #[inline(always)]
    pub fn zz425(self) -> &'a mut crate::W<REG> {
        self.variant(SmwArray::Zz425)
    }
    #[doc = "IFR space only or main (and REDEN space) with IFR space for mass erase"]
    #[inline(always)]
    pub fn zz426(self) -> &'a mut crate::W<REG> {
        self.variant(SmwArray::Zz426)
    }
    #[doc = "IFR1 space"]
    #[inline(always)]
    pub fn zz427(self) -> &'a mut crate::W<REG> {
        self.variant(SmwArray::Zz427)
    }
    #[doc = "REDEN space"]
    #[inline(always)]
    pub fn zz428(self) -> &'a mut crate::W<REG> {
        self.variant(SmwArray::Zz428)
    }
}
#[doc = "IFR1 Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UserIfren1 {
    #[doc = "0: IFREN1 input to the flash array is driven LOW"]
    Zz423 = 0,
    #[doc = "1: IFREN1 input to the flash array is driven HIGH"]
    Zz424 = 1,
}
impl From<UserIfren1> for bool {
    #[inline(always)]
    fn from(variant: UserIfren1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USER_IFREN1` reader - IFR1 Enable"]
pub type UserIfren1R = crate::BitReader<UserIfren1>;
impl UserIfren1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UserIfren1 {
        match self.bits {
            false => UserIfren1::Zz423,
            true => UserIfren1::Zz424,
        }
    }
    #[doc = "IFREN1 input to the flash array is driven LOW"]
    #[inline(always)]
    pub fn is_zz423(&self) -> bool {
        *self == UserIfren1::Zz423
    }
    #[doc = "IFREN1 input to the flash array is driven HIGH"]
    #[inline(always)]
    pub fn is_zz424(&self) -> bool {
        *self == UserIfren1::Zz424
    }
}
#[doc = "Field `USER_IFREN1` writer - IFR1 Enable"]
pub type UserIfren1W<'a, REG> = crate::BitWriter<'a, REG, UserIfren1>;
impl<'a, REG> UserIfren1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IFREN1 input to the flash array is driven LOW"]
    #[inline(always)]
    pub fn zz423(self) -> &'a mut crate::W<REG> {
        self.variant(UserIfren1::Zz423)
    }
    #[doc = "IFREN1 input to the flash array is driven HIGH"]
    #[inline(always)]
    pub fn zz424(self) -> &'a mut crate::W<REG> {
        self.variant(UserIfren1::Zz424)
    }
}
#[doc = "Program Verify\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UserPv {
    #[doc = "0: PV input to the flash array is driven LOW"]
    Zz421 = 0,
    #[doc = "1: PV input to the flash array is driven HIGH"]
    Zz422 = 1,
}
impl From<UserPv> for bool {
    #[inline(always)]
    fn from(variant: UserPv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USER_PV` reader - Program Verify"]
pub type UserPvR = crate::BitReader<UserPv>;
impl UserPvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UserPv {
        match self.bits {
            false => UserPv::Zz421,
            true => UserPv::Zz422,
        }
    }
    #[doc = "PV input to the flash array is driven LOW"]
    #[inline(always)]
    pub fn is_zz421(&self) -> bool {
        *self == UserPv::Zz421
    }
    #[doc = "PV input to the flash array is driven HIGH"]
    #[inline(always)]
    pub fn is_zz422(&self) -> bool {
        *self == UserPv::Zz422
    }
}
#[doc = "Field `USER_PV` writer - Program Verify"]
pub type UserPvW<'a, REG> = crate::BitWriter<'a, REG, UserPv>;
impl<'a, REG> UserPvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PV input to the flash array is driven LOW"]
    #[inline(always)]
    pub fn zz421(self) -> &'a mut crate::W<REG> {
        self.variant(UserPv::Zz421)
    }
    #[doc = "PV input to the flash array is driven HIGH"]
    #[inline(always)]
    pub fn zz422(self) -> &'a mut crate::W<REG> {
        self.variant(UserPv::Zz422)
    }
}
#[doc = "Erase Verify\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UserEv {
    #[doc = "0: EV input to the flash array is driven LOW"]
    Zz419 = 0,
    #[doc = "1: EV input to the flash array is driven HIGH"]
    Zz420 = 1,
}
impl From<UserEv> for bool {
    #[inline(always)]
    fn from(variant: UserEv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USER_EV` reader - Erase Verify"]
pub type UserEvR = crate::BitReader<UserEv>;
impl UserEvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UserEv {
        match self.bits {
            false => UserEv::Zz419,
            true => UserEv::Zz420,
        }
    }
    #[doc = "EV input to the flash array is driven LOW"]
    #[inline(always)]
    pub fn is_zz419(&self) -> bool {
        *self == UserEv::Zz419
    }
    #[doc = "EV input to the flash array is driven HIGH"]
    #[inline(always)]
    pub fn is_zz420(&self) -> bool {
        *self == UserEv::Zz420
    }
}
#[doc = "Field `USER_EV` writer - Erase Verify"]
pub type UserEvW<'a, REG> = crate::BitWriter<'a, REG, UserEv>;
impl<'a, REG> UserEvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "EV input to the flash array is driven LOW"]
    #[inline(always)]
    pub fn zz419(self) -> &'a mut crate::W<REG> {
        self.variant(UserEv::Zz419)
    }
    #[doc = "EV input to the flash array is driven HIGH"]
    #[inline(always)]
    pub fn zz420(self) -> &'a mut crate::W<REG> {
        self.variant(UserEv::Zz420)
    }
}
#[doc = "IFR Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UserIfren {
    #[doc = "0: IFREN input to the flash array is driven LOW"]
    Zz417 = 0,
    #[doc = "1: IFREN input to the flash array is driven HIGH"]
    Zz418 = 1,
}
impl From<UserIfren> for bool {
    #[inline(always)]
    fn from(variant: UserIfren) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USER_IFREN` reader - IFR Enable"]
pub type UserIfrenR = crate::BitReader<UserIfren>;
impl UserIfrenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UserIfren {
        match self.bits {
            false => UserIfren::Zz417,
            true => UserIfren::Zz418,
        }
    }
    #[doc = "IFREN input to the flash array is driven LOW"]
    #[inline(always)]
    pub fn is_zz417(&self) -> bool {
        *self == UserIfren::Zz417
    }
    #[doc = "IFREN input to the flash array is driven HIGH"]
    #[inline(always)]
    pub fn is_zz418(&self) -> bool {
        *self == UserIfren::Zz418
    }
}
#[doc = "Field `USER_IFREN` writer - IFR Enable"]
pub type UserIfrenW<'a, REG> = crate::BitWriter<'a, REG, UserIfren>;
impl<'a, REG> UserIfrenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IFREN input to the flash array is driven LOW"]
    #[inline(always)]
    pub fn zz417(self) -> &'a mut crate::W<REG> {
        self.variant(UserIfren::Zz417)
    }
    #[doc = "IFREN input to the flash array is driven HIGH"]
    #[inline(always)]
    pub fn zz418(self) -> &'a mut crate::W<REG> {
        self.variant(UserIfren::Zz418)
    }
}
#[doc = "Repair Read Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UserReden {
    #[doc = "0: REDEN input to the flash array is driven LOW"]
    Zz415 = 0,
    #[doc = "1: REDEN input to the flash array is driven HIGH"]
    Zz416 = 1,
}
impl From<UserReden> for bool {
    #[inline(always)]
    fn from(variant: UserReden) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USER_REDEN` reader - Repair Read Enable"]
pub type UserRedenR = crate::BitReader<UserReden>;
impl UserRedenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UserReden {
        match self.bits {
            false => UserReden::Zz415,
            true => UserReden::Zz416,
        }
    }
    #[doc = "REDEN input to the flash array is driven LOW"]
    #[inline(always)]
    pub fn is_zz415(&self) -> bool {
        *self == UserReden::Zz415
    }
    #[doc = "REDEN input to the flash array is driven HIGH"]
    #[inline(always)]
    pub fn is_zz416(&self) -> bool {
        *self == UserReden::Zz416
    }
}
#[doc = "Field `USER_REDEN` writer - Repair Read Enable"]
pub type UserRedenW<'a, REG> = crate::BitWriter<'a, REG, UserReden>;
impl<'a, REG> UserRedenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "REDEN input to the flash array is driven LOW"]
    #[inline(always)]
    pub fn zz415(self) -> &'a mut crate::W<REG> {
        self.variant(UserReden::Zz415)
    }
    #[doc = "REDEN input to the flash array is driven HIGH"]
    #[inline(always)]
    pub fn zz416(self) -> &'a mut crate::W<REG> {
        self.variant(UserReden::Zz416)
    }
}
#[doc = "High Endurance Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UserHem {
    #[doc = "0: HEM input to SMW / BIST PIN_CTRL\\[24\\] is driven LOW"]
    Zz413 = 0,
    #[doc = "1: HEM input to SMW / BIST PIN_CTRL\\[24\\] is driven HIGH"]
    Zz414 = 1,
}
impl From<UserHem> for bool {
    #[inline(always)]
    fn from(variant: UserHem) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USER_HEM` reader - High Endurance Enable"]
pub type UserHemR = crate::BitReader<UserHem>;
impl UserHemR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UserHem {
        match self.bits {
            false => UserHem::Zz413,
            true => UserHem::Zz414,
        }
    }
    #[doc = "HEM input to SMW / BIST PIN_CTRL\\[24\\] is driven LOW"]
    #[inline(always)]
    pub fn is_zz413(&self) -> bool {
        *self == UserHem::Zz413
    }
    #[doc = "HEM input to SMW / BIST PIN_CTRL\\[24\\] is driven HIGH"]
    #[inline(always)]
    pub fn is_zz414(&self) -> bool {
        *self == UserHem::Zz414
    }
}
#[doc = "Field `USER_HEM` writer - High Endurance Enable"]
pub type UserHemW<'a, REG> = crate::BitWriter<'a, REG, UserHem>;
impl<'a, REG> UserHemW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HEM input to SMW / BIST PIN_CTRL\\[24\\] is driven LOW"]
    #[inline(always)]
    pub fn zz413(self) -> &'a mut crate::W<REG> {
        self.variant(UserHem::Zz413)
    }
    #[doc = "HEM input to SMW / BIST PIN_CTRL\\[24\\] is driven HIGH"]
    #[inline(always)]
    pub fn zz414(self) -> &'a mut crate::W<REG> {
        self.variant(UserHem::Zz414)
    }
}
impl R {
    #[doc = "Bits 0:2 - SMW Region Select"]
    #[inline(always)]
    pub fn smw_array(&self) -> SmwArrayR {
        SmwArrayR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - IFR1 Enable"]
    #[inline(always)]
    pub fn user_ifren1(&self) -> UserIfren1R {
        UserIfren1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Program Verify"]
    #[inline(always)]
    pub fn user_pv(&self) -> UserPvR {
        UserPvR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Erase Verify"]
    #[inline(always)]
    pub fn user_ev(&self) -> UserEvR {
        UserEvR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IFR Enable"]
    #[inline(always)]
    pub fn user_ifren(&self) -> UserIfrenR {
        UserIfrenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Repair Read Enable"]
    #[inline(always)]
    pub fn user_reden(&self) -> UserRedenR {
        UserRedenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - High Endurance Enable"]
    #[inline(always)]
    pub fn user_hem(&self) -> UserHemR {
        UserHemR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - SMW Region Select"]
    #[inline(always)]
    pub fn smw_array(&mut self) -> SmwArrayW<'_, SmwHbSignalsSpec> {
        SmwArrayW::new(self, 0)
    }
    #[doc = "Bit 3 - IFR1 Enable"]
    #[inline(always)]
    pub fn user_ifren1(&mut self) -> UserIfren1W<'_, SmwHbSignalsSpec> {
        UserIfren1W::new(self, 3)
    }
    #[doc = "Bit 4 - Program Verify"]
    #[inline(always)]
    pub fn user_pv(&mut self) -> UserPvW<'_, SmwHbSignalsSpec> {
        UserPvW::new(self, 4)
    }
    #[doc = "Bit 5 - Erase Verify"]
    #[inline(always)]
    pub fn user_ev(&mut self) -> UserEvW<'_, SmwHbSignalsSpec> {
        UserEvW::new(self, 5)
    }
    #[doc = "Bit 6 - IFR Enable"]
    #[inline(always)]
    pub fn user_ifren(&mut self) -> UserIfrenW<'_, SmwHbSignalsSpec> {
        UserIfrenW::new(self, 6)
    }
    #[doc = "Bit 7 - Repair Read Enable"]
    #[inline(always)]
    pub fn user_reden(&mut self) -> UserRedenW<'_, SmwHbSignalsSpec> {
        UserRedenW::new(self, 7)
    }
    #[doc = "Bit 8 - High Endurance Enable"]
    #[inline(always)]
    pub fn user_hem(&mut self) -> UserHemW<'_, SmwHbSignalsSpec> {
        UserHemW::new(self, 8)
    }
}
#[doc = "SMW HB Signals Register\n\nYou can [`read`](crate::Reg::read) this register and get [`smw_hb_signals::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smw_hb_signals::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmwHbSignalsSpec;
impl crate::RegisterSpec for SmwHbSignalsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smw_hb_signals::R`](R) reader structure"]
impl crate::Readable for SmwHbSignalsSpec {}
#[doc = "`write(|w| ..)` method takes [`smw_hb_signals::W`](W) writer structure"]
impl crate::Writable for SmwHbSignalsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SMW_HB_SIGNALS to value 0x08"]
impl crate::Resettable for SmwHbSignalsSpec {
    const RESET_VALUE: u32 = 0x08;
}
