#[doc = "Register `FCCOB4` reader"]
pub type R = crate::R<Fccob4Spec>;
#[doc = "Register `FCCOB4` writer"]
pub type W = crate::W<Fccob4Spec>;
#[doc = "Field `CMDDATA0` reader - Command data word 0"]
pub type Cmddata0R = crate::FieldReader<u32>;
#[doc = "Field `CMDDATA0` writer - Command data word 0"]
pub type Cmddata0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Command data word 0"]
    #[inline(always)]
    pub fn cmddata0(&self) -> Cmddata0R {
        Cmddata0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Command data word 0"]
    #[inline(always)]
    pub fn cmddata0(&mut self) -> Cmddata0W<'_, Fccob4Spec> {
        Cmddata0W::new(self, 0)
    }
}
#[doc = "Flash Command Control 4 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fccob4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fccob4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fccob4Spec;
impl crate::RegisterSpec for Fccob4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fccob4::R`](R) reader structure"]
impl crate::Readable for Fccob4Spec {}
#[doc = "`write(|w| ..)` method takes [`fccob4::W`](W) writer structure"]
impl crate::Writable for Fccob4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FCCOB4 to value 0"]
impl crate::Resettable for Fccob4Spec {}
