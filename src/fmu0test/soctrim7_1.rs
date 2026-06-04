#[doc = "Register `SOCTRIM7_1` reader"]
pub type R = crate::R<Soctrim7_1Spec>;
#[doc = "Register `SOCTRIM7_1` writer"]
pub type W = crate::W<Soctrim7_1Spec>;
#[doc = "Field `TRIM7_1` reader - TRIM7_1"]
pub type Trim7_1R = crate::FieldReader<u32>;
#[doc = "Field `TRIM7_1` writer - TRIM7_1"]
pub type Trim7_1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - TRIM7_1"]
    #[inline(always)]
    pub fn trim7_1(&self) -> Trim7_1R {
        Trim7_1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - TRIM7_1"]
    #[inline(always)]
    pub fn trim7_1(&mut self) -> Trim7_1W<'_, Soctrim7_1Spec> {
        Trim7_1W::new(self, 0)
    }
}
#[doc = "SoC Trim Phrase 7 Word 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`soctrim7_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soctrim7_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Soctrim7_1Spec;
impl crate::RegisterSpec for Soctrim7_1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`soctrim7_1::R`](R) reader structure"]
impl crate::Readable for Soctrim7_1Spec {}
#[doc = "`write(|w| ..)` method takes [`soctrim7_1::W`](W) writer structure"]
impl crate::Writable for Soctrim7_1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SOCTRIM7_1 to value 0xffff_ffff"]
impl crate::Resettable for Soctrim7_1Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
