#[doc = "Register `SOCTRIM5_1` reader"]
pub type R = crate::R<Soctrim5_1Spec>;
#[doc = "Register `SOCTRIM5_1` writer"]
pub type W = crate::W<Soctrim5_1Spec>;
#[doc = "Field `TRIM5_1` reader - TRIM5_1"]
pub type Trim5_1R = crate::FieldReader<u32>;
#[doc = "Field `TRIM5_1` writer - TRIM5_1"]
pub type Trim5_1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - TRIM5_1"]
    #[inline(always)]
    pub fn trim5_1(&self) -> Trim5_1R {
        Trim5_1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - TRIM5_1"]
    #[inline(always)]
    pub fn trim5_1(&mut self) -> Trim5_1W<'_, Soctrim5_1Spec> {
        Trim5_1W::new(self, 0)
    }
}
#[doc = "SoC Trim Phrase 5 Word 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`soctrim5_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soctrim5_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Soctrim5_1Spec;
impl crate::RegisterSpec for Soctrim5_1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`soctrim5_1::R`](R) reader structure"]
impl crate::Readable for Soctrim5_1Spec {}
#[doc = "`write(|w| ..)` method takes [`soctrim5_1::W`](W) writer structure"]
impl crate::Writable for Soctrim5_1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SOCTRIM5_1 to value 0xffff_ffff"]
impl crate::Resettable for Soctrim5_1Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
