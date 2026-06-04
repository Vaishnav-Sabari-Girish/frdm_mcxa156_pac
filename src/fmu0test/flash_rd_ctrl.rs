#[doc = "Register `FLASH_RD_CTRL` reader"]
pub type R = crate::R<FlashRdCtrlSpec>;
#[doc = "Register `FLASH_RD_CTRL` writer"]
pub type W = crate::W<FlashRdCtrlSpec>;
#[doc = "Flash Read Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlashRd {
    #[doc = "0: Manual flash read not enabled.(default)"]
    Zz155 = 0,
    #[doc = "1: Manual flash read enabled"]
    Zz156 = 1,
}
impl From<FlashRd> for bool {
    #[inline(always)]
    fn from(variant: FlashRd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLASH_RD` reader - Flash Read Enable"]
pub type FlashRdR = crate::BitReader<FlashRd>;
impl FlashRdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FlashRd {
        match self.bits {
            false => FlashRd::Zz155,
            true => FlashRd::Zz156,
        }
    }
    #[doc = "Manual flash read not enabled.(default)"]
    #[inline(always)]
    pub fn is_zz155(&self) -> bool {
        *self == FlashRd::Zz155
    }
    #[doc = "Manual flash read enabled"]
    #[inline(always)]
    pub fn is_zz156(&self) -> bool {
        *self == FlashRd::Zz156
    }
}
#[doc = "Field `FLASH_RD` writer - Flash Read Enable"]
pub type FlashRdW<'a, REG> = crate::BitWriter<'a, REG, FlashRd>;
impl<'a, REG> FlashRdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Manual flash read not enabled.(default)"]
    #[inline(always)]
    pub fn zz155(self) -> &'a mut crate::W<REG> {
        self.variant(FlashRd::Zz155)
    }
    #[doc = "Manual flash read enabled"]
    #[inline(always)]
    pub fn zz156(self) -> &'a mut crate::W<REG> {
        self.variant(FlashRd::Zz156)
    }
}
#[doc = "Wide Load Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WideLoad {
    #[doc = "0: Wide load mode disabled (default)"]
    Zz153 = 0,
    #[doc = "1: Wide load mode enabled"]
    Zz154 = 1,
}
impl From<WideLoad> for bool {
    #[inline(always)]
    fn from(variant: WideLoad) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIDE_LOAD` reader - Wide Load Enable"]
pub type WideLoadR = crate::BitReader<WideLoad>;
impl WideLoadR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WideLoad {
        match self.bits {
            false => WideLoad::Zz153,
            true => WideLoad::Zz154,
        }
    }
    #[doc = "Wide load mode disabled (default)"]
    #[inline(always)]
    pub fn is_zz153(&self) -> bool {
        *self == WideLoad::Zz153
    }
    #[doc = "Wide load mode enabled"]
    #[inline(always)]
    pub fn is_zz154(&self) -> bool {
        *self == WideLoad::Zz154
    }
}
#[doc = "Field `WIDE_LOAD` writer - Wide Load Enable"]
pub type WideLoadW<'a, REG> = crate::BitWriter<'a, REG, WideLoad>;
impl<'a, REG> WideLoadW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wide load mode disabled (default)"]
    #[inline(always)]
    pub fn zz153(self) -> &'a mut crate::W<REG> {
        self.variant(WideLoad::Zz153)
    }
    #[doc = "Wide load mode enabled"]
    #[inline(always)]
    pub fn zz154(self) -> &'a mut crate::W<REG> {
        self.variant(WideLoad::Zz154)
    }
}
#[doc = "Single Flash Read\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SingleRd {
    #[doc = "0: Normal UINT operation"]
    Zz151 = 0,
    #[doc = "1: UINT configured for single cycle reads"]
    Zz152 = 1,
}
impl From<SingleRd> for bool {
    #[inline(always)]
    fn from(variant: SingleRd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SINGLE_RD` reader - Single Flash Read"]
pub type SingleRdR = crate::BitReader<SingleRd>;
impl SingleRdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SingleRd {
        match self.bits {
            false => SingleRd::Zz151,
            true => SingleRd::Zz152,
        }
    }
    #[doc = "Normal UINT operation"]
    #[inline(always)]
    pub fn is_zz151(&self) -> bool {
        *self == SingleRd::Zz151
    }
    #[doc = "UINT configured for single cycle reads"]
    #[inline(always)]
    pub fn is_zz152(&self) -> bool {
        *self == SingleRd::Zz152
    }
}
#[doc = "Field `SINGLE_RD` writer - Single Flash Read"]
pub type SingleRdW<'a, REG> = crate::BitWriter<'a, REG, SingleRd>;
impl<'a, REG> SingleRdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal UINT operation"]
    #[inline(always)]
    pub fn zz151(self) -> &'a mut crate::W<REG> {
        self.variant(SingleRd::Zz151)
    }
    #[doc = "UINT configured for single cycle reads"]
    #[inline(always)]
    pub fn zz152(self) -> &'a mut crate::W<REG> {
        self.variant(SingleRd::Zz152)
    }
}
impl R {
    #[doc = "Bit 0 - Flash Read Enable"]
    #[inline(always)]
    pub fn flash_rd(&self) -> FlashRdR {
        FlashRdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wide Load Enable"]
    #[inline(always)]
    pub fn wide_load(&self) -> WideLoadR {
        WideLoadR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Single Flash Read"]
    #[inline(always)]
    pub fn single_rd(&self) -> SingleRdR {
        SingleRdR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flash Read Enable"]
    #[inline(always)]
    pub fn flash_rd(&mut self) -> FlashRdW<'_, FlashRdCtrlSpec> {
        FlashRdW::new(self, 0)
    }
    #[doc = "Bit 1 - Wide Load Enable"]
    #[inline(always)]
    pub fn wide_load(&mut self) -> WideLoadW<'_, FlashRdCtrlSpec> {
        WideLoadW::new(self, 1)
    }
    #[doc = "Bit 2 - Single Flash Read"]
    #[inline(always)]
    pub fn single_rd(&mut self) -> SingleRdW<'_, FlashRdCtrlSpec> {
        SingleRdW::new(self, 2)
    }
}
#[doc = "Flash Read Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_rd_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_rd_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashRdCtrlSpec;
impl crate::RegisterSpec for FlashRdCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_rd_ctrl::R`](R) reader structure"]
impl crate::Readable for FlashRdCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`flash_rd_ctrl::W`](W) writer structure"]
impl crate::Writable for FlashRdCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLASH_RD_CTRL to value 0"]
impl crate::Resettable for FlashRdCtrlSpec {}
