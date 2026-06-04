#[doc = "Register `SOCTRIM1_0` reader"]
pub type R = crate::R<Soctrim1_0Spec>;
#[doc = "Register `SOCTRIM1_0` writer"]
pub type W = crate::W<Soctrim1_0Spec>;
#[doc = "Field `TRIM1_0` reader - TRIM1_0"]
pub type Trim1_0R = crate::FieldReader<u32>;
#[doc = "Field `TRIM1_0` writer - TRIM1_0"]
pub type Trim1_0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - TRIM1_0"]
    #[inline(always)]
    pub fn trim1_0(&self) -> Trim1_0R {
        Trim1_0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - TRIM1_0"]
    #[inline(always)]
    pub fn trim1_0(&mut self) -> Trim1_0W<'_, Soctrim1_0Spec> {
        Trim1_0W::new(self, 0)
    }
}
#[doc = "SoC Trim Phrase 1 Word 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`soctrim1_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soctrim1_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Soctrim1_0Spec;
impl crate::RegisterSpec for Soctrim1_0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`soctrim1_0::R`](R) reader structure"]
impl crate::Readable for Soctrim1_0Spec {}
#[doc = "`write(|w| ..)` method takes [`soctrim1_0::W`](W) writer structure"]
impl crate::Writable for Soctrim1_0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SOCTRIM1_0 to value 0xffff_ffff"]
impl crate::Resettable for Soctrim1_0Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
