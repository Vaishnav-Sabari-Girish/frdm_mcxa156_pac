#[doc = "Register `SMW_TIMER_OPTION` reader"]
pub type R = crate::R<SmwTimerOptionSpec>;
#[doc = "Register `SMW_TIMER_OPTION` writer"]
pub type W = crate::W<SmwTimerOptionSpec>;
#[doc = "Field `SMW_CDIVL` reader - Clock Divide Scalar for Long Pulse"]
pub type SmwCdivlR = crate::FieldReader;
#[doc = "Field `SMW_CDIVL` writer - Clock Divide Scalar for Long Pulse"]
pub type SmwCdivlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SMW_TVFY` reader - Timer Adjust for Verify"]
pub type SmwTvfyR = crate::FieldReader;
#[doc = "Field `SMW_TVFY` writer - Timer Adjust for Verify"]
pub type SmwTvfyW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:7 - Clock Divide Scalar for Long Pulse"]
    #[inline(always)]
    pub fn smw_cdivl(&self) -> SmwCdivlR {
        SmwCdivlR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:12 - Timer Adjust for Verify"]
    #[inline(always)]
    pub fn smw_tvfy(&self) -> SmwTvfyR {
        SmwTvfyR::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock Divide Scalar for Long Pulse"]
    #[inline(always)]
    pub fn smw_cdivl(&mut self) -> SmwCdivlW<'_, SmwTimerOptionSpec> {
        SmwCdivlW::new(self, 0)
    }
    #[doc = "Bits 8:12 - Timer Adjust for Verify"]
    #[inline(always)]
    pub fn smw_tvfy(&mut self) -> SmwTvfyW<'_, SmwTimerOptionSpec> {
        SmwTvfyW::new(self, 8)
    }
}
#[doc = "SMW Timer Option Register\n\nYou can [`read`](crate::Reg::read) this register and get [`smw_timer_option::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smw_timer_option::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmwTimerOptionSpec;
impl crate::RegisterSpec for SmwTimerOptionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smw_timer_option::R`](R) reader structure"]
impl crate::Readable for SmwTimerOptionSpec {}
#[doc = "`write(|w| ..)` method takes [`smw_timer_option::W`](W) writer structure"]
impl crate::Writable for SmwTimerOptionSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SMW_TIMER_OPTION to value 0"]
impl crate::Resettable for SmwTimerOptionSpec {}
