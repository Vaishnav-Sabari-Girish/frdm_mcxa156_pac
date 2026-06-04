#[doc = "Register `PWR_OPT` reader"]
pub type R = crate::R<PwrOptSpec>;
#[doc = "Register `PWR_OPT` writer"]
pub type W = crate::W<PwrOptSpec>;
#[doc = "Field `PD_CDIV` reader - Power Down Clock Divider Setting"]
pub type PdCdivR = crate::FieldReader;
#[doc = "Field `PD_CDIV` writer - Power Down Clock Divider Setting"]
pub type PdCdivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SLM_COUNT` reader - Sleep Recovery Timer Count"]
pub type SlmCountR = crate::FieldReader<u16>;
#[doc = "Field `SLM_COUNT` writer - Sleep Recovery Timer Count"]
pub type SlmCountW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Power Down BIST Timer Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdTimerEn {
    #[doc = "0: BIST timer is not triggered during Power Down recovery"]
    Zz127 = 0,
    #[doc = "1: BIST timer is triggered during Power Down recovery (default behavior)"]
    Zz128 = 1,
}
impl From<PdTimerEn> for bool {
    #[inline(always)]
    fn from(variant: PdTimerEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_TIMER_EN` reader - Power Down BIST Timer Enable"]
pub type PdTimerEnR = crate::BitReader<PdTimerEn>;
impl PdTimerEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdTimerEn {
        match self.bits {
            false => PdTimerEn::Zz127,
            true => PdTimerEn::Zz128,
        }
    }
    #[doc = "BIST timer is not triggered during Power Down recovery"]
    #[inline(always)]
    pub fn is_zz127(&self) -> bool {
        *self == PdTimerEn::Zz127
    }
    #[doc = "BIST timer is triggered during Power Down recovery (default behavior)"]
    #[inline(always)]
    pub fn is_zz128(&self) -> bool {
        *self == PdTimerEn::Zz128
    }
}
#[doc = "Field `PD_TIMER_EN` writer - Power Down BIST Timer Enable"]
pub type PdTimerEnW<'a, REG> = crate::BitWriter<'a, REG, PdTimerEn>;
impl<'a, REG> PdTimerEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "BIST timer is not triggered during Power Down recovery"]
    #[inline(always)]
    pub fn zz127(self) -> &'a mut crate::W<REG> {
        self.variant(PdTimerEn::Zz127)
    }
    #[doc = "BIST timer is triggered during Power Down recovery (default behavior)"]
    #[inline(always)]
    pub fn zz128(self) -> &'a mut crate::W<REG> {
        self.variant(PdTimerEn::Zz128)
    }
}
impl R {
    #[doc = "Bits 0:7 - Power Down Clock Divider Setting"]
    #[inline(always)]
    pub fn pd_cdiv(&self) -> PdCdivR {
        PdCdivR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:25 - Sleep Recovery Timer Count"]
    #[inline(always)]
    pub fn slm_count(&self) -> SlmCountR {
        SlmCountR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 31 - Power Down BIST Timer Enable"]
    #[inline(always)]
    pub fn pd_timer_en(&self) -> PdTimerEnR {
        PdTimerEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Power Down Clock Divider Setting"]
    #[inline(always)]
    pub fn pd_cdiv(&mut self) -> PdCdivW<'_, PwrOptSpec> {
        PdCdivW::new(self, 0)
    }
    #[doc = "Bits 16:25 - Sleep Recovery Timer Count"]
    #[inline(always)]
    pub fn slm_count(&mut self) -> SlmCountW<'_, PwrOptSpec> {
        SlmCountW::new(self, 16)
    }
    #[doc = "Bit 31 - Power Down BIST Timer Enable"]
    #[inline(always)]
    pub fn pd_timer_en(&mut self) -> PdTimerEnW<'_, PwrOptSpec> {
        PdTimerEnW::new(self, 31)
    }
}
#[doc = "Power Mode Options Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwr_opt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_opt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwrOptSpec;
impl crate::RegisterSpec for PwrOptSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwr_opt::R`](R) reader structure"]
impl crate::Readable for PwrOptSpec {}
#[doc = "`write(|w| ..)` method takes [`pwr_opt::W`](W) writer structure"]
impl crate::Writable for PwrOptSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWR_OPT to value 0x80fa_0032"]
impl crate::Resettable for PwrOptSpec {
    const RESET_VALUE: u32 = 0x80fa_0032;
}
