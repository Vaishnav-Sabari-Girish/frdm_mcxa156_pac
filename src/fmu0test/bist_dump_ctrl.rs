#[doc = "Register `BIST_DUMP_CTRL` reader"]
pub type R = crate::R<BistDumpCtrlSpec>;
#[doc = "Register `BIST_DUMP_CTRL` writer"]
pub type W = crate::W<BistDumpCtrlSpec>;
#[doc = "BIST Done\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BistDone {
    #[doc = "0: The BIST (or data dump) is running"]
    Zz439 = 0,
    #[doc = "1: The BIST (or data dump) has completed"]
    Zz440 = 1,
}
impl From<BistDone> for bool {
    #[inline(always)]
    fn from(variant: BistDone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BIST_DONE` reader - BIST Done"]
pub type BistDoneR = crate::BitReader<BistDone>;
impl BistDoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BistDone {
        match self.bits {
            false => BistDone::Zz439,
            true => BistDone::Zz440,
        }
    }
    #[doc = "The BIST (or data dump) is running"]
    #[inline(always)]
    pub fn is_zz439(&self) -> bool {
        *self == BistDone::Zz439
    }
    #[doc = "The BIST (or data dump) has completed"]
    #[inline(always)]
    pub fn is_zz440(&self) -> bool {
        *self == BistDone::Zz440
    }
}
#[doc = "Field `BIST_DONE` writer - BIST Done"]
pub type BistDoneW<'a, REG> = crate::BitWriter<'a, REG, BistDone>;
impl<'a, REG> BistDoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The BIST (or data dump) is running"]
    #[inline(always)]
    pub fn zz439(self) -> &'a mut crate::W<REG> {
        self.variant(BistDone::Zz439)
    }
    #[doc = "The BIST (or data dump) has completed"]
    #[inline(always)]
    pub fn zz440(self) -> &'a mut crate::W<REG> {
        self.variant(BistDone::Zz440)
    }
}
#[doc = "BIST Fail\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BistFail {
    #[doc = "0: The last BIST operation completed successfully (or could not fail)"]
    Zz437 = 0,
    #[doc = "1: The last BIST operation failed"]
    Zz438 = 1,
}
impl From<BistFail> for bool {
    #[inline(always)]
    fn from(variant: BistFail) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BIST_FAIL` reader - BIST Fail"]
pub type BistFailR = crate::BitReader<BistFail>;
impl BistFailR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BistFail {
        match self.bits {
            false => BistFail::Zz437,
            true => BistFail::Zz438,
        }
    }
    #[doc = "The last BIST operation completed successfully (or could not fail)"]
    #[inline(always)]
    pub fn is_zz437(&self) -> bool {
        *self == BistFail::Zz437
    }
    #[doc = "The last BIST operation failed"]
    #[inline(always)]
    pub fn is_zz438(&self) -> bool {
        *self == BistFail::Zz438
    }
}
#[doc = "Field `BIST_FAIL` writer - BIST Fail"]
pub type BistFailW<'a, REG> = crate::BitWriter<'a, REG, BistFail>;
impl<'a, REG> BistFailW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The last BIST operation completed successfully (or could not fail)"]
    #[inline(always)]
    pub fn zz437(self) -> &'a mut crate::W<REG> {
        self.variant(BistFail::Zz437)
    }
    #[doc = "The last BIST operation failed"]
    #[inline(always)]
    pub fn zz438(self) -> &'a mut crate::W<REG> {
        self.variant(BistFail::Zz438)
    }
}
#[doc = "Field `DATADUMP` reader - Data Dump Enable"]
pub type DatadumpR = crate::BitReader;
#[doc = "Field `DATADUMP` writer - Data Dump Enable"]
pub type DatadumpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATADUMP_TRIG` reader - Data Dump Trigger"]
pub type DatadumpTrigR = crate::BitReader;
#[doc = "Field `DATADUMP_TRIG` writer - Data Dump Trigger"]
pub type DatadumpTrigW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Data Dump Pattern Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DatadumpPatt {
    #[doc = "0: All ones"]
    Zz433 = 0,
    #[doc = "1: All zeroes"]
    Zz434 = 1,
    #[doc = "2: Checkerboard"]
    Zz435 = 2,
    #[doc = "3: Inverse checkerboard"]
    Zz436 = 3,
}
impl From<DatadumpPatt> for u8 {
    #[inline(always)]
    fn from(variant: DatadumpPatt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DatadumpPatt {
    type Ux = u8;
}
impl crate::IsEnum for DatadumpPatt {}
#[doc = "Field `DATADUMP_PATT` reader - Data Dump Pattern Select"]
pub type DatadumpPattR = crate::FieldReader<DatadumpPatt>;
impl DatadumpPattR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DatadumpPatt {
        match self.bits {
            0 => DatadumpPatt::Zz433,
            1 => DatadumpPatt::Zz434,
            2 => DatadumpPatt::Zz435,
            3 => DatadumpPatt::Zz436,
            _ => unreachable!(),
        }
    }
    #[doc = "All ones"]
    #[inline(always)]
    pub fn is_zz433(&self) -> bool {
        *self == DatadumpPatt::Zz433
    }
    #[doc = "All zeroes"]
    #[inline(always)]
    pub fn is_zz434(&self) -> bool {
        *self == DatadumpPatt::Zz434
    }
    #[doc = "Checkerboard"]
    #[inline(always)]
    pub fn is_zz435(&self) -> bool {
        *self == DatadumpPatt::Zz435
    }
    #[doc = "Inverse checkerboard"]
    #[inline(always)]
    pub fn is_zz436(&self) -> bool {
        *self == DatadumpPatt::Zz436
    }
}
#[doc = "Field `DATADUMP_PATT` writer - Data Dump Pattern Select"]
pub type DatadumpPattW<'a, REG> = crate::FieldWriter<'a, REG, 2, DatadumpPatt, crate::Safe>;
impl<'a, REG> DatadumpPattW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "All ones"]
    #[inline(always)]
    pub fn zz433(self) -> &'a mut crate::W<REG> {
        self.variant(DatadumpPatt::Zz433)
    }
    #[doc = "All zeroes"]
    #[inline(always)]
    pub fn zz434(self) -> &'a mut crate::W<REG> {
        self.variant(DatadumpPatt::Zz434)
    }
    #[doc = "Checkerboard"]
    #[inline(always)]
    pub fn zz435(self) -> &'a mut crate::W<REG> {
        self.variant(DatadumpPatt::Zz435)
    }
    #[doc = "Inverse checkerboard"]
    #[inline(always)]
    pub fn zz436(self) -> &'a mut crate::W<REG> {
        self.variant(DatadumpPatt::Zz436)
    }
}
#[doc = "Data Dump Margin Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DatadumpMrgen {
    #[doc = "0: Normal read pulse shape"]
    Zz431 = 0,
    #[doc = "1: Margin read pulse shape"]
    Zz432 = 1,
}
impl From<DatadumpMrgen> for bool {
    #[inline(always)]
    fn from(variant: DatadumpMrgen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATADUMP_MRGEN` reader - Data Dump Margin Enable"]
