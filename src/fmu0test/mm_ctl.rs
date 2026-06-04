#[doc = "Register `MM_CTL` reader"]
pub type R = crate::R<MmCtlSpec>;
#[doc = "Register `MM_CTL` writer"]
pub type W = crate::W<MmCtlSpec>;
#[doc = "Field `MM_SEL` reader - Register Access Enable"]
pub type MmSelR = crate::BitReader;
#[doc = "Field `MM_SEL` writer - Register Access Enable"]
pub type MmSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Register R/W Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MmRd {
    #[doc = "0: Write to register"]
    Zz161 = 0,
    #[doc = "1: Read register"]
    Zz162 = 1,
}
impl From<MmRd> for bool {
    #[inline(always)]
    fn from(variant: MmRd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MM_RD` reader - Register R/W Control"]
pub type MmRdR = crate::BitReader<MmRd>;
impl MmRdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MmRd {
        match self.bits {
            false => MmRd::Zz161,
            true => MmRd::Zz162,
        }
    }
    #[doc = "Write to register"]
    #[inline(always)]
    pub fn is_zz161(&self) -> bool {
        *self == MmRd::Zz161
    }
    #[doc = "Read register"]
    #[inline(always)]
    pub fn is_zz162(&self) -> bool {
        *self == MmRd::Zz162
    }
}
#[doc = "Field `MM_RD` writer - Register R/W Control"]
pub type MmRdW<'a, REG> = crate::BitWriter<'a, REG, MmRd>;
impl<'a, REG> MmRdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write to register"]
    #[inline(always)]
    pub fn zz161(self) -> &'a mut crate::W<REG> {
        self.variant(MmRd::Zz161)
    }
    #[doc = "Read register"]
    #[inline(always)]
    pub fn zz162(self) -> &'a mut crate::W<REG> {
        self.variant(MmRd::Zz162)
    }
}
#[doc = "BIST on\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BistOn {
    #[doc = "0: BIST enable not forced by user interface"]
    Zz159 = 0,
    #[doc = "1: BIST enable control by user interface"]
    Zz160 = 1,
}
impl From<BistOn> for bool {
    #[inline(always)]
    fn from(variant: BistOn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BIST_ON` reader - BIST on"]
pub type BistOnR = crate::BitReader<BistOn>;
impl BistOnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BistOn {
        match self.bits {
            false => BistOn::Zz159,
            true => BistOn::Zz160,
        }
    }
    #[doc = "BIST enable not forced by user interface"]
    #[inline(always)]
    pub fn is_zz159(&self) -> bool {
        *self == BistOn::Zz159
    }
    #[doc = "BIST enable control by user interface"]
    #[inline(always)]
    pub fn is_zz160(&self) -> bool {
        *self == BistOn::Zz160
    }
}
#[doc = "Field `BIST_ON` writer - BIST on"]
pub type BistOnW<'a, REG> = crate::BitWriter<'a, REG, BistOn>;
impl<'a, REG> BistOnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "BIST enable not forced by user interface"]
    #[inline(always)]
    pub fn zz159(self) -> &'a mut crate::W<REG> {
        self.variant(BistOn::Zz159)
    }
    #[doc = "BIST enable control by user interface"]
    #[inline(always)]
    pub fn zz160(self) -> &'a mut crate::W<REG> {
        self.variant(BistOn::Zz160)
    }
}
#[doc = "Force Switch Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ForceSwClk {
    #[doc = "0: Switch clock not forced on (gated normally)"]
    Zz157 = 0,
    #[doc = "1: Switch clock forced on"]
    Zz158 = 1,
}
impl From<ForceSwClk> for bool {
    #[inline(always)]
    fn from(variant: ForceSwClk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FORCE_SW_CLK` reader - Force Switch Clock"]
pub type ForceSwClkR = crate::BitReader<ForceSwClk>;
impl ForceSwClkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ForceSwClk {
        match self.bits {
            false => ForceSwClk::Zz157,
            true => ForceSwClk::Zz158,
        }
    }
    #[doc = "Switch clock not forced on (gated normally)"]
    #[inline(always)]
    pub fn is_zz157(&self) -> bool {
        *self == ForceSwClk::Zz157
    }
    #[doc = "Switch clock forced on"]
    #[inline(always)]
    pub fn is_zz158(&self) -> bool {
        *self == ForceSwClk::Zz158
    }
}
#[doc = "Field `FORCE_SW_CLK` writer - Force Switch Clock"]
pub type ForceSwClkW<'a, REG> = crate::BitWriter<'a, REG, ForceSwClk>;
impl<'a, REG> ForceSwClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Switch clock not forced on (gated normally)"]
    #[inline(always)]
    pub fn zz157(self) -> &'a mut crate::W<REG> {
        self.variant(ForceSwClk::Zz157)
    }
    #[doc = "Switch clock forced on"]
    #[inline(always)]
    pub fn zz158(self) -> &'a mut crate::W<REG> {
        self.variant(ForceSwClk::Zz158)
    }
}
impl R {
    #[doc = "Bit 0 - Register Access Enable"]
    #[inline(always)]
    pub fn mm_sel(&self) -> MmSelR {
        MmSelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Register R/W Control"]
    #[inline(always)]
    pub fn mm_rd(&self) -> MmRdR {
        MmRdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BIST on"]
    #[inline(always)]
    pub fn bist_on(&self) -> BistOnR {
        BistOnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Force Switch Clock"]
    #[inline(always)]
    pub fn force_sw_clk(&self) -> ForceSwClkR {
        ForceSwClkR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Register Access Enable"]
    #[inline(always)]
    pub fn mm_sel(&mut self) -> MmSelW<'_, MmCtlSpec> {
        MmSelW::new(self, 0)
    }
    #[doc = "Bit 1 - Register R/W Control"]
    #[inline(always)]
    pub fn mm_rd(&mut self) -> MmRdW<'_, MmCtlSpec> {
        MmRdW::new(self, 1)
    }
    #[doc = "Bit 2 - BIST on"]
    #[inline(always)]
    pub fn bist_on(&mut self) -> BistOnW<'_, MmCtlSpec> {
        BistOnW::new(self, 2)
    }
    #[doc = "Bit 3 - Force Switch Clock"]
    #[inline(always)]
    pub fn force_sw_clk(&mut self) -> ForceSwClkW<'_, MmCtlSpec> {
        ForceSwClkW::new(self, 3)
    }
}
#[doc = "Memory Map Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mm_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mm_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmCtlSpec;
impl crate::RegisterSpec for MmCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mm_ctl::R`](R) reader structure"]
impl crate::Readable for MmCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`mm_ctl::W`](W) writer structure"]
impl crate::Writable for MmCtlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MM_CTL to value 0"]
impl crate::Resettable for MmCtlSpec {}
