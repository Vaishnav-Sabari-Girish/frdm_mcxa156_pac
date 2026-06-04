#[doc = "Register `CMD_CHECK` reader"]
pub type R = crate::R<CmdCheckSpec>;
#[doc = "Phrase Alignment Fail\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AlignfailPhr {
    #[doc = "0: The address is phrase-aligned"]
    Zz149 = 0,
    #[doc = "1: The address is not phrase-aligned"]
    Zz150 = 1,
}
impl From<AlignfailPhr> for bool {
    #[inline(always)]
    fn from(variant: AlignfailPhr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALIGNFAIL_PHR` reader - Phrase Alignment Fail"]
pub type AlignfailPhrR = crate::BitReader<AlignfailPhr>;
impl AlignfailPhrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AlignfailPhr {
        match self.bits {
            false => AlignfailPhr::Zz149,
            true => AlignfailPhr::Zz150,
        }
    }
    #[doc = "The address is phrase-aligned"]
    #[inline(always)]
    pub fn is_zz149(&self) -> bool {
        *self == AlignfailPhr::Zz149
    }
    #[doc = "The address is not phrase-aligned"]
    #[inline(always)]
    pub fn is_zz150(&self) -> bool {
        *self == AlignfailPhr::Zz150
    }
}
#[doc = "Page Alignment Fail\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AlignfailPg {
    #[doc = "0: The address is page-aligned"]
    Zz147 = 0,
    #[doc = "1: The address is not page-aligned"]
    Zz148 = 1,
}
impl From<AlignfailPg> for bool {
    #[inline(always)]
    fn from(variant: AlignfailPg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALIGNFAIL_PG` reader - Page Alignment Fail"]
pub type AlignfailPgR = crate::BitReader<AlignfailPg>;
impl AlignfailPgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AlignfailPg {
        match self.bits {
            false => AlignfailPg::Zz147,
            true => AlignfailPg::Zz148,
        }
    }
    #[doc = "The address is page-aligned"]
    #[inline(always)]
    pub fn is_zz147(&self) -> bool {
        *self == AlignfailPg::Zz147
    }
    #[doc = "The address is not page-aligned"]
    #[inline(always)]
    pub fn is_zz148(&self) -> bool {
        *self == AlignfailPg::Zz148
    }
}
#[doc = "Sector Alignment Fail\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AlignfailScr {
    #[doc = "0: The address is sector-aligned"]
    Zz145 = 0,
    #[doc = "1: The address is not sector-aligned"]
    Zz146 = 1,
}
impl From<AlignfailScr> for bool {
    #[inline(always)]
    fn from(variant: AlignfailScr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALIGNFAIL_SCR` reader - Sector Alignment Fail"]
pub type AlignfailScrR = crate::BitReader<AlignfailScr>;
impl AlignfailScrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AlignfailScr {
        match self.bits {
            false => AlignfailScr::Zz145,
            true => AlignfailScr::Zz146,
        }
    }
    #[doc = "The address is sector-aligned"]
    #[inline(always)]
    pub fn is_zz145(&self) -> bool {
        *self == AlignfailScr::Zz145
    }
    #[doc = "The address is not sector-aligned"]
    #[inline(always)]
    pub fn is_zz146(&self) -> bool {
        *self == AlignfailScr::Zz146
    }
}
#[doc = "Block Alignment Fail\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AlignfailBlk {
    #[doc = "0: The address is block-aligned"]
    Zz143 = 0,
    #[doc = "1: The address is not block-aligned"]
    Zz144 = 1,
}
impl From<AlignfailBlk> for bool {
    #[inline(always)]
    fn from(variant: AlignfailBlk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALIGNFAIL_BLK` reader - Block Alignment Fail"]
pub type AlignfailBlkR = crate::BitReader<AlignfailBlk>;
impl AlignfailBlkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AlignfailBlk {
        match self.bits {
            false => AlignfailBlk::Zz143,
            true => AlignfailBlk::Zz144,
        }
    }
    #[doc = "The address is block-aligned"]
    #[inline(always)]
    pub fn is_zz143(&self) -> bool {
        *self == AlignfailBlk::Zz143
    }
    #[doc = "The address is not block-aligned"]
    #[inline(always)]
    pub fn is_zz144(&self) -> bool {
        *self == AlignfailBlk::Zz144
    }
}
#[doc = "Address Fail\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AddrFail {
    #[doc = "0: The address is within the flash or IFR address space"]
    Zz141 = 0,
    #[doc = "1: The address is outside the flash or IFR address space"]
    Zz142 = 1,
}
impl From<AddrFail> for bool {
    #[inline(always)]
    fn from(variant: AddrFail) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDR_FAIL` reader - Address Fail"]
pub type AddrFailR = crate::BitReader<AddrFail>;
impl AddrFailR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AddrFail {
        match self.bits {
            false => AddrFail::Zz141,
            true => AddrFail::Zz142,
        }
    }
    #[doc = "The address is within the flash or IFR address space"]
    #[inline(always)]
    pub fn is_zz141(&self) -> bool {
        *self == AddrFail::Zz141
    }
    #[doc = "The address is outside the flash or IFR address space"]
    #[inline(always)]
    pub fn is_zz142(&self) -> bool {
        *self == AddrFail::Zz142
    }
}
#[doc = "IFR Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IfrCmd {
    #[doc = "0: The command operates on a main flash address"]
    Zz139 = 0,
    #[doc = "1: The command operates on an IFR address"]
    Zz140 = 1,
}
impl From<IfrCmd> for bool {
    #[inline(always)]
    fn from(variant: IfrCmd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IFR_CMD` reader - IFR Command"]
pub type IfrCmdR = crate::BitReader<IfrCmd>;
impl IfrCmdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IfrCmd {
        match self.bits {
            false => IfrCmd::Zz139,
            true => IfrCmd::Zz140,
        }
    }
    #[doc = "The command operates on a main flash address"]
    #[inline(always)]
    pub fn is_zz139(&self) -> bool {
        *self == IfrCmd::Zz139
    }
    #[doc = "The command operates on an IFR address"]
    #[inline(always)]
    pub fn is_zz140(&self) -> bool {
        *self == IfrCmd::Zz140
    }
}
#[doc = "All Blocks Command\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AllCmd {
    #[doc = "0: The command operates on a single flash block"]
    Zz137 = 0,
    #[doc = "1: The command operates on all flash blocks"]
    Zz138 = 1,
}
impl From<AllCmd> for bool {
    #[inline(always)]
    fn from(variant: AllCmd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALL_CMD` reader - All Blocks Command"]
pub type AllCmdR = crate::BitReader<AllCmd>;
impl AllCmdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AllCmd {
        match self.bits {
            false => AllCmd::Zz137,
            true => AllCmd::Zz138,
        }
    }
    #[doc = "The command operates on a single flash block"]
    #[inline(always)]
    pub fn is_zz137(&self) -> bool {
        *self == AllCmd::Zz137
    }
    #[doc = "The command operates on all flash blocks"]
    #[inline(always)]
    pub fn is_zz138(&self) -> bool {
        *self == AllCmd::Zz138
    }
}
#[doc = "Address Range Fail\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RangeFail {
    #[doc = "0: The address range is valid"]
    Zz135 = 0,
    #[doc = "1: The address range is invalid"]
    Zz136 = 1,
}
impl From<RangeFail> for bool {
    #[inline(always)]
    fn from(variant: RangeFail) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RANGE_FAIL` reader - Address Range Fail"]
pub type RangeFailR = crate::BitReader<RangeFail>;
impl RangeFailR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RangeFail {
        match self.bits {
            false => RangeFail::Zz135,
            true => RangeFail::Zz136,
        }
    }
    #[doc = "The address range is valid"]
    #[inline(always)]
    pub fn is_zz135(&self) -> bool {
        *self == RangeFail::Zz135
    }
    #[doc = "The address range is invalid"]
    #[inline(always)]
    pub fn is_zz136(&self) -> bool {
        *self == RangeFail::Zz136
    }
}
#[doc = "Sector Alignment Check\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScrAlignChk {
    #[doc = "0: No sector alignment check"]
    Zz133 = 0,
    #[doc = "1: Sector alignment check"]
    Zz134 = 1,
}
impl From<ScrAlignChk> for bool {
    #[inline(always)]
    fn from(variant: ScrAlignChk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCR_ALIGN_CHK` reader - Sector Alignment Check"]
pub type ScrAlignChkR = crate::BitReader<ScrAlignChk>;
impl ScrAlignChkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ScrAlignChk {
        match self.bits {
            false => ScrAlignChk::Zz133,
            true => ScrAlignChk::Zz134,
        }
    }
    #[doc = "No sector alignment check"]
    #[inline(always)]
    pub fn is_zz133(&self) -> bool {
        *self == ScrAlignChk::Zz133
    }
    #[doc = "Sector alignment check"]
    #[inline(always)]
    pub fn is_zz134(&self) -> bool {
        *self == ScrAlignChk::Zz134
    }
}
#[doc = "Option Check Fail\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OptionFail {
    #[doc = "0: Option check passes for read command or command is not a read command"]
    Zz131 = 0,
    #[doc = "1: Option check fails for read command"]
    Zz132 = 1,
}
impl From<OptionFail> for bool {
    #[inline(always)]
    fn from(variant: OptionFail) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPTION_FAIL` reader - Option Check Fail"]
pub type OptionFailR = crate::BitReader<OptionFail>;
impl OptionFailR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OptionFail {
        match self.bits {
            false => OptionFail::Zz131,
            true => OptionFail::Zz132,
        }
    }
    #[doc = "Option check passes for read command or command is not a read command"]
    #[inline(always)]
    pub fn is_zz131(&self) -> bool {
        *self == OptionFail::Zz131
    }
    #[doc = "Option check fails for read command"]
    #[inline(always)]
    pub fn is_zz132(&self) -> bool {
        *self == OptionFail::Zz132
    }
}
#[doc = "Illegal Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IllegalCmd {
    #[doc = "0: Command is legal"]
    Zz129 = 0,
    #[doc = "1: Command is illegal"]
    Zz130 = 1,
}
impl From<IllegalCmd> for bool {
    #[inline(always)]
    fn from(variant: IllegalCmd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ILLEGAL_CMD` reader - Illegal Command"]
pub type IllegalCmdR = crate::BitReader<IllegalCmd>;
impl IllegalCmdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IllegalCmd {
        match self.bits {
            false => IllegalCmd::Zz129,
            true => IllegalCmd::Zz130,
        }
    }
    #[doc = "Command is legal"]
    #[inline(always)]
    pub fn is_zz129(&self) -> bool {
        *self == IllegalCmd::Zz129
    }
    #[doc = "Command is illegal"]
    #[inline(always)]
    pub fn is_zz130(&self) -> bool {
        *self == IllegalCmd::Zz130
    }
}
impl R {
    #[doc = "Bit 0 - Phrase Alignment Fail"]
    #[inline(always)]
    pub fn alignfail_phr(&self) -> AlignfailPhrR {
        AlignfailPhrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Page Alignment Fail"]
    #[inline(always)]
    pub fn alignfail_pg(&self) -> AlignfailPgR {
        AlignfailPgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Sector Alignment Fail"]
    #[inline(always)]
    pub fn alignfail_scr(&self) -> AlignfailScrR {
        AlignfailScrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Block Alignment Fail"]
    #[inline(always)]
    pub fn alignfail_blk(&self) -> AlignfailBlkR {
        AlignfailBlkR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Address Fail"]
    #[inline(always)]
    pub fn addr_fail(&self) -> AddrFailR {
        AddrFailR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IFR Command"]
    #[inline(always)]
    pub fn ifr_cmd(&self) -> IfrCmdR {
        IfrCmdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - All Blocks Command"]
    #[inline(always)]
    pub fn all_cmd(&self) -> AllCmdR {
        AllCmdR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Address Range Fail"]
    #[inline(always)]
    pub fn range_fail(&self) -> RangeFailR {
        RangeFailR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Sector Alignment Check"]
    #[inline(always)]
    pub fn scr_align_chk(&self) -> ScrAlignChkR {
        ScrAlignChkR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Option Check Fail"]
    #[inline(always)]
    pub fn option_fail(&self) -> OptionFailR {
        OptionFailR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Illegal Command"]
    #[inline(always)]
    pub fn illegal_cmd(&self) -> IllegalCmdR {
        IllegalCmdR::new(((self.bits >> 10) & 1) != 0)
    }
}
#[doc = "FMU Command Check Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd_check::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdCheckSpec;
impl crate::RegisterSpec for CmdCheckSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd_check::R`](R) reader structure"]
impl crate::Readable for CmdCheckSpec {}
#[doc = "`reset()` method sets CMD_CHECK to value 0x40"]
impl crate::Resettable for CmdCheckSpec {
    const RESET_VALUE: u32 = 0x40;
}
