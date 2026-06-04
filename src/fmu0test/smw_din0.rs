#[doc = "Register `SMW_DIN0` reader"]
pub type R = crate::R<SmwDin0Spec>;
#[doc = "Register `SMW_DIN0` writer"]
pub type W = crate::W<SmwDin0Spec>;
#[doc = "Field `SMW_DIN0` reader - SMW DIN 0"]
pub type SmwDin0R = crate::FieldReader<u32>;
#[doc = "Field `SMW_DIN0` writer - SMW DIN 0"]
pub type SmwDin0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SMW DIN 0"]
    #[inline(always)]
    pub fn smw_din0(&self) -> SmwDin0R {
        SmwDin0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SMW DIN 0"]
    #[inline(always)]
    pub fn smw_din0(&mut self) -> SmwDin0W<'_, SmwDin0Spec> {
        SmwDin0W::new(self, 0)
    }
}
#[doc = "SMW DIN 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`smw_din0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smw_din0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmwDin0Spec;
impl crate::RegisterSpec for SmwDin0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smw_din0::R`](R) reader structure"]
impl crate::Readable for SmwDin0Spec {}
#[doc = "`write(|w| ..)` method takes [`smw_din0::W`](W) writer structure"]
impl crate::Writable for SmwDin0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SMW_DIN0 to value 0"]
impl crate::Resettable for SmwDin0Spec {}
