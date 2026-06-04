#[doc = "Register `RESET_STATUS` reader"]
pub type R = crate::R<ResetStatusSpec>;
#[doc = "Register `RESET_STATUS` writer"]
pub type W = crate::W<ResetStatusSpec>;
#[doc = "Array Trim Complete\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AryTrimDone {
    #[doc = "0: Recall register load operation has not been completed"]
    Zz93 = 0,
    #[doc = "1: Recall register load operation has completed"]
    Zz94 = 1,
}
impl From<AryTrimDone> for bool {
    #[inline(always)]
    fn from(variant: AryTrimDone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARY_TRIM_DONE` reader - Array Trim Complete"]
pub type AryTrimDoneR = crate::BitReader<AryTrimDone>;
impl AryTrimDoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AryTrimDone {
        match self.bits {
            false => AryTrimDone::Zz93,
            true => AryTrimDone::Zz94,
        }
    }
    #[doc = "Recall register load operation has not been completed"]
    #[inline(always)]
    pub fn is_zz93(&self) -> bool {
        *self == AryTrimDone::Zz93
    }
    #[doc = "Recall register load operation has completed"]
    #[inline(always)]
    pub fn is_zz94(&self) -> bool {
        *self == AryTrimDone::Zz94
    }
}
#[doc = "Field `ARY_TRIM_DONE` writer - Array Trim Complete"]
pub type AryTrimDoneW<'a, REG> = crate::BitWriter<'a, REG, AryTrimDone>;
impl<'a, REG> AryTrimDoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Recall register load operation has not been completed"]
    #[inline(always)]
    pub fn zz93(self) -> &'a mut crate::W<REG> {
        self.variant(AryTrimDone::Zz93)
    }
    #[doc = "Recall register load operation has completed"]
    #[inline(always)]
    pub fn zz94(self) -> &'a mut crate::W<REG> {
        self.variant(AryTrimDone::Zz94)
    }
}
#[doc = "Status of the C0DE_C0DEh check to enable loading of the FMU parameters\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FmuParmEn {
    #[doc = "0: C0DE_C0DEh check not attempted"]
    Zz91 = 0,
    #[doc = "1: C0DE_C0DEh check completed"]
    Zz92 = 1,
}
impl From<FmuParmEn> for bool {
    #[inline(always)]
    fn from(variant: FmuParmEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FMU_PARM_EN` reader - Status of the C0DE_C0DEh check to enable loading of the FMU parameters"]
pub type FmuParmEnR = crate::BitReader<FmuParmEn>;
impl FmuParmEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FmuParmEn {
        match self.bits {
            false => FmuParmEn::Zz91,
            true => FmuParmEn::Zz92,
        }
    }
    #[doc = "C0DE_C0DEh check not attempted"]
    #[inline(always)]
    pub fn is_zz91(&self) -> bool {
        *self == FmuParmEn::Zz91
    }
    #[doc = "C0DE_C0DEh check completed"]
    #[inline(always)]
    pub fn is_zz92(&self) -> bool {
        *self == FmuParmEn::Zz92
    }
}
#[doc = "Field `FMU_PARM_EN` writer - Status of the C0DE_C0DEh check to enable loading of the FMU parameters"]
pub type FmuParmEnW<'a, REG> = crate::BitWriter<'a, REG, FmuParmEn>;
impl<'a, REG> FmuParmEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "C0DE_C0DEh check not attempted"]
    #[inline(always)]
    pub fn zz91(self) -> &'a mut crate::W<REG> {
        self.variant(FmuParmEn::Zz91)
    }
    #[doc = "C0DE_C0DEh check completed"]
    #[inline(always)]
    pub fn zz92(self) -> &'a mut crate::W<REG> {
        self.variant(FmuParmEn::Zz92)
    }
}
#[doc = "FMU Register Load Complete\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FmuParmDone {
    #[doc = "0: FMU registers have not been loaded"]
    Zz89 = 0,
    #[doc = "1: FMU registers have been loaded"]
    Zz90 = 1,
}
impl From<FmuParmDone> for bool {
    #[inline(always)]
    fn from(variant: FmuParmDone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FMU_PARM_DONE` reader - FMU Register Load Complete"]
pub type FmuParmDoneR = crate::BitReader<FmuParmDone>;
impl FmuParmDoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FmuParmDone {
        match self.bits {
            false => FmuParmDone::Zz89,
            true => FmuParmDone::Zz90,
        }
    }
    #[doc = "FMU registers have not been loaded"]
    #[inline(always)]
    pub fn is_zz89(&self) -> bool {
        *self == FmuParmDone::Zz89
    }
    #[doc = "FMU registers have been loaded"]
    #[inline(always)]
    pub fn is_zz90(&self) -> bool {
        *self == FmuParmDone::Zz90
    }
}
#[doc = "Field `FMU_PARM_DONE` writer - FMU Register Load Complete"]
pub type FmuParmDoneW<'a, REG> = crate::BitWriter<'a, REG, FmuParmDone>;
impl<'a, REG> FmuParmDoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FMU registers have not been loaded"]
    #[inline(always)]
    pub fn zz89(self) -> &'a mut crate::W<REG> {
        self.variant(FmuParmDone::Zz89)
    }
    #[doc = "FMU registers have been loaded"]
    #[inline(always)]
    pub fn zz90(self) -> &'a mut crate::W<REG> {
        self.variant(FmuParmDone::Zz90)
    }
}
#[doc = "Status of the C0DE_C0DEh check to enable loading of the SoC trim settings\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SocTrimEn {
    #[doc = "0: C0DE_C0DEh check not attempted"]
    Zz87 = 0,
    #[doc = "1: C0DE_C0DEh check completed"]
    Zz88 = 1,
}
impl From<SocTrimEn> for bool {
    #[inline(always)]
    fn from(variant: SocTrimEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOC_TRIM_EN` reader - Status of the C0DE_C0DEh check to enable loading of the SoC trim settings"]
pub type SocTrimEnR = crate::BitReader<SocTrimEn>;
impl SocTrimEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SocTrimEn {
        match self.bits {
            false => SocTrimEn::Zz87,
            true => SocTrimEn::Zz88,
        }
    }
    #[doc = "C0DE_C0DEh check not attempted"]
    #[inline(always)]
    pub fn is_zz87(&self) -> bool {
        *self == SocTrimEn::Zz87
    }
    #[doc = "C0DE_C0DEh check completed"]
    #[inline(always)]
    pub fn is_zz88(&self) -> bool {
        *self == SocTrimEn::Zz88
    }
}
#[doc = "Field `SOC_TRIM_EN` writer - Status of the C0DE_C0DEh check to enable loading of the SoC trim settings"]
pub type SocTrimEnW<'a, REG> = crate::BitWriter<'a, REG, SocTrimEn>;
impl<'a, REG> SocTrimEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "C0DE_C0DEh check not attempted"]
    #[inline(always)]
    pub fn zz87(self) -> &'a mut crate::W<REG> {
        self.variant(SocTrimEn::Zz87)
    }
    #[doc = "C0DE_C0DEh check completed"]
    #[inline(always)]
    pub fn zz88(self) -> &'a mut crate::W<REG> {
        self.variant(SocTrimEn::Zz88)
    }
}
#[doc = "Status of the C0DE_C0DEh check for enabling ECC decoder during reads of SoC trim settings\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SocTrimEcc {
    #[doc = "0: C0DE_C0DEh check failed"]
    Zz85 = 0,
    #[doc = "1: C0DE_C0DEh check passed"]
    Zz86 = 1,
}
impl From<SocTrimEcc> for bool {
    #[inline(always)]
    fn from(variant: SocTrimEcc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOC_TRIM_ECC` reader - Status of the C0DE_C0DEh check for enabling ECC decoder during reads of SoC trim settings"]
pub type SocTrimEccR = crate::BitReader<SocTrimEcc>;
impl SocTrimEccR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SocTrimEcc {
        match self.bits {
            false => SocTrimEcc::Zz85,
            true => SocTrimEcc::Zz86,
        }
    }
    #[doc = "C0DE_C0DEh check failed"]
    #[inline(always)]
    pub fn is_zz85(&self) -> bool {
        *self == SocTrimEcc::Zz85
    }
    #[doc = "C0DE_C0DEh check passed"]
    #[inline(always)]
    pub fn is_zz86(&self) -> bool {
        *self == SocTrimEcc::Zz86
    }
}
#[doc = "Field `SOC_TRIM_ECC` writer - Status of the C0DE_C0DEh check for enabling ECC decoder during reads of SoC trim settings"]
pub type SocTrimEccW<'a, REG> = crate::BitWriter<'a, REG, SocTrimEcc>;
impl<'a, REG> SocTrimEccW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "C0DE_C0DEh check failed"]
    #[inline(always)]
    pub fn zz85(self) -> &'a mut crate::W<REG> {
        self.variant(SocTrimEcc::Zz85)
    }
    #[doc = "C0DE_C0DEh check passed"]
    #[inline(always)]
    pub fn zz86(self) -> &'a mut crate::W<REG> {
        self.variant(SocTrimEcc::Zz86)
    }
}
#[doc = "SoC Trim Complete\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SocTrimDone {
    #[doc = "0: SoC Trim registers have not been updated"]
    Zz83 = 0,
    #[doc = "1: All SoC Trim registers have been updated"]
    Zz84 = 1,
}
impl From<SocTrimDone> for bool {
    #[inline(always)]
    fn from(variant: SocTrimDone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOC_TRIM_DONE` reader - SoC Trim Complete"]
pub type SocTrimDoneR = crate::BitReader<SocTrimDone>;
impl SocTrimDoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SocTrimDone {
        match self.bits {
            false => SocTrimDone::Zz83,
            true => SocTrimDone::Zz84,
        }
    }
    #[doc = "SoC Trim registers have not been updated"]
    #[inline(always)]
    pub fn is_zz83(&self) -> bool {
        *self == SocTrimDone::Zz83
    }
    #[doc = "All SoC Trim registers have been updated"]
    #[inline(always)]
    pub fn is_zz84(&self) -> bool {
        *self == SocTrimDone::Zz84
    }
}
#[doc = "Field `SOC_TRIM_DONE` writer - SoC Trim Complete"]
pub type SocTrimDoneW<'a, REG> = crate::BitWriter<'a, REG, SocTrimDone>;
impl<'a, REG> SocTrimDoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SoC Trim registers have not been updated"]
    #[inline(always)]
    pub fn zz83(self) -> &'a mut crate::W<REG> {
        self.variant(SocTrimDone::Zz83)
    }
    #[doc = "All SoC Trim registers have been updated"]
    #[inline(always)]
    pub fn zz84(self) -> &'a mut crate::W<REG> {
        self.variant(SocTrimDone::Zz84)
    }
}
#[doc = "Array Repair Complete\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RprDone {
    #[doc = "0: Repair registers have not been loaded"]
    Zz81 = 0,
    #[doc = "1: Repair registers have been loaded"]
    Zz82 = 1,
}
impl From<RprDone> for bool {
    #[inline(always)]
    fn from(variant: RprDone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RPR_DONE` reader - Array Repair Complete"]
pub type RprDoneR = crate::BitReader<RprDone>;
impl RprDoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RprDone {
        match self.bits {
            false => RprDone::Zz81,
            true => RprDone::Zz82,
        }
    }
    #[doc = "Repair registers have not been loaded"]
    #[inline(always)]
    pub fn is_zz81(&self) -> bool {
        *self == RprDone::Zz81
    }
    #[doc = "Repair registers have been loaded"]
    #[inline(always)]
    pub fn is_zz82(&self) -> bool {
        *self == RprDone::Zz82
    }
}
#[doc = "Field `RPR_DONE` writer - Array Repair Complete"]
pub type RprDoneW<'a, REG> = crate::BitWriter<'a, REG, RprDone>;
impl<'a, REG> RprDoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Repair registers have not been loaded"]
    #[inline(always)]
    pub fn zz81(self) -> &'a mut crate::W<REG> {
        self.variant(RprDone::Zz81)
    }
    #[doc = "Repair registers have been loaded"]
    #[inline(always)]
    pub fn zz82(self) -> &'a mut crate::W<REG> {
        self.variant(RprDone::Zz82)
    }
}
#[doc = "Initialization Done\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InitDone {
    #[doc = "0: All initialization steps did not complete"]
    Zz79 = 0,
    #[doc = "1: All initialization steps completed"]
    Zz80 = 1,
}
impl From<InitDone> for bool {
    #[inline(always)]
    fn from(variant: InitDone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INIT_DONE` reader - Initialization Done"]
pub type InitDoneR = crate::BitReader<InitDone>;
impl InitDoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> InitDone {
        match self.bits {
            false => InitDone::Zz79,
            true => InitDone::Zz80,
        }
    }
    #[doc = "All initialization steps did not complete"]
    #[inline(always)]
    pub fn is_zz79(&self) -> bool {
        *self == InitDone::Zz79
    }
    #[doc = "All initialization steps completed"]
    #[inline(always)]
    pub fn is_zz80(&self) -> bool {
        *self == InitDone::Zz80
    }
}
#[doc = "Field `INIT_DONE` writer - Initialization Done"]
pub type InitDoneW<'a, REG> = crate::BitWriter<'a, REG, InitDone>;
impl<'a, REG> InitDoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "All initialization steps did not complete"]
    #[inline(always)]
    pub fn zz79(self) -> &'a mut crate::W<REG> {
        self.variant(InitDone::Zz79)
    }
    #[doc = "All initialization steps completed"]
    #[inline(always)]
    pub fn zz80(self) -> &'a mut crate::W<REG> {
        self.variant(InitDone::Zz80)
    }
}
#[doc = "ECC Single Fault during Reset Recovery\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RstSfErr {
    #[doc = "0: No single-bit faults detected during initialization"]
    Zz77 = 0,
    #[doc = "1: At least one single ECC fault was detected during initialization"]
    Zz78 = 1,
}
impl From<RstSfErr> for bool {
    #[inline(always)]
    fn from(variant: RstSfErr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RST_SF_ERR` reader - ECC Single Fault during Reset Recovery"]
pub type RstSfErrR = crate::BitReader<RstSfErr>;
impl RstSfErrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RstSfErr {
        match self.bits {
            false => RstSfErr::Zz77,
            true => RstSfErr::Zz78,
        }
    }
    #[doc = "No single-bit faults detected during initialization"]
    #[inline(always)]
    pub fn is_zz77(&self) -> bool {
        *self == RstSfErr::Zz77
    }
    #[doc = "At least one single ECC fault was detected during initialization"]
    #[inline(always)]
    pub fn is_zz78(&self) -> bool {
        *self == RstSfErr::Zz78
    }
}
#[doc = "Field `RST_SF_ERR` writer - ECC Single Fault during Reset Recovery"]
pub type RstSfErrW<'a, REG> = crate::BitWriter<'a, REG, RstSfErr>;
impl<'a, REG> RstSfErrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No single-bit faults detected during initialization"]
    #[inline(always)]
    pub fn zz77(self) -> &'a mut crate::W<REG> {
        self.variant(RstSfErr::Zz77)
    }
    #[doc = "At least one single ECC fault was detected during initialization"]
    #[inline(always)]
    pub fn zz78(self) -> &'a mut crate::W<REG> {
        self.variant(RstSfErr::Zz78)
    }
}
#[doc = "ECC Double Fault during Reset Recovery\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RstDfErr {
    #[doc = "0: No double-bit faults detected during initialization"]
    Zz75 = 0,
    #[doc = "1: Double-bit ECC fault was detected during initialization"]
    Zz76 = 1,
}
impl From<RstDfErr> for bool {
    #[inline(always)]
    fn from(variant: RstDfErr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RST_DF_ERR` reader - ECC Double Fault during Reset Recovery"]
pub type RstDfErrR = crate::BitReader<RstDfErr>;
impl RstDfErrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RstDfErr {
        match self.bits {
            false => RstDfErr::Zz75,
            true => RstDfErr::Zz76,
        }
    }
    #[doc = "No double-bit faults detected during initialization"]
    #[inline(always)]
    pub fn is_zz75(&self) -> bool {
        *self == RstDfErr::Zz75
    }
    #[doc = "Double-bit ECC fault was detected during initialization"]
    #[inline(always)]
    pub fn is_zz76(&self) -> bool {
        *self == RstDfErr::Zz76
    }
}
#[doc = "Field `RST_DF_ERR` writer - ECC Double Fault during Reset Recovery"]
pub type RstDfErrW<'a, REG> = crate::BitWriter<'a, REG, RstDfErr>;
impl<'a, REG> RstDfErrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No double-bit faults detected during initialization"]
    #[inline(always)]
    pub fn zz75(self) -> &'a mut crate::W<REG> {
        self.variant(RstDfErr::Zz75)
    }
    #[doc = "Double-bit ECC fault was detected during initialization"]
    #[inline(always)]
    pub fn zz76(self) -> &'a mut crate::W<REG> {
        self.variant(RstDfErr::Zz76)
    }
}
#[doc = "Field `SOC_TRIM_DF_ERR` reader - ECC Double Fault during load of SoC Trim phrases"]
pub type SocTrimDfErrR = crate::FieldReader;
#[doc = "Field `SOC_TRIM_DF_ERR` writer - ECC Double Fault during load of SoC Trim phrases"]
pub type SocTrimDfErrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Reset Patch Required\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RstPatchLd {
    #[doc = "0: No patch required to be loaded during reset"]
    Zz73 = 0,
    #[doc = "1: Patch loaded during reset"]
    Zz74 = 1,
}
impl From<RstPatchLd> for bool {
    #[inline(always)]
    fn from(variant: RstPatchLd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RST_PATCH_LD` reader - Reset Patch Required"]
pub type RstPatchLdR = crate::BitReader<RstPatchLd>;
impl RstPatchLdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RstPatchLd {
        match self.bits {
            false => RstPatchLd::Zz73,
            true => RstPatchLd::Zz74,
        }
    }
    #[doc = "No patch required to be loaded during reset"]
    #[inline(always)]
    pub fn is_zz73(&self) -> bool {
        *self == RstPatchLd::Zz73
    }
    #[doc = "Patch loaded during reset"]
    #[inline(always)]
    pub fn is_zz74(&self) -> bool {
        *self == RstPatchLd::Zz74
    }
}
#[doc = "Field `RST_PATCH_LD` writer - Reset Patch Required"]
pub type RstPatchLdW<'a, REG> = crate::BitWriter<'a, REG, RstPatchLd>;
impl<'a, REG> RstPatchLdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No patch required to be loaded during reset"]
    #[inline(always)]
    pub fn zz73(self) -> &'a mut crate::W<REG> {
        self.variant(RstPatchLd::Zz73)
    }
    #[doc = "Patch loaded during reset"]
    #[inline(always)]
    pub fn zz74(self) -> &'a mut crate::W<REG> {
        self.variant(RstPatchLd::Zz74)
    }
}
#[doc = "Recall Data Mismatch\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RecallDataMismatch {
    #[doc = "0: Data read towards end of reset matched data read for Recall"]
    Zz71 = 0,
    #[doc = "1: Data read towards end of reset did not match data read for recall"]
    Zz72 = 1,
}
impl From<RecallDataMismatch> for bool {
    #[inline(always)]
    fn from(variant: RecallDataMismatch) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECALL_DATA_MISMATCH` reader - Recall Data Mismatch"]
pub type RecallDataMismatchR = crate::BitReader<RecallDataMismatch>;
impl RecallDataMismatchR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RecallDataMismatch {
        match self.bits {
            false => RecallDataMismatch::Zz71,
            true => RecallDataMismatch::Zz72,
        }
    }
    #[doc = "Data read towards end of reset matched data read for Recall"]
    #[inline(always)]
    pub fn is_zz71(&self) -> bool {
        *self == RecallDataMismatch::Zz71
    }
    #[doc = "Data read towards end of reset did not match data read for recall"]
    #[inline(always)]
    pub fn is_zz72(&self) -> bool {
        *self == RecallDataMismatch::Zz72
    }
}
#[doc = "Field `RECALL_DATA_MISMATCH` writer - Recall Data Mismatch"]
pub type RecallDataMismatchW<'a, REG> = crate::BitWriter<'a, REG, RecallDataMismatch>;
impl<'a, REG> RecallDataMismatchW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data read towards end of reset matched data read for Recall"]
    #[inline(always)]
    pub fn zz71(self) -> &'a mut crate::W<REG> {
        self.variant(RecallDataMismatch::Zz71)
    }
    #[doc = "Data read towards end of reset did not match data read for recall"]
    #[inline(always)]
    pub fn zz72(self) -> &'a mut crate::W<REG> {
        self.variant(RecallDataMismatch::Zz72)
    }
}
impl R {
    #[doc = "Bit 0 - Array Trim Complete"]
    #[inline(always)]
    pub fn ary_trim_done(&self) -> AryTrimDoneR {
        AryTrimDoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Status of the C0DE_C0DEh check to enable loading of the FMU parameters"]
    #[inline(always)]
    pub fn fmu_parm_en(&self) -> FmuParmEnR {
        FmuParmEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FMU Register Load Complete"]
    #[inline(always)]
    pub fn fmu_parm_done(&self) -> FmuParmDoneR {
        FmuParmDoneR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Status of the C0DE_C0DEh check to enable loading of the SoC trim settings"]
    #[inline(always)]
    pub fn soc_trim_en(&self) -> SocTrimEnR {
        SocTrimEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Status of the C0DE_C0DEh check for enabling ECC decoder during reads of SoC trim settings"]
    #[inline(always)]
    pub fn soc_trim_ecc(&self) -> SocTrimEccR {
        SocTrimEccR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SoC Trim Complete"]
    #[inline(always)]
    pub fn soc_trim_done(&self) -> SocTrimDoneR {
        SocTrimDoneR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Array Repair Complete"]
    #[inline(always)]
    pub fn rpr_done(&self) -> RprDoneR {
        RprDoneR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Initialization Done"]
    #[inline(always)]
    pub fn init_done(&self) -> InitDoneR {
        InitDoneR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ECC Single Fault during Reset Recovery"]
    #[inline(always)]
    pub fn rst_sf_err(&self) -> RstSfErrR {
        RstSfErrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ECC Double Fault during Reset Recovery"]
    #[inline(always)]
    pub fn rst_df_err(&self) -> RstDfErrR {
        RstDfErrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:17 - ECC Double Fault during load of SoC Trim phrases"]
    #[inline(always)]
    pub fn soc_trim_df_err(&self) -> SocTrimDfErrR {
        SocTrimDfErrR::new(((self.bits >> 10) & 0xff) as u8)
    }
    #[doc = "Bit 18 - Reset Patch Required"]
    #[inline(always)]
    pub fn rst_patch_ld(&self) -> RstPatchLdR {
        RstPatchLdR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Recall Data Mismatch"]
    #[inline(always)]
    pub fn recall_data_mismatch(&self) -> RecallDataMismatchR {
        RecallDataMismatchR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Array Trim Complete"]
    #[inline(always)]
    pub fn ary_trim_done(&mut self) -> AryTrimDoneW<'_, ResetStatusSpec> {
        AryTrimDoneW::new(self, 0)
    }
    #[doc = "Bit 1 - Status of the C0DE_C0DEh check to enable loading of the FMU parameters"]
    #[inline(always)]
    pub fn fmu_parm_en(&mut self) -> FmuParmEnW<'_, ResetStatusSpec> {
        FmuParmEnW::new(self, 1)
    }
    #[doc = "Bit 2 - FMU Register Load Complete"]
    #[inline(always)]
    pub fn fmu_parm_done(&mut self) -> FmuParmDoneW<'_, ResetStatusSpec> {
        FmuParmDoneW::new(self, 2)
    }
    #[doc = "Bit 3 - Status of the C0DE_C0DEh check to enable loading of the SoC trim settings"]
    #[inline(always)]
    pub fn soc_trim_en(&mut self) -> SocTrimEnW<'_, ResetStatusSpec> {
        SocTrimEnW::new(self, 3)
    }
    #[doc = "Bit 4 - Status of the C0DE_C0DEh check for enabling ECC decoder during reads of SoC trim settings"]
    #[inline(always)]
    pub fn soc_trim_ecc(&mut self) -> SocTrimEccW<'_, ResetStatusSpec> {
        SocTrimEccW::new(self, 4)
    }
    #[doc = "Bit 5 - SoC Trim Complete"]
    #[inline(always)]
    pub fn soc_trim_done(&mut self) -> SocTrimDoneW<'_, ResetStatusSpec> {
        SocTrimDoneW::new(self, 5)
    }
    #[doc = "Bit 6 - Array Repair Complete"]
    #[inline(always)]
    pub fn rpr_done(&mut self) -> RprDoneW<'_, ResetStatusSpec> {
        RprDoneW::new(self, 6)
    }
    #[doc = "Bit 7 - Initialization Done"]
    #[inline(always)]
    pub fn init_done(&mut self) -> InitDoneW<'_, ResetStatusSpec> {
        InitDoneW::new(self, 7)
    }
    #[doc = "Bit 8 - ECC Single Fault during Reset Recovery"]
    #[inline(always)]
    pub fn rst_sf_err(&mut self) -> RstSfErrW<'_, ResetStatusSpec> {
        RstSfErrW::new(self, 8)
    }
    #[doc = "Bit 9 - ECC Double Fault during Reset Recovery"]
    #[inline(always)]
    pub fn rst_df_err(&mut self) -> RstDfErrW<'_, ResetStatusSpec> {
        RstDfErrW::new(self, 9)
    }
    #[doc = "Bits 10:17 - ECC Double Fault during load of SoC Trim phrases"]
    #[inline(always)]
    pub fn soc_trim_df_err(&mut self) -> SocTrimDfErrW<'_, ResetStatusSpec> {
        SocTrimDfErrW::new(self, 10)
    }
    #[doc = "Bit 18 - Reset Patch Required"]
    #[inline(always)]
    pub fn rst_patch_ld(&mut self) -> RstPatchLdW<'_, ResetStatusSpec> {
        RstPatchLdW::new(self, 18)
    }
    #[doc = "Bit 19 - Recall Data Mismatch"]
    #[inline(always)]
    pub fn recall_data_mismatch(&mut self) -> RecallDataMismatchW<'_, ResetStatusSpec> {
        RecallDataMismatchW::new(self, 19)
    }
}
#[doc = "FMU Initialization Tracking Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reset_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reset_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ResetStatusSpec;
impl crate::RegisterSpec for ResetStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reset_status::R`](R) reader structure"]
impl crate::Readable for ResetStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`reset_status::W`](W) writer structure"]
impl crate::Writable for ResetStatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RESET_STATUS to value 0"]
impl crate::Resettable for ResetStatusSpec {}
