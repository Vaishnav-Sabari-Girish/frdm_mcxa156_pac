#[doc = "Register `FCCOB2` reader"]
pub type R = crate::R<Fccob2Spec>;
#[doc = "Register `FCCOB2` writer"]
pub type W = crate::W<Fccob2Spec>;
#[doc = "Field `CMDADDR` reader - Command starting address"]
pub type CmdaddrR = crate::FieldReader<u32>;
#[doc = "Field `CMDADDR` writer - Command starting address"]
pub type CmdaddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Command starting address"]
    #[inline(always)]
    pub fn cmdaddr(&self) -> CmdaddrR {
        CmdaddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Command starting address"]
    #[inline(always)]
    pub fn cmdaddr(&mut self) -> CmdaddrW<'_, Fccob2Spec> {
        CmdaddrW::new(self, 0)
    }
}
#[doc = "Flash Command Control 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fccob2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fccob2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fccob2Spec;
impl crate::RegisterSpec for Fccob2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fccob2::R`](R) reader structure"]
impl crate::Readable for Fccob2Spec {}
#[doc = "`write(|w| ..)` method takes [`fccob2::W`](W) writer structure"]
impl crate::Writable for Fccob2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FCCOB2 to value 0"]
impl crate::Resettable for Fccob2Spec {}
