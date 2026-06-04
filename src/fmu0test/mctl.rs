#[doc = "Register `MCTL` reader"]
pub type R = crate::R<MctlSpec>;
#[doc = "Register `MCTL` writer"]
pub type W = crate::W<MctlSpec>;
#[doc = "Core Hold\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Corehld {
    #[doc = "0: CPU access is allowed"]
    Zz125 = 0,
    #[doc = "1: CPU access must be blocked"]
    Zz126 = 1,
}
impl From<Corehld> for bool {
    #[inline(always)]
    fn from(variant: Corehld) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COREHLD` reader - Core Hold"]
pub type CorehldR = crate::BitReader<Corehld>;
impl CorehldR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Corehld {
        match self.bits {
            false => Corehld::Zz125,
            true => Corehld::Zz126,
        }
    }
    #[doc = "CPU access is allowed"]
    #[inline(always)]
    pub fn is_zz125(&self) -> bool {
        *self == Corehld::Zz125
    }
    #[doc = "CPU access must be blocked"]
    #[inline(always)]
    pub fn is_zz126(&self) -> bool {
        *self == Corehld::Zz126
    }
}
#[doc = "Field `COREHLD` writer - Core Hold"]
pub type CorehldW<'a, REG> = crate::BitWriter<'a, REG, Corehld>;
impl<'a, REG> CorehldW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CPU access is allowed"]
    #[inline(always)]
    pub fn zz125(self) -> &'a mut crate::W<REG> {
        self.variant(Corehld::Zz125)
    }
    #[doc = "CPU access must be blocked"]
    #[inline(always)]
    pub fn zz126(self) -> &'a mut crate::W<REG> {
        self.variant(Corehld::Zz126)
    }
}
#[doc = "LSACTIVE Feature Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LsactEn {
    #[doc = "0: LSACTIVE feature disabled completely: FCTRL\\[LSACTIVE\\] is forced low and no longer writable, LVE cannot assert at the TSMC array interface."]
    Zz123 = 0,
    #[doc = "1: LSACTIVE feature fully enabled and controllable by SoC and internal UINT SM."]
    Zz124 = 1,
}
impl From<LsactEn> for bool {
    #[inline(always)]
    fn from(variant: LsactEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSACT_EN` reader - LSACTIVE Feature Enable"]
pub type LsactEnR = crate::BitReader<LsactEn>;
impl LsactEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LsactEn {
        match self.bits {
            false => LsactEn::Zz123,
            true => LsactEn::Zz124,
        }
    }
    #[doc = "LSACTIVE feature disabled completely: FCTRL\\[LSACTIVE\\] is forced low and no longer writable, LVE cannot assert at the TSMC array interface."]
    #[inline(always)]
    pub fn is_zz123(&self) -> bool {
        *self == LsactEn::Zz123
    }
    #[doc = "LSACTIVE feature fully enabled and controllable by SoC and internal UINT SM."]
    #[inline(always)]
    pub fn is_zz124(&self) -> bool {
        *self == LsactEn::Zz124
    }
}
#[doc = "Field `LSACT_EN` writer - LSACTIVE Feature Enable"]
pub type LsactEnW<'a, REG> = crate::BitWriter<'a, REG, LsactEn>;
impl<'a, REG> LsactEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LSACTIVE feature disabled completely: FCTRL\\[LSACTIVE\\] is forced low and no longer writable, LVE cannot assert at the TSMC array interface."]
    #[inline(always)]
    pub fn zz123(self) -> &'a mut crate::W<REG> {
        self.variant(LsactEn::Zz123)
    }
    #[doc = "LSACTIVE feature fully enabled and controllable by SoC and internal UINT SM."]
    #[inline(always)]
    pub fn zz124(self) -> &'a mut crate::W<REG> {
        self.variant(LsactEn::Zz124)
    }
}
#[doc = "LSACTIVE Write Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lsactwren {
    #[doc = "0: Unrestricted write access allowed"]
    Zz121 = 0,
    #[doc = "1: Write access while CMP set must match CMDDID and CMDPRT"]
    Zz122 = 1,
}
impl From<Lsactwren> for bool {
    #[inline(always)]
    fn from(variant: Lsactwren) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSACTWREN` reader - LSACTIVE Write Enable"]
pub type LsactwrenR = crate::BitReader<Lsactwren>;
impl LsactwrenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lsactwren {
        match self.bits {
            false => Lsactwren::Zz121,
            true => Lsactwren::Zz122,
        }
    }
    #[doc = "Unrestricted write access allowed"]
    #[inline(always)]
    pub fn is_zz121(&self) -> bool {
        *self == Lsactwren::Zz121
    }
    #[doc = "Write access while CMP set must match CMDDID and CMDPRT"]
    #[inline(always)]
    pub fn is_zz122(&self) -> bool {
        *self == Lsactwren::Zz122
    }
}
#[doc = "Field `LSACTWREN` writer - LSACTIVE Write Enable"]
pub type LsactwrenW<'a, REG> = crate::BitWriter<'a, REG, Lsactwren>;
impl<'a, REG> LsactwrenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Unrestricted write access allowed"]
    #[inline(always)]
    pub fn zz121(self) -> &'a mut crate::W<REG> {
        self.variant(Lsactwren::Zz121)
    }
    #[doc = "Write access while CMP set must match CMDDID and CMDPRT"]
    #[inline(always)]
    pub fn zz122(self) -> &'a mut crate::W<REG> {
        self.variant(Lsactwren::Zz122)
    }
}
#[doc = "Master Repair Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MasterRepairEn {
    #[doc = "0: Repair disabled"]
    Zz119 = 0,
    #[doc = "1: Repair enable determined by bit 0 of each REPAIR register"]
    Zz120 = 1,
}
impl From<MasterRepairEn> for bool {
    #[inline(always)]
    fn from(variant: MasterRepairEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MASTER_REPAIR_EN` reader - Master Repair Enable"]
pub type MasterRepairEnR = crate::BitReader<MasterRepairEn>;
impl MasterRepairEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MasterRepairEn {
        match self.bits {
            false => MasterRepairEn::Zz119,
            true => MasterRepairEn::Zz120,
        }
    }
    #[doc = "Repair disabled"]
    #[inline(always)]
    pub fn is_zz119(&self) -> bool {
        *self == MasterRepairEn::Zz119
    }
    #[doc = "Repair enable determined by bit 0 of each REPAIR register"]
    #[inline(always)]
    pub fn is_zz120(&self) -> bool {
        *self == MasterRepairEn::Zz120
    }
}
#[doc = "Field `MASTER_REPAIR_EN` writer - Master Repair Enable"]
pub type MasterRepairEnW<'a, REG> = crate::BitWriter<'a, REG, MasterRepairEn>;
impl<'a, REG> MasterRepairEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Repair disabled"]
    #[inline(always)]
    pub fn zz119(self) -> &'a mut crate::W<REG> {
        self.variant(MasterRepairEn::Zz119)
    }
    #[doc = "Repair enable determined by bit 0 of each REPAIR register"]
    #[inline(always)]
    pub fn zz120(self) -> &'a mut crate::W<REG> {
        self.variant(MasterRepairEn::Zz120)
    }
}
#[doc = "RF Active Command Enable Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rfcmden {
    #[doc = "0: Flash commands blocked (CCIF not writable)"]
    Zz117 = 0,
    #[doc = "1: Flash commands allowed"]
    Zz118 = 1,
}
impl From<Rfcmden> for bool {
    #[inline(always)]
    fn from(variant: Rfcmden) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFCMDEN` reader - RF Active Command Enable Control"]
pub type RfcmdenR = crate::BitReader<Rfcmden>;
impl RfcmdenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rfcmden {
        match self.bits {
            false => Rfcmden::Zz117,
            true => Rfcmden::Zz118,
        }
    }
    #[doc = "Flash commands blocked (CCIF not writable)"]
    #[inline(always)]
    pub fn is_zz117(&self) -> bool {
        *self == Rfcmden::Zz117
    }
    #[doc = "Flash commands allowed"]
    #[inline(always)]
    pub fn is_zz118(&self) -> bool {
        *self == Rfcmden::Zz118
    }
}
#[doc = "Field `RFCMDEN` writer - RF Active Command Enable Control"]
pub type RfcmdenW<'a, REG> = crate::BitWriter<'a, REG, Rfcmden>;
impl<'a, REG> RfcmdenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flash commands blocked (CCIF not writable)"]
    #[inline(always)]
    pub fn zz117(self) -> &'a mut crate::W<REG> {
        self.variant(Rfcmden::Zz117)
    }
    #[doc = "Flash commands allowed"]
    #[inline(always)]
    pub fn zz118(self) -> &'a mut crate::W<REG> {
        self.variant(Rfcmden::Zz118)
    }
}
#[doc = "Command Write Sequence Abort Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cwsabten {
    #[doc = "0: CWS abort feature is disabled"]
    Zz115 = 0,
    #[doc = "1: CWS abort feature is enabled"]
    Zz116 = 1,
}
impl From<Cwsabten> for bool {
    #[inline(always)]
    fn from(variant: Cwsabten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CWSABTEN` reader - Command Write Sequence Abort Enable"]
pub type CwsabtenR = crate::BitReader<Cwsabten>;
impl CwsabtenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cwsabten {
        match self.bits {
            false => Cwsabten::Zz115,
            true => Cwsabten::Zz116,
        }
    }
    #[doc = "CWS abort feature is disabled"]
    #[inline(always)]
    pub fn is_zz115(&self) -> bool {
        *self == Cwsabten::Zz115
    }
    #[doc = "CWS abort feature is enabled"]
    #[inline(always)]
    pub fn is_zz116(&self) -> bool {
        *self == Cwsabten::Zz116
    }
}
#[doc = "Field `CWSABTEN` writer - Command Write Sequence Abort Enable"]
pub type CwsabtenW<'a, REG> = crate::BitWriter<'a, REG, Cwsabten>;
impl<'a, REG> CwsabtenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CWS abort feature is disabled"]
    #[inline(always)]
    pub fn zz115(self) -> &'a mut crate::W<REG> {
        self.variant(Cwsabten::Zz115)
    }
    #[doc = "CWS abort feature is enabled"]
    #[inline(always)]
    pub fn zz116(self) -> &'a mut crate::W<REG> {
        self.variant(Cwsabten::Zz116)
    }
}
#[doc = "Margin Read Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mrgrddis {
    #[doc = "0: Margin Read Settings are enabled"]
    Zz113 = 0,
    #[doc = "1: Margin Read Settings are disabled"]
    Zz114 = 1,
}
impl From<Mrgrddis> for bool {
    #[inline(always)]
    fn from(variant: Mrgrddis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MRGRDDIS` reader - Margin Read Disable"]
pub type MrgrddisR = crate::BitReader<Mrgrddis>;
impl MrgrddisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mrgrddis {
        match self.bits {
            false => Mrgrddis::Zz113,
            true => Mrgrddis::Zz114,
        }
    }
    #[doc = "Margin Read Settings are enabled"]
    #[inline(always)]
    pub fn is_zz113(&self) -> bool {
        *self == Mrgrddis::Zz113
    }
    #[doc = "Margin Read Settings are disabled"]
    #[inline(always)]
    pub fn is_zz114(&self) -> bool {
        *self == Mrgrddis::Zz114
    }
}
#[doc = "Field `MRGRDDIS` writer - Margin Read Disable"]
pub type MrgrddisW<'a, REG> = crate::BitWriter<'a, REG, Mrgrddis>;
impl<'a, REG> MrgrddisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Margin Read Settings are enabled"]
    #[inline(always)]
    pub fn zz113(self) -> &'a mut crate::W<REG> {
        self.variant(Mrgrddis::Zz113)
    }
    #[doc = "Margin Read Settings are disabled"]
    #[inline(always)]
    pub fn zz114(self) -> &'a mut crate::W<REG> {
        self.variant(Mrgrddis::Zz114)
    }
}
#[doc = "Field `MRGRD0` reader - Margin Read Setting for Program"]
pub type Mrgrd0R = crate::FieldReader;
#[doc = "Field `MRGRD0` writer - Margin Read Setting for Program"]
pub type Mrgrd0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MRGRD1` reader - Margin Read Setting for Erase"]
pub type Mrgrd1R = crate::FieldReader;
#[doc = "Field `MRGRD1` writer - Margin Read Setting for Erase"]
pub type Mrgrd1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Mass Erase (Erase All) Acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ersaack {
    #[doc = "0: Mass Erase operation is not active (operation has completed or has not started)"]
    Zz111 = 0,
    #[doc = "1: Mass Erase operation is active (controller acknowledges that the soc_ersall_req input is asserted and will continue with the operation)"]
    Zz112 = 1,
}
impl From<Ersaack> for bool {
    #[inline(always)]
    fn from(variant: Ersaack) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERSAACK` reader - Mass Erase (Erase All) Acknowledge"]
pub type ErsaackR = crate::BitReader<Ersaack>;
impl ErsaackR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ersaack {
        match self.bits {
            false => Ersaack::Zz111,
            true => Ersaack::Zz112,
        }
    }
    #[doc = "Mass Erase operation is not active (operation has completed or has not started)"]
    #[inline(always)]
    pub fn is_zz111(&self) -> bool {
        *self == Ersaack::Zz111
    }
    #[doc = "Mass Erase operation is active (controller acknowledges that the soc_ersall_req input is asserted and will continue with the operation)"]
    #[inline(always)]
    pub fn is_zz112(&self) -> bool {
        *self == Ersaack::Zz112
    }
}
#[doc = "Field `ERSAACK` writer - Mass Erase (Erase All) Acknowledge"]
pub type ErsaackW<'a, REG> = crate::BitWriter<'a, REG, Ersaack>;
impl<'a, REG> ErsaackW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Mass Erase operation is not active (operation has completed or has not started)"]
    #[inline(always)]
    pub fn zz111(self) -> &'a mut crate::W<REG> {
        self.variant(Ersaack::Zz111)
    }
    #[doc = "Mass Erase operation is active (controller acknowledges that the soc_ersall_req input is asserted and will continue with the operation)"]
    #[inline(always)]
    pub fn zz112(self) -> &'a mut crate::W<REG> {
        self.variant(Ersaack::Zz112)
    }
}
#[doc = "Scan Observability Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScanObs {
    #[doc = "0: Normal functional behavior"]
    Zz109 = 0,
    #[doc = "1: Enables observation of signals that may otherwise be ATPG untestable"]
    Zz110 = 1,
}
impl From<ScanObs> for bool {
    #[inline(always)]
    fn from(variant: ScanObs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCAN_OBS` reader - Scan Observability Control"]
pub type ScanObsR = crate::BitReader<ScanObs>;
impl ScanObsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ScanObs {
        match self.bits {
            false => ScanObs::Zz109,
            true => ScanObs::Zz110,
        }
    }
    #[doc = "Normal functional behavior"]
    #[inline(always)]
    pub fn is_zz109(&self) -> bool {
        *self == ScanObs::Zz109
    }
    #[doc = "Enables observation of signals that may otherwise be ATPG untestable"]
    #[inline(always)]
    pub fn is_zz110(&self) -> bool {
        *self == ScanObs::Zz110
    }
}
#[doc = "Field `SCAN_OBS` writer - Scan Observability Control"]
pub type ScanObsW<'a, REG> = crate::BitWriter<'a, REG, ScanObs>;
impl<'a, REG> ScanObsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal functional behavior"]
    #[inline(always)]
    pub fn zz109(self) -> &'a mut crate::W<REG> {
        self.variant(ScanObs::Zz109)
    }
    #[doc = "Enables observation of signals that may otherwise be ATPG untestable"]
    #[inline(always)]
    pub fn zz110(self) -> &'a mut crate::W<REG> {
        self.variant(ScanObs::Zz110)
    }
}
#[doc = "BIST IP Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BistCtl {
    #[doc = "0: BIST IP disabled"]
    Zz107 = 0,
    #[doc = "1: BIST IP enabled"]
    Zz108 = 1,
}
impl From<BistCtl> for bool {
    #[inline(always)]
    fn from(variant: BistCtl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BIST_CTL` reader - BIST IP Control"]
pub type BistCtlR = crate::BitReader<BistCtl>;
impl BistCtlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BistCtl {
        match self.bits {
            false => BistCtl::Zz107,
            true => BistCtl::Zz108,
        }
    }
    #[doc = "BIST IP disabled"]
    #[inline(always)]
    pub fn is_zz107(&self) -> bool {
        *self == BistCtl::Zz107
    }
    #[doc = "BIST IP enabled"]
    #[inline(always)]
    pub fn is_zz108(&self) -> bool {
        *self == BistCtl::Zz108
    }
}
#[doc = "Field `BIST_CTL` writer - BIST IP Control"]
pub type BistCtlW<'a, REG> = crate::BitWriter<'a, REG, BistCtl>;
impl<'a, REG> BistCtlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "BIST IP disabled"]
    #[inline(always)]
    pub fn zz107(self) -> &'a mut crate::W<REG> {
        self.variant(BistCtl::Zz107)
    }
    #[doc = "BIST IP enabled"]
    #[inline(always)]
    pub fn zz108(self) -> &'a mut crate::W<REG> {
        self.variant(BistCtl::Zz108)
    }
}
#[doc = "SMWR IP Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SmwrCtl {
    #[doc = "0: SMWR IP disabled"]
    Zz105 = 0,
    #[doc = "1: SMWR IP enabled"]
    Zz106 = 1,
}
impl From<SmwrCtl> for bool {
    #[inline(always)]
    fn from(variant: SmwrCtl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMWR_CTL` reader - SMWR IP Control"]
pub type SmwrCtlR = crate::BitReader<SmwrCtl>;
impl SmwrCtlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SmwrCtl {
        match self.bits {
            false => SmwrCtl::Zz105,
            true => SmwrCtl::Zz106,
        }
    }
    #[doc = "SMWR IP disabled"]
    #[inline(always)]
    pub fn is_zz105(&self) -> bool {
        *self == SmwrCtl::Zz105
    }
    #[doc = "SMWR IP enabled"]
    #[inline(always)]
    pub fn is_zz106(&self) -> bool {
        *self == SmwrCtl::Zz106
    }
}
#[doc = "Field `SMWR_CTL` writer - SMWR IP Control"]
pub type SmwrCtlW<'a, REG> = crate::BitWriter<'a, REG, SmwrCtl>;
impl<'a, REG> SmwrCtlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SMWR IP disabled"]
    #[inline(always)]
    pub fn zz105(self) -> &'a mut crate::W<REG> {
        self.variant(SmwrCtl::Zz105)
    }
    #[doc = "SMWR IP enabled"]
    #[inline(always)]
    pub fn zz106(self) -> &'a mut crate::W<REG> {
        self.variant(SmwrCtl::Zz106)
    }
}
#[doc = "Salvage Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SalvDis {
    #[doc = "0: Salvage enabled (ECC used during erase verify)"]
    Zz103 = 0,
    #[doc = "1: Salvage disabled (ECC not used during erase verify)"]
    Zz104 = 1,
}
impl From<SalvDis> for bool {
    #[inline(always)]
    fn from(variant: SalvDis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SALV_DIS` reader - Salvage Disable"]
pub type SalvDisR = crate::BitReader<SalvDis>;
impl SalvDisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SalvDis {
        match self.bits {
            false => SalvDis::Zz103,
            true => SalvDis::Zz104,
        }
    }
    #[doc = "Salvage enabled (ECC used during erase verify)"]
    #[inline(always)]
    pub fn is_zz103(&self) -> bool {
        *self == SalvDis::Zz103
    }
    #[doc = "Salvage disabled (ECC not used during erase verify)"]
    #[inline(always)]
    pub fn is_zz104(&self) -> bool {
        *self == SalvDis::Zz104
    }
}
#[doc = "Field `SALV_DIS` writer - Salvage Disable"]
pub type SalvDisW<'a, REG> = crate::BitWriter<'a, REG, SalvDis>;
impl<'a, REG> SalvDisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Salvage enabled (ECC used during erase verify)"]
    #[inline(always)]
    pub fn zz103(self) -> &'a mut crate::W<REG> {
        self.variant(SalvDis::Zz103)
    }
    #[doc = "Salvage disabled (ECC not used during erase verify)"]
    #[inline(always)]
    pub fn zz104(self) -> &'a mut crate::W<REG> {
        self.variant(SalvDis::Zz104)
    }
}
#[doc = "SOC ECC Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SocEccCtl {
    #[doc = "0: ECC is enabled for SOC read access"]
    Zz101 = 0,
    #[doc = "1: ECC is disabled for SOC read access"]
    Zz102 = 1,
}
impl From<SocEccCtl> for bool {
    #[inline(always)]
    fn from(variant: SocEccCtl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOC_ECC_CTL` reader - SOC ECC Control"]
pub type SocEccCtlR = crate::BitReader<SocEccCtl>;
impl SocEccCtlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SocEccCtl {
        match self.bits {
            false => SocEccCtl::Zz101,
            true => SocEccCtl::Zz102,
        }
    }
    #[doc = "ECC is enabled for SOC read access"]
    #[inline(always)]
    pub fn is_zz101(&self) -> bool {
        *self == SocEccCtl::Zz101
    }
    #[doc = "ECC is disabled for SOC read access"]
    #[inline(always)]
    pub fn is_zz102(&self) -> bool {
        *self == SocEccCtl::Zz102
    }
}
#[doc = "Field `SOC_ECC_CTL` writer - SOC ECC Control"]
pub type SocEccCtlW<'a, REG> = crate::BitWriter<'a, REG, SocEccCtl>;
impl<'a, REG> SocEccCtlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ECC is enabled for SOC read access"]
    #[inline(always)]
    pub fn zz101(self) -> &'a mut crate::W<REG> {
        self.variant(SocEccCtl::Zz101)
    }
    #[doc = "ECC is disabled for SOC read access"]
    #[inline(always)]
    pub fn zz102(self) -> &'a mut crate::W<REG> {
        self.variant(SocEccCtl::Zz102)
    }
}
#[doc = "FMU ECC Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FmuEccCtl {
    #[doc = "0: ECC is enabled for FMU program operations"]
    Zz99 = 0,
    #[doc = "1: ECC is disabled for FMU program operations"]
    Zz100 = 1,
}
impl From<FmuEccCtl> for bool {
    #[inline(always)]
    fn from(variant: FmuEccCtl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FMU_ECC_CTL` reader - FMU ECC Control"]
pub type FmuEccCtlR = crate::BitReader<FmuEccCtl>;
impl FmuEccCtlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FmuEccCtl {
        match self.bits {
            false => FmuEccCtl::Zz99,
            true => FmuEccCtl::Zz100,
        }
    }
    #[doc = "ECC is enabled for FMU program operations"]
    #[inline(always)]
    pub fn is_zz99(&self) -> bool {
        *self == FmuEccCtl::Zz99
    }
    #[doc = "ECC is disabled for FMU program operations"]
    #[inline(always)]
    pub fn is_zz100(&self) -> bool {
        *self == FmuEccCtl::Zz100
    }
}
#[doc = "Field `FMU_ECC_CTL` writer - FMU ECC Control"]
pub type FmuEccCtlW<'a, REG> = crate::BitWriter<'a, REG, FmuEccCtl>;
impl<'a, REG> FmuEccCtlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ECC is enabled for FMU program operations"]
    #[inline(always)]
    pub fn zz99(self) -> &'a mut crate::W<REG> {
        self.variant(FmuEccCtl::Zz99)
    }
    #[doc = "ECC is disabled for FMU program operations"]
    #[inline(always)]
    pub fn zz100(self) -> &'a mut crate::W<REG> {
        self.variant(FmuEccCtl::Zz100)
    }
}
#[doc = "BIST Power Mode Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BistPwrDis {
    #[doc = "0: BIST DFT logic has full control of SLM and LVE when BIST is enabled (including during commands)"]
    Zz97 = 0,
    #[doc = "1: BIST DFT logic has no control of SLM and LVE; power mode RTL is in complete control of SLM and LVE values"]
    Zz98 = 1,
}
impl From<BistPwrDis> for bool {
    #[inline(always)]
    fn from(variant: BistPwrDis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BIST_PWR_DIS` reader - BIST Power Mode Disable"]
pub type BistPwrDisR = crate::BitReader<BistPwrDis>;
impl BistPwrDisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BistPwrDis {
        match self.bits {
            false => BistPwrDis::Zz97,
            true => BistPwrDis::Zz98,
        }
    }
    #[doc = "BIST DFT logic has full control of SLM and LVE when BIST is enabled (including during commands)"]
    #[inline(always)]
    pub fn is_zz97(&self) -> bool {
        *self == BistPwrDis::Zz97
    }
    #[doc = "BIST DFT logic has no control of SLM and LVE; power mode RTL is in complete control of SLM and LVE values"]
    #[inline(always)]
    pub fn is_zz98(&self) -> bool {
        *self == BistPwrDis::Zz98
    }
}
#[doc = "Field `BIST_PWR_DIS` writer - BIST Power Mode Disable"]
pub type BistPwrDisW<'a, REG> = crate::BitWriter<'a, REG, BistPwrDis>;
impl<'a, REG> BistPwrDisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "BIST DFT logic has full control of SLM and LVE when BIST is enabled (including during commands)"]
    #[inline(always)]
    pub fn zz97(self) -> &'a mut crate::W<REG> {
        self.variant(BistPwrDis::Zz97)
    }
    #[doc = "BIST DFT logic has no control of SLM and LVE; power mode RTL is in complete control of SLM and LVE values"]
    #[inline(always)]
    pub fn zz98(self) -> &'a mut crate::W<REG> {
        self.variant(BistPwrDis::Zz98)
    }
}
#[doc = "Oscillator control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OscH {
    #[doc = "0: Use APB clock"]
    Zz95 = 0,
    #[doc = "1: Use a known fixed-frequency clock, e.g. 12 MHz"]
    Zz96 = 1,
}
impl From<OscH> for bool {
    #[inline(always)]
    fn from(variant: OscH) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSC_H` reader - Oscillator control"]
pub type OscHR = crate::BitReader<OscH>;
impl OscHR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OscH {
        match self.bits {
            false => OscH::Zz95,
            true => OscH::Zz96,
        }
    }
    #[doc = "Use APB clock"]
    #[inline(always)]
    pub fn is_zz95(&self) -> bool {
        *self == OscH::Zz95
    }
    #[doc = "Use a known fixed-frequency clock, e.g. 12 MHz"]
    #[inline(always)]
    pub fn is_zz96(&self) -> bool {
        *self == OscH::Zz96
    }
}
#[doc = "Field `OSC_H` writer - Oscillator control"]
pub type OscHW<'a, REG> = crate::BitWriter<'a, REG, OscH>;
impl<'a, REG> OscHW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use APB clock"]
    #[inline(always)]
    pub fn zz95(self) -> &'a mut crate::W<REG> {
        self.variant(OscH::Zz95)
    }
    #[doc = "Use a known fixed-frequency clock, e.g. 12 MHz"]
    #[inline(always)]
    pub fn zz96(self) -> &'a mut crate::W<REG> {
        self.variant(OscH::Zz96)
    }
}
impl R {
    #[doc = "Bit 0 - Core Hold"]
    #[inline(always)]
    pub fn corehld(&self) -> CorehldR {
        CorehldR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - LSACTIVE Feature Enable"]
    #[inline(always)]
    pub fn lsact_en(&self) -> LsactEnR {
        LsactEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LSACTIVE Write Enable"]
    #[inline(always)]
    pub fn lsactwren(&self) -> LsactwrenR {
        LsactwrenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Master Repair Enable"]
    #[inline(always)]
    pub fn master_repair_en(&self) -> MasterRepairEnR {
        MasterRepairEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RF Active Command Enable Control"]
    #[inline(always)]
    pub fn rfcmden(&self) -> RfcmdenR {
        RfcmdenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Command Write Sequence Abort Enable"]
    #[inline(always)]
    pub fn cwsabten(&self) -> CwsabtenR {
        CwsabtenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Margin Read Disable"]
    #[inline(always)]
    pub fn mrgrddis(&self) -> MrgrddisR {
        MrgrddisR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Margin Read Setting for Program"]
    #[inline(always)]
    pub fn mrgrd0(&self) -> Mrgrd0R {
        Mrgrd0R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Margin Read Setting for Erase"]
    #[inline(always)]
    pub fn mrgrd1(&self) -> Mrgrd1R {
        Mrgrd1R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Mass Erase (Erase All) Acknowledge"]
    #[inline(always)]
    pub fn ersaack(&self) -> ErsaackR {
        ErsaackR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 19 - Scan Observability Control"]
    #[inline(always)]
    pub fn scan_obs(&self) -> ScanObsR {
        ScanObsR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - BIST IP Control"]
    #[inline(always)]
    pub fn bist_ctl(&self) -> BistCtlR {
        BistCtlR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - SMWR IP Control"]
    #[inline(always)]
    pub fn smwr_ctl(&self) -> SmwrCtlR {
        SmwrCtlR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - Salvage Disable"]
    #[inline(always)]
    pub fn salv_dis(&self) -> SalvDisR {
        SalvDisR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SOC ECC Control"]
    #[inline(always)]
    pub fn soc_ecc_ctl(&self) -> SocEccCtlR {
        SocEccCtlR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - FMU ECC Control"]
    #[inline(always)]
    pub fn fmu_ecc_ctl(&self) -> FmuEccCtlR {
        FmuEccCtlR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 29 - BIST Power Mode Disable"]
    #[inline(always)]
    pub fn bist_pwr_dis(&self) -> BistPwrDisR {
        BistPwrDisR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - Oscillator control"]
    #[inline(always)]
    pub fn osc_h(&self) -> OscHR {
        OscHR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Core Hold"]
    #[inline(always)]
    pub fn corehld(&mut self) -> CorehldW<'_, MctlSpec> {
        CorehldW::new(self, 0)
    }
    #[doc = "Bit 2 - LSACTIVE Feature Enable"]
    #[inline(always)]
    pub fn lsact_en(&mut self) -> LsactEnW<'_, MctlSpec> {
        LsactEnW::new(self, 2)
    }
    #[doc = "Bit 3 - LSACTIVE Write Enable"]
    #[inline(always)]
    pub fn lsactwren(&mut self) -> LsactwrenW<'_, MctlSpec> {
        LsactwrenW::new(self, 3)
    }
    #[doc = "Bit 4 - Master Repair Enable"]
    #[inline(always)]
    pub fn master_repair_en(&mut self) -> MasterRepairEnW<'_, MctlSpec> {
        MasterRepairEnW::new(self, 4)
    }
    #[doc = "Bit 5 - RF Active Command Enable Control"]
    #[inline(always)]
    pub fn rfcmden(&mut self) -> RfcmdenW<'_, MctlSpec> {
        RfcmdenW::new(self, 5)
    }
    #[doc = "Bit 6 - Command Write Sequence Abort Enable"]
    #[inline(always)]
    pub fn cwsabten(&mut self) -> CwsabtenW<'_, MctlSpec> {
        CwsabtenW::new(self, 6)
    }
    #[doc = "Bit 7 - Margin Read Disable"]
    #[inline(always)]
    pub fn mrgrddis(&mut self) -> MrgrddisW<'_, MctlSpec> {
        MrgrddisW::new(self, 7)
    }
    #[doc = "Bits 8:11 - Margin Read Setting for Program"]
    #[inline(always)]
    pub fn mrgrd0(&mut self) -> Mrgrd0W<'_, MctlSpec> {
        Mrgrd0W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Margin Read Setting for Erase"]
    #[inline(always)]
    pub fn mrgrd1(&mut self) -> Mrgrd1W<'_, MctlSpec> {
        Mrgrd1W::new(self, 12)
    }
    #[doc = "Bit 16 - Mass Erase (Erase All) Acknowledge"]
    #[inline(always)]
    pub fn ersaack(&mut self) -> ErsaackW<'_, MctlSpec> {
        ErsaackW::new(self, 16)
    }
    #[doc = "Bit 19 - Scan Observability Control"]
    #[inline(always)]
    pub fn scan_obs(&mut self) -> ScanObsW<'_, MctlSpec> {
        ScanObsW::new(self, 19)
    }
    #[doc = "Bit 20 - BIST IP Control"]
    #[inline(always)]
    pub fn bist_ctl(&mut self) -> BistCtlW<'_, MctlSpec> {
        BistCtlW::new(self, 20)
    }
    #[doc = "Bit 21 - SMWR IP Control"]
    #[inline(always)]
    pub fn smwr_ctl(&mut self) -> SmwrCtlW<'_, MctlSpec> {
        SmwrCtlW::new(self, 21)
    }
    #[doc = "Bit 24 - Salvage Disable"]
    #[inline(always)]
    pub fn salv_dis(&mut self) -> SalvDisW<'_, MctlSpec> {
        SalvDisW::new(self, 24)
    }
    #[doc = "Bit 25 - SOC ECC Control"]
    #[inline(always)]
    pub fn soc_ecc_ctl(&mut self) -> SocEccCtlW<'_, MctlSpec> {
        SocEccCtlW::new(self, 25)
    }
    #[doc = "Bit 26 - FMU ECC Control"]
    #[inline(always)]
    pub fn fmu_ecc_ctl(&mut self) -> FmuEccCtlW<'_, MctlSpec> {
        FmuEccCtlW::new(self, 26)
    }
    #[doc = "Bit 29 - BIST Power Mode Disable"]
    #[inline(always)]
    pub fn bist_pwr_dis(&mut self) -> BistPwrDisW<'_, MctlSpec> {
        BistPwrDisW::new(self, 29)
    }
    #[doc = "Bit 31 - Oscillator control"]
    #[inline(always)]
    pub fn osc_h(&mut self) -> OscHW<'_, MctlSpec> {
        OscHW::new(self, 31)
    }
}
#[doc = "FMU Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MctlSpec;
impl crate::RegisterSpec for MctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mctl::R`](R) reader structure"]
impl crate::Readable for MctlSpec {}
#[doc = "`write(|w| ..)` method takes [`mctl::W`](W) writer structure"]
impl crate::Writable for MctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MCTL to value 0x2130_48fd"]
impl crate::Resettable for MctlSpec {
    const RESET_VALUE: u32 = 0x2130_48fd;
}
