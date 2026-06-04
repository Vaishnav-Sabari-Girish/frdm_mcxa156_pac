#[doc = "Register `SMW_SETTING_OPTION1` reader"]
pub type R = crate::R<SmwSettingOption1Spec>;
#[doc = "Register `SMW_SETTING_OPTION1` writer"]
pub type W = crate::W<SmwSettingOption1Spec>;
#[doc = "Ters Control\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TersCtrl0 {
    #[doc = "0: 50 usec"]
    Zz397 = 0,
    #[doc = "1: 100 usec"]
    Zz398 = 1,
    #[doc = "2: 200 usec"]
    Zz399 = 2,
    #[doc = "3: 300 usec"]
    Zz400 = 3,
    #[doc = "4: 500 usec"]
    Zz401 = 4,
    #[doc = "5: 1 msec"]
    Zz402 = 5,
    #[doc = "6: 1.5 msec"]
    Zz403 = 6,
    #[doc = "7: 2 msec"]
    Zz404 = 7,
}
impl From<TersCtrl0> for u8 {
    #[inline(always)]
    fn from(variant: TersCtrl0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TersCtrl0 {
    type Ux = u8;
}
impl crate::IsEnum for TersCtrl0 {}
#[doc = "Field `TERS_CTRL0` reader - Ters Control"]
pub type TersCtrl0R = crate::FieldReader<TersCtrl0>;
impl TersCtrl0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TersCtrl0 {
        match self.bits {
            0 => TersCtrl0::Zz397,
            1 => TersCtrl0::Zz398,
            2 => TersCtrl0::Zz399,
            3 => TersCtrl0::Zz400,
            4 => TersCtrl0::Zz401,
            5 => TersCtrl0::Zz402,
            6 => TersCtrl0::Zz403,
            7 => TersCtrl0::Zz404,
            _ => unreachable!(),
        }
    }
    #[doc = "50 usec"]
    #[inline(always)]
    pub fn is_zz397(&self) -> bool {
        *self == TersCtrl0::Zz397
    }
    #[doc = "100 usec"]
    #[inline(always)]
    pub fn is_zz398(&self) -> bool {
        *self == TersCtrl0::Zz398
    }
    #[doc = "200 usec"]
    #[inline(always)]
    pub fn is_zz399(&self) -> bool {
        *self == TersCtrl0::Zz399
    }
    #[doc = "300 usec"]
    #[inline(always)]
    pub fn is_zz400(&self) -> bool {
        *self == TersCtrl0::Zz400
    }
    #[doc = "500 usec"]
    #[inline(always)]
    pub fn is_zz401(&self) -> bool {
        *self == TersCtrl0::Zz401
    }
    #[doc = "1 msec"]
    #[inline(always)]
    pub fn is_zz402(&self) -> bool {
        *self == TersCtrl0::Zz402
    }
    #[doc = "1.5 msec"]
    #[inline(always)]
    pub fn is_zz403(&self) -> bool {
        *self == TersCtrl0::Zz403
    }
    #[doc = "2 msec"]
    #[inline(always)]
    pub fn is_zz404(&self) -> bool {
        *self == TersCtrl0::Zz404
    }
}
#[doc = "Field `TERS_CTRL0` writer - Ters Control"]
pub type TersCtrl0W<'a, REG> = crate::FieldWriter<'a, REG, 3, TersCtrl0, crate::Safe>;
impl<'a, REG> TersCtrl0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "50 usec"]
    #[inline(always)]
    pub fn zz397(self) -> &'a mut crate::W<REG> {
        self.variant(TersCtrl0::Zz397)
    }
    #[doc = "100 usec"]
    #[inline(always)]
    pub fn zz398(self) -> &'a mut crate::W<REG> {
        self.variant(TersCtrl0::Zz398)
    }
    #[doc = "200 usec"]
    #[inline(always)]
    pub fn zz399(self) -> &'a mut crate::W<REG> {
        self.variant(TersCtrl0::Zz399)
    }
    #[doc = "300 usec"]
    #[inline(always)]
    pub fn zz400(self) -> &'a mut crate::W<REG> {
        self.variant(TersCtrl0::Zz400)
    }
    #[doc = "500 usec"]
    #[inline(always)]
    pub fn zz401(self) -> &'a mut crate::W<REG> {
        self.variant(TersCtrl0::Zz401)
    }
    #[doc = "1 msec"]
    #[inline(always)]
    pub fn zz402(self) -> &'a mut crate::W<REG> {
        self.variant(TersCtrl0::Zz402)
    }
    #[doc = "1.5 msec"]
    #[inline(always)]
    pub fn zz403(self) -> &'a mut crate::W<REG> {
        self.variant(TersCtrl0::Zz403)
    }
    #[doc = "2 msec"]
    #[inline(always)]
    pub fn zz404(self) -> &'a mut crate::W<REG> {
        self.variant(TersCtrl0::Zz404)
    }
}
#[doc = "Tpgm Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TpgmCtrl {
    #[doc = "0: 1 usec"]
    Zz393 = 0,
    #[doc = "1: 2 usec"]
    Zz394 = 1,
    #[doc = "2: 4 usec"]
    Zz395 = 2,
    #[doc = "3: 8 usec"]
    Zz396 = 3,
}
impl From<TpgmCtrl> for u8 {
    #[inline(always)]
    fn from(variant: TpgmCtrl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TpgmCtrl {
    type Ux = u8;
}
impl crate::IsEnum for TpgmCtrl {}
#[doc = "Field `TPGM_CTRL` reader - Tpgm Control"]
pub type TpgmCtrlR = crate::FieldReader<TpgmCtrl>;
impl TpgmCtrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TpgmCtrl {
        match self.bits {
            0 => TpgmCtrl::Zz393,
            1 => TpgmCtrl::Zz394,
            2 => TpgmCtrl::Zz395,
            3 => TpgmCtrl::Zz396,
            _ => unreachable!(),
        }
    }
    #[doc = "1 usec"]
    #[inline(always)]
    pub fn is_zz393(&self) -> bool {
        *self == TpgmCtrl::Zz393
    }
    #[doc = "2 usec"]
    #[inline(always)]
    pub fn is_zz394(&self) -> bool {
        *self == TpgmCtrl::Zz394
    }
    #[doc = "4 usec"]
    #[inline(always)]
    pub fn is_zz395(&self) -> bool {
        *self == TpgmCtrl::Zz395
    }
    #[doc = "8 usec"]
    #[inline(always)]
    pub fn is_zz396(&self) -> bool {
        *self == TpgmCtrl::Zz396
    }
}
#[doc = "Field `TPGM_CTRL` writer - Tpgm Control"]
pub type TpgmCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 2, TpgmCtrl, crate::Safe>;
impl<'a, REG> TpgmCtrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 usec"]
    #[inline(always)]
    pub fn zz393(self) -> &'a mut crate::W<REG> {
        self.variant(TpgmCtrl::Zz393)
    }
    #[doc = "2 usec"]
    #[inline(always)]
    pub fn zz394(self) -> &'a mut crate::W<REG> {
        self.variant(TpgmCtrl::Zz394)
    }
    #[doc = "4 usec"]
    #[inline(always)]
    pub fn zz395(self) -> &'a mut crate::W<REG> {
        self.variant(TpgmCtrl::Zz395)
    }
    #[doc = "8 usec"]
    #[inline(always)]
    pub fn zz396(self) -> &'a mut crate::W<REG> {
        self.variant(TpgmCtrl::Zz396)
    }
}
#[doc = "Tnvs Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TnvsCtrl {
    #[doc = "0: 5 usec"]
    Zz385 = 0,
    #[doc = "1: 8 usec"]
    Zz386 = 1,
    #[doc = "2: 11 usec"]
    Zz387 = 2,
    #[doc = "3: 14 usec"]
    Zz388 = 3,
    #[doc = "4: 17 usec"]
    Zz389 = 4,
    #[doc = "5: 20 usec"]
    Zz390 = 5,
    #[doc = "6: 23 usec"]
    Zz391 = 6,
    #[doc = "7: 26 usec"]
    Zz392 = 7,
}
impl From<TnvsCtrl> for u8 {
    #[inline(always)]
    fn from(variant: TnvsCtrl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TnvsCtrl {
    type Ux = u8;
}
impl crate::IsEnum for TnvsCtrl {}
#[doc = "Field `TNVS_CTRL` reader - Tnvs Control"]
pub type TnvsCtrlR = crate::FieldReader<TnvsCtrl>;
impl TnvsCtrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TnvsCtrl {
        match self.bits {
            0 => TnvsCtrl::Zz385,
            1 => TnvsCtrl::Zz386,
            2 => TnvsCtrl::Zz387,
            3 => TnvsCtrl::Zz388,
            4 => TnvsCtrl::Zz389,
            5 => TnvsCtrl::Zz390,
            6 => TnvsCtrl::Zz391,
            7 => TnvsCtrl::Zz392,
            _ => unreachable!(),
        }
    }
    #[doc = "5 usec"]
    #[inline(always)]
    pub fn is_zz385(&self) -> bool {
        *self == TnvsCtrl::Zz385
    }
    #[doc = "8 usec"]
    #[inline(always)]
    pub fn is_zz386(&self) -> bool {
        *self == TnvsCtrl::Zz386
    }
    #[doc = "11 usec"]
    #[inline(always)]
    pub fn is_zz387(&self) -> bool {
        *self == TnvsCtrl::Zz387
    }
    #[doc = "14 usec"]
    #[inline(always)]
    pub fn is_zz388(&self) -> bool {
        *self == TnvsCtrl::Zz388
    }
    #[doc = "17 usec"]
    #[inline(always)]
    pub fn is_zz389(&self) -> bool {
        *self == TnvsCtrl::Zz389
    }
    #[doc = "20 usec"]
    #[inline(always)]
    pub fn is_zz390(&self) -> bool {
        *self == TnvsCtrl::Zz390
    }
    #[doc = "23 usec"]
    #[inline(always)]
    pub fn is_zz391(&self) -> bool {
        *self == TnvsCtrl::Zz391
    }
    #[doc = "26 usec"]
    #[inline(always)]
    pub fn is_zz392(&self) -> bool {
        *self == TnvsCtrl::Zz392
    }
}
#[doc = "Field `TNVS_CTRL` writer - Tnvs Control"]
pub type TnvsCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 3, TnvsCtrl, crate::Safe>;
impl<'a, REG> TnvsCtrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "5 usec"]
    #[inline(always)]
    pub fn zz385(self) -> &'a mut crate::W<REG> {
        self.variant(TnvsCtrl::Zz385)
    }
    #[doc = "8 usec"]
    #[inline(always)]
    pub fn zz386(self) -> &'a mut crate::W<REG> {
        self.variant(TnvsCtrl::Zz386)
    }
    #[doc = "11 usec"]
    #[inline(always)]
    pub fn zz387(self) -> &'a mut crate::W<REG> {
        self.variant(TnvsCtrl::Zz387)
    }
    #[doc = "14 usec"]
    #[inline(always)]
    pub fn zz388(self) -> &'a mut crate::W<REG> {
        self.variant(TnvsCtrl::Zz388)
    }
    #[doc = "17 usec"]
    #[inline(always)]
    pub fn zz389(self) -> &'a mut crate::W<REG> {
        self.variant(TnvsCtrl::Zz389)
    }
    #[doc = "20 usec"]
    #[inline(always)]
    pub fn zz390(self) -> &'a mut crate::W<REG> {
        self.variant(TnvsCtrl::Zz390)
    }
    #[doc = "23 usec"]
    #[inline(always)]
    pub fn zz391(self) -> &'a mut crate::W<REG> {
        self.variant(TnvsCtrl::Zz391)
    }
    #[doc = "26 usec"]
    #[inline(always)]
    pub fn zz392(self) -> &'a mut crate::W<REG> {
        self.variant(TnvsCtrl::Zz392)
    }
}
#[doc = "Tnvh Control\n\nValue on reset: 6"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TnvhCtrl {
    #[doc = "0: 2 usec"]
    Zz377 = 0,
    #[doc = "1: 2.5 usec"]
    Zz378 = 1,
    #[doc = "2: 3 usec"]
    Zz379 = 2,
    #[doc = "3: 3.5 usec"]
    Zz380 = 3,
    #[doc = "4: 4 usec"]
    Zz381 = 4,
    #[doc = "5: 4.5 usec"]
    Zz382 = 5,
    #[doc = "6: 5 usec"]
    Zz383 = 6,
    #[doc = "7: 5.5 usec"]
    Zz384 = 7,
}
impl From<TnvhCtrl> for u8 {
    #[inline(always)]
    fn from(variant: TnvhCtrl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TnvhCtrl {
    type Ux = u8;
}
impl crate::IsEnum for TnvhCtrl {}
#[doc = "Field `TNVH_CTRL` reader - Tnvh Control"]
pub type TnvhCtrlR = crate::FieldReader<TnvhCtrl>;
impl TnvhCtrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TnvhCtrl {
        match self.bits {
            0 => TnvhCtrl::Zz377,
            1 => TnvhCtrl::Zz378,
            2 => TnvhCtrl::Zz379,
            3 => TnvhCtrl::Zz380,
            4 => TnvhCtrl::Zz381,
            5 => TnvhCtrl::Zz382,
            6 => TnvhCtrl::Zz383,
            7 => TnvhCtrl::Zz384,
            _ => unreachable!(),
        }
    }
    #[doc = "2 usec"]
    #[inline(always)]
    pub fn is_zz377(&self) -> bool {
        *self == TnvhCtrl::Zz377
    }
    #[doc = "2.5 usec"]
    #[inline(always)]
    pub fn is_zz378(&self) -> bool {
        *self == TnvhCtrl::Zz378
    }
    #[doc = "3 usec"]
    #[inline(always)]
    pub fn is_zz379(&self) -> bool {
        *self == TnvhCtrl::Zz379
    }
    #[doc = "3.5 usec"]
    #[inline(always)]
    pub fn is_zz380(&self) -> bool {
        *self == TnvhCtrl::Zz380
    }
    #[doc = "4 usec"]
    #[inline(always)]
    pub fn is_zz381(&self) -> bool {
        *self == TnvhCtrl::Zz381
    }
    #[doc = "4.5 usec"]
    #[inline(always)]
    pub fn is_zz382(&self) -> bool {
        *self == TnvhCtrl::Zz382
    }
    #[doc = "5 usec"]
    #[inline(always)]
    pub fn is_zz383(&self) -> bool {
        *self == TnvhCtrl::Zz383
    }
    #[doc = "5.5 usec"]
    #[inline(always)]
    pub fn is_zz384(&self) -> bool {
        *self == TnvhCtrl::Zz384
    }
}
#[doc = "Field `TNVH_CTRL` writer - Tnvh Control"]
pub type TnvhCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 3, TnvhCtrl, crate::Safe>;
impl<'a, REG> TnvhCtrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2 usec"]
    #[inline(always)]
    pub fn zz377(self) -> &'a mut crate::W<REG> {
        self.variant(TnvhCtrl::Zz377)
    }
    #[doc = "2.5 usec"]
    #[inline(always)]
    pub fn zz378(self) -> &'a mut crate::W<REG> {
        self.variant(TnvhCtrl::Zz378)
    }
    #[doc = "3 usec"]
    #[inline(always)]
    pub fn zz379(self) -> &'a mut crate::W<REG> {
        self.variant(TnvhCtrl::Zz379)
    }
    #[doc = "3.5 usec"]
    #[inline(always)]
    pub fn zz380(self) -> &'a mut crate::W<REG> {
        self.variant(TnvhCtrl::Zz380)
    }
    #[doc = "4 usec"]
    #[inline(always)]
    pub fn zz381(self) -> &'a mut crate::W<REG> {
        self.variant(TnvhCtrl::Zz381)
    }
    #[doc = "4.5 usec"]
    #[inline(always)]
    pub fn zz382(self) -> &'a mut crate::W<REG> {
        self.variant(TnvhCtrl::Zz382)
    }
    #[doc = "5 usec"]
    #[inline(always)]
    pub fn zz383(self) -> &'a mut crate::W<REG> {
        self.variant(TnvhCtrl::Zz383)
    }
    #[doc = "5.5 usec"]
    #[inline(always)]
    pub fn zz384(self) -> &'a mut crate::W<REG> {
        self.variant(TnvhCtrl::Zz384)
    }
}
#[doc = "Tpgs Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TpgsCtrl {
    #[doc = "0: 1 usec"]
    Zz369 = 0,
    #[doc = "1: 2 usec"]
    Zz370 = 1,
    #[doc = "2: 3 usec"]
    Zz371 = 2,
    #[doc = "3: 4 usec"]
    Zz372 = 3,
    #[doc = "4: 5 usec"]
    Zz373 = 4,
    #[doc = "5: 6 usec"]
    Zz374 = 5,
    #[doc = "6: 7 usec"]
    Zz375 = 6,
    #[doc = "7: 8 usec"]
    Zz376 = 7,
}
impl From<TpgsCtrl> for u8 {
    #[inline(always)]
    fn from(variant: TpgsCtrl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TpgsCtrl {
    type Ux = u8;
}
impl crate::IsEnum for TpgsCtrl {}
#[doc = "Field `TPGS_CTRL` reader - Tpgs Control"]
pub type TpgsCtrlR = crate::FieldReader<TpgsCtrl>;
impl TpgsCtrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TpgsCtrl {
        match self.bits {
            0 => TpgsCtrl::Zz369,
            1 => TpgsCtrl::Zz370,
            2 => TpgsCtrl::Zz371,
            3 => TpgsCtrl::Zz372,
            4 => TpgsCtrl::Zz373,
            5 => TpgsCtrl::Zz374,
            6 => TpgsCtrl::Zz375,
            7 => TpgsCtrl::Zz376,
            _ => unreachable!(),
        }
    }
    #[doc = "1 usec"]
    #[inline(always)]
    pub fn is_zz369(&self) -> bool {
        *self == TpgsCtrl::Zz369
    }
    #[doc = "2 usec"]
    #[inline(always)]
    pub fn is_zz370(&self) -> bool {
        *self == TpgsCtrl::Zz370
    }
    #[doc = "3 usec"]
    #[inline(always)]
    pub fn is_zz371(&self) -> bool {
        *self == TpgsCtrl::Zz371
    }
    #[doc = "4 usec"]
    #[inline(always)]
    pub fn is_zz372(&self) -> bool {
        *self == TpgsCtrl::Zz372
    }
    #[doc = "5 usec"]
    #[inline(always)]
    pub fn is_zz373(&self) -> bool {
        *self == TpgsCtrl::Zz373
    }
    #[doc = "6 usec"]
    #[inline(always)]
    pub fn is_zz374(&self) -> bool {
        *self == TpgsCtrl::Zz374
    }
    #[doc = "7 usec"]
    #[inline(always)]
    pub fn is_zz375(&self) -> bool {
        *self == TpgsCtrl::Zz375
    }
    #[doc = "8 usec"]
    #[inline(always)]
    pub fn is_zz376(&self) -> bool {
        *self == TpgsCtrl::Zz376
    }
}
#[doc = "Field `TPGS_CTRL` writer - Tpgs Control"]
pub type TpgsCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 3, TpgsCtrl, crate::Safe>;
impl<'a, REG> TpgsCtrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 usec"]
    #[inline(always)]
    pub fn zz369(self) -> &'a mut crate::W<REG> {
        self.variant(TpgsCtrl::Zz369)
    }
    #[doc = "2 usec"]
    #[inline(always)]
    pub fn zz370(self) -> &'a mut crate::W<REG> {
        self.variant(TpgsCtrl::Zz370)
    }
    #[doc = "3 usec"]
    #[inline(always)]
    pub fn zz371(self) -> &'a mut crate::W<REG> {
        self.variant(TpgsCtrl::Zz371)
    }
    #[doc = "4 usec"]
    #[inline(always)]
    pub fn zz372(self) -> &'a mut crate::W<REG> {
        self.variant(TpgsCtrl::Zz372)
    }
    #[doc = "5 usec"]
    #[inline(always)]
    pub fn zz373(self) -> &'a mut crate::W<REG> {
        self.variant(TpgsCtrl::Zz373)
    }
    #[doc = "6 usec"]
    #[inline(always)]
    pub fn zz374(self) -> &'a mut crate::W<REG> {
        self.variant(TpgsCtrl::Zz374)
    }
    #[doc = "7 usec"]
    #[inline(always)]
    pub fn zz375(self) -> &'a mut crate::W<REG> {
        self.variant(TpgsCtrl::Zz375)
    }
    #[doc = "8 usec"]
    #[inline(always)]
    pub fn zz376(self) -> &'a mut crate::W<REG> {
        self.variant(TpgsCtrl::Zz376)
    }
}
#[doc = "Field `MAX_ERASE` reader - Number of Erase Shots"]
pub type MaxEraseR = crate::FieldReader<u16>;
#[doc = "Field `MAX_ERASE` writer - Number of Erase Shots"]
pub type MaxEraseW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `MAX_PROG` reader - Number of Program Shots"]
pub type MaxProgR = crate::FieldReader;
#[doc = "Field `MAX_PROG` writer - Number of Program Shots"]
pub type MaxProgW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:2 - Ters Control"]
    #[inline(always)]
    pub fn ters_ctrl0(&self) -> TersCtrl0R {
        TersCtrl0R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4 - Tpgm Control"]
    #[inline(always)]
    pub fn tpgm_ctrl(&self) -> TpgmCtrlR {
        TpgmCtrlR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:7 - Tnvs Control"]
    #[inline(always)]
    pub fn tnvs_ctrl(&self) -> TnvsCtrlR {
        TnvsCtrlR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Tnvh Control"]
    #[inline(always)]
    pub fn tnvh_ctrl(&self) -> TnvhCtrlR {
        TnvhCtrlR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:13 - Tpgs Control"]
    #[inline(always)]
    pub fn tpgs_ctrl(&self) -> TpgsCtrlR {
        TpgsCtrlR::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bits 14:22 - Number of Erase Shots"]
    #[inline(always)]
    pub fn max_erase(&self) -> MaxEraseR {
        MaxEraseR::new(((self.bits >> 14) & 0x01ff) as u16)
    }
    #[doc = "Bits 23:27 - Number of Program Shots"]
    #[inline(always)]
    pub fn max_prog(&self) -> MaxProgR {
        MaxProgR::new(((self.bits >> 23) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Ters Control"]
    #[inline(always)]
    pub fn ters_ctrl0(&mut self) -> TersCtrl0W<'_, SmwSettingOption1Spec> {
        TersCtrl0W::new(self, 0)
    }
    #[doc = "Bits 3:4 - Tpgm Control"]
    #[inline(always)]
    pub fn tpgm_ctrl(&mut self) -> TpgmCtrlW<'_, SmwSettingOption1Spec> {
        TpgmCtrlW::new(self, 3)
    }
    #[doc = "Bits 5:7 - Tnvs Control"]
    #[inline(always)]
    pub fn tnvs_ctrl(&mut self) -> TnvsCtrlW<'_, SmwSettingOption1Spec> {
        TnvsCtrlW::new(self, 5)
    }
    #[doc = "Bits 8:10 - Tnvh Control"]
    #[inline(always)]
    pub fn tnvh_ctrl(&mut self) -> TnvhCtrlW<'_, SmwSettingOption1Spec> {
        TnvhCtrlW::new(self, 8)
    }
    #[doc = "Bits 11:13 - Tpgs Control"]
    #[inline(always)]
    pub fn tpgs_ctrl(&mut self) -> TpgsCtrlW<'_, SmwSettingOption1Spec> {
        TpgsCtrlW::new(self, 11)
    }
    #[doc = "Bits 14:22 - Number of Erase Shots"]
    #[inline(always)]
    pub fn max_erase(&mut self) -> MaxEraseW<'_, SmwSettingOption1Spec> {
        MaxEraseW::new(self, 14)
    }
    #[doc = "Bits 23:27 - Number of Program Shots"]
    #[inline(always)]
    pub fn max_prog(&mut self) -> MaxProgW<'_, SmwSettingOption1Spec> {
        MaxProgW::new(self, 23)
    }
}
#[doc = "SMW Setting Option 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`smw_setting_option1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smw_setting_option1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmwSettingOption1Spec;
impl crate::RegisterSpec for SmwSettingOption1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smw_setting_option1::R`](R) reader structure"]
impl crate::Readable for SmwSettingOption1Spec {}
#[doc = "`write(|w| ..)` method takes [`smw_setting_option1::W`](W) writer structure"]
impl crate::Writable for SmwSettingOption1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SMW_SETTING_OPTION1 to value 0x0295_8e2a"]
impl crate::Resettable for SmwSettingOption1Spec {
    const RESET_VALUE: u32 = 0x0295_8e2a;
}
