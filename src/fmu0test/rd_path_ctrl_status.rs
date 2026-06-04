#[doc = "Register `RD_PATH_CTRL_STATUS` reader"]
pub type R = crate::R<RdPathCtrlStatusSpec>;
#[doc = "Register `RD_PATH_CTRL_STATUS` writer"]
pub type W = crate::W<RdPathCtrlStatusSpec>;
#[doc = "Field `RD_CAPT` reader - Read Capture Clock Periods"]
pub type RdCaptR = crate::FieldReader;
#[doc = "Field `RD_CAPT` writer - Read Capture Clock Periods"]
pub type RdCaptW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SE_SIZE` reader - SE Clock Periods"]
pub type SeSizeR = crate::FieldReader;
#[doc = "Field `SE_SIZE` writer - SE Clock Periods"]
pub type SeSizeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "ECC Decoder Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EccEnableb {
    #[doc = "0: ECC decoder enabled (default)"]
    Zz189 = 0,
    #[doc = "1: ECC decoder disabled"]
    Zz190 = 1,
}
impl From<EccEnableb> for bool {
    #[inline(always)]
    fn from(variant: EccEnableb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECC_ENABLEB` reader - ECC Decoder Control"]
pub type EccEnablebR = crate::BitReader<EccEnableb>;
impl EccEnablebR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EccEnableb {
        match self.bits {
            false => EccEnableb::Zz189,
            true => EccEnableb::Zz190,
        }
    }
    #[doc = "ECC decoder enabled (default)"]
    #[inline(always)]
    pub fn is_zz189(&self) -> bool {
        *self == EccEnableb::Zz189
    }
    #[doc = "ECC decoder disabled"]
    #[inline(always)]
    pub fn is_zz190(&self) -> bool {
        *self == EccEnableb::Zz190
    }
}
#[doc = "Field `ECC_ENABLEB` writer - ECC Decoder Control"]
pub type EccEnablebW<'a, REG> = crate::BitWriter<'a, REG, EccEnableb>;
impl<'a, REG> EccEnablebW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ECC decoder enabled (default)"]
    #[inline(always)]
    pub fn zz189(self) -> &'a mut crate::W<REG> {
        self.variant(EccEnableb::Zz189)
    }
    #[doc = "ECC decoder disabled"]
    #[inline(always)]
    pub fn zz190(self) -> &'a mut crate::W<REG> {
        self.variant(EccEnableb::Zz190)
    }
}
#[doc = "MISR Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MisrEn {
    #[doc = "0: MISR option disabled (default)"]
    Zz187 = 0,
    #[doc = "1: MISR option enabled"]
    Zz188 = 1,
}
impl From<MisrEn> for bool {
    #[inline(always)]
    fn from(variant: MisrEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MISR_EN` reader - MISR Enable"]
pub type MisrEnR = crate::BitReader<MisrEn>;
impl MisrEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MisrEn {
        match self.bits {
            false => MisrEn::Zz187,
            true => MisrEn::Zz188,
        }
    }
    #[doc = "MISR option disabled (default)"]
    #[inline(always)]
    pub fn is_zz187(&self) -> bool {
        *self == MisrEn::Zz187
    }
    #[doc = "MISR option enabled"]
    #[inline(always)]
    pub fn is_zz188(&self) -> bool {
        *self == MisrEn::Zz188
    }
}
#[doc = "Field `MISR_EN` writer - MISR Enable"]
pub type MisrEnW<'a, REG> = crate::BitWriter<'a, REG, MisrEn>;
impl<'a, REG> MisrEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MISR option disabled (default)"]
    #[inline(always)]
    pub fn zz187(self) -> &'a mut crate::W<REG> {
        self.variant(MisrEn::Zz187)
    }
    #[doc = "MISR option enabled"]
    #[inline(always)]
    pub fn zz188(self) -> &'a mut crate::W<REG> {
        self.variant(MisrEn::Zz188)
    }
}
#[doc = "Copy Parity Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CpyParEn {
    #[doc = "0: Copy parity disabled"]
    Zz185 = 0,
    #[doc = "1: Copy parity enabled"]
    Zz186 = 1,
}
impl From<CpyParEn> for bool {
    #[inline(always)]
    fn from(variant: CpyParEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPY_PAR_EN` reader - Copy Parity Enable"]
pub type CpyParEnR = crate::BitReader<CpyParEn>;
impl CpyParEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CpyParEn {
        match self.bits {
            false => CpyParEn::Zz185,
            true => CpyParEn::Zz186,
        }
    }
    #[doc = "Copy parity disabled"]
    #[inline(always)]
    pub fn is_zz185(&self) -> bool {
        *self == CpyParEn::Zz185
    }
    #[doc = "Copy parity enabled"]
    #[inline(always)]
    pub fn is_zz186(&self) -> bool {
        *self == CpyParEn::Zz186
    }
}
#[doc = "Field `CPY_PAR_EN` writer - Copy Parity Enable"]
pub type CpyParEnW<'a, REG> = crate::BitWriter<'a, REG, CpyParEn>;
impl<'a, REG> CpyParEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Copy parity disabled"]
    #[inline(always)]
    pub fn zz185(self) -> &'a mut crate::W<REG> {
        self.variant(CpyParEn::Zz185)
    }
    #[doc = "Copy parity enabled"]
    #[inline(always)]
    pub fn zz186(self) -> &'a mut crate::W<REG> {
        self.variant(CpyParEn::Zz186)
    }
}
#[doc = "BIST Mux to SMW\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BistMuxToSmw {
    #[doc = "0: BIST drives fields"]
    Zz183 = 0,
    #[doc = "1: SMW registers drive fields"]
    Zz184 = 1,
}
impl From<BistMuxToSmw> for bool {
    #[inline(always)]
    fn from(variant: BistMuxToSmw) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BIST_MUX_TO_SMW` reader - BIST Mux to SMW"]
pub type BistMuxToSmwR = crate::BitReader<BistMuxToSmw>;
impl BistMuxToSmwR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BistMuxToSmw {
        match self.bits {
            false => BistMuxToSmw::Zz183,
            true => BistMuxToSmw::Zz184,
        }
    }
    #[doc = "BIST drives fields"]
    #[inline(always)]
    pub fn is_zz183(&self) -> bool {
        *self == BistMuxToSmw::Zz183
    }
    #[doc = "SMW registers drive fields"]
    #[inline(always)]
    pub fn is_zz184(&self) -> bool {
        *self == BistMuxToSmw::Zz184
    }
}
#[doc = "Field `BIST_MUX_TO_SMW` writer - BIST Mux to SMW"]
pub type BistMuxToSmwW<'a, REG> = crate::BitWriter<'a, REG, BistMuxToSmw>;
impl<'a, REG> BistMuxToSmwW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "BIST drives fields"]
    #[inline(always)]
    pub fn zz183(self) -> &'a mut crate::W<REG> {
        self.variant(BistMuxToSmw::Zz183)
    }
    #[doc = "SMW registers drive fields"]
    #[inline(always)]
    pub fn zz184(self) -> &'a mut crate::W<REG> {
        self.variant(BistMuxToSmw::Zz184)
    }
}
#[doc = "Field `AD_SET` reader - Multi-Cycle Address Setup Time"]
pub type AdSetR = crate::FieldReader;
#[doc = "Field `AD_SET` writer - Multi-Cycle Address Setup Time"]
pub type AdSetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Write Path Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WrPathEn {
    #[doc = "0: Writes to BIST setting registers driven by MM_WDATA"]
    Zz181 = 0,
    #[doc = "1: Writes to BIST setting registers driven by SMW_DIN"]
    Zz182 = 1,
}
impl From<WrPathEn> for bool {
    #[inline(always)]
    fn from(variant: WrPathEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WR_PATH_EN` reader - Write Path Enable"]
pub type WrPathEnR = crate::BitReader<WrPathEn>;
impl WrPathEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WrPathEn {
        match self.bits {
            false => WrPathEn::Zz181,
            true => WrPathEn::Zz182,
        }
    }
    #[doc = "Writes to BIST setting registers driven by MM_WDATA"]
    #[inline(always)]
    pub fn is_zz181(&self) -> bool {
        *self == WrPathEn::Zz181
    }
    #[doc = "Writes to BIST setting registers driven by SMW_DIN"]
    #[inline(always)]
    pub fn is_zz182(&self) -> bool {
        *self == WrPathEn::Zz182
    }
}
#[doc = "Field `WR_PATH_EN` writer - Write Path Enable"]
pub type WrPathEnW<'a, REG> = crate::BitWriter<'a, REG, WrPathEn>;
impl<'a, REG> WrPathEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writes to BIST setting registers driven by MM_WDATA"]
    #[inline(always)]
    pub fn zz181(self) -> &'a mut crate::W<REG> {
        self.variant(WrPathEn::Zz181)
    }
    #[doc = "Writes to BIST setting registers driven by SMW_DIN"]
    #[inline(always)]
    pub fn zz182(self) -> &'a mut crate::W<REG> {
        self.variant(WrPathEn::Zz182)
    }
}
#[doc = "Write Path ECC Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WrPathEccEn {
    #[doc = "0: ECC encoding disabled"]
    Zz179 = 0,
    #[doc = "1: ECC encoding enabled"]
    Zz180 = 1,
}
impl From<WrPathEccEn> for bool {
    #[inline(always)]
    fn from(variant: WrPathEccEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WR_PATH_ECC_EN` reader - Write Path ECC Enable"]
pub type WrPathEccEnR = crate::BitReader<WrPathEccEn>;
impl WrPathEccEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WrPathEccEn {
        match self.bits {
            false => WrPathEccEn::Zz179,
            true => WrPathEccEn::Zz180,
        }
    }
    #[doc = "ECC encoding disabled"]
    #[inline(always)]
    pub fn is_zz179(&self) -> bool {
        *self == WrPathEccEn::Zz179
    }
    #[doc = "ECC encoding enabled"]
    #[inline(always)]
    pub fn is_zz180(&self) -> bool {
        *self == WrPathEccEn::Zz180
    }
}
#[doc = "Field `WR_PATH_ECC_EN` writer - Write Path ECC Enable"]
pub type WrPathEccEnW<'a, REG> = crate::BitWriter<'a, REG, WrPathEccEn>;
impl<'a, REG> WrPathEccEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ECC encoding disabled"]
    #[inline(always)]
    pub fn zz179(self) -> &'a mut crate::W<REG> {
        self.variant(WrPathEccEn::Zz179)
    }
    #[doc = "ECC encoding enabled"]
    #[inline(always)]
    pub fn zz180(self) -> &'a mut crate::W<REG> {
        self.variant(WrPathEccEn::Zz180)
    }
}
#[doc = "Double-Bit Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DberrReg {
    #[doc = "0: Double-bit fault not detected"]
    Zz177 = 0,
    #[doc = "1: Double-bit fault detected on previous UINT flash read"]
    Zz178 = 1,
}
impl From<DberrReg> for bool {
    #[inline(always)]
    fn from(variant: DberrReg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBERR_REG` reader - Double-Bit Error"]
pub type DberrRegR = crate::BitReader<DberrReg>;
impl DberrRegR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DberrReg {
        match self.bits {
            false => DberrReg::Zz177,
            true => DberrReg::Zz178,
        }
    }
    #[doc = "Double-bit fault not detected"]
    #[inline(always)]
    pub fn is_zz177(&self) -> bool {
        *self == DberrReg::Zz177
    }
    #[doc = "Double-bit fault detected on previous UINT flash read"]
    #[inline(always)]
    pub fn is_zz178(&self) -> bool {
        *self == DberrReg::Zz178
    }
}
#[doc = "Single-Bit Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SberrReg {
    #[doc = "0: Single-bit fault not detected"]
    Zz175 = 0,
    #[doc = "1: Single-bit fault detected on previous UINT flash read"]
    Zz176 = 1,
}
impl From<SberrReg> for bool {
    #[inline(always)]
    fn from(variant: SberrReg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SBERR_REG` reader - Single-Bit Error"]
pub type SberrRegR = crate::BitReader<SberrReg>;
impl SberrRegR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SberrReg {
        match self.bits {
            false => SberrReg::Zz175,
            true => SberrReg::Zz176,
        }
    }
    #[doc = "Single-bit fault not detected"]
    #[inline(always)]
    pub fn is_zz175(&self) -> bool {
        *self == SberrReg::Zz175
    }
    #[doc = "Single-bit fault detected on previous UINT flash read"]
    #[inline(always)]
    pub fn is_zz176(&self) -> bool {
        *self == SberrReg::Zz176
    }
}
#[doc = "Copy Phrase Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CpyPhraseEn {
    #[doc = "0: Copy Flash read data disabled"]
    Zz173 = 0,
    #[doc = "1: Copy Flash read data enabled"]
    Zz174 = 1,
}
impl From<CpyPhraseEn> for bool {
    #[inline(always)]
    fn from(variant: CpyPhraseEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPY_PHRASE_EN` reader - Copy Phrase Enable"]
pub type CpyPhraseEnR = crate::BitReader<CpyPhraseEn>;
impl CpyPhraseEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CpyPhraseEn {
        match self.bits {
            false => CpyPhraseEn::Zz173,
            true => CpyPhraseEn::Zz174,
        }
    }
    #[doc = "Copy Flash read data disabled"]
    #[inline(always)]
    pub fn is_zz173(&self) -> bool {
        *self == CpyPhraseEn::Zz173
    }
    #[doc = "Copy Flash read data enabled"]
    #[inline(always)]
    pub fn is_zz174(&self) -> bool {
        *self == CpyPhraseEn::Zz174
    }
}
#[doc = "Field `CPY_PHRASE_EN` writer - Copy Phrase Enable"]
pub type CpyPhraseEnW<'a, REG> = crate::BitWriter<'a, REG, CpyPhraseEn>;
impl<'a, REG> CpyPhraseEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Copy Flash read data disabled"]
    #[inline(always)]
    pub fn zz173(self) -> &'a mut crate::W<REG> {
        self.variant(CpyPhraseEn::Zz173)
    }
    #[doc = "Copy Flash read data enabled"]
    #[inline(always)]
    pub fn zz174(self) -> &'a mut crate::W<REG> {
        self.variant(CpyPhraseEn::Zz174)
    }
}
#[doc = "SMW_ARRAY1_SMW0_SEL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SmwArray1Smw0Sel {
    #[doc = "0: Select block 0"]
    Zz171 = 0,
    #[doc = "1: Select block 1"]
    Zz172 = 1,
}
impl From<SmwArray1Smw0Sel> for bool {
    #[inline(always)]
    fn from(variant: SmwArray1Smw0Sel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMW_ARRAY1_SMW0_SEL` reader - SMW_ARRAY1_SMW0_SEL"]
pub type SmwArray1Smw0SelR = crate::BitReader<SmwArray1Smw0Sel>;
impl SmwArray1Smw0SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SmwArray1Smw0Sel {
        match self.bits {
            false => SmwArray1Smw0Sel::Zz171,
            true => SmwArray1Smw0Sel::Zz172,
        }
    }
    #[doc = "Select block 0"]
    #[inline(always)]
    pub fn is_zz171(&self) -> bool {
        *self == SmwArray1Smw0Sel::Zz171
    }
    #[doc = "Select block 1"]
    #[inline(always)]
    pub fn is_zz172(&self) -> bool {
        *self == SmwArray1Smw0Sel::Zz172
    }
}
#[doc = "Field `SMW_ARRAY1_SMW0_SEL` writer - SMW_ARRAY1_SMW0_SEL"]
pub type SmwArray1Smw0SelW<'a, REG> = crate::BitWriter<'a, REG, SmwArray1Smw0Sel>;
impl<'a, REG> SmwArray1Smw0SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Select block 0"]
    #[inline(always)]
    pub fn zz171(self) -> &'a mut crate::W<REG> {
        self.variant(SmwArray1Smw0Sel::Zz171)
    }
    #[doc = "Select block 1"]
    #[inline(always)]
    pub fn zz172(self) -> &'a mut crate::W<REG> {
        self.variant(SmwArray1Smw0Sel::Zz172)
    }
}
#[doc = "BIST ECC Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BistEccEn {
    #[doc = "0: ECC correction disabled"]
    Zz169 = 0,
    #[doc = "1: ECC correction enabled"]
    Zz170 = 1,
}
impl From<BistEccEn> for bool {
    #[inline(always)]
    fn from(variant: BistEccEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BIST_ECC_EN` reader - BIST ECC Enable"]
pub type BistEccEnR = crate::BitReader<BistEccEn>;
impl BistEccEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BistEccEn {
        match self.bits {
            false => BistEccEn::Zz169,
            true => BistEccEn::Zz170,
        }
    }
    #[doc = "ECC correction disabled"]
    #[inline(always)]
    pub fn is_zz169(&self) -> bool {
        *self == BistEccEn::Zz169
    }
    #[doc = "ECC correction enabled"]
    #[inline(always)]
    pub fn is_zz170(&self) -> bool {
        *self == BistEccEn::Zz170
    }
}
#[doc = "Field `BIST_ECC_EN` writer - BIST ECC Enable"]
pub type BistEccEnW<'a, REG> = crate::BitWriter<'a, REG, BistEccEn>;
impl<'a, REG> BistEccEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ECC correction disabled"]
    #[inline(always)]
    pub fn zz169(self) -> &'a mut crate::W<REG> {
        self.variant(BistEccEn::Zz169)
    }
    #[doc = "ECC correction enabled"]
    #[inline(always)]
    pub fn zz170(self) -> &'a mut crate::W<REG> {
        self.variant(BistEccEn::Zz170)
    }
}
#[doc = "Last Read\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LastRead {
    #[doc = "0: Latest read not last in multi-address operation"]
    Zz167 = 0,
    #[doc = "1: Latest read last in multi-address operation"]
    Zz168 = 1,
}
impl From<LastRead> for bool {
    #[inline(always)]
    fn from(variant: LastRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LAST_READ` reader - Last Read"]
pub type LastReadR = crate::BitReader<LastRead>;
impl LastReadR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LastRead {
        match self.bits {
            false => LastRead::Zz167,
            true => LastRead::Zz168,
        }
    }
    #[doc = "Latest read not last in multi-address operation"]
    #[inline(always)]
    pub fn is_zz167(&self) -> bool {
        *self == LastRead::Zz167
    }
    #[doc = "Latest read last in multi-address operation"]
    #[inline(always)]
    pub fn is_zz168(&self) -> bool {
        *self == LastRead::Zz168
    }
}
#[doc = "Field `LAST_READ` writer - Last Read"]
pub type LastReadW<'a, REG> = crate::BitWriter<'a, REG, LastRead>;
impl<'a, REG> LastReadW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Latest read not last in multi-address operation"]
    #[inline(always)]
    pub fn zz167(self) -> &'a mut crate::W<REG> {
        self.variant(LastRead::Zz167)
    }
    #[doc = "Latest read last in multi-address operation"]
    #[inline(always)]
    pub fn zz168(self) -> &'a mut crate::W<REG> {
        self.variant(LastRead::Zz168)
    }
}
impl R {
    #[doc = "Bits 0:7 - Read Capture Clock Periods"]
    #[inline(always)]
    pub fn rd_capt(&self) -> RdCaptR {
        RdCaptR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - SE Clock Periods"]
    #[inline(always)]
    pub fn se_size(&self) -> SeSizeR {
        SeSizeR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - ECC Decoder Control"]
    #[inline(always)]
    pub fn ecc_enableb(&self) -> EccEnablebR {
        EccEnablebR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - MISR Enable"]
    #[inline(always)]
    pub fn misr_en(&self) -> MisrEnR {
        MisrEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Copy Parity Enable"]
    #[inline(always)]
    pub fn cpy_par_en(&self) -> CpyParEnR {
        CpyParEnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - BIST Mux to SMW"]
    #[inline(always)]
    pub fn bist_mux_to_smw(&self) -> BistMuxToSmwR {
        BistMuxToSmwR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - Multi-Cycle Address Setup Time"]
    #[inline(always)]
    pub fn ad_set(&self) -> AdSetR {
        AdSetR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - Write Path Enable"]
    #[inline(always)]
    pub fn wr_path_en(&self) -> WrPathEnR {
        WrPathEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Write Path ECC Enable"]
    #[inline(always)]
    pub fn wr_path_ecc_en(&self) -> WrPathEccEnR {
        WrPathEccEnR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Double-Bit Error"]
    #[inline(always)]
    pub fn dberr_reg(&self) -> DberrRegR {
        DberrRegR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Single-Bit Error"]
    #[inline(always)]
    pub fn sberr_reg(&self) -> SberrRegR {
        SberrRegR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Copy Phrase Enable"]
    #[inline(always)]
    pub fn cpy_phrase_en(&self) -> CpyPhraseEnR {
        CpyPhraseEnR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - SMW_ARRAY1_SMW0_SEL"]
    #[inline(always)]
    pub fn smw_array1_smw0_sel(&self) -> SmwArray1Smw0SelR {
        SmwArray1Smw0SelR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - BIST ECC Enable"]
    #[inline(always)]
    pub fn bist_ecc_en(&self) -> BistEccEnR {
        BistEccEnR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Last Read"]
    #[inline(always)]
    pub fn last_read(&self) -> LastReadR {
        LastReadR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Read Capture Clock Periods"]
    #[inline(always)]
    pub fn rd_capt(&mut self) -> RdCaptW<'_, RdPathCtrlStatusSpec> {
        RdCaptW::new(self, 0)
    }
    #[doc = "Bits 8:15 - SE Clock Periods"]
    #[inline(always)]
    pub fn se_size(&mut self) -> SeSizeW<'_, RdPathCtrlStatusSpec> {
        SeSizeW::new(self, 8)
    }
    #[doc = "Bit 16 - ECC Decoder Control"]
    #[inline(always)]
    pub fn ecc_enableb(&mut self) -> EccEnablebW<'_, RdPathCtrlStatusSpec> {
        EccEnablebW::new(self, 16)
    }
    #[doc = "Bit 17 - MISR Enable"]
    #[inline(always)]
    pub fn misr_en(&mut self) -> MisrEnW<'_, RdPathCtrlStatusSpec> {
        MisrEnW::new(self, 17)
    }
    #[doc = "Bit 18 - Copy Parity Enable"]
    #[inline(always)]
    pub fn cpy_par_en(&mut self) -> CpyParEnW<'_, RdPathCtrlStatusSpec> {
        CpyParEnW::new(self, 18)
    }
    #[doc = "Bit 19 - BIST Mux to SMW"]
    #[inline(always)]
    pub fn bist_mux_to_smw(&mut self) -> BistMuxToSmwW<'_, RdPathCtrlStatusSpec> {
        BistMuxToSmwW::new(self, 19)
    }
    #[doc = "Bits 20:23 - Multi-Cycle Address Setup Time"]
    #[inline(always)]
    pub fn ad_set(&mut self) -> AdSetW<'_, RdPathCtrlStatusSpec> {
        AdSetW::new(self, 20)
    }
    #[doc = "Bit 24 - Write Path Enable"]
    #[inline(always)]
    pub fn wr_path_en(&mut self) -> WrPathEnW<'_, RdPathCtrlStatusSpec> {
        WrPathEnW::new(self, 24)
    }
    #[doc = "Bit 25 - Write Path ECC Enable"]
    #[inline(always)]
    pub fn wr_path_ecc_en(&mut self) -> WrPathEccEnW<'_, RdPathCtrlStatusSpec> {
        WrPathEccEnW::new(self, 25)
    }
    #[doc = "Bit 28 - Copy Phrase Enable"]
    #[inline(always)]
    pub fn cpy_phrase_en(&mut self) -> CpyPhraseEnW<'_, RdPathCtrlStatusSpec> {
        CpyPhraseEnW::new(self, 28)
    }
    #[doc = "Bit 29 - SMW_ARRAY1_SMW0_SEL"]
    #[inline(always)]
    pub fn smw_array1_smw0_sel(&mut self) -> SmwArray1Smw0SelW<'_, RdPathCtrlStatusSpec> {
        SmwArray1Smw0SelW::new(self, 29)
    }
    #[doc = "Bit 30 - BIST ECC Enable"]
    #[inline(always)]
    pub fn bist_ecc_en(&mut self) -> BistEccEnW<'_, RdPathCtrlStatusSpec> {
        BistEccEnW::new(self, 30)
    }
    #[doc = "Bit 31 - Last Read"]
    #[inline(always)]
    pub fn last_read(&mut self) -> LastReadW<'_, RdPathCtrlStatusSpec> {
        LastReadW::new(self, 31)
    }
}
#[doc = "Read Path Control and Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_path_ctrl_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rd_path_ctrl_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RdPathCtrlStatusSpec;
impl crate::RegisterSpec for RdPathCtrlStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_path_ctrl_status::R`](R) reader structure"]
impl crate::Readable for RdPathCtrlStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`rd_path_ctrl_status::W`](W) writer structure"]
impl crate::Writable for RdPathCtrlStatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RD_PATH_CTRL_STATUS to value 0"]
impl crate::Resettable for RdPathCtrlStatusSpec {}
