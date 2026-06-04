#[doc = "Register `FIRCATC2` reader"]
pub type R = crate::R<Fircatc2Spec>;
#[doc = "Register `FIRCATC2` writer"]
pub type W = crate::W<Fircatc2Spec>;
#[doc = "Field `COARMINC` reader - Coarse Trim Minimum Counter"]
pub type CoarmincR = crate::FieldReader<u16>;
#[doc = "Field `COARMINC` writer - Coarse Trim Minimum Counter"]
pub type CoarmincW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `COARMAXC` reader - Coarse Trim Maximum Counter"]
pub type CoarmaxcR = crate::FieldReader<u16>;
#[doc = "Field `COARMAXC` writer - Coarse Trim Maximum Counter"]
pub type CoarmaxcW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Coarse Trim Minimum Counter"]
    #[inline(always)]
    pub fn coarminc(&self) -> CoarmincR {
        CoarmincR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Coarse Trim Maximum Counter"]
    #[inline(always)]
    pub fn coarmaxc(&self) -> CoarmaxcR {
        CoarmaxcR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Coarse Trim Minimum Counter"]
    #[inline(always)]
    pub fn coarminc(&mut self) -> CoarmincW<'_, Fircatc2Spec> {
        CoarmincW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Coarse Trim Maximum Counter"]
    #[inline(always)]
    pub fn coarmaxc(&mut self) -> CoarmaxcW<'_, Fircatc2Spec> {
        CoarmaxcW::new(self, 16)
    }
}
#[doc = "FIRC Auto-trimming Counter 2\n\nYou can [`read`](crate::Reg::read) this register and get [`fircatc2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fircatc2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fircatc2Spec;
impl crate::RegisterSpec for Fircatc2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fircatc2::R`](R) reader structure"]
impl crate::Readable for Fircatc2Spec {}
#[doc = "`write(|w| ..)` method takes [`fircatc2::W`](W) writer structure"]
impl crate::Writable for Fircatc2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIRCATC2 to value 0x9afb_91f5"]
impl crate::Resettable for Fircatc2Spec {
    const RESET_VALUE: u32 = 0x9afb_91f5;
}
