#[doc = "Register `FIRCATC3` reader"]
pub type R = crate::R<Fircatc3Spec>;
#[doc = "Register `FIRCATC3` writer"]
pub type W = crate::W<Fircatc3Spec>;
#[doc = "Field `FINEMINC` reader - Fine Trim Minimum Counter"]
pub type FinemincR = crate::FieldReader<u16>;
#[doc = "Field `FINEMINC` writer - Fine Trim Minimum Counter"]
pub type FinemincW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `FINEMAXC` reader - Fine Trim Maximum Counter"]
pub type FinemaxcR = crate::FieldReader<u16>;
#[doc = "Field `FINEMAXC` writer - Fine Trim Maximum Counter"]
pub type FinemaxcW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Fine Trim Minimum Counter"]
    #[inline(always)]
    pub fn fineminc(&self) -> FinemincR {
        FinemincR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Fine Trim Maximum Counter"]
    #[inline(always)]
    pub fn finemaxc(&self) -> FinemaxcR {
        FinemaxcR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Fine Trim Minimum Counter"]
    #[inline(always)]
    pub fn fineminc(&mut self) -> FinemincW<'_, Fircatc3Spec> {
        FinemincW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Fine Trim Maximum Counter"]
    #[inline(always)]
    pub fn finemaxc(&mut self) -> FinemaxcW<'_, Fircatc3Spec> {
        FinemaxcW::new(self, 16)
    }
}
#[doc = "FIRC Auto-trimming Counter 2\n\nYou can [`read`](crate::Reg::read) this register and get [`fircatc3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fircatc3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fircatc3Spec;
impl crate::RegisterSpec for Fircatc3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fircatc3::R`](R) reader structure"]
impl crate::Readable for Fircatc3Spec {}
#[doc = "`write(|w| ..)` method takes [`fircatc3::W`](W) writer structure"]
impl crate::Writable for Fircatc3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIRCATC3 to value 0x96d8_9618"]
impl crate::Resettable for Fircatc3Spec {
    const RESET_VALUE: u32 = 0x96d8_9618;
}
