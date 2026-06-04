#[doc = "Register `SMW_SETTING_OPTION2` reader"]
pub type R = crate::R<SmwSettingOption2Spec>;
#[doc = "Register `SMW_SETTING_OPTION2` writer"]
pub type W = crate::W<SmwSettingOption2Spec>;
#[doc = "Field `THVS_CTRL` reader - Thvs control"]
pub type ThvsCtrlR = crate::FieldReader;
#[doc = "Field `THVS_CTRL` writer - Thvs control"]
pub type ThvsCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TRCV_CTRL` reader - Trcv Control"]
pub type TrcvCtrlR = crate::FieldReader;
#[doc = "Field `TRCV_CTRL` writer - Trcv Control"]
pub type TrcvCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `XTRA_ERS` reader - Number of Post Shots for SME"]
pub type XtraErsR = crate::FieldReader;
#[doc = "Field `XTRA_ERS` writer - Number of Post Shots for SME"]
pub type XtraErsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `XTRA_PGM` reader - Number of Post Shots for SMP"]
pub type XtraPgmR = crate::FieldReader;
#[doc = "Field `XTRA_PGM` writer - Number of Post Shots for SMP"]
pub type XtraPgmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WHV_CNTR` reader - WHV Counter"]
pub type WhvCntrR = crate::FieldReader;
#[doc = "Field `WHV_CNTR` writer - WHV Counter"]
pub type WhvCntrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Post Ters Time\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PostTers {
    #[doc = "0: 50 usec"]
    Zz361 = 0,
    #[doc = "1: 100 usec"]
    Zz362 = 1,
    #[doc = "2: 200 usec"]
    Zz363 = 2,
    #[doc = "3: 300 usec"]
    Zz364 = 3,
    #[doc = "4: 500 usec"]
    Zz365 = 4,
    #[doc = "5: 1 msec"]
    Zz366 = 5,
    #[doc = "6: 1.5 msec"]
    Zz367 = 6,
    #[doc = "7: 2 msec"]
    Zz368 = 7,
}
impl From<PostTers> for u8 {
    #[inline(always)]
    fn from(variant: PostTers) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PostTers {
    type Ux = u8;
}
impl crate::IsEnum for PostTers {}
#[doc = "Field `POST_TERS` reader - Post Ters Time"]
pub type PostTersR = crate::FieldReader<PostTers>;
impl PostTersR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PostTers {
        match self.bits {
            0 => PostTers::Zz361,
            1 => PostTers::Zz362,
            2 => PostTers::Zz363,
            3 => PostTers::Zz364,
            4 => PostTers::Zz365,
            5 => PostTers::Zz366,
            6 => PostTers::Zz367,
            7 => PostTers::Zz368,
            _ => unreachable!(),
        }
    }
    #[doc = "50 usec"]
    #[inline(always)]
    pub fn is_zz361(&self) -> bool {
        *self == PostTers::Zz361
    }
    #[doc = "100 usec"]
    #[inline(always)]
    pub fn is_zz362(&self) -> bool {
        *self == PostTers::Zz362
    }
    #[doc = "200 usec"]
    #[inline(always)]
    pub fn is_zz363(&self) -> bool {
        *self == PostTers::Zz363
    }
    #[doc = "300 usec"]
    #[inline(always)]
    pub fn is_zz364(&self) -> bool {
        *self == PostTers::Zz364
    }
    #[doc = "500 usec"]
    #[inline(always)]
    pub fn is_zz365(&self) -> bool {
        *self == PostTers::Zz365
    }
    #[doc = "1 msec"]
    #[inline(always)]
    pub fn is_zz366(&self) -> bool {
        *self == PostTers::Zz366
    }
    #[doc = "1.5 msec"]
    #[inline(always)]
    pub fn is_zz367(&self) -> bool {
        *self == PostTers::Zz367
    }
    #[doc = "2 msec"]
    #[inline(always)]
    pub fn is_zz368(&self) -> bool {
        *self == PostTers::Zz368
    }
}
#[doc = "Field `POST_TERS` writer - Post Ters Time"]
pub type PostTersW<'a, REG> = crate::FieldWriter<'a, REG, 3, PostTers, crate::Safe>;
impl<'a, REG> PostTersW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "50 usec"]
    #[inline(always)]
    pub fn zz361(self) -> &'a mut crate::W<REG> {
        self.variant(PostTers::Zz361)
    }
    #[doc = "100 usec"]
    #[inline(always)]
    pub fn zz362(self) -> &'a mut crate::W<REG> {
        self.variant(PostTers::Zz362)
    }
    #[doc = "200 usec"]
    #[inline(always)]
    pub fn zz363(self) -> &'a mut crate::W<REG> {
        self.variant(PostTers::Zz363)
    }
    #[doc = "300 usec"]
    #[inline(always)]
    pub fn zz364(self) -> &'a mut crate::W<REG> {
        self.variant(PostTers::Zz364)
    }
    #[doc = "500 usec"]
    #[inline(always)]
    pub fn zz365(self) -> &'a mut crate::W<REG> {
        self.variant(PostTers::Zz365)
    }
    #[doc = "1 msec"]
    #[inline(always)]
    pub fn zz366(self) -> &'a mut crate::W<REG> {
        self.variant(PostTers::Zz366)
    }
    #[doc = "1.5 msec"]
    #[inline(always)]
    pub fn zz367(self) -> &'a mut crate::W<REG> {
        self.variant(PostTers::Zz367)
    }
    #[doc = "2 msec"]
    #[inline(always)]
    pub fn zz368(self) -> &'a mut crate::W<REG> {
        self.variant(PostTers::Zz368)
    }
}
#[doc = "Post Tpgm Time\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PostTpgm {
    #[doc = "0: 1 usec"]
    Zz357 = 0,
    #[doc = "1: 2 usec"]
    Zz358 = 1,
    #[doc = "2: 4 usec"]
    Zz359 = 2,
    #[doc = "3: 8 usec"]
    Zz360 = 3,
}
impl From<PostTpgm> for u8 {
    #[inline(always)]
    fn from(variant: PostTpgm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PostTpgm {
    type Ux = u8;
}
impl crate::IsEnum for PostTpgm {}
#[doc = "Field `POST_TPGM` reader - Post Tpgm Time"]
pub type PostTpgmR = crate::FieldReader<PostTpgm>;
impl PostTpgmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PostTpgm {
        match self.bits {
            0 => PostTpgm::Zz357,
            1 => PostTpgm::Zz358,
            2 => PostTpgm::Zz359,
            3 => PostTpgm::Zz360,
            _ => unreachable!(),
        }
    }
    #[doc = "1 usec"]
    #[inline(always)]
    pub fn is_zz357(&self) -> bool {
        *self == PostTpgm::Zz357
    }
    #[doc = "2 usec"]
    #[inline(always)]
    pub fn is_zz358(&self) -> bool {
        *self == PostTpgm::Zz358
    }
    #[doc = "4 usec"]
    #[inline(always)]
    pub fn is_zz359(&self) -> bool {
        *self == PostTpgm::Zz359
    }
    #[doc = "8 usec"]
    #[inline(always)]
    pub fn is_zz360(&self) -> bool {
        *self == PostTpgm::Zz360
    }
}
#[doc = "Field `POST_TPGM` writer - Post Tpgm Time"]
pub type PostTpgmW<'a, REG> = crate::FieldWriter<'a, REG, 2, PostTpgm, crate::Safe>;
impl<'a, REG> PostTpgmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 usec"]
    #[inline(always)]
    pub fn zz357(self) -> &'a mut crate::W<REG> {
        self.variant(PostTpgm::Zz357)
    }
    #[doc = "2 usec"]
    #[inline(always)]
    pub fn zz358(self) -> &'a mut crate::W<REG> {
        self.variant(PostTpgm::Zz358)
    }
    #[doc = "4 usec"]
    #[inline(always)]
    pub fn zz359(self) -> &'a mut crate::W<REG> {
        self.variant(PostTpgm::Zz359)
    }
    #[doc = "8 usec"]
    #[inline(always)]
    pub fn zz360(self) -> &'a mut crate::W<REG> {
        self.variant(PostTpgm::Zz360)
    }
}
#[doc = "Verify Option\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VfyOpt {
    #[doc = "0: Skip verify for post shot only, verify for all other shots"]
    Zz353 = 0,
    #[doc = "1: Skip verify for the 1st and post shots"]
    Zz354 = 1,
    #[doc = "2: Skip the 1st, 2nd, and post shots"]
    Zz355 = 2,
    #[doc = "3: Skip verify for all shots"]
    Zz356 = 3,
}
impl From<VfyOpt> for u8 {
    #[inline(always)]
    fn from(variant: VfyOpt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for VfyOpt {
    type Ux = u8;
}
impl crate::IsEnum for VfyOpt {}
#[doc = "Field `VFY_OPT` reader - Verify Option"]
pub type VfyOptR = crate::FieldReader<VfyOpt>;
impl VfyOptR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VfyOpt {
        match self.bits {
            0 => VfyOpt::Zz353,
            1 => VfyOpt::Zz354,
            2 => VfyOpt::Zz355,
            3 => VfyOpt::Zz356,
            _ => unreachable!(),
        }
    }
    #[doc = "Skip verify for post shot only, verify for all other shots"]
    #[inline(always)]
    pub fn is_zz353(&self) -> bool {
        *self == VfyOpt::Zz353
    }
    #[doc = "Skip verify for the 1st and post shots"]
    #[inline(always)]
    pub fn is_zz354(&self) -> bool {
        *self == VfyOpt::Zz354
    }
    #[doc = "Skip the 1st, 2nd, and post shots"]
    #[inline(always)]
    pub fn is_zz355(&self) -> bool {
        *self == VfyOpt::Zz355
    }
    #[doc = "Skip verify for all shots"]
    #[inline(always)]
    pub fn is_zz356(&self) -> bool {
        *self == VfyOpt::Zz356
    }
}
#[doc = "Field `VFY_OPT` writer - Verify Option"]
pub type VfyOptW<'a, REG> = crate::FieldWriter<'a, REG, 2, VfyOpt, crate::Safe>;
impl<'a, REG> VfyOptW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Skip verify for post shot only, verify for all other shots"]
    #[inline(always)]
    pub fn zz353(self) -> &'a mut crate::W<REG> {
        self.variant(VfyOpt::Zz353)
    }
    #[doc = "Skip verify for the 1st and post shots"]
    #[inline(always)]
    pub fn zz354(self) -> &'a mut crate::W<REG> {
        self.variant(VfyOpt::Zz354)
    }
    #[doc = "Skip the 1st, 2nd, and post shots"]
    #[inline(always)]
    pub fn zz355(self) -> &'a mut crate::W<REG> {
        self.variant(VfyOpt::Zz355)
    }
    #[doc = "Skip verify for all shots"]
    #[inline(always)]
    pub fn zz356(self) -> &'a mut crate::W<REG> {
        self.variant(VfyOpt::Zz356)
    }
}
#[doc = "Tpgm Option\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TpgmOpt {
    #[doc = "0: Fixed Tpgm for all shots, except post shot"]
    Zz349 = 0,
    #[doc = "1: Increase Tpgm option by 1 for each loop until Tpgm reaches 4 usec"]
    Zz350 = 1,
    #[doc = "2: Increase Tpgm option by 1 for each loop until Tpgm reaches 8 usec"]
    Zz351 = 2,
    #[doc = "3: Unused"]
    Zz352 = 3,
}
impl From<TpgmOpt> for u8 {
    #[inline(always)]
    fn from(variant: TpgmOpt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TpgmOpt {
    type Ux = u8;
}
impl crate::IsEnum for TpgmOpt {}
#[doc = "Field `TPGM_OPT` reader - Tpgm Option"]
pub type TpgmOptR = crate::FieldReader<TpgmOpt>;
impl TpgmOptR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TpgmOpt {
        match self.bits {
            0 => TpgmOpt::Zz349,
            1 => TpgmOpt::Zz350,
            2 => TpgmOpt::Zz351,
            3 => TpgmOpt::Zz352,
            _ => unreachable!(),
        }
    }
    #[doc = "Fixed Tpgm for all shots, except post shot"]
    #[inline(always)]
    pub fn is_zz349(&self) -> bool {
        *self == TpgmOpt::Zz349
    }
    #[doc = "Increase Tpgm option by 1 for each loop until Tpgm reaches 4 usec"]
    #[inline(always)]
    pub fn is_zz350(&self) -> bool {
        *self == TpgmOpt::Zz350
    }
    #[doc = "Increase Tpgm option by 1 for each loop until Tpgm reaches 8 usec"]
    #[inline(always)]
    pub fn is_zz351(&self) -> bool {
        *self == TpgmOpt::Zz351
    }
    #[doc = "Unused"]
    #[inline(always)]
    pub fn is_zz352(&self) -> bool {
        *self == TpgmOpt::Zz352
    }
}
#[doc = "Field `TPGM_OPT` writer - Tpgm Option"]
pub type TpgmOptW<'a, REG> = crate::FieldWriter<'a, REG, 2, TpgmOpt, crate::Safe>;
impl<'a, REG> TpgmOptW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Fixed Tpgm for all shots, except post shot"]
    #[inline(always)]
    pub fn zz349(self) -> &'a mut crate::W<REG> {
        self.variant(TpgmOpt::Zz349)
    }
    #[doc = "Increase Tpgm option by 1 for each loop until Tpgm reaches 4 usec"]
    #[inline(always)]
    pub fn zz350(self) -> &'a mut crate::W<REG> {
        self.variant(TpgmOpt::Zz350)
    }
    #[doc = "Increase Tpgm option by 1 for each loop until Tpgm reaches 8 usec"]
    #[inline(always)]
    pub fn zz351(self) -> &'a mut crate::W<REG> {
        self.variant(TpgmOpt::Zz351)
    }
    #[doc = "Unused"]
    #[inline(always)]
    pub fn zz352(self) -> &'a mut crate::W<REG> {
        self.variant(TpgmOpt::Zz352)
    }
}
#[doc = "MASK0_OPT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mask0Opt {
    #[doc = "0: Mask programmed bits passing PV until extra shot"]
    Zz347 = 0,
    #[doc = "1: Always program bits even if they pass PV"]
    Zz348 = 1,
}
impl From<Mask0Opt> for bool {
    #[inline(always)]
    fn from(variant: Mask0Opt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MASK0_OPT` reader - MASK0_OPT"]
pub type Mask0OptR = crate::BitReader<Mask0Opt>;
impl Mask0OptR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mask0Opt {
        match self.bits {
            false => Mask0Opt::Zz347,
            true => Mask0Opt::Zz348,
        }
    }
    #[doc = "Mask programmed bits passing PV until extra shot"]
    #[inline(always)]
    pub fn is_zz347(&self) -> bool {
        *self == Mask0Opt::Zz347
    }
    #[doc = "Always program bits even if they pass PV"]
    #[inline(always)]
    pub fn is_zz348(&self) -> bool {
        *self == Mask0Opt::Zz348
    }
}
#[doc = "Field `MASK0_OPT` writer - MASK0_OPT"]
pub type Mask0OptW<'a, REG> = crate::BitWriter<'a, REG, Mask0Opt>;
impl<'a, REG> Mask0OptW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Mask programmed bits passing PV until extra shot"]
    #[inline(always)]
    pub fn zz347(self) -> &'a mut crate::W<REG> {
        self.variant(Mask0Opt::Zz347)
    }
    #[doc = "Always program bits even if they pass PV"]
    #[inline(always)]
    pub fn zz348(self) -> &'a mut crate::W<REG> {
        self.variant(Mask0Opt::Zz348)
    }
}
#[doc = "Disable pre-PV Read before First Program Shot\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DisPrer {
    #[doc = "0: Enable pre-PV read before first program shot"]
    Zz345 = 0,
    #[doc = "1: Disable pre-PV read before first program shot"]
    Zz346 = 1,
}
impl From<DisPrer> for bool {
    #[inline(always)]
    fn from(variant: DisPrer) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIS_PRER` reader - Disable pre-PV Read before First Program Shot"]
pub type DisPrerR = crate::BitReader<DisPrer>;
impl DisPrerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DisPrer {
        match self.bits {
            false => DisPrer::Zz345,
            true => DisPrer::Zz346,
        }
    }
    #[doc = "Enable pre-PV read before first program shot"]
    #[inline(always)]
    pub fn is_zz345(&self) -> bool {
        *self == DisPrer::Zz345
    }
    #[doc = "Disable pre-PV read before first program shot"]
    #[inline(always)]
    pub fn is_zz346(&self) -> bool {
        *self == DisPrer::Zz346
    }
}
#[doc = "Field `DIS_PRER` writer - Disable pre-PV Read before First Program Shot"]
pub type DisPrerW<'a, REG> = crate::BitWriter<'a, REG, DisPrer>;
impl<'a, REG> DisPrerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable pre-PV read before first program shot"]
    #[inline(always)]
    pub fn zz345(self) -> &'a mut crate::W<REG> {
        self.variant(DisPrer::Zz345)
    }
    #[doc = "Disable pre-PV read before first program shot"]
    #[inline(always)]
    pub fn zz346(self) -> &'a mut crate::W<REG> {
        self.variant(DisPrer::Zz346)
    }
}
impl R {
    #[doc = "Bits 0:2 - Thvs control"]
    #[inline(always)]
    pub fn thvs_ctrl(&self) -> ThvsCtrlR {
        ThvsCtrlR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Trcv Control"]
    #[inline(always)]
    pub fn trcv_ctrl(&self) -> TrcvCtrlR {
        TrcvCtrlR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:7 - Number of Post Shots for SME"]
    #[inline(always)]
    pub fn xtra_ers(&self) -> XtraErsR {
        XtraErsR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Number of Post Shots for SMP"]
    #[inline(always)]
    pub fn xtra_pgm(&self) -> XtraPgmR {
        XtraPgmR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:17 - WHV Counter"]
    #[inline(always)]
    pub fn whv_cntr(&self) -> WhvCntrR {
        WhvCntrR::new(((self.bits >> 10) & 0xff) as u8)
    }
    #[doc = "Bits 18:20 - Post Ters Time"]
    #[inline(always)]
    pub fn post_ters(&self) -> PostTersR {
        PostTersR::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:22 - Post Tpgm Time"]
    #[inline(always)]
    pub fn post_tpgm(&self) -> PostTpgmR {
        PostTpgmR::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bits 23:24 - Verify Option"]
    #[inline(always)]
    pub fn vfy_opt(&self) -> VfyOptR {
        VfyOptR::new(((self.bits >> 23) & 3) as u8)
    }
    #[doc = "Bits 25:26 - Tpgm Option"]
    #[inline(always)]
    pub fn tpgm_opt(&self) -> TpgmOptR {
        TpgmOptR::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bit 27 - MASK0_OPT"]
    #[inline(always)]
    pub fn mask0_opt(&self) -> Mask0OptR {
        Mask0OptR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Disable pre-PV Read before First Program Shot"]
    #[inline(always)]
    pub fn dis_prer(&self) -> DisPrerR {
        DisPrerR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Thvs control"]
    #[inline(always)]
    pub fn thvs_ctrl(&mut self) -> ThvsCtrlW<'_, SmwSettingOption2Spec> {
        ThvsCtrlW::new(self, 0)
    }
    #[doc = "Bits 3:5 - Trcv Control"]
    #[inline(always)]
    pub fn trcv_ctrl(&mut self) -> TrcvCtrlW<'_, SmwSettingOption2Spec> {
        TrcvCtrlW::new(self, 3)
    }
    #[doc = "Bits 6:7 - Number of Post Shots for SME"]
    #[inline(always)]
    pub fn xtra_ers(&mut self) -> XtraErsW<'_, SmwSettingOption2Spec> {
        XtraErsW::new(self, 6)
    }
    #[doc = "Bits 8:9 - Number of Post Shots for SMP"]
    #[inline(always)]
    pub fn xtra_pgm(&mut self) -> XtraPgmW<'_, SmwSettingOption2Spec> {
        XtraPgmW::new(self, 8)
    }
    #[doc = "Bits 10:17 - WHV Counter"]
    #[inline(always)]
    pub fn whv_cntr(&mut self) -> WhvCntrW<'_, SmwSettingOption2Spec> {
        WhvCntrW::new(self, 10)
    }
    #[doc = "Bits 18:20 - Post Ters Time"]
    #[inline(always)]
    pub fn post_ters(&mut self) -> PostTersW<'_, SmwSettingOption2Spec> {
        PostTersW::new(self, 18)
    }
    #[doc = "Bits 21:22 - Post Tpgm Time"]
    #[inline(always)]
    pub fn post_tpgm(&mut self) -> PostTpgmW<'_, SmwSettingOption2Spec> {
        PostTpgmW::new(self, 21)
    }
    #[doc = "Bits 23:24 - Verify Option"]
    #[inline(always)]
    pub fn vfy_opt(&mut self) -> VfyOptW<'_, SmwSettingOption2Spec> {
        VfyOptW::new(self, 23)
    }
    #[doc = "Bits 25:26 - Tpgm Option"]
    #[inline(always)]
    pub fn tpgm_opt(&mut self) -> TpgmOptW<'_, SmwSettingOption2Spec> {
        TpgmOptW::new(self, 25)
    }
    #[doc = "Bit 27 - MASK0_OPT"]
    #[inline(always)]
    pub fn mask0_opt(&mut self) -> Mask0OptW<'_, SmwSettingOption2Spec> {
        Mask0OptW::new(self, 27)
    }
    #[doc = "Bit 28 - Disable pre-PV Read before First Program Shot"]
    #[inline(always)]
    pub fn dis_prer(&mut self) -> DisPrerW<'_, SmwSettingOption2Spec> {
        DisPrerW::new(self, 28)
    }
}
#[doc = "SMW Setting Option 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`smw_setting_option2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smw_setting_option2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmwSettingOption2Spec;
impl crate::RegisterSpec for SmwSettingOption2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smw_setting_option2::R`](R) reader structure"]
impl crate::Readable for SmwSettingOption2Spec {}
#[doc = "`write(|w| ..)` method takes [`smw_setting_option2::W`](W) writer structure"]
impl crate::Writable for SmwSettingOption2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SMW_SETTING_OPTION2 to value 0x00a8_0151"]
impl crate::Resettable for SmwSettingOption2Spec {
    const RESET_VALUE: u32 = 0x00a8_0151;
}
