#[doc = "Register `SOCTRIM6_1` reader"]
pub type R = crate::R<Soctrim6_1Spec>;
#[doc = "Register `SOCTRIM6_1` writer"]
pub type W = crate::W<Soctrim6_1Spec>;
#[doc = "Field `TRIM6_1` reader - TRIM6_1"]
pub type Trim6_1R = crate::FieldReader<u32>;
#[doc = "Field `TRIM6_1` writer - TRIM6_1"]
pub type Trim6_1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - TRIM6_1"]
    #[inline(always)]
    pub fn trim6_1(&self) -> Trim6_1R {
        Trim6_1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - TRIM6_1"]
    #[inline(always)]
    pub fn trim6_1(&mut self) -> Trim6_1W<'_, Soctrim6_1Spec> {
        Trim6_1W::new(self, 0)
    }
}
#[doc = "SoC Trim Phrase 6 Word 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`soctrim6_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soctrim6_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Soctrim6_1Spec;
impl crate::RegisterSpec for Soctrim6_1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`soctrim6_1::R`](R) reader structure"]
impl crate::Readable for Soctrim6_1Spec {}
#[doc = "`write(|w| ..)` method takes [`soctrim6_1::W`](W) writer structure"]
impl crate::Writable for Soctrim6_1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SOCTRIM6_1 to value 0xffff_ffff"]
impl crate::Resettable for Soctrim6_1Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
