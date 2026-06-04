#[doc = "Register `FIRCATC1` reader"]
pub type R = crate::R<Fircatc1Spec>;
#[doc = "Register `FIRCATC1` writer"]
pub type W = crate::W<Fircatc1Spec>;
#[doc = "Field `IDEALC` reader - Ideal Counter"]
pub type IdealcR = crate::FieldReader<u16>;
#[doc = "Field `IDEALC` writer - Ideal Counter"]
pub type IdealcW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Ideal Counter"]
    #[inline(always)]
    pub fn idealc(&self) -> IdealcR {
        IdealcR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Ideal Counter"]
    #[inline(always)]
    pub fn idealc(&mut self) -> IdealcW<'_, Fircatc1Spec> {
        IdealcW::new(self, 0)
    }
}
#[doc = "FIRC Auto-trimming Counter 1\n\nYou can [`read`](crate::Reg::read) this register and get [`fircatc1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fircatc1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fircatc1Spec;
impl crate::RegisterSpec for Fircatc1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fircatc1::R`](R) reader structure"]
impl crate::Readable for Fircatc1Spec {}
#[doc = "`write(|w| ..)` method takes [`fircatc1::W`](W) writer structure"]
impl crate::Writable for Fircatc1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIRCATC1 to value 0x9678"]
impl crate::Resettable for Fircatc1Spec {
    const RESET_VALUE: u32 = 0x9678;
}
