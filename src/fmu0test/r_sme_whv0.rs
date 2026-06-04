#[doc = "Register `R_SME_WHV0` reader"]
pub type R = crate::R<RSmeWhv0Spec>;
#[doc = "Register `R_SME_WHV0` writer"]
pub type W = crate::W<RSmeWhv0Spec>;
#[doc = "Field `SMEWHV0` reader - SME WHV Parameter Set 0"]
pub type Smewhv0R = crate::FieldReader<u32>;
#[doc = "Field `SMEWHV0` writer - SME WHV Parameter Set 0"]
pub type Smewhv0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SME WHV Parameter Set 0"]
    #[inline(always)]
    pub fn smewhv0(&self) -> Smewhv0R {
        Smewhv0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SME WHV Parameter Set 0"]
    #[inline(always)]
    pub fn smewhv0(&mut self) -> Smewhv0W<'_, RSmeWhv0Spec> {
        Smewhv0W::new(self, 0)
    }
}
#[doc = "BIST SME WHV Setting 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r_sme_whv0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r_sme_whv0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RSmeWhv0Spec;
impl crate::RegisterSpec for RSmeWhv0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r_sme_whv0::R`](R) reader structure"]
impl crate::Readable for RSmeWhv0Spec {}
#[doc = "`write(|w| ..)` method takes [`r_sme_whv0::W`](W) writer structure"]
impl crate::Writable for RSmeWhv0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R_SME_WHV0 to value 0xccba_9876"]
impl crate::Resettable for RSmeWhv0Spec {
    const RESET_VALUE: u32 = 0xccba_9876;
}
