#[doc = "Register `SMW_SME_WHV_OPTION1` reader"]
pub type R = crate::R<SmwSmeWhvOption1Spec>;
#[doc = "Register `SMW_SME_WHV_OPTION1` writer"]
pub type W = crate::W<SmwSmeWhvOption1Spec>;
#[doc = "Field `SME_WHV_OPT1` reader - Smart Erase WHV Option High"]
pub type SmeWhvOpt1R = crate::FieldReader<u32>;
#[doc = "Field `SME_WHV_OPT1` writer - Smart Erase WHV Option High"]
pub type SmeWhvOpt1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Smart Erase WHV Option High"]
    #[inline(always)]
    pub fn sme_whv_opt1(&self) -> SmeWhvOpt1R {
        SmeWhvOpt1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Smart Erase WHV Option High"]
    #[inline(always)]
    pub fn sme_whv_opt1(&mut self) -> SmeWhvOpt1W<'_, SmwSmeWhvOption1Spec> {
        SmeWhvOpt1W::new(self, 0)
    }
}
#[doc = "SMW SME WHV Option 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`smw_sme_whv_option1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smw_sme_whv_option1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmwSmeWhvOption1Spec;
impl crate::RegisterSpec for SmwSmeWhvOption1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smw_sme_whv_option1::R`](R) reader structure"]
impl crate::Readable for SmwSmeWhvOption1Spec {}
#[doc = "`write(|w| ..)` method takes [`smw_sme_whv_option1::W`](W) writer structure"]
impl crate::Writable for SmwSmeWhvOption1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SMW_SME_WHV_OPTION1 to value 0xcccc_cccc"]
impl crate::Resettable for SmwSmeWhvOption1Spec {
    const RESET_VALUE: u32 = 0xcccc_cccc;
}
