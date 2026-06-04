#[doc = "Register `FCCOB0` reader"]
pub type R = crate::R<Fccob0Spec>;
#[doc = "Register `FCCOB0` writer"]
pub type W = crate::W<Fccob0Spec>;
#[doc = "Field `CMDCODE` reader - Command code"]
pub type CmdcodeR = crate::FieldReader;
#[doc = "Field `CMDCODE` writer - Command code"]
pub type CmdcodeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Command code"]
    #[inline(always)]
    pub fn cmdcode(&self) -> CmdcodeR {
        CmdcodeR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Command code"]
    #[inline(always)]
    pub fn cmdcode(&mut self) -> CmdcodeW<'_, Fccob0Spec> {
        CmdcodeW::new(self, 0)
    }
}
#[doc = "Flash Command Control 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fccob0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fccob0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fccob0Spec;
impl crate::RegisterSpec for Fccob0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fccob0::R`](R) reader structure"]
impl crate::Readable for Fccob0Spec {}
#[doc = "`write(|w| ..)` method takes [`fccob0::W`](W) writer structure"]
impl crate::Writable for Fccob0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FCCOB0 to value 0"]
impl crate::Resettable for Fccob0Spec {}
