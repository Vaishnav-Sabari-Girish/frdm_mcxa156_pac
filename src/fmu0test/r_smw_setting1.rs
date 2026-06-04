#[doc = "Register `R_SMW_SETTING1` reader"]
pub type R = crate::R<RSmwSetting1Spec>;
#[doc = "Register `R_SMW_SETTING1` writer"]
pub type W = crate::W<RSmwSetting1Spec>;
#[doc = "Field `SMWPARM1` reader - SMW Parameter Set 1"]
pub type Smwparm1R = crate::FieldReader<u32>;
#[doc = "Field `SMWPARM1` writer - SMW Parameter Set 1"]
pub type Smwparm1W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:27 - SMW Parameter Set 1"]
    #[inline(always)]
    pub fn smwparm1(&self) -> Smwparm1R {
        Smwparm1R::new(self.bits & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:27 - SMW Parameter Set 1"]
    #[inline(always)]
    pub fn smwparm1(&mut self) -> Smwparm1W<'_, RSmwSetting1Spec> {
        Smwparm1W::new(self, 0)
    }
}
#[doc = "BIST SMW Setting 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r_smw_setting1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r_smw_setting1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RSmwSetting1Spec;
impl crate::RegisterSpec for RSmwSetting1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r_smw_setting1::R`](R) reader structure"]
impl crate::Readable for RSmwSetting1Spec {}
#[doc = "`write(|w| ..)` method takes [`r_smw_setting1::W`](W) writer structure"]
impl crate::Writable for RSmwSetting1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R_SMW_SETTING1 to value 0x0315_ce2a"]
impl crate::Resettable for RSmwSetting1Spec {
    const RESET_VALUE: u32 = 0x0315_ce2a;
}
