#[doc = "Register `SMW_DIN3` reader"]
pub type R = crate::R<SmwDin3Spec>;
#[doc = "Register `SMW_DIN3` writer"]
pub type W = crate::W<SmwDin3Spec>;
#[doc = "Field `SMW_DIN3` reader - SMW DIN 3"]
pub type SmwDin3R = crate::FieldReader<u32>;
#[doc = "Field `SMW_DIN3` writer - SMW DIN 3"]
pub type SmwDin3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SMW DIN 3"]
    #[inline(always)]
    pub fn smw_din3(&self) -> SmwDin3R {
        SmwDin3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SMW DIN 3"]
    #[inline(always)]
    pub fn smw_din3(&mut self) -> SmwDin3W<'_, SmwDin3Spec> {
        SmwDin3W::new(self, 0)
    }
}
#[doc = "SMW DIN 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`smw_din3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smw_din3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmwDin3Spec;
impl crate::RegisterSpec for SmwDin3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smw_din3::R`](R) reader structure"]
impl crate::Readable for SmwDin3Spec {}
#[doc = "`write(|w| ..)` method takes [`smw_din3::W`](W) writer structure"]
impl crate::Writable for SmwDin3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SMW_DIN3 to value 0"]
impl crate::Resettable for SmwDin3Spec {}
