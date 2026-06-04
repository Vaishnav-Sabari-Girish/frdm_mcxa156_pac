#[doc = "Register `SOCTRIM1_3` reader"]
pub type R = crate::R<Soctrim1_3Spec>;
#[doc = "Register `SOCTRIM1_3` writer"]
pub type W = crate::W<Soctrim1_3Spec>;
#[doc = "Field `TRIM1_3` reader - TRIM1_3"]
pub type Trim1_3R = crate::FieldReader<u32>;
#[doc = "Field `TRIM1_3` writer - TRIM1_3"]
pub type Trim1_3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - TRIM1_3"]
    #[inline(always)]
    pub fn trim1_3(&self) -> Trim1_3R {
        Trim1_3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - TRIM1_3"]
    #[inline(always)]
    pub fn trim1_3(&mut self) -> Trim1_3W<'_, Soctrim1_3Spec> {
        Trim1_3W::new(self, 0)
    }
}
#[doc = "SoC Trim Phrase 1 Word 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`soctrim1_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soctrim1_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Soctrim1_3Spec;
impl crate::RegisterSpec for Soctrim1_3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`soctrim1_3::R`](R) reader structure"]
impl crate::Readable for Soctrim1_3Spec {}
#[doc = "`write(|w| ..)` method takes [`soctrim1_3::W`](W) writer structure"]
impl crate::Writable for Soctrim1_3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SOCTRIM1_3 to value 0xffff_ffff"]
impl crate::Resettable for Soctrim1_3Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
