#[doc = "Register `SMW_SETTING_OPTION0` reader"]
pub type R = crate::R<SmwSettingOption0Spec>;
#[doc = "Register `SMW_SETTING_OPTION0` writer"]
pub type W = crate::W<SmwSettingOption0Spec>;
#[doc = "Field `MV_INIT` reader - Medium Voltage Level Select Initial"]
pub type MvInitR = crate::FieldReader;
#[doc = "Field `MV_INIT` writer - Medium Voltage Level Select Initial"]
pub type MvInitW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MV_END` reader - Medium Voltage Level Select Final"]
pub type MvEndR = crate::FieldReader;
#[doc = "Field `MV_END` writer - Medium Voltage Level Select Final"]
pub type MvEndW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MV_MISC` reader - Medium Voltage Control Misc"]
pub type MvMiscR = crate::FieldReader;
#[doc = "Field `MV_MISC` writer - Medium Voltage Control Misc"]
pub type MvMiscW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `IPGM_INIT` reader - Program Current Control Initial"]
pub type IpgmInitR = crate::FieldReader;
#[doc = "Field `IPGM_INIT` writer - Program Current Control Initial"]
pub type IpgmInitW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IPGM_END` reader - Program Current Control Final"]
pub type IpgmEndR = crate::FieldReader;
#[doc = "Field `IPGM_END` writer - Program Current Control Final"]
pub type IpgmEndW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IPGM_MISC` reader - Program Current Control Misc"]
pub type IpgmMiscR = crate::FieldReader;
#[doc = "Field `IPGM_MISC` writer - Program Current Control Misc"]
pub type IpgmMiscW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 14:16 - Medium Voltage Level Select Initial"]
    #[inline(always)]
    pub fn mv_init(&self) -> MvInitR {
        MvInitR::new(((self.bits >> 14) & 7) as u8)
    }
    #[doc = "Bits 17:19 - Medium Voltage Level Select Final"]
    #[inline(always)]
    pub fn mv_end(&self) -> MvEndR {
        MvEndR::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:23 - Medium Voltage Control Misc"]
    #[inline(always)]
    pub fn mv_misc(&self) -> MvMiscR {
        MvMiscR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:25 - Program Current Control Initial"]
    #[inline(always)]
    pub fn ipgm_init(&self) -> IpgmInitR {
        IpgmInitR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Program Current Control Final"]
    #[inline(always)]
    pub fn ipgm_end(&self) -> IpgmEndR {
        IpgmEndR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:30 - Program Current Control Misc"]
    #[inline(always)]
    pub fn ipgm_misc(&self) -> IpgmMiscR {
        IpgmMiscR::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 14:16 - Medium Voltage Level Select Initial"]
    #[inline(always)]
    pub fn mv_init(&mut self) -> MvInitW<'_, SmwSettingOption0Spec> {
        MvInitW::new(self, 14)
    }
    #[doc = "Bits 17:19 - Medium Voltage Level Select Final"]
    #[inline(always)]
    pub fn mv_end(&mut self) -> MvEndW<'_, SmwSettingOption0Spec> {
        MvEndW::new(self, 17)
    }
    #[doc = "Bits 20:23 - Medium Voltage Control Misc"]
    #[inline(always)]
    pub fn mv_misc(&mut self) -> MvMiscW<'_, SmwSettingOption0Spec> {
        MvMiscW::new(self, 20)
    }
    #[doc = "Bits 24:25 - Program Current Control Initial"]
    #[inline(always)]
    pub fn ipgm_init(&mut self) -> IpgmInitW<'_, SmwSettingOption0Spec> {
        IpgmInitW::new(self, 24)
    }
    #[doc = "Bits 26:27 - Program Current Control Final"]
    #[inline(always)]
    pub fn ipgm_end(&mut self) -> IpgmEndW<'_, SmwSettingOption0Spec> {
        IpgmEndW::new(self, 26)
    }
    #[doc = "Bits 28:30 - Program Current Control Misc"]
    #[inline(always)]
    pub fn ipgm_misc(&mut self) -> IpgmMiscW<'_, SmwSettingOption0Spec> {
        IpgmMiscW::new(self, 28)
    }
}
#[doc = "SMW Setting Option 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`smw_setting_option0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smw_setting_option0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmwSettingOption0Spec;
impl crate::RegisterSpec for SmwSettingOption0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smw_setting_option0::R`](R) reader structure"]
impl crate::Readable for SmwSettingOption0Spec {}
#[doc = "`write(|w| ..)` method takes [`smw_setting_option0::W`](W) writer structure"]
impl crate::Writable for SmwSettingOption0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SMW_SETTING_OPTION0 to value 0x0f0b_4000"]
impl crate::Resettable for SmwSettingOption0Spec {
    const RESET_VALUE: u32 = 0x0f0b_4000;
}
