#[doc = "Register `SOCTRIM6_3` reader"]
pub type R = crate::R<Soctrim6_3Spec>;
#[doc = "Register `SOCTRIM6_3` writer"]
pub type W = crate::W<Soctrim6_3Spec>;
#[doc = "Field `TRIM6_3` reader - TRIM6_3"]
pub type Trim6_3R = crate::FieldReader<u32>;
#[doc = "Field `TRIM6_3` writer - TRIM6_3"]
pub type Trim6_3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - TRIM6_3"]
    #[inline(always)]
    pub fn trim6_3(&self) -> Trim6_3R {
        Trim6_3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - TRIM6_3"]
    #[inline(always)]
    pub fn trim6_3(&mut self) -> Trim6_3W<'_, Soctrim6_3Spec> {
        Trim6_3W::new(self, 0)
    }
}
#[doc = "SoC Trim Phrase 6 Word 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`soctrim6_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soctrim6_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Soctrim6_3Spec;
impl crate::RegisterSpec for Soctrim6_3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`soctrim6_3::R`](R) reader structure"]
impl crate::Readable for Soctrim6_3Spec {}
#[doc = "`write(|w| ..)` method takes [`soctrim6_3::W`](W) writer structure"]
impl crate::Writable for Soctrim6_3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SOCTRIM6_3 to value 0xffff_ffff"]
impl crate::Resettable for Soctrim6_3Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
