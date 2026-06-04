#[doc = "Register `R_DFT_CTRL` reader"]
pub type R = crate::R<RDftCtrlSpec>;
#[doc = "Register `R_DFT_CTRL` writer"]
pub type W = crate::W<RDftCtrlSpec>;
#[doc = "DFT XADR Pattern\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DftXadr {
    #[doc = "0: XADR fixed, no change at all"]
    Zz252 = 0,
    #[doc = "1: XADR increased by 1 after row. For READ operation, XADR increases by 1 after reading the last word of row. For PROG operation, XADR increases by 1 after NVSTR falls."]
    Zz253 = 1,
    #[doc = "2: XADR increased for diagonal. For PROG-DIAGONAL operation, XADR is increased to create diagonal pattern."]
    Zz254 = 2,
    #[doc = "3: XADR increased by sector. During ERASE operation, XADR increased by number of rows in a sector when NVSTR falls."]
    Zz255 = 3,
    #[doc = "4: XADR inversed. XADR is inversed after reading one word or after programming one row when NVSTR falls."]
    Zz256 = 4,
    #[doc = "5: XADR increased by 2 after row. For READ operation, XADR is increased by 2 after reading the last word of a row. For PROG operation, XADR is increased by 2 when NVSTR falls."]
    Zz257 = 5,
    #[doc = "6: XADR\\[0\\] inversed. XADR\\[0\\] is inversed after reading one word or after programming one row when NVSTR falls."]
    Zz258 = 6,
    #[doc = "7: XADR increased by 1. For READ operations only, XADR increased by 1 after each read cycle."]
    Zz259 = 7,
    #[doc = "8: XADR decreased by 1 after row. For READ operations only, XADR is decreased by 1 after YADR decreases to 0."]
    Zz260 = 8,
    #[doc = "9: XADR decreased by 1. For READ operations only, XADR is decreased by 1 after each read cycle."]
    Zz261 = 9,
}
impl From<DftXadr> for u8 {
    #[inline(always)]
    fn from(variant: DftXadr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DftXadr {
    type Ux = u8;
}
impl crate::IsEnum for DftXadr {}
#[doc = "Field `DFT_XADR` reader - DFT XADR Pattern"]
pub type DftXadrR = crate::FieldReader<DftXadr>;
impl DftXadrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DftXadr> {
        match self.bits {
            0 => Some(DftXadr::Zz252),
            1 => Some(DftXadr::Zz253),
            2 => Some(DftXadr::Zz254),
            3 => Some(DftXadr::Zz255),
            4 => Some(DftXadr::Zz256),
            5 => Some(DftXadr::Zz257),
            6 => Some(DftXadr::Zz258),
            7 => Some(DftXadr::Zz259),
            8 => Some(DftXadr::Zz260),
            9 => Some(DftXadr::Zz261),
            _ => None,
        }
    }
    #[doc = "XADR fixed, no change at all"]
    #[inline(always)]
    pub fn is_zz252(&self) -> bool {
        *self == DftXadr::Zz252
    }
    #[doc = "XADR increased by 1 after row. For READ operation, XADR increases by 1 after reading the last word of row. For PROG operation, XADR increases by 1 after NVSTR falls."]
    #[inline(always)]
    pub fn is_zz253(&self) -> bool {
        *self == DftXadr::Zz253
    }
    #[doc = "XADR increased for diagonal. For PROG-DIAGONAL operation, XADR is increased to create diagonal pattern."]
    #[inline(always)]
    pub fn is_zz254(&self) -> bool {
        *self == DftXadr::Zz254
    }
    #[doc = "XADR increased by sector. During ERASE operation, XADR increased by number of rows in a sector when NVSTR falls."]
    #[inline(always)]
    pub fn is_zz255(&self) -> bool {
        *self == DftXadr::Zz255
    }
    #[doc = "XADR inversed. XADR is inversed after reading one word or after programming one row when NVSTR falls."]
    #[inline(always)]
    pub fn is_zz256(&self) -> bool {
        *self == DftXadr::Zz256
    }
    #[doc = "XADR increased by 2 after row. For READ operation, XADR is increased by 2 after reading the last word of a row. For PROG operation, XADR is increased by 2 when NVSTR falls."]
    #[inline(always)]
    pub fn is_zz257(&self) -> bool {
        *self == DftXadr::Zz257
    }
    #[doc = "XADR\\[0\\] inversed. XADR\\[0\\] is inversed after reading one word or after programming one row when NVSTR falls."]
    #[inline(always)]
    pub fn is_zz258(&self) -> bool {
        *self == DftXadr::Zz258
    }
    #[doc = "XADR increased by 1. For READ operations only, XADR increased by 1 after each read cycle."]
    #[inline(always)]
    pub fn is_zz259(&self) -> bool {
        *self == DftXadr::Zz259
    }
    #[doc = "XADR decreased by 1 after row. For READ operations only, XADR is decreased by 1 after YADR decreases to 0."]
    #[inline(always)]
    pub fn is_zz260(&self) -> bool {
        *self == DftXadr::Zz260
    }
    #[doc = "XADR decreased by 1. For READ operations only, XADR is decreased by 1 after each read cycle."]
    #[inline(always)]
    pub fn is_zz261(&self) -> bool {
        *self == DftXadr::Zz261
    }
}
#[doc = "Field `DFT_XADR` writer - DFT XADR Pattern"]
pub type DftXadrW<'a, REG> = crate::FieldWriter<'a, REG, 4, DftXadr>;
impl<'a, REG> DftXadrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "XADR fixed, no change at all"]
    #[inline(always)]
    pub fn zz252(self) -> &'a mut crate::W<REG> {
        self.variant(DftXadr::Zz252)
    }
    #[doc = "XADR increased by 1 after row. For READ operation, XADR increases by 1 after reading the last word of row. For PROG operation, XADR increases by 1 after NVSTR falls."]
    #[inline(always)]
    pub fn zz253(self) -> &'a mut crate::W<REG> {
        self.variant(DftXadr::Zz253)
    }
    #[doc = "XADR increased for diagonal. For PROG-DIAGONAL operation, XADR is increased to create diagonal pattern."]
    #[inline(always)]
    pub fn zz254(self) -> &'a mut crate::W<REG> {
        self.variant(DftXadr::Zz254)
    }
    #[doc = "XADR increased by sector. During ERASE operation, XADR increased by number of rows in a sector when NVSTR falls."]
    #[inline(always)]
    pub fn zz255(self) -> &'a mut crate::W<REG> {
        self.variant(DftXadr::Zz255)
    }
    #[doc = "XADR inversed. XADR is inversed after reading one word or after programming one row when NVSTR falls."]
    #[inline(always)]
    pub fn zz256(self) -> &'a mut crate::W<REG> {
        self.variant(DftXadr::Zz256)
    }
    #[doc = "XADR increased by 2 after row. For READ operation, XADR is increased by 2 after reading the last word of a row. For PROG operation, XADR is increased by 2 when NVSTR falls."]
    #[inline(always)]
    pub fn zz257(self) -> &'a mut crate::W<REG> {
        self.variant(DftXadr::Zz257)
    }
    #[doc = "XADR\\[0\\] inversed. XADR\\[0\\] is inversed after reading one word or after programming one row when NVSTR falls."]
    #[inline(always)]
    pub fn zz258(self) -> &'a mut crate::W<REG> {
        self.variant(DftXadr::Zz258)
    }
    #[doc = "XADR increased by 1. For READ operations only, XADR increased by 1 after each read cycle."]
    #[inline(always)]
    pub fn zz259(self) -> &'a mut crate::W<REG> {
        self.variant(DftXadr::Zz259)
    }
    #[doc = "XADR decreased by 1 after row. For READ operations only, XADR is decreased by 1 after YADR decreases to 0."]
    #[inline(always)]
    pub fn zz260(self) -> &'a mut crate::W<REG> {
        self.variant(DftXadr::Zz260)
    }
    #[doc = "XADR decreased by 1. For READ operations only, XADR is decreased by 1 after each read cycle."]
    #[inline(always)]
    pub fn zz261(self) -> &'a mut crate::W<REG> {
        self.variant(DftXadr::Zz261)
    }
}
#[doc = "DFT YADR Pattern\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DftYadr {
    #[doc = "0: YADR fixed, no change at all"]
    Zz242 = 0,
    #[doc = "1: YADR for ICKBD. For PROG and READ operations, YADR changed to generate inverse checkerboard pattern."]
    Zz243 = 1,
    #[doc = "2: YADR for CKBD. For PROG and READ operations, YADR changed to generate checkerboard pattern."]
    Zz244 = 2,
    #[doc = "3: YADR increased by 1. For READ operations, YADR increased by 1 after each read cycle. For PROG operations, YADR increased by 1 after YE falls."]
    Zz245 = 3,
    #[doc = "4: YADR increased for diagonal. For PROG-DIAGONAL operation, YADR is increased to create diagonal pattern."]
    Zz246 = 4,
    #[doc = "5: YADR inversed. YADR is inversed after reading one word or after programming one word when YE falls."]
    Zz247 = 5,
    #[doc = "6: YADR\\[0\\] inversed. YADR\\[0\\] is inversed after reading one word or after programming one word when YE falls."]
    Zz248 = 6,
    #[doc = "7: YADR increased by 1 after last row. For READ operations only, YADR is increased by 1 after XADR reaches last row."]
    Zz249 = 7,
    #[doc = "8: YADR decreased by 1. For READ operations only, YADR is decreased by 1 after each read cycle."]
    Zz250 = 8,
    #[doc = "9: YADR decreased by 1 after first row. For READ operations only, YADR is decreased by 1 after XADR decreases to 0."]
    Zz251 = 9,
}
impl From<DftYadr> for u8 {
    #[inline(always)]
    fn from(variant: DftYadr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DftYadr {
    type Ux = u8;
}
impl crate::IsEnum for DftYadr {}
#[doc = "Field `DFT_YADR` reader - DFT YADR Pattern"]
pub type DftYadrR = crate::FieldReader<DftYadr>;
impl DftYadrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DftYadr> {
        match self.bits {
            0 => Some(DftYadr::Zz242),
            1 => Some(DftYadr::Zz243),
            2 => Some(DftYadr::Zz244),
            3 => Some(DftYadr::Zz245),
            4 => Some(DftYadr::Zz246),
            5 => Some(DftYadr::Zz247),
            6 => Some(DftYadr::Zz248),
            7 => Some(DftYadr::Zz249),
            8 => Some(DftYadr::Zz250),
            9 => Some(DftYadr::Zz251),
            _ => None,
        }
    }
    #[doc = "YADR fixed, no change at all"]
    #[inline(always)]
    pub fn is_zz242(&self) -> bool {
        *self == DftYadr::Zz242
    }
    #[doc = "YADR for ICKBD. For PROG and READ operations, YADR changed to generate inverse checkerboard pattern."]
    #[inline(always)]
    pub fn is_zz243(&self) -> bool {
        *self == DftYadr::Zz243
    }
    #[doc = "YADR for CKBD. For PROG and READ operations, YADR changed to generate checkerboard pattern."]
    #[inline(always)]
    pub fn is_zz244(&self) -> bool {
        *self == DftYadr::Zz244
    }
    #[doc = "YADR increased by 1. For READ operations, YADR increased by 1 after each read cycle. For PROG operations, YADR increased by 1 after YE falls."]
    #[inline(always)]
    pub fn is_zz245(&self) -> bool {
        *self == DftYadr::Zz245
    }
    #[doc = "YADR increased for diagonal. For PROG-DIAGONAL operation, YADR is increased to create diagonal pattern."]
    #[inline(always)]
    pub fn is_zz246(&self) -> bool {
        *self == DftYadr::Zz246
    }
    #[doc = "YADR inversed. YADR is inversed after reading one word or after programming one word when YE falls."]
    #[inline(always)]
    pub fn is_zz247(&self) -> bool {
        *self == DftYadr::Zz247
    }
    #[doc = "YADR\\[0\\] inversed. YADR\\[0\\] is inversed after reading one word or after programming one word when YE falls."]
    #[inline(always)]
    pub fn is_zz248(&self) -> bool {
        *self == DftYadr::Zz248
    }
    #[doc = "YADR increased by 1 after last row. For READ operations only, YADR is increased by 1 after XADR reaches last row."]
    #[inline(always)]
    pub fn is_zz249(&self) -> bool {
        *self == DftYadr::Zz249
    }
    #[doc = "YADR decreased by 1. For READ operations only, YADR is decreased by 1 after each read cycle."]
    #[inline(always)]
    pub fn is_zz250(&self) -> bool {
        *self == DftYadr::Zz250
    }
    #[doc = "YADR decreased by 1 after first row. For READ operations only, YADR is decreased by 1 after XADR decreases to 0."]
    #[inline(always)]
    pub fn is_zz251(&self) -> bool {
        *self == DftYadr::Zz251
    }
}
#[doc = "Field `DFT_YADR` writer - DFT YADR Pattern"]
pub type DftYadrW<'a, REG> = crate::FieldWriter<'a, REG, 4, DftYadr>;
impl<'a, REG> DftYadrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "YADR fixed, no change at all"]
    #[inline(always)]
    pub fn zz242(self) -> &'a mut crate::W<REG> {
        self.variant(DftYadr::Zz242)
    }
    #[doc = "YADR for ICKBD. For PROG and READ operations, YADR changed to generate inverse checkerboard pattern."]
    #[inline(always)]
    pub fn zz243(self) -> &'a mut crate::W<REG> {
        self.variant(DftYadr::Zz243)
    }
    #[doc = "YADR for CKBD. For PROG and READ operations, YADR changed to generate checkerboard pattern."]
    #[inline(always)]
    pub fn zz244(self) -> &'a mut crate::W<REG> {
        self.variant(DftYadr::Zz244)
    }
    #[doc = "YADR increased by 1. For READ operations, YADR increased by 1 after each read cycle. For PROG operations, YADR increased by 1 after YE falls."]
    #[inline(always)]
    pub fn zz245(self) -> &'a mut crate::W<REG> {
        self.variant(DftYadr::Zz245)
    }
    #[doc = "YADR increased for diagonal. For PROG-DIAGONAL operation, YADR is increased to create diagonal pattern."]
    #[inline(always)]
    pub fn zz246(self) -> &'a mut crate::W<REG> {
        self.variant(DftYadr::Zz246)
    }
    #[doc = "YADR inversed. YADR is inversed after reading one word or after programming one word when YE falls."]
    #[inline(always)]
    pub fn zz247(self) -> &'a mut crate::W<REG> {
        self.variant(DftYadr::Zz247)
    }
    #[doc = "YADR\\[0\\] inversed. YADR\\[0\\] is inversed after reading one word or after programming one word when YE falls."]
    #[inline(always)]
    pub fn zz248(self) -> &'a mut crate::W<REG> {
        self.variant(DftYadr::Zz248)
    }
    #[doc = "YADR increased by 1 after last row. For READ operations only, YADR is increased by 1 after XADR reaches last row."]
    #[inline(always)]
    pub fn zz249(self) -> &'a mut crate::W<REG> {
        self.variant(DftYadr::Zz249)
    }
    #[doc = "YADR decreased by 1. For READ operations only, YADR is decreased by 1 after each read cycle."]
    #[inline(always)]
    pub fn zz250(self) -> &'a mut crate::W<REG> {
        self.variant(DftYadr::Zz250)
    }
    #[doc = "YADR decreased by 1 after first row. For READ operations only, YADR is decreased by 1 after XADR decreases to 0."]
    #[inline(always)]
    pub fn zz251(self) -> &'a mut crate::W<REG> {
        self.variant(DftYadr::Zz251)
    }
}
#[doc = "DFT Data Pattern\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DftData {
    #[doc = "0: CKBD pattern. For READ operations only, compare DOUT with checkerboard data pattern for each read cycle."]
    Zz232 = 0,
    #[doc = "1: ICKBD pattern. For READ operations only, compare DOUT with inverse checkerboard data pattern for each read cycle."]
    Zz233 = 1,
    #[doc = "2: Diagonal pattern. Used for READ operations only, compare DOUT to diagonal pattern."]
    Zz234 = 2,
    #[doc = "3: Fixed data pattern. For READ operations, comparison to DOUT for selected groups; refer to R_ADR_CTRL\\[GRPSEL\\] for modules with multiple groups."]
    Zz235 = 3,
    #[doc = "4: Random data pattern which will be generated based on the initial seed set in R_DATA; for READ operations, used for DOUT comparison of selected groups. For PROG operations, used to control DIN of selected groups."]
    Zz236 = 4,
    #[doc = "5: DOUT based pattern. For READ operations only, DOUT of selected group will be latched in R_DATA. If more than one group is selected in R_ADR_CTRL\\[GRPSEL\\], the group with the lower index will be latched."]
    Zz237 = 5,
    #[doc = "6: R_DATA based pattern. For READ operations, expected DOUT value of selected groups equals to R_DATA when XADR\\[0\\]==YADR\\[0\\] or ~R_DATA when XADR\\[0\\]!=YADR\\[0\\]. For PROG operations, DIN of selected groups equals R_DATA when XADR\\[0\\]==YADR\\[0\\] or ~R_DATA when XADR\\[0\\]!=YADR\\[0\\]."]
    Zz238 = 6,
    #[doc = "7: SCAN-IO pattern. For READ operations, control expected DOUT value of selected groups to SCAN-IO data pattern. For PROG operations, control DIN of selected groups to SCAN-IO data pattern."]
    Zz239 = 7,
    #[doc = "8: REPAIR set. For PROG operation to IFR1(7,1) and IFR1(7,2), R_REPAIR0_0 and R_REPAIR0_1 or R_REPAIR1_0 and R_REPAIR1_1 will control DIN. For READ operation on IFR1(7,1) and IFR1(7,2), DOUT will be compared against R_REPAIR0_0 and R_REPAIR0_1 or R_REPAIR1_0 andR_REPAIR1_1. When this option is selected, only one flash block can be selected."]
    Zz240 = 8,
    #[doc = "9: REPAIR load. For READ operation only, DOUT from IFR1(7,1) and IFR1(7,2) is loaded to R_REPAIR0 and R_REPAIR1."]
    Zz241 = 9,
}
impl From<DftData> for u8 {
    #[inline(always)]
    fn from(variant: DftData) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DftData {
    type Ux = u8;
}
impl crate::IsEnum for DftData {}
#[doc = "Field `DFT_DATA` reader - DFT Data Pattern"]
pub type DftDataR = crate::FieldReader<DftData>;
impl DftDataR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DftData> {
        match self.bits {
            0 => Some(DftData::Zz232),
            1 => Some(DftData::Zz233),
            2 => Some(DftData::Zz234),
            3 => Some(DftData::Zz235),
            4 => Some(DftData::Zz236),
            5 => Some(DftData::Zz237),
            6 => Some(DftData::Zz238),
            7 => Some(DftData::Zz239),
            8 => Some(DftData::Zz240),
            9 => Some(DftData::Zz241),
            _ => None,
        }
    }
    #[doc = "CKBD pattern. For READ operations only, compare DOUT with checkerboard data pattern for each read cycle."]
    #[inline(always)]
    pub fn is_zz232(&self) -> bool {
        *self == DftData::Zz232
    }
    #[doc = "ICKBD pattern. For READ operations only, compare DOUT with inverse checkerboard data pattern for each read cycle."]
    #[inline(always)]
    pub fn is_zz233(&self) -> bool {
        *self == DftData::Zz233
    }
    #[doc = "Diagonal pattern. Used for READ operations only, compare DOUT to diagonal pattern."]
    #[inline(always)]
    pub fn is_zz234(&self) -> bool {
        *self == DftData::Zz234
    }
    #[doc = "Fixed data pattern. For READ operations, comparison to DOUT for selected groups; refer to R_ADR_CTRL\\[GRPSEL\\] for modules with multiple groups."]
    #[inline(always)]
    pub fn is_zz235(&self) -> bool {
        *self == DftData::Zz235
    }
    #[doc = "Random data pattern which will be generated based on the initial seed set in R_DATA; for READ operations, used for DOUT comparison of selected groups. For PROG operations, used to control DIN of selected groups."]
    #[inline(always)]
    pub fn is_zz236(&self) -> bool {
        *self == DftData::Zz236
    }
    #[doc = "DOUT based pattern. For READ operations only, DOUT of selected group will be latched in R_DATA. If more than one group is selected in R_ADR_CTRL\\[GRPSEL\\], the group with the lower index will be latched."]
    #[inline(always)]
    pub fn is_zz237(&self) -> bool {
        *self == DftData::Zz237
    }
    #[doc = "R_DATA based pattern. For READ operations, expected DOUT value of selected groups equals to R_DATA when XADR\\[0\\]==YADR\\[0\\] or ~R_DATA when XADR\\[0\\]!=YADR\\[0\\]. For PROG operations, DIN of selected groups equals R_DATA when XADR\\[0\\]==YADR\\[0\\] or ~R_DATA when XADR\\[0\\]!=YADR\\[0\\]."]
    #[inline(always)]
    pub fn is_zz238(&self) -> bool {
        *self == DftData::Zz238
    }
    #[doc = "SCAN-IO pattern. For READ operations, control expected DOUT value of selected groups to SCAN-IO data pattern. For PROG operations, control DIN of selected groups to SCAN-IO data pattern."]
    #[inline(always)]
    pub fn is_zz239(&self) -> bool {
        *self == DftData::Zz239
    }
    #[doc = "REPAIR set. For PROG operation to IFR1(7,1) and IFR1(7,2), R_REPAIR0_0 and R_REPAIR0_1 or R_REPAIR1_0 and R_REPAIR1_1 will control DIN. For READ operation on IFR1(7,1) and IFR1(7,2), DOUT will be compared against R_REPAIR0_0 and R_REPAIR0_1 or R_REPAIR1_0 andR_REPAIR1_1. When this option is selected, only one flash block can be selected."]
    #[inline(always)]
    pub fn is_zz240(&self) -> bool {
        *self == DftData::Zz240
    }
    #[doc = "REPAIR load. For READ operation only, DOUT from IFR1(7,1) and IFR1(7,2) is loaded to R_REPAIR0 and R_REPAIR1."]
    #[inline(always)]
    pub fn is_zz241(&self) -> bool {
        *self == DftData::Zz241
    }
}
#[doc = "Field `DFT_DATA` writer - DFT Data Pattern"]
pub type DftDataW<'a, REG> = crate::FieldWriter<'a, REG, 4, DftData>;
impl<'a, REG> DftDataW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CKBD pattern. For READ operations only, compare DOUT with checkerboard data pattern for each read cycle."]
    #[inline(always)]
    pub fn zz232(self) -> &'a mut crate::W<REG> {
        self.variant(DftData::Zz232)
    }
    #[doc = "ICKBD pattern. For READ operations only, compare DOUT with inverse checkerboard data pattern for each read cycle."]
    #[inline(always)]
    pub fn zz233(self) -> &'a mut crate::W<REG> {
        self.variant(DftData::Zz233)
    }
    #[doc = "Diagonal pattern. Used for READ operations only, compare DOUT to diagonal pattern."]
    #[inline(always)]
    pub fn zz234(self) -> &'a mut crate::W<REG> {
        self.variant(DftData::Zz234)
    }
    #[doc = "Fixed data pattern. For READ operations, comparison to DOUT for selected groups; refer to R_ADR_CTRL\\[GRPSEL\\] for modules with multiple groups."]
    #[inline(always)]
    pub fn zz235(self) -> &'a mut crate::W<REG> {
        self.variant(DftData::Zz235)
    }
    #[doc = "Random data pattern which will be generated based on the initial seed set in R_DATA; for READ operations, used for DOUT comparison of selected groups. For PROG operations, used to control DIN of selected groups."]
    #[inline(always)]
    pub fn zz236(self) -> &'a mut crate::W<REG> {
        self.variant(DftData::Zz236)
    }
    #[doc = "DOUT based pattern. For READ operations only, DOUT of selected group will be latched in R_DATA. If more than one group is selected in R_ADR_CTRL\\[GRPSEL\\], the group with the lower index will be latched."]
    #[inline(always)]
    pub fn zz237(self) -> &'a mut crate::W<REG> {
        self.variant(DftData::Zz237)
    }
    #[doc = "R_DATA based pattern. For READ operations, expected DOUT value of selected groups equals to R_DATA when XADR\\[0\\]==YADR\\[0\\] or ~R_DATA when XADR\\[0\\]!=YADR\\[0\\]. For PROG operations, DIN of selected groups equals R_DATA when XADR\\[0\\]==YADR\\[0\\] or ~R_DATA when XADR\\[0\\]!=YADR\\[0\\]."]
    #[inline(always)]
    pub fn zz238(self) -> &'a mut crate::W<REG> {
        self.variant(DftData::Zz238)
    }
    #[doc = "SCAN-IO pattern. For READ operations, control expected DOUT value of selected groups to SCAN-IO data pattern. For PROG operations, control DIN of selected groups to SCAN-IO data pattern."]
    #[inline(always)]
    pub fn zz239(self) -> &'a mut crate::W<REG> {
        self.variant(DftData::Zz239)
    }
    #[doc = "REPAIR set. For PROG operation to IFR1(7,1) and IFR1(7,2), R_REPAIR0_0 and R_REPAIR0_1 or R_REPAIR1_0 and R_REPAIR1_1 will control DIN. For READ operation on IFR1(7,1) and IFR1(7,2), DOUT will be compared against R_REPAIR0_0 and R_REPAIR0_1 or R_REPAIR1_0 andR_REPAIR1_1. When this option is selected, only one flash block can be selected."]
    #[inline(always)]
    pub fn zz240(self) -> &'a mut crate::W<REG> {
        self.variant(DftData::Zz240)
    }
    #[doc = "REPAIR load. For READ operation only, DOUT from IFR1(7,1) and IFR1(7,2) is loaded to R_REPAIR0 and R_REPAIR1."]
    #[inline(always)]
    pub fn zz241(self) -> &'a mut crate::W<REG> {
        self.variant(DftData::Zz241)
    }
}
#[doc = "Data Compare Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CmpMask {
    #[doc = "0: Expected data is compared to DOUT"]
    Zz229 = 0,
    #[doc = "1: Expected data (only 0s are considered) are compared to DOUT"]
    Zz230 = 1,
    #[doc = "2: Expected data (only 1s are considered) are compared to DOUT"]
    Zz231 = 2,
}
impl From<CmpMask> for u8 {
    #[inline(always)]
    fn from(variant: CmpMask) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CmpMask {
    type Ux = u8;
}
impl crate::IsEnum for CmpMask {}
#[doc = "Field `CMP_MASK` reader - Data Compare Mask"]
pub type CmpMaskR = crate::FieldReader<CmpMask>;
impl CmpMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CmpMask> {
        match self.bits {
            0 => Some(CmpMask::Zz229),
            1 => Some(CmpMask::Zz230),
            2 => Some(CmpMask::Zz231),
            _ => None,
        }
    }
    #[doc = "Expected data is compared to DOUT"]
    #[inline(always)]
    pub fn is_zz229(&self) -> bool {
        *self == CmpMask::Zz229
    }
    #[doc = "Expected data (only 0s are considered) are compared to DOUT"]
    #[inline(always)]
    pub fn is_zz230(&self) -> bool {
        *self == CmpMask::Zz230
    }
    #[doc = "Expected data (only 1s are considered) are compared to DOUT"]
    #[inline(always)]
    pub fn is_zz231(&self) -> bool {
        *self == CmpMask::Zz231
    }
}
#[doc = "Field `CMP_MASK` writer - Data Compare Mask"]
pub type CmpMaskW<'a, REG> = crate::FieldWriter<'a, REG, 2, CmpMask>;
impl<'a, REG> CmpMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Expected data is compared to DOUT"]
    #[inline(always)]
    pub fn zz229(self) -> &'a mut crate::W<REG> {
        self.variant(CmpMask::Zz229)
    }
    #[doc = "Expected data (only 0s are considered) are compared to DOUT"]
    #[inline(always)]
    pub fn zz230(self) -> &'a mut crate::W<REG> {
        self.variant(CmpMask::Zz230)
    }
    #[doc = "Expected data (only 1s are considered) are compared to DOUT"]
    #[inline(always)]
    pub fn zz231(self) -> &'a mut crate::W<REG> {
        self.variant(CmpMask::Zz231)
    }
}
#[doc = "DFT Data Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DftDataSrc {
    #[doc = "0: {R_DATA_CTRL0,R_DATA_CTRL_EX\\[2:0\\],R_DATA_CTRL0,R_DATA_CTRL_EX\\[2:0\\],R_DATA_CTRL0,R_DATA_CTRL_EX\\[2:0\\],R_DATA_CTRL0} is used"]
    Zz227 = 0,
    #[doc = "1: {R_DATA_CTRL3,R_DATA_CTRL2_EX\\[2:0\\],R_DATA_CTRL2,R_DATA_CTRL1_EX\\[2:0\\],R_DATA_CTRL1,R_DATA_CTRL_EX\\[2:0\\],R_DATA_CTRL0} is used"]
    Zz228 = 1,
}
impl From<DftDataSrc> for bool {
    #[inline(always)]
    fn from(variant: DftDataSrc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DFT_DATA_SRC` reader - DFT Data Source"]
pub type DftDataSrcR = crate::BitReader<DftDataSrc>;
impl DftDataSrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DftDataSrc {
        match self.bits {
            false => DftDataSrc::Zz227,
            true => DftDataSrc::Zz228,
        }
    }
    #[doc = "{R_DATA_CTRL0,R_DATA_CTRL_EX\\[2:0\\],R_DATA_CTRL0,R_DATA_CTRL_EX\\[2:0\\],R_DATA_CTRL0,R_DATA_CTRL_EX\\[2:0\\],R_DATA_CTRL0} is used"]
    #[inline(always)]
    pub fn is_zz227(&self) -> bool {
        *self == DftDataSrc::Zz227
    }
    #[doc = "{R_DATA_CTRL3,R_DATA_CTRL2_EX\\[2:0\\],R_DATA_CTRL2,R_DATA_CTRL1_EX\\[2:0\\],R_DATA_CTRL1,R_DATA_CTRL_EX\\[2:0\\],R_DATA_CTRL0} is used"]
    #[inline(always)]
    pub fn is_zz228(&self) -> bool {
        *self == DftDataSrc::Zz228
    }
}
#[doc = "Field `DFT_DATA_SRC` writer - DFT Data Source"]
pub type DftDataSrcW<'a, REG> = crate::BitWriter<'a, REG, DftDataSrc>;
impl<'a, REG> DftDataSrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "{R_DATA_CTRL0,R_DATA_CTRL_EX\\[2:0\\],R_DATA_CTRL0,R_DATA_CTRL_EX\\[2:0\\],R_DATA_CTRL0,R_DATA_CTRL_EX\\[2:0\\],R_DATA_CTRL0} is used"]
    #[inline(always)]
    pub fn zz227(self) -> &'a mut crate::W<REG> {
        self.variant(DftDataSrc::Zz227)
    }
    #[doc = "{R_DATA_CTRL3,R_DATA_CTRL2_EX\\[2:0\\],R_DATA_CTRL2,R_DATA_CTRL1_EX\\[2:0\\],R_DATA_CTRL1,R_DATA_CTRL_EX\\[2:0\\],R_DATA_CTRL0} is used"]
    #[inline(always)]
    pub fn zz228(self) -> &'a mut crate::W<REG> {
        self.variant(DftDataSrc::Zz228)
    }
}
impl R {
    #[doc = "Bits 0:3 - DFT XADR Pattern"]
    #[inline(always)]
    pub fn dft_xadr(&self) -> DftXadrR {
        DftXadrR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - DFT YADR Pattern"]
    #[inline(always)]
    pub fn dft_yadr(&self) -> DftYadrR {
        DftYadrR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - DFT Data Pattern"]
    #[inline(always)]
    pub fn dft_data(&self) -> DftDataR {
        DftDataR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - Data Compare Mask"]
    #[inline(always)]
    pub fn cmp_mask(&self) -> CmpMaskR {
        CmpMaskR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - DFT Data Source"]
    #[inline(always)]
    pub fn dft_data_src(&self) -> DftDataSrcR {
        DftDataSrcR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - DFT XADR Pattern"]
    #[inline(always)]
    pub fn dft_xadr(&mut self) -> DftXadrW<'_, RDftCtrlSpec> {
        DftXadrW::new(self, 0)
    }
    #[doc = "Bits 4:7 - DFT YADR Pattern"]
    #[inline(always)]
    pub fn dft_yadr(&mut self) -> DftYadrW<'_, RDftCtrlSpec> {
        DftYadrW::new(self, 4)
    }
    #[doc = "Bits 8:11 - DFT Data Pattern"]
    #[inline(always)]
    pub fn dft_data(&mut self) -> DftDataW<'_, RDftCtrlSpec> {
        DftDataW::new(self, 8)
    }
    #[doc = "Bits 12:13 - Data Compare Mask"]
    #[inline(always)]
    pub fn cmp_mask(&mut self) -> CmpMaskW<'_, RDftCtrlSpec> {
        CmpMaskW::new(self, 12)
    }
    #[doc = "Bit 14 - DFT Data Source"]
    #[inline(always)]
    pub fn dft_data_src(&mut self) -> DftDataSrcW<'_, RDftCtrlSpec> {
        DftDataSrcW::new(self, 14)
    }
}
#[doc = "BIST DFT Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r_dft_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r_dft_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RDftCtrlSpec;
impl crate::RegisterSpec for RDftCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r_dft_ctrl::R`](R) reader structure"]
impl crate::Readable for RDftCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`r_dft_ctrl::W`](W) writer structure"]
impl crate::Writable for RDftCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R_DFT_CTRL to value 0"]
impl crate::Resettable for RDftCtrlSpec {}
