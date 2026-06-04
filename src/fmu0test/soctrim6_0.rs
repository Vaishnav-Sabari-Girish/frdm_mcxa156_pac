#[doc = "Register `SOCTRIM6_0` reader"]
pub type R = crate::R<Soctrim6_0Spec>;
#[doc = "Register `SOCTRIM6_0` writer"]
pub type W = crate::W<Soctrim6_0Spec>;
#[doc = "Field `TRIM6_0` reader - TRIM6_0"]
pub type Trim6_0R = crate::FieldReader<u32>;
#[doc = "Field `TRIM6_0` writer - TRIM6_0"]
pub type Trim6_0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - TRIM6_0"]
    #[inline(always)]
    pub fn trim6_0(&self) -> Trim6_0R {
        Trim6_0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - TRIM6_0"]
    #[inline(always)]
    pub fn trim6_0(&mut self) -> Trim6_0W<'_, Soctrim6_0Spec> {
        Trim6_0W::new(self, 0)
    }
}
#[doc = "SoC Trim Phrase 6 Word 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`soctrim6_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soctrim6_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Soctrim6_0Spec;
impl crate::RegisterSpec for Soctrim6_0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`soctrim6_0::R`](R) reader structure"]
impl crate::Readable for Soctrim6_0Spec {}
#[doc = "`write(|w| ..)` method takes [`soctrim6_0::W`](W) writer structure"]
impl crate::Writable for Soctrim6_0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SOCTRIM6_0 to value 0xffff_ffff"]
impl crate::Resettable for Soctrim6_0Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
