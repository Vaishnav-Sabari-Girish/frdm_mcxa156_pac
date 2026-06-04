#[doc = "Register `FCCOB1` reader"]
pub type R = crate::R<Fccob1Spec>;
#[doc = "Register `FCCOB1` writer"]
pub type W = crate::W<Fccob1Spec>;
#[doc = "Field `CMDOPT` reader - Command options"]
pub type CmdoptR = crate::FieldReader;
#[doc = "Field `CMDOPT` writer - Command options"]
pub type CmdoptW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Command options"]
    #[inline(always)]
    pub fn cmdopt(&self) -> CmdoptR {
        CmdoptR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Command options"]
    #[inline(always)]
    pub fn cmdopt(&mut self) -> CmdoptW<'_, Fccob1Spec> {
        CmdoptW::new(self, 0)
    }
}
#[doc = "Flash Command Control 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fccob1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fccob1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fccob1Spec;
impl crate::RegisterSpec for Fccob1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fccob1::R`](R) reader structure"]
impl crate::Readable for Fccob1Spec {}
#[doc = "`write(|w| ..)` method takes [`fccob1::W`](W) writer structure"]
impl crate::Writable for Fccob1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FCCOB1 to value 0"]
impl crate::Resettable for Fccob1Spec {}
