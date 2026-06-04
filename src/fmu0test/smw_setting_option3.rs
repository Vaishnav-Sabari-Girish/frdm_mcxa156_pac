#[doc = "Register `SMW_SETTING_OPTION3` reader"]
pub type R = crate::R<SmwSettingOption3Spec>;
#[doc = "Register `SMW_SETTING_OPTION3` writer"]
pub type W = crate::W<SmwSettingOption3Spec>;
#[doc = "Field `HEM_WHV_CNTR` reader - WHV_COUNTER for HEM-erase Cycle"]
pub type HemWhvCntrR = crate::FieldReader;
#[doc = "Field `HEM_WHV_CNTR` writer - WHV_COUNTER for HEM-erase Cycle"]
pub type HemWhvCntrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HEM_MAX_ERS` reader - HEM Max Erase Shot Count"]
pub type HemMaxErsR = crate::FieldReader<u16>;
#[doc = "Field `HEM_MAX_ERS` writer - HEM Max Erase Shot Count"]
pub type HemMaxErsW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:7 - WHV_COUNTER for HEM-erase Cycle"]
    #[inline(always)]
    pub fn hem_whv_cntr(&self) -> HemWhvCntrR {
        HemWhvCntrR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:16 - HEM Max Erase Shot Count"]
    #[inline(always)]
    pub fn hem_max_ers(&self) -> HemMaxErsR {
        HemMaxErsR::new(((self.bits >> 8) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - WHV_COUNTER for HEM-erase Cycle"]
    #[inline(always)]
    pub fn hem_whv_cntr(&mut self) -> HemWhvCntrW<'_, SmwSettingOption3Spec> {
        HemWhvCntrW::new(self, 0)
    }
    #[doc = "Bits 8:16 - HEM Max Erase Shot Count"]
    #[inline(always)]
    pub fn hem_max_ers(&mut self) -> HemMaxErsW<'_, SmwSettingOption3Spec> {
        HemMaxErsW::new(self, 8)
    }
}
#[doc = "SMW Setting Option 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`smw_setting_option3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smw_setting_option3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmwSettingOption3Spec;
impl crate::RegisterSpec for SmwSettingOption3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smw_setting_option3::R`](R) reader structure"]
impl crate::Readable for SmwSettingOption3Spec {}
#[doc = "`write(|w| ..)` method takes [`smw_setting_option3::W`](W) writer structure"]
impl crate::Writable for SmwSettingOption3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SMW_SETTING_OPTION3 to value 0x0001_bd00"]
impl crate::Resettable for SmwSettingOption3Spec {
    const RESET_VALUE: u32 = 0x0001_bd00;
}
