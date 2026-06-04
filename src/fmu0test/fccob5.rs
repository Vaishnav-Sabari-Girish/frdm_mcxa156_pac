#[doc = "Register `FCCOB5` reader"]
pub type R = crate::R<Fccob5Spec>;
#[doc = "Register `FCCOB5` writer"]
pub type W = crate::W<Fccob5Spec>;
#[doc = "Field `CMDDATA1` reader - Command data word 1"]
pub type Cmddata1R = crate::FieldReader<u32>;
#[doc = "Field `CMDDATA1` writer - Command data word 1"]
pub type Cmddata1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Command data word 1"]
    #[inline(always)]
    pub fn cmddata1(&self) -> Cmddata1R {
        Cmddata1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Command data word 1"]
    #[inline(always)]
    pub fn cmddata1(&mut self) -> Cmddata1W<'_, Fccob5Spec> {
        Cmddata1W::new(self, 0)
    }
}
#[doc = "Flash Command Control 5 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fccob5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fccob5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fccob5Spec;
impl crate::RegisterSpec for Fccob5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fccob5::R`](R) reader structure"]
impl crate::Readable for Fccob5Spec {}
#[doc = "`write(|w| ..)` method takes [`fccob5::W`](W) writer structure"]
impl crate::Writable for Fccob5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FCCOB5 to value 0"]
impl crate::Resettable for Fccob5Spec {}
