#[doc = "Register `SOCTRIM6_2` reader"]
pub type R = crate::R<Soctrim6_2Spec>;
#[doc = "Register `SOCTRIM6_2` writer"]
pub type W = crate::W<Soctrim6_2Spec>;
#[doc = "Field `TRIM6_2` reader - TRIM6_2"]
pub type Trim6_2R = crate::FieldReader<u32>;
#[doc = "Field `TRIM6_2` writer - TRIM6_2"]
pub type Trim6_2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - TRIM6_2"]
    #[inline(always)]
    pub fn trim6_2(&self) -> Trim6_2R {
        Trim6_2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - TRIM6_2"]
    #[inline(always)]
    pub fn trim6_2(&mut self) -> Trim6_2W<'_, Soctrim6_2Spec> {
        Trim6_2W::new(self, 0)
    }
}
#[doc = "SoC Trim Phrase 6 Word 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`soctrim6_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soctrim6_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Soctrim6_2Spec;
impl crate::RegisterSpec for Soctrim6_2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`soctrim6_2::R`](R) reader structure"]
impl crate::Readable for Soctrim6_2Spec {}
#[doc = "`write(|w| ..)` method takes [`soctrim6_2::W`](W) writer structure"]
impl crate::Writable for Soctrim6_2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SOCTRIM6_2 to value 0xffff_ffff"]
impl crate::Resettable for Soctrim6_2Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
