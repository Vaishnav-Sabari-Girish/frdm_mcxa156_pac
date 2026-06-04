#[doc = "Register `R_SME_WHV1` reader"]
pub type R = crate::R<RSmeWhv1Spec>;
#[doc = "Register `R_SME_WHV1` writer"]
pub type W = crate::W<RSmeWhv1Spec>;
#[doc = "Field `SMEWHV1` reader - SME WHV Parameter Set 1"]
pub type Smewhv1R = crate::FieldReader<u32>;
#[doc = "Field `SMEWHV1` writer - SME WHV Parameter Set 1"]
pub type Smewhv1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SME WHV Parameter Set 1"]
    #[inline(always)]
    pub fn smewhv1(&self) -> Smewhv1R {
        Smewhv1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SME WHV Parameter Set 1"]
    #[inline(always)]
    pub fn smewhv1(&mut self) -> Smewhv1W<'_, RSmeWhv1Spec> {
        Smewhv1W::new(self, 0)
    }
}
#[doc = "BIST SME WHV Setting 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r_sme_whv1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r_sme_whv1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RSmeWhv1Spec;
impl crate::RegisterSpec for RSmeWhv1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r_sme_whv1::R`](R) reader structure"]
impl crate::Readable for RSmeWhv1Spec {}
#[doc = "`write(|w| ..)` method takes [`r_sme_whv1::W`](W) writer structure"]
impl crate::Writable for RSmeWhv1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R_SME_WHV1 to value 0xcccc_cccc"]
impl crate::Resettable for RSmeWhv1Spec {
    const RESET_VALUE: u32 = 0xcccc_cccc;
}
