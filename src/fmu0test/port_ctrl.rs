#[doc = "Register `PORT_CTRL` reader"]
pub type R = crate::R<PortCtrlSpec>;
#[doc = "Register `PORT_CTRL` writer"]
pub type W = crate::W<PortCtrlSpec>;
#[doc = "BIST Done Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BdoneSel {
    #[doc = "0: Select internal bist_done signal from current module instantiation"]
    Zz453 = 0,
    #[doc = "1: Select ipt_bist_fail signal from current module instantiation"]
    Zz454 = 1,
    #[doc = "2: Select ipt_bist_done signal from other module instantiation"]
    Zz455 = 2,
    #[doc = "3: Select AND of internal bist_done signal from current module instantiation with ipt_bist_done signal from other module instantiation"]
    Zz456 = 3,
}
impl From<BdoneSel> for u8 {
    #[inline(always)]
    fn from(variant: BdoneSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BdoneSel {
    type Ux = u8;
}
impl crate::IsEnum for BdoneSel {}
#[doc = "Field `BDONE_SEL` reader - BIST Done Select"]
pub type BdoneSelR = crate::FieldReader<BdoneSel>;
impl BdoneSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BdoneSel {
        match self.bits {
            0 => BdoneSel::Zz453,
            1 => BdoneSel::Zz454,
            2 => BdoneSel::Zz455,
            3 => BdoneSel::Zz456,
            _ => unreachable!(),
        }
    }
    #[doc = "Select internal bist_done signal from current module instantiation"]
    #[inline(always)]
    pub fn is_zz453(&self) -> bool {
        *self == BdoneSel::Zz453
    }
    #[doc = "Select ipt_bist_fail signal from current module instantiation"]
    #[inline(always)]
    pub fn is_zz454(&self) -> bool {
        *self == BdoneSel::Zz454
    }
    #[doc = "Select ipt_bist_done signal from other module instantiation"]
    #[inline(always)]
    pub fn is_zz455(&self) -> bool {
        *self == BdoneSel::Zz455
    }
    #[doc = "Select AND of internal bist_done signal from current module instantiation with ipt_bist_done signal from other module instantiation"]
    #[inline(always)]
    pub fn is_zz456(&self) -> bool {
        *self == BdoneSel::Zz456
    }
}
#[doc = "Field `BDONE_SEL` writer - BIST Done Select"]
pub type BdoneSelW<'a, REG> = crate::FieldWriter<'a, REG, 2, BdoneSel, crate::Safe>;
impl<'a, REG> BdoneSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select internal bist_done signal from current module instantiation"]
    #[inline(always)]
    pub fn zz453(self) -> &'a mut crate::W<REG> {
        self.variant(BdoneSel::Zz453)
    }
    #[doc = "Select ipt_bist_fail signal from current module instantiation"]
    #[inline(always)]
    pub fn zz454(self) -> &'a mut crate::W<REG> {
        self.variant(BdoneSel::Zz454)
    }
    #[doc = "Select ipt_bist_done signal from other module instantiation"]
    #[inline(always)]
    pub fn zz455(self) -> &'a mut crate::W<REG> {
        self.variant(BdoneSel::Zz455)
    }
    #[doc = "Select AND of internal bist_done signal from current module instantiation with ipt_bist_done signal from other module instantiation"]
    #[inline(always)]
    pub fn zz456(self) -> &'a mut crate::W<REG> {
        self.variant(BdoneSel::Zz456)
    }
}
#[doc = "BIST Serial Data Output Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BsdoSel {
    #[doc = "0: Select internal bist_sdo signal from current module instantiation"]
    Zz449 = 0,
    #[doc = "1: Select ipt_bist_done signal from current module instantiation"]
    Zz450 = 1,
    #[doc = "2: Select ipt_bist_sdo signal from other module instantiation"]
    Zz451 = 2,
    #[doc = "3: Select ipt_bist_done signal from other module instantiation"]
    Zz452 = 3,
}
impl From<BsdoSel> for u8 {
    #[inline(always)]
    fn from(variant: BsdoSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BsdoSel {
    type Ux = u8;
}
impl crate::IsEnum for BsdoSel {}
#[doc = "Field `BSDO_SEL` reader - BIST Serial Data Output Select"]
pub type BsdoSelR = crate::FieldReader<BsdoSel>;
impl BsdoSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BsdoSel {
        match self.bits {
            0 => BsdoSel::Zz449,
            1 => BsdoSel::Zz450,
            2 => BsdoSel::Zz451,
            3 => BsdoSel::Zz452,
            _ => unreachable!(),
        }
    }
    #[doc = "Select internal bist_sdo signal from current module instantiation"]
    #[inline(always)]
    pub fn is_zz449(&self) -> bool {
        *self == BsdoSel::Zz449
    }
    #[doc = "Select ipt_bist_done signal from current module instantiation"]
    #[inline(always)]
    pub fn is_zz450(&self) -> bool {
        *self == BsdoSel::Zz450
    }
    #[doc = "Select ipt_bist_sdo signal from other module instantiation"]
    #[inline(always)]
    pub fn is_zz451(&self) -> bool {
        *self == BsdoSel::Zz451
    }
    #[doc = "Select ipt_bist_done signal from other module instantiation"]
    #[inline(always)]
    pub fn is_zz452(&self) -> bool {
        *self == BsdoSel::Zz452
    }
}
#[doc = "Field `BSDO_SEL` writer - BIST Serial Data Output Select"]
pub type BsdoSelW<'a, REG> = crate::FieldWriter<'a, REG, 2, BsdoSel, crate::Safe>;
impl<'a, REG> BsdoSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select internal bist_sdo signal from current module instantiation"]
    #[inline(always)]
    pub fn zz449(self) -> &'a mut crate::W<REG> {
        self.variant(BsdoSel::Zz449)
    }
    #[doc = "Select ipt_bist_done signal from current module instantiation"]
    #[inline(always)]
    pub fn zz450(self) -> &'a mut crate::W<REG> {
        self.variant(BsdoSel::Zz450)
    }
    #[doc = "Select ipt_bist_sdo signal from other module instantiation"]
    #[inline(always)]
    pub fn zz451(self) -> &'a mut crate::W<REG> {
        self.variant(BsdoSel::Zz451)
    }
    #[doc = "Select ipt_bist_done signal from other module instantiation"]
    #[inline(always)]
    pub fn zz452(self) -> &'a mut crate::W<REG> {
        self.variant(BsdoSel::Zz452)
    }
}
impl R {
    #[doc = "Bits 0:1 - BIST Done Select"]
    #[inline(always)]
    pub fn bdone_sel(&self) -> BdoneSelR {
        BdoneSelR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - BIST Serial Data Output Select"]
    #[inline(always)]
    pub fn bsdo_sel(&self) -> BsdoSelR {
        BsdoSelR::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - BIST Done Select"]
    #[inline(always)]
    pub fn bdone_sel(&mut self) -> BdoneSelW<'_, PortCtrlSpec> {
        BdoneSelW::new(self, 0)
    }
    #[doc = "Bits 2:3 - BIST Serial Data Output Select"]
    #[inline(always)]
    pub fn bsdo_sel(&mut self) -> BsdoSelW<'_, PortCtrlSpec> {
        BsdoSelW::new(self, 2)
    }
}
#[doc = "Port Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`port_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`port_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PortCtrlSpec;
impl crate::RegisterSpec for PortCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`port_ctrl::R`](R) reader structure"]
impl crate::Readable for PortCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`port_ctrl::W`](W) writer structure"]
impl crate::Writable for PortCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PORT_CTRL to value 0"]
impl crate::Resettable for PortCtrlSpec {}
