#[doc = "Register `SMW_SMP_WHV_OPTION1` reader"]
pub type R = crate::R<SmwSmpWhvOption1Spec>;
#[doc = "Register `SMW_SMP_WHV_OPTION1` writer"]
pub type W = crate::W<SmwSmpWhvOption1Spec>;
#[doc = "Field `SMP_WHV_OPT1` reader - Smart Program WHV Option High"]
pub type SmpWhvOpt1R = crate::FieldReader<u32>;
#[doc = "Field `SMP_WHV_OPT1` writer - Smart Program WHV Option High"]
pub type SmpWhvOpt1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Smart Program WHV Option High"]
    #[inline(always)]
    pub fn smp_whv_opt1(&self) -> SmpWhvOpt1R {
        SmpWhvOpt1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Smart Program WHV Option High"]
    #[inline(always)]
    pub fn smp_whv_opt1(&mut self) -> SmpWhvOpt1W<'_, SmwSmpWhvOption1Spec> {
        SmpWhvOpt1W::new(self, 0)
    }
}
#[doc = "SMW SMP WHV Option 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`smw_smp_whv_option1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smw_smp_whv_option1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmwSmpWhvOption1Spec;
impl crate::RegisterSpec for SmwSmpWhvOption1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smw_smp_whv_option1::R`](R) reader structure"]
impl crate::Readable for SmwSmpWhvOption1Spec {}
#[doc = "`write(|w| ..)` method takes [`smw_smp_whv_option1::W`](W) writer structure"]
impl crate::Writable for SmwSmpWhvOption1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SMW_SMP_WHV_OPTION1 to value 0x7777_7777"]
impl crate::Resettable for SmwSmpWhvOption1Spec {
    const RESET_VALUE: u32 = 0x7777_7777;
}
