#[doc = "Register `FCCOB3` reader"]
pub type R = crate::R<Fccob3Spec>;
#[doc = "Register `FCCOB3` writer"]
pub type W = crate::W<Fccob3Spec>;
#[doc = "Field `CMDADDRE` reader - Command ending address"]
pub type CmdaddreR = crate::FieldReader<u32>;
#[doc = "Field `CMDADDRE` writer - Command ending address"]
pub type CmdaddreW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Command ending address"]
    #[inline(always)]
    pub fn cmdaddre(&self) -> CmdaddreR {
        CmdaddreR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Command ending address"]
    #[inline(always)]
    pub fn cmdaddre(&mut self) -> CmdaddreW<'_, Fccob3Spec> {
        CmdaddreW::new(self, 0)
    }
}
#[doc = "Flash Command Control 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fccob3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fccob3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fccob3Spec;
impl crate::RegisterSpec for Fccob3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fccob3::R`](R) reader structure"]
impl crate::Readable for Fccob3Spec {}
#[doc = "`write(|w| ..)` method takes [`fccob3::W`](W) writer structure"]
impl crate::Writable for Fccob3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FCCOB3 to value 0"]
impl crate::Resettable for Fccob3Spec {}
