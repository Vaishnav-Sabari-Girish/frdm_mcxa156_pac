#[doc = "Register `FAILCNT` reader"]
pub type R = crate::R<FailcntSpec>;
#[doc = "Register `FAILCNT` writer"]
pub type W = crate::W<FailcntSpec>;
#[doc = "Field `FAILCNT` reader - Fail Count"]
pub type FailcntR = crate::FieldReader<u32>;
#[doc = "Field `FAILCNT` writer - Fail Count"]
pub type FailcntW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Fail Count"]
    #[inline(always)]
    pub fn failcnt(&self) -> FailcntR {
        FailcntR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Fail Count"]
    #[inline(always)]
    pub fn failcnt(&mut self) -> FailcntW<'_, FailcntSpec> {
        FailcntW::new(self, 0)
    }
}
#[doc = "Fail Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`failcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`failcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FailcntSpec;
impl crate::RegisterSpec for FailcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`failcnt::R`](R) reader structure"]
impl crate::Readable for FailcntSpec {}
#[doc = "`write(|w| ..)` method takes [`failcnt::W`](W) writer structure"]
impl crate::Writable for FailcntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FAILCNT to value 0"]
impl crate::Resettable for FailcntSpec {}
