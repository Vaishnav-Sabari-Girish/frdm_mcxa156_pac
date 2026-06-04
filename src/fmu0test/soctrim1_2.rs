#[doc = "Register `SOCTRIM1_2` reader"]
pub type R = crate::R<Soctrim1_2Spec>;
#[doc = "Register `SOCTRIM1_2` writer"]
pub type W = crate::W<Soctrim1_2Spec>;
#[doc = "Field `TRIM1_2` reader - TRIM1_2"]
pub type Trim1_2R = crate::FieldReader<u32>;
#[doc = "Field `TRIM1_2` writer - TRIM1_2"]
pub type Trim1_2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - TRIM1_2"]
    #[inline(always)]
    pub fn trim1_2(&self) -> Trim1_2R {
        Trim1_2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - TRIM1_2"]
    #[inline(always)]
    pub fn trim1_2(&mut self) -> Trim1_2W<'_, Soctrim1_2Spec> {
        Trim1_2W::new(self, 0)
    }
}
#[doc = "SoC Trim Phrase 1 Word 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`soctrim1_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soctrim1_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Soctrim1_2Spec;
impl crate::RegisterSpec for Soctrim1_2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`soctrim1_2::R`](R) reader structure"]
impl crate::Readable for Soctrim1_2Spec {}
#[doc = "`write(|w| ..)` method takes [`soctrim1_2::W`](W) writer structure"]
impl crate::Writable for Soctrim1_2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SOCTRIM1_2 to value 0xffff_ffff"]
impl crate::Resettable for Soctrim1_2Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
