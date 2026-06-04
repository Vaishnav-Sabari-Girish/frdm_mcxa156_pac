#[doc = "Register `R_SMW_SETTING2` reader"]
pub type R = crate::R<RSmwSetting2Spec>;
#[doc = "Register `R_SMW_SETTING2` writer"]
pub type W = crate::W<RSmwSetting2Spec>;
#[doc = "Field `SMWPARM2` reader - SMW Parameter Set 2"]
pub type Smwparm2R = crate::FieldReader<u32>;
#[doc = "Field `SMWPARM2` writer - SMW Parameter Set 2"]
pub type Smwparm2W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bits 0:28 - SMW Parameter Set 2"]
    #[inline(always)]
    pub fn smwparm2(&self) -> Smwparm2R {
        Smwparm2R::new(self.bits & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:28 - SMW Parameter Set 2"]
    #[inline(always)]
    pub fn smwparm2(&mut self) -> Smwparm2W<'_, RSmwSetting2Spec> {
        Smwparm2W::new(self, 0)
    }
}
#[doc = "BIST SMW Setting 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r_smw_setting2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r_smw_setting2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RSmwSetting2Spec;
impl crate::RegisterSpec for RSmwSetting2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r_smw_setting2::R`](R) reader structure"]
impl crate::Readable for RSmwSetting2Spec {}
#[doc = "`write(|w| ..)` method takes [`r_smw_setting2::W`](W) writer structure"]
impl crate::Writable for RSmwSetting2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R_SMW_SETTING2 to value 0x00a8_0151"]
impl crate::Resettable for RSmwSetting2Spec {
    const RESET_VALUE: u32 = 0x00a8_0151;
}
