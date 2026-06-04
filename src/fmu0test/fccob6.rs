#[doc = "Register `FCCOB6` reader"]
pub type R = crate::R<Fccob6Spec>;
#[doc = "Register `FCCOB6` writer"]
pub type W = crate::W<Fccob6Spec>;
#[doc = "Field `CMDDATA2` reader - Command data word 2"]
pub type Cmddata2R = crate::FieldReader<u32>;
#[doc = "Field `CMDDATA2` writer - Command data word 2"]
pub type Cmddata2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Command data word 2"]
    #[inline(always)]
    pub fn cmddata2(&self) -> Cmddata2R {
        Cmddata2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Command data word 2"]
    #[inline(always)]
    pub fn cmddata2(&mut self) -> Cmddata2W<'_, Fccob6Spec> {
        Cmddata2W::new(self, 0)
    }
}
#[doc = "Flash Command Control 6 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fccob6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fccob6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fccob6Spec;
impl crate::RegisterSpec for Fccob6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fccob6::R`](R) reader structure"]
impl crate::Readable for Fccob6Spec {}
#[doc = "`write(|w| ..)` method takes [`fccob6::W`](W) writer structure"]
impl crate::Writable for Fccob6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FCCOB6 to value 0"]
impl crate::Resettable for Fccob6Spec {}
