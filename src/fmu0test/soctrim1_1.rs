#[doc = "Register `SOCTRIM1_1` reader"]
pub type R = crate::R<Soctrim1_1Spec>;
#[doc = "Register `SOCTRIM1_1` writer"]
pub type W = crate::W<Soctrim1_1Spec>;
#[doc = "Field `TRIM1_1` reader - TRIM1_1"]
pub type Trim1_1R = crate::FieldReader<u32>;
#[doc = "Field `TRIM1_1` writer - TRIM1_1"]
pub type Trim1_1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - TRIM1_1"]
    #[inline(always)]
    pub fn trim1_1(&self) -> Trim1_1R {
        Trim1_1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - TRIM1_1"]
    #[inline(always)]
    pub fn trim1_1(&mut self) -> Trim1_1W<'_, Soctrim1_1Spec> {
        Trim1_1W::new(self, 0)
    }
}
#[doc = "SoC Trim Phrase 1 Word 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`soctrim1_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soctrim1_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Soctrim1_1Spec;
impl crate::RegisterSpec for Soctrim1_1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`soctrim1_1::R`](R) reader structure"]
impl crate::Readable for Soctrim1_1Spec {}
#[doc = "`write(|w| ..)` method takes [`soctrim1_1::W`](W) writer structure"]
impl crate::Writable for Soctrim1_1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SOCTRIM1_1 to value 0xffff_ffff"]
impl crate::Resettable for Soctrim1_1Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