pub type DatadumpMrgenR = crate::BitReader<DatadumpMrgen>;
impl DatadumpMrgenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DatadumpMrgen {
        match self.bits {
            false => DatadumpMrgen::Zz431,
            true => DatadumpMrgen::Zz432,
        }
    }
    #[doc = "Normal read pulse shape"]
    #[inline(always)]
    pub fn is_zz431(&self) -> bool {
        *self == DatadumpMrgen::Zz431
    }
    #[doc = "Margin read pulse shape"]
    #[inline(always)]
    pub fn is_zz432(&self) -> bool {
        *self == DatadumpMrgen::Zz432
    }
}
#[doc = "Field `DATADUMP_MRGEN` writer - Data Dump Margin Enable"]
pub type DatadumpMrgenW<'a, REG> = crate::BitWriter<'a, REG, DatadumpMrgen>;
impl<'a, REG> DatadumpMrgenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal read pulse shape"]
    #[inline(always)]
    pub fn zz431(self) -> &'a mut crate::W<REG> {
        self.variant(DatadumpMrgen::Zz431)
    }
    #[doc = "Margin read pulse shape"]
    #[inline(always)]
    pub fn zz432(self) -> &'a mut crate::W<REG> {
        self.variant(DatadumpMrgen::Zz432)
    }
}
#[doc = "Data Dump Margin Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DatadumpMrgtype {
    #[doc = "0: DIN method used"]
    Zz429 = 0,
    #[doc = "1: TM method used"]
    Zz430 = 1,
}
impl From<DatadumpMrgtype> for bool {
    #[inline(always)]
    fn from(variant: DatadumpMrgtype) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATADUMP_MRGTYPE` reader - Data Dump Margin Type"]
pub type DatadumpMrgtypeR = crate::BitReader<DatadumpMrgtype>;
impl DatadumpMrgtypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DatadumpMrgtype {
        match self.bits {
            false => DatadumpMrgtype::Zz429,
            true => DatadumpMrgtype::Zz430,
        }
    }
    #[doc = "DIN method used"]
    #[inline(always)]
    pub fn is_zz429(&self) -> bool {
        *self == DatadumpMrgtype::Zz429
    }
    #[doc = "TM method used"]
    #[inline(always)]
    pub fn is_zz430(&self) -> bool {
        *self == DatadumpMrgtype::Zz430
    }
}
#[doc = "Field `DATADUMP_MRGTYPE` writer - Data Dump Margin Type"]
pub type DatadumpMrgtypeW<'a, REG> = crate::BitWriter<'a, REG, DatadumpMrgtype>;
impl<'a, REG> DatadumpMrgtypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DIN method used"]
    #[inline(always)]
    pub fn zz429(self) -> &'a mut crate::W<REG> {
        self.variant(DatadumpMrgtype::Zz429)
    }
    #[doc = "TM method used"]
    #[inline(always)]
    pub fn zz430(self) -> &'a mut crate::W<REG> {
        self.variant(DatadumpMrgtype::Zz430)
    }
}
impl R {
    #[doc = "Bit 16 - BIST Done"]
    #[inline(always)]
    pub fn bist_done(&self) -> BistDoneR {
        BistDoneR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - BIST Fail"]
    #[inline(always)]
    pub fn bist_fail(&self) -> BistFailR {
        BistFailR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Data Dump Enable"]
    #[inline(always)]
    pub fn datadump(&self) -> DatadumpR {
        DatadumpR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Data Dump Trigger"]
    #[inline(always)]
    pub fn datadump_trig(&self) -> DatadumpTrigR {
        DatadumpTrigR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21 - Data Dump Pattern Select"]
    #[inline(always)]
    pub fn datadump_patt(&self) -> DatadumpPattR {
        DatadumpPattR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - Data Dump Margin Enable"]
    #[inline(always)]
    pub fn datadump_mrgen(&self) -> DatadumpMrgenR {
        DatadumpMrgenR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Data Dump Margin Type"]
    #[inline(always)]
    pub fn datadump_mrgtype(&self) -> DatadumpMrgtypeR {
        DatadumpMrgtypeR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - BIST Done"]
    #[inline(always)]
    pub fn bist_done(&mut self) -> BistDoneW<'_, BistDumpCtrlSpec> {
        BistDoneW::new(self, 16)
    }
    #[doc = "Bit 17 - BIST Fail"]
    #[inline(always)]
    pub fn bist_fail(&mut self) -> BistFailW<'_, BistDumpCtrlSpec> {
        BistFailW::new(self, 17)
    }
    #[doc = "Bit 18 - Data Dump Enable"]
    #[inline(always)]
    pub fn datadump(&mut self) -> DatadumpW<'_, BistDumpCtrlSpec> {
        DatadumpW::new(self, 18)
    }
    #[doc = "Bit 19 - Data Dump Trigger"]
    #[inline(always)]
    pub fn datadump_trig(&mut self) -> DatadumpTrigW<'_, BistDumpCtrlSpec> {
        DatadumpTrigW::new(self, 19)
    }
    #[doc = "Bits 20:21 - Data Dump Pattern Select"]
    #[inline(always)]
    pub fn datadump_patt(&mut self) -> DatadumpPattW<'_, BistDumpCtrlSpec> {
        DatadumpPattW::new(self, 20)
    }
    #[doc = "Bit 22 - Data Dump Margin Enable"]
    #[inline(always)]
    pub fn datadump_mrgen(&mut self) -> DatadumpMrgenW<'_, BistDumpCtrlSpec> {
        DatadumpMrgenW::new(self, 22)
    }
    #[doc = "Bit 23 - Data Dump Margin Type"]
    #[inline(always)]
    pub fn datadump_mrgtype(&mut self) -> DatadumpMrgtypeW<'_, BistDumpCtrlSpec> {
        DatadumpMrgtypeW::new(self, 23)
    }
}
#[doc = "BIST Datadump Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bist_dump_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bist_dump_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BistDumpCtrlSpec;
impl crate::RegisterSpec for BistDumpCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bist_dump_ctrl::R`](R) reader structure"]
impl crate::Readable for BistDumpCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`bist_dump_ctrl::W`](W) writer structure"]
impl crate::Writable for BistDumpCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BIST_DUMP_CTRL to value 0x0001_0000"]
impl crate::Resettable for BistDumpCtrlSpec {
    const RESET_VALUE: u32 = 0x0001_0000;
}
