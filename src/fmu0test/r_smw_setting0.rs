#[doc = "Register `R_SMW_SETTING0` reader"]
pub type R = crate::R<RSmwSetting0Spec>;
#[doc = "Register `R_SMW_SETTING0` writer"]
pub type W = crate::W<RSmwSetting0Spec>;
#[doc = "Field `SMWPARM0` reader - SMW Parameter Set 0"]
pub type Smwparm0R = crate::FieldReader<u32>;
#[doc = "Field `SMWPARM0` writer - SMW Parameter Set 0"]
pub type Smwparm0W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bits 0:30 - SMW Parameter Set 0"]
    #[inline(always)]
    pub fn smwparm0(&self) -> Smwparm0R {
        Smwparm0R::new(self.bits & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:30 - SMW Parameter Set 0"]
    #[inline(always)]
    pub fn smwparm0(&mut self) -> Smwparm0W<'_, RSmwSetting0Spec> {
        Smwparm0W::new(self, 0)
    }
}
#[doc = "BIST SMW Setting 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r_smw_setting0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r_smw_setting0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RSmwSetting0Spec;
impl crate::RegisterSpec for RSmwSetting0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r_smw_setting0::R`](R) reader structure"]
impl crate::Readable for RSmwSetting0Spec {}
#[doc = "`write(|w| ..)` method takes [`r_smw_setting0::W`](W) writer structure"]
impl crate::Writable for RSmwSetting0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R_SMW_SETTING0 to value 0x0f0b_4000"]
impl crate::Resettable for RSmwSetting0Spec {
    const RESET_VALUE: u32 = 0x0f0b_4000;
}
