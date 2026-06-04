#[doc = "Register `SMW_DIN2` reader"]
pub type R = crate::R<SmwDin2Spec>;
#[doc = "Register `SMW_DIN2` writer"]
pub type W = crate::W<SmwDin2Spec>;
#[doc = "Field `SMW_DIN2` reader - SMW DIN 2"]
pub type SmwDin2R = crate::FieldReader<u32>;
#[doc = "Field `SMW_DIN2` writer - SMW DIN 2"]
pub type SmwDin2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SMW DIN 2"]
    #[inline(always)]
    pub fn smw_din2(&self) -> SmwDin2R {
        SmwDin2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SMW DIN 2"]
    #[inline(always)]
    pub fn smw_din2(&mut self) -> SmwDin2W<'_, SmwDin2Spec> {
        SmwDin2W::new(self, 0)
    }
}
#[doc = "SMW DIN 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`smw_din2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smw_din2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmwDin2Spec;
impl crate::RegisterSpec for SmwDin2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smw_din2::R`](R) reader structure"]
impl crate::Readable for SmwDin2Spec {}
#[doc = "`write(|w| ..)` method takes [`smw_din2::W`](W) writer structure"]
impl crate::Writable for SmwDin2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SMW_DIN2 to value 0"]
impl crate::Resettable for SmwDin2Spec {}
