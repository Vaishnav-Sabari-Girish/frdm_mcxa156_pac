#[doc = "Register `MISCCTRL` reader"]
pub type R = crate::R<MiscctrlSpec>;
#[doc = "Register `MISCCTRL` writer"]
pub type W = crate::W<MiscctrlSpec>;
#[doc = "OWN Error Detect for ISO IN and ISO OUT Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ownerrisodis {
    #[doc = "0: Enable"]
    DisOwnErrorDetectIso = 0,
    #[doc = "1: Disable"]
    EnOwnErrorDetectIso = 1,
}
impl From<Ownerrisodis> for bool {
    #[inline(always)]
    fn from(variant: Ownerrisodis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OWNERRISODIS` reader - OWN Error Detect for ISO IN and ISO OUT Disable"]
pub type OwnerrisodisR = crate::BitReader<Ownerrisodis>;
impl OwnerrisodisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ownerrisodis {
        match self.bits {
            false => Ownerrisodis::DisOwnErrorDetectIso,
            true => Ownerrisodis::EnOwnErrorDetectIso,
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_dis_own_error_detect_iso(&self) -> bool {
        *self == Ownerrisodis::DisOwnErrorDetectIso
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_en_own_error_detect_iso(&self) -> bool {
        *self == Ownerrisodis::EnOwnErrorDetectIso
    }
}
#[doc = "Field `OWNERRISODIS` writer - OWN Error Detect for ISO IN and ISO OUT Disable"]
pub type OwnerrisodisW<'a, REG> = crate::BitWriter<'a, REG, Ownerrisodis>;
impl<'a, REG> OwnerrisodisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn dis_own_error_detect_iso(self) -> &'a mut crate::W<REG> {
        self.variant(Ownerrisodis::DisOwnErrorDetectIso)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn en_own_error_detect_iso(self) -> &'a mut crate::W<REG> {
        self.variant(Ownerrisodis::EnOwnErrorDetectIso)
    }
}
#[doc = "VREGIN Rising Edge Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VredgEn {
    #[doc = "0: Disable"]
    DisVreginReInt = 0,
    #[doc = "1: Enable"]
    EnVreginReInt = 1,
}
impl From<VredgEn> for bool {
    #[inline(always)]
    fn from(variant: VredgEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VREDG_EN` reader - VREGIN Rising Edge Interrupt Enable"]
pub type VredgEnR = crate::BitReader<VredgEn>;
impl VredgEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VredgEn {
        match self.bits {
            false => VredgEn::DisVreginReInt,
            true => VredgEn::EnVreginReInt,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_dis_vregin_re_int(&self) -> bool {
        *self == VredgEn::DisVreginReInt
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_en_vregin_re_int(&self) -> bool {
        *self == VredgEn::EnVreginReInt
    }
}
#[doc = "Field `VREDG_EN` writer - VREGIN Rising Edge Interrupt Enable"]
pub type VredgEnW<'a, REG> = crate::BitWriter<'a, REG, VredgEn>;
impl<'a, REG> VredgEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn dis_vregin_re_int(self) -> &'a mut crate::W<REG> {
        self.variant(VredgEn::DisVreginReInt)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn en_vregin_re_int(self) -> &'a mut crate::W<REG> {
        self.variant(VredgEn::EnVreginReInt)
    }
}
#[doc = "VREGIN Falling Edge Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VfedgEn {
    #[doc = "0: Disable"]
    DisVreginFeInt = 0,
    #[doc = "1: Enable"]
    EnVreginFeInt = 1,
}
impl From<VfedgEn> for bool {
    #[inline(always)]
    fn from(variant: VfedgEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VFEDG_EN` reader - VREGIN Falling Edge Interrupt Enable"]
pub type VfedgEnR = crate::BitReader<VfedgEn>;
impl VfedgEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VfedgEn {
        match self.bits {
            false => VfedgEn::DisVreginFeInt,
            true => VfedgEn::EnVreginFeInt,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_dis_vregin_fe_int(&self) -> bool {
        *self == VfedgEn::DisVreginFeInt
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_en_vregin_fe_int(&self) -> bool {
        *self == VfedgEn::EnVreginFeInt
    }
}
#[doc = "Field `VFEDG_EN` writer - VREGIN Falling Edge Interrupt Enable"]
pub type VfedgEnW<'a, REG> = crate::BitWriter<'a, REG, VfedgEn>;
impl<'a, REG> VfedgEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn dis_vregin_fe_int(self) -> &'a mut crate::W<REG> {
        self.variant(VfedgEn::DisVreginFeInt)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn en_vregin_fe_int(self) -> &'a mut crate::W<REG> {
        self.variant(VfedgEn::EnVreginFeInt)
    }
}
#[doc = "USB Peripheral Mode Stall Adjust Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StlAdjEn {
    #[doc = "0: If ENDPTn\\[END_STALL\\] = 1, both IN and OUT directions for the associated endpoint stalls."]
    StallBothInOut = 0,
    #[doc = "1: If ENDPTn\\[END_STALL\\] = 1, the STALL_xx_DIS registers control which directions for the associated endpoint stalls."]
    StallSingleDirection = 1,
}
impl From<StlAdjEn> for bool {
    #[inline(always)]
    fn from(variant: StlAdjEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STL_ADJ_EN` reader - USB Peripheral Mode Stall Adjust Enable"]
pub type StlAdjEnR = crate::BitReader<StlAdjEn>;
impl StlAdjEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> StlAdjEn {
        match self.bits {
            false => StlAdjEn::StallBothInOut,
            true => StlAdjEn::StallSingleDirection,
        }
    }
    #[doc = "If ENDPTn\\[END_STALL\\] = 1, both IN and OUT directions for the associated endpoint stalls."]
    #[inline(always)]
    pub fn is_stall_both_in_out(&self) -> bool {
        *self == StlAdjEn::StallBothInOut
    }
    #[doc = "If ENDPTn\\[END_STALL\\] = 1, the STALL_xx_DIS registers control which directions for the associated endpoint stalls."]
    #[inline(always)]
    pub fn is_stall_single_direction(&self) -> bool {
        *self == StlAdjEn::StallSingleDirection
    }
}
#[doc = "Field `STL_ADJ_EN` writer - USB Peripheral Mode Stall Adjust Enable"]
pub type StlAdjEnW<'a, REG> = crate::BitWriter<'a, REG, StlAdjEn>;
impl<'a, REG> StlAdjEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "If ENDPTn\\[END_STALL\\] = 1, both IN and OUT directions for the associated endpoint stalls."]
    #[inline(always)]
    pub fn stall_both_in_out(self) -> &'a mut crate::W<REG> {
        self.variant(StlAdjEn::StallBothInOut)
    }
    #[doc = "If ENDPTn\\[END_STALL\\] = 1, the STALL_xx_DIS registers control which directions for the associated endpoint stalls."]
    #[inline(always)]
    pub fn stall_single_direction(self) -> &'a mut crate::W<REG> {
        self.variant(StlAdjEn::StallSingleDirection)
    }
}
impl R {
    #[doc = "Bit 2 - OWN Error Detect for ISO IN and ISO OUT Disable"]
    #[inline(always)]
    pub fn ownerrisodis(&self) -> OwnerrisodisR {
        OwnerrisodisR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - VREGIN Rising Edge Interrupt Enable"]
    #[inline(always)]
    pub fn vredg_en(&self) -> VredgEnR {
        VredgEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - VREGIN Falling Edge Interrupt Enable"]
    #[inline(always)]
    pub fn vfedg_en(&self) -> VfedgEnR {
        VfedgEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - USB Peripheral Mode Stall Adjust Enable"]
    #[inline(always)]
    pub fn stl_adj_en(&self) -> StlAdjEnR {
        StlAdjEnR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - OWN Error Detect for ISO IN and ISO OUT Disable"]
    #[inline(always)]
    pub fn ownerrisodis(&mut self) -> OwnerrisodisW<'_, MiscctrlSpec> {
        OwnerrisodisW::new(self, 2)
    }
    #[doc = "Bit 3 - VREGIN Rising Edge Interrupt Enable"]
    #[inline(always)]
    pub fn vredg_en(&mut self) -> VredgEnW<'_, MiscctrlSpec> {
        VredgEnW::new(self, 3)
    }
    #[doc = "Bit 4 - VREGIN Falling Edge Interrupt Enable"]
    #[inline(always)]
    pub fn vfedg_en(&mut self) -> VfedgEnW<'_, MiscctrlSpec> {
        VfedgEnW::new(self, 4)
    }
    #[doc = "Bit 7 - USB Peripheral Mode Stall Adjust Enable"]
    #[inline(always)]
    pub fn stl_adj_en(&mut self) -> StlAdjEnW<'_, MiscctrlSpec> {
        StlAdjEnW::new(self, 7)
    }
}
#[doc = "Miscellaneous Control\n\nYou can [`read`](crate::Reg::read) this register and get [`miscctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`miscctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiscctrlSpec;
impl crate::RegisterSpec for MiscctrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`miscctrl::R`](R) reader structure"]
impl crate::Readable for MiscctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`miscctrl::W`](W) writer structure"]
impl crate::Writable for MiscctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MISCCTRL to value 0"]
impl crate::Resettable for MiscctrlSpec {}
