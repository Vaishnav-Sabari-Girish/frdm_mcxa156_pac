#[doc = "Register `FSTAT` reader"]
pub type R = crate::R<FstatSpec>;
#[doc = "Register `FSTAT` writer"]
pub type W = crate::W<FstatSpec>;
#[doc = "Command Fail Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fail {
    #[doc = "0: Error not detected"]
    Zz27 = 0,
    #[doc = "1: Error detected"]
    Zz28 = 1,
}
impl From<Fail> for bool {
    #[inline(always)]
    fn from(variant: Fail) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FAIL` reader - Command Fail Flag"]
pub type FailR = crate::BitReader<Fail>;
impl FailR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fail {
        match self.bits {
            false => Fail::Zz27,
            true => Fail::Zz28,
        }
    }
    #[doc = "Error not detected"]
    #[inline(always)]
    pub fn is_zz27(&self) -> bool {
        *self == Fail::Zz27
    }
    #[doc = "Error detected"]
    #[inline(always)]
    pub fn is_zz28(&self) -> bool {
        *self == Fail::Zz28
    }
}
#[doc = "Field `FAIL` writer - Command Fail Flag"]
pub type FailW<'a, REG> = crate::BitWriter<'a, REG, Fail>;
impl<'a, REG> FailW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Error not detected"]
    #[inline(always)]
    pub fn zz27(self) -> &'a mut crate::W<REG> {
        self.variant(Fail::Zz27)
    }
    #[doc = "Error detected"]
    #[inline(always)]
    pub fn zz28(self) -> &'a mut crate::W<REG> {
        self.variant(Fail::Zz28)
    }
}
#[doc = "Command Abort Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmdabt {
    #[doc = "0: No command abort detected"]
    Zz25 = 0,
    #[doc = "1: Command abort detected"]
    Zz26 = 1,
}
impl From<Cmdabt> for bool {
    #[inline(always)]
    fn from(variant: Cmdabt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMDABT` reader - Command Abort Flag"]
pub type CmdabtR = crate::BitReader<Cmdabt>;
impl CmdabtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmdabt {
        match self.bits {
            false => Cmdabt::Zz25,
            true => Cmdabt::Zz26,
        }
    }
    #[doc = "No command abort detected"]
    #[inline(always)]
    pub fn is_zz25(&self) -> bool {
        *self == Cmdabt::Zz25
    }
    #[doc = "Command abort detected"]
    #[inline(always)]
    pub fn is_zz26(&self) -> bool {
        *self == Cmdabt::Zz26
    }
}
#[doc = "Command Protection Violation Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pviol {
    #[doc = "0: No protection violation detected"]
    Zz23 = 0,
    #[doc = "1: Protection violation detected"]
    Zz24 = 1,
}
impl From<Pviol> for bool {
    #[inline(always)]
    fn from(variant: Pviol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PVIOL` reader - Command Protection Violation Flag"]
pub type PviolR = crate::BitReader<Pviol>;
impl PviolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pviol {
        match self.bits {
            false => Pviol::Zz23,
            true => Pviol::Zz24,
        }
    }
    #[doc = "No protection violation detected"]
    #[inline(always)]
    pub fn is_zz23(&self) -> bool {
        *self == Pviol::Zz23
    }
    #[doc = "Protection violation detected"]
    #[inline(always)]
    pub fn is_zz24(&self) -> bool {
        *self == Pviol::Zz24
    }
}
#[doc = "Field `PVIOL` writer - Command Protection Violation Flag"]
pub type PviolW<'a, REG> = crate::BitWriter1C<'a, REG, Pviol>;
impl<'a, REG> PviolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No protection violation detected"]
    #[inline(always)]
    pub fn zz23(self) -> &'a mut crate::W<REG> {
        self.variant(Pviol::Zz23)
    }
    #[doc = "Protection violation detected"]
    #[inline(always)]
    pub fn zz24(self) -> &'a mut crate::W<REG> {
        self.variant(Pviol::Zz24)
    }
}
#[doc = "Command Access Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Accerr {
    #[doc = "0: No access error detected"]
    Zz21 = 0,
    #[doc = "1: Access error detected"]
    Zz22 = 1,
}
impl From<Accerr> for bool {
    #[inline(always)]
    fn from(variant: Accerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACCERR` reader - Command Access Error Flag"]
pub type AccerrR = crate::BitReader<Accerr>;
impl AccerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Accerr {
        match self.bits {
            false => Accerr::Zz21,
            true => Accerr::Zz22,
        }
    }
    #[doc = "No access error detected"]
    #[inline(always)]
    pub fn is_zz21(&self) -> bool {
        *self == Accerr::Zz21
    }
    #[doc = "Access error detected"]
    #[inline(always)]
    pub fn is_zz22(&self) -> bool {
        *self == Accerr::Zz22
    }
}
#[doc = "Field `ACCERR` writer - Command Access Error Flag"]
pub type AccerrW<'a, REG> = crate::BitWriter1C<'a, REG, Accerr>;
impl<'a, REG> AccerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No access error detected"]
    #[inline(always)]
    pub fn zz21(self) -> &'a mut crate::W<REG> {
        self.variant(Accerr::Zz21)
    }
    #[doc = "Access error detected"]
    #[inline(always)]
    pub fn zz22(self) -> &'a mut crate::W<REG> {
        self.variant(Accerr::Zz22)
    }
}
#[doc = "Command Write Sequence Abort Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cwsabt {
    #[doc = "0: Command write sequence not aborted"]
    Zz19 = 0,
    #[doc = "1: Command write sequence aborted"]
    Zz20 = 1,
}
impl From<Cwsabt> for bool {
    #[inline(always)]
    fn from(variant: Cwsabt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CWSABT` reader - Command Write Sequence Abort Flag"]
pub type CwsabtR = crate::BitReader<Cwsabt>;
impl CwsabtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cwsabt {
        match self.bits {
            false => Cwsabt::Zz19,
            true => Cwsabt::Zz20,
        }
    }
    #[doc = "Command write sequence not aborted"]
    #[inline(always)]
    pub fn is_zz19(&self) -> bool {
        *self == Cwsabt::Zz19
    }
    #[doc = "Command write sequence aborted"]
    #[inline(always)]
    pub fn is_zz20(&self) -> bool {
        *self == Cwsabt::Zz20
    }
}
#[doc = "Command Complete Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccif {
    #[doc = "0: Flash command or initialization in progress"]
    Zz17 = 0,
    #[doc = "1: Flash command or initialization has completed"]
    Zz18 = 1,
}
impl From<Ccif> for bool {
    #[inline(always)]
    fn from(variant: Ccif) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCIF` reader - Command Complete Interrupt Flag"]
pub type CcifR = crate::BitReader<Ccif>;
impl CcifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccif {
        match self.bits {
            false => Ccif::Zz17,
            true => Ccif::Zz18,
        }
    }
    #[doc = "Flash command or initialization in progress"]
    #[inline(always)]
    pub fn is_zz17(&self) -> bool {
        *self == Ccif::Zz17
    }
    #[doc = "Flash command or initialization has completed"]
    #[inline(always)]
    pub fn is_zz18(&self) -> bool {
        *self == Ccif::Zz18
    }
}
#[doc = "Field `CCIF` writer - Command Complete Interrupt Flag"]
pub type CcifW<'a, REG> = crate::BitWriter1C<'a, REG, Ccif>;
impl<'a, REG> CcifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flash command or initialization in progress"]
    #[inline(always)]
    pub fn zz17(self) -> &'a mut crate::W<REG> {
        self.variant(Ccif::Zz17)
    }
    #[doc = "Flash command or initialization has completed"]
    #[inline(always)]
    pub fn zz18(self) -> &'a mut crate::W<REG> {
        self.variant(Ccif::Zz18)
    }
}
#[doc = "Command Protection Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmdprt {
    #[doc = "0: Secure, normal access"]
    Zz13 = 0,
    #[doc = "1: Secure, privileged access"]
    Zz14 = 1,
    #[doc = "2: Nonsecure, normal access"]
    Zz15 = 2,
    #[doc = "3: Nonsecure, privileged access"]
    Zz16 = 3,
}
impl From<Cmdprt> for u8 {
    #[inline(always)]
    fn from(variant: Cmdprt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmdprt {
    type Ux = u8;
}
impl crate::IsEnum for Cmdprt {}
#[doc = "Field `CMDPRT` reader - Command Protection Level"]
pub type CmdprtR = crate::FieldReader<Cmdprt>;
impl CmdprtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmdprt {
        match self.bits {
            0 => Cmdprt::Zz13,
            1 => Cmdprt::Zz14,
            2 => Cmdprt::Zz15,
            3 => Cmdprt::Zz16,
            _ => unreachable!(),
        }
    }
    #[doc = "Secure, normal access"]
    #[inline(always)]
    pub fn is_zz13(&self) -> bool {
        *self == Cmdprt::Zz13
    }
    #[doc = "Secure, privileged access"]
    #[inline(always)]
    pub fn is_zz14(&self) -> bool {
        *self == Cmdprt::Zz14
    }
    #[doc = "Nonsecure, normal access"]
    #[inline(always)]
    pub fn is_zz15(&self) -> bool {
        *self == Cmdprt::Zz15
    }
    #[doc = "Nonsecure, privileged access"]
    #[inline(always)]
    pub fn is_zz16(&self) -> bool {
        *self == Cmdprt::Zz16
    }
}
#[doc = "Command Protection Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmdp {
    #[doc = "0: Command protection level and domain ID are stale"]
    Zz11 = 0,
    #[doc = "1: Command protection level (CMDPRT) and domain ID (CMDDID) are set"]
    Zz12 = 1,
}
impl From<Cmdp> for bool {
    #[inline(always)]
    fn from(variant: Cmdp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMDP` reader - Command Protection Status Flag"]
pub type CmdpR = crate::BitReader<Cmdp>;
impl CmdpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmdp {
        match self.bits {
            false => Cmdp::Zz11,
            true => Cmdp::Zz12,
        }
    }
    #[doc = "Command protection level and domain ID are stale"]
    #[inline(always)]
    pub fn is_zz11(&self) -> bool {
        *self == Cmdp::Zz11
    }
    #[doc = "Command protection level (CMDPRT) and domain ID (CMDDID) are set"]
    #[inline(always)]
    pub fn is_zz12(&self) -> bool {
        *self == Cmdp::Zz12
    }
}
#[doc = "Field `CMDDID` reader - Command Domain ID"]
pub type CmddidR = crate::FieldReader;
#[doc = "Double Bit Fault Detect Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dfdif {
    #[doc = "0: Double bit fault not detected during a valid flash read access from the FMC"]
    Zz9 = 0,
    #[doc = "1: Double bit fault detected (or FCTRL\\[FDFD\\] is set) during a valid flash read access from the FMC"]
    Zz10 = 1,
}
impl From<Dfdif> for bool {
    #[inline(always)]
    fn from(variant: Dfdif) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DFDIF` reader - Double Bit Fault Detect Interrupt Flag"]
pub type DfdifR = crate::BitReader<Dfdif>;
impl DfdifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dfdif {
        match self.bits {
            false => Dfdif::Zz9,
            true => Dfdif::Zz10,
        }
    }
    #[doc = "Double bit fault not detected during a valid flash read access from the FMC"]
    #[inline(always)]
    pub fn is_zz9(&self) -> bool {
        *self == Dfdif::Zz9
    }
    #[doc = "Double bit fault detected (or FCTRL\\[FDFD\\] is set) during a valid flash read access from the FMC"]
    #[inline(always)]
    pub fn is_zz10(&self) -> bool {
        *self == Dfdif::Zz10
    }
}
#[doc = "Salvage Used for Erase operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SalvUsed {
    #[doc = "0: Salvage not used during the last operation"]
    Zz7 = 0,
    #[doc = "1: Salvage used during the last erase operation"]
    Zz8 = 1,
}
impl From<SalvUsed> for bool {
    #[inline(always)]
    fn from(variant: SalvUsed) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SALV_USED` reader - Salvage Used for Erase operation"]
pub type SalvUsedR = crate::BitReader<SalvUsed>;
impl SalvUsedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SalvUsed {
        match self.bits {
            false => SalvUsed::Zz7,
            true => SalvUsed::Zz8,
        }
    }
    #[doc = "Salvage not used during the last operation"]
    #[inline(always)]
    pub fn is_zz7(&self) -> bool {
        *self == SalvUsed::Zz7
    }
    #[doc = "Salvage used during the last erase operation"]
    #[inline(always)]
    pub fn is_zz8(&self) -> bool {
        *self == SalvUsed::Zz8
    }
}
#[doc = "Field `SALV_USED` writer - Salvage Used for Erase operation"]
pub type SalvUsedW<'a, REG> = crate::BitWriter<'a, REG, SalvUsed>;
impl<'a, REG> SalvUsedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Salvage not used during the last operation"]
    #[inline(always)]
    pub fn zz7(self) -> &'a mut crate::W<REG> {
        self.variant(SalvUsed::Zz7)
    }
    #[doc = "Salvage used during the last erase operation"]
    #[inline(always)]
    pub fn zz8(self) -> &'a mut crate::W<REG> {
        self.variant(SalvUsed::Zz8)
    }
}
#[doc = "Program-Erase Write Enable Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pewen {
    #[doc = "0: Writes are not enabled"]
    Zz3 = 0,
    #[doc = "1: Writes are enabled for one flash or IFR phrase (phrase programming, sector erase)"]
    Zz4 = 1,
    #[doc = "2: Writes are enabled for one flash or IFR page (page programming)"]
    Zz5 = 2,
}
impl From<Pewen> for u8 {
    #[inline(always)]
    fn from(variant: Pewen) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pewen {
    type Ux = u8;
}
impl crate::IsEnum for Pewen {}
#[doc = "Field `PEWEN` reader - Program-Erase Write Enable Control"]
pub type PewenR = crate::FieldReader<Pewen>;
impl PewenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pewen> {
        match self.bits {
            0 => Some(Pewen::Zz3),
            1 => Some(Pewen::Zz4),
            2 => Some(Pewen::Zz5),
            _ => None,
        }
    }
    #[doc = "Writes are not enabled"]
    #[inline(always)]
    pub fn is_zz3(&self) -> bool {
        *self == Pewen::Zz3
    }
    #[doc = "Writes are enabled for one flash or IFR phrase (phrase programming, sector erase)"]
    #[inline(always)]
    pub fn is_zz4(&self) -> bool {
        *self == Pewen::Zz4
    }
    #[doc = "Writes are enabled for one flash or IFR page (page programming)"]
    #[inline(always)]
    pub fn is_zz5(&self) -> bool {
        *self == Pewen::Zz5
    }
}
#[doc = "Field `PEWEN` writer - Program-Erase Write Enable Control"]
pub type PewenW<'a, REG> = crate::FieldWriter<'a, REG, 2, Pewen>;
impl<'a, REG> PewenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Writes are not enabled"]
    #[inline(always)]
    pub fn zz3(self) -> &'a mut crate::W<REG> {
        self.variant(Pewen::Zz3)
    }
    #[doc = "Writes are enabled for one flash or IFR phrase (phrase programming, sector erase)"]
    #[inline(always)]
    pub fn zz4(self) -> &'a mut crate::W<REG> {
        self.variant(Pewen::Zz4)
    }
    #[doc = "Writes are enabled for one flash or IFR page (page programming)"]
    #[inline(always)]
    pub fn zz5(self) -> &'a mut crate::W<REG> {
        self.variant(Pewen::Zz5)
    }
}
#[doc = "Program/Erase Ready Control/Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Perdy {
    #[doc = "0: Program or sector erase command operation is not stalled"]
    Zz1 = 0,
    #[doc = "1: Program or sector erase command operation is stalled"]
    Zz2 = 1,
}
impl From<Perdy> for bool {
    #[inline(always)]
    fn from(variant: Perdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERDY` reader - Program/Erase Ready Control/Status Flag"]
pub type PerdyR = crate::BitReader<Perdy>;
impl PerdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Perdy {
        match self.bits {
            false => Perdy::Zz1,
            true => Perdy::Zz2,
        }
    }
    #[doc = "Program or sector erase command operation is not stalled"]
    #[inline(always)]
    pub fn is_zz1(&self) -> bool {
        *self == Perdy::Zz1
    }
    #[doc = "Program or sector erase command operation is stalled"]
    #[inline(always)]
    pub fn is_zz2(&self) -> bool {
        *self == Perdy::Zz2
    }
}
#[doc = "Field `PERDY` writer - Program/Erase Ready Control/Status Flag"]
pub type PerdyW<'a, REG> = crate::BitWriter<'a, REG, Perdy>;
impl<'a, REG> PerdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Program or sector erase command operation is not stalled"]
    #[inline(always)]
    pub fn zz1(self) -> &'a mut crate::W<REG> {
        self.variant(Perdy::Zz1)
    }
    #[doc = "Program or sector erase command operation is stalled"]
    #[inline(always)]
    pub fn zz2(self) -> &'a mut crate::W<REG> {
        self.variant(Perdy::Zz2)
    }
}
impl R {
    #[doc = "Bit 0 - Command Fail Flag"]
    #[inline(always)]
    pub fn fail(&self) -> FailR {
        FailR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Command Abort Flag"]
    #[inline(always)]
    pub fn cmdabt(&self) -> CmdabtR {
        CmdabtR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Command Protection Violation Flag"]
    #[inline(always)]
    pub fn pviol(&self) -> PviolR {
        PviolR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Command Access Error Flag"]
    #[inline(always)]
    pub fn accerr(&self) -> AccerrR {
        AccerrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Command Write Sequence Abort Flag"]
    #[inline(always)]
    pub fn cwsabt(&self) -> CwsabtR {
        CwsabtR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Command Complete Interrupt Flag"]
    #[inline(always)]
    pub fn ccif(&self) -> CcifR {
        CcifR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Command Protection Level"]
    #[inline(always)]
    pub fn cmdprt(&self) -> CmdprtR {
        CmdprtR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 11 - Command Protection Status Flag"]
    #[inline(always)]
    pub fn cmdp(&self) -> CmdpR {
        CmdpR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - Command Domain ID"]
    #[inline(always)]
    pub fn cmddid(&self) -> CmddidR {
        CmddidR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Double Bit Fault Detect Interrupt Flag"]
    #[inline(always)]
    pub fn dfdif(&self) -> DfdifR {
        DfdifR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Salvage Used for Erase operation"]
    #[inline(always)]
    pub fn salv_used(&self) -> SalvUsedR {
        SalvUsedR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Program-Erase Write Enable Control"]
    #[inline(always)]
    pub fn pewen(&self) -> PewenR {
        PewenR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 31 - Program/Erase Ready Control/Status Flag"]
    #[inline(always)]
    pub fn perdy(&self) -> PerdyR {
        PerdyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command Fail Flag"]
    #[inline(always)]
    pub fn fail(&mut self) -> FailW<'_, FstatSpec> {
        FailW::new(self, 0)
    }
    #[doc = "Bit 4 - Command Protection Violation Flag"]
    #[inline(always)]
    pub fn pviol(&mut self) -> PviolW<'_, FstatSpec> {
        PviolW::new(self, 4)
    }
    #[doc = "Bit 5 - Command Access Error Flag"]
    #[inline(always)]
    pub fn accerr(&mut self) -> AccerrW<'_, FstatSpec> {
        AccerrW::new(self, 5)
    }
    #[doc = "Bit 7 - Command Complete Interrupt Flag"]
    #[inline(always)]
    pub fn ccif(&mut self) -> CcifW<'_, FstatSpec> {
        CcifW::new(self, 7)
    }
    #[doc = "Bit 17 - Salvage Used for Erase operation"]
    #[inline(always)]
    pub fn salv_used(&mut self) -> SalvUsedW<'_, FstatSpec> {
        SalvUsedW::new(self, 17)
    }
    #[doc = "Bits 24:25 - Program-Erase Write Enable Control"]
    #[inline(always)]
    pub fn pewen(&mut self) -> PewenW<'_, FstatSpec> {
        PewenW::new(self, 24)
    }
    #[doc = "Bit 31 - Program/Erase Ready Control/Status Flag"]
    #[inline(always)]
    pub fn perdy(&mut self) -> PerdyW<'_, FstatSpec> {
        PerdyW::new(self, 31)
    }
}
#[doc = "Flash Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FstatSpec;
impl crate::RegisterSpec for FstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fstat::R`](R) reader structure"]
impl crate::Readable for FstatSpec {}
#[doc = "`write(|w| ..)` method takes [`fstat::W`](W) writer structure"]
impl crate::Writable for FstatSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xb0;
}
#[doc = "`reset()` method sets FSTAT to value 0"]
impl crate::Resettable for FstatSpec {}
