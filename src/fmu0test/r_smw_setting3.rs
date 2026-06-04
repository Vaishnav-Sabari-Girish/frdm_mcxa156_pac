#[doc = "Register `R_SMW_SETTING3` reader"]
pub type R = crate::R<RSmwSetting3Spec>;
#[doc = "Register `R_SMW_SETTING3` writer"]
pub type W = crate::W<RSmwSetting3Spec>;
#[doc = "Field `SMWPARM3` reader - SMW Parameter Set 3"]
pub type Smwparm3R = crate::FieldReader<u32>;
#[doc = "Field `SMWPARM3` writer - SMW Parameter Set 3"]
pub type Smwparm3W<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:16 - SMW Parameter Set 3"]
    #[inline(always)]
    pub fn smwparm3(&self) -> Smwparm3R {
        Smwparm3R::new(self.bits & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:16 - SMW Parameter Set 3"]
    #[inline(always)]
    pub fn smwparm3(&mut self) -> Smwparm3W<'_, RSmwSetting3Spec> {
        Smwparm3W::new(self, 0)
    }
}
#[doc = "BIST SMW Setting 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r_smw_setting3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r_smw_setting3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RSmwSetting3Spec;
impl crate::RegisterSpec for RSmwSetting3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r_smw_setting3::R`](R) reader structure"]
impl crate::Readable for RSmwSetting3Spec {}
#[doc = "`write(|w| ..)` method takes [`r_smw_setting3::W`](W) writer structure"]
impl crate::Writable for RSmwSetting3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R_SMW_SETTING3 to value 0x0001_be00"]
impl crate::Resettable for RSmwSetting3Spec {
    const RESET_VALUE: u32 = 0x0001_be00;
}
