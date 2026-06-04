#[doc = "Register `BSEL` reader"]
pub type R = crate::R<BselSpec>;
#[doc = "Register `BSEL` writer"]
pub type W = crate::W<BselSpec>;
#[doc = "Field `SBSEL` reader - Slave Block Select"]
pub type SbselR = crate::FieldReader;
#[doc = "Field `SBSEL` writer - Slave Block Select"]
pub type SbselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MBSEL` reader - Master Block Select"]
pub type MbselR = crate::FieldReader;
#[doc = "Field `MBSEL` writer - Master Block Select"]
pub type MbselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Slave Block Select"]
    #[inline(always)]
    pub fn sbsel(&self) -> SbselR {
        SbselR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - Master Block Select"]
    #[inline(always)]
    pub fn mbsel(&self) -> MbselR {
        MbselR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Slave Block Select"]
    #[inline(always)]
    pub fn sbsel(&mut self) -> SbselW<'_, BselSpec> {
        SbselW::new(self, 0)
    }
    #[doc = "Bits 8:9 - Master Block Select"]
    #[inline(always)]
    pub fn mbsel(&mut self) -> MbselW<'_, BselSpec> {
        MbselW::new(self, 8)
    }
}
#[doc = "FMU Block Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BselSpec;
impl crate::RegisterSpec for BselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bsel::R`](R) reader structure"]
impl crate::Readable for BselSpec {}
#[doc = "`write(|w| ..)` method takes [`bsel::W`](W) writer structure"]
impl crate::Writable for BselSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BSEL to value 0x0101"]
impl crate::Resettable for BselSpec {
    const RESET_VALUE: u32 = 0x0101;
}
