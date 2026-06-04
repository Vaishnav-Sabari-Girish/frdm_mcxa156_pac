#[doc = "Register `SOCTRIM2_1` reader"]
pub type R = crate::R<Soctrim2_1Spec>;
#[doc = "Register `SOCTRIM2_1` writer"]
pub type W = crate::W<Soctrim2_1Spec>;
#[doc = "Field `TRIM2_1` reader - TRIM2_1"]
pub type Trim2_1R = crate::FieldReader<u32>;
#[doc = "Field `TRIM2_1` writer - TRIM2_1"]
pub type Trim2_1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - TRIM2_1"]
    #[inline(always)]
    pub fn trim2_1(&self) -> Trim2_1R {
        Trim2_1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - TRIM2_1"]
    #[inline(always)]
    pub fn trim2_1(&mut self) -> Trim2_1W<'_, Soctrim2_1Spec> {
        Trim2_1W::new(self, 0)
    }
}
#[doc = "SoC Trim Phrase 2 Word 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`soctrim2_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soctrim2_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Soctrim2_1Spec;
impl crate::RegisterSpec for Soctrim2_1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`soctrim2_1::R`](R) reader structure"]
impl crate::Readable for Soctrim2_1Spec {}
#[doc = "`write(|w| ..)` method takes [`soctrim2_1::W`](W) writer structure"]
impl crate::Writable for Soctrim2_1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SOCTRIM2_1 to value 0xffff_ffff"]
impl crate::Resettable for Soctrim2_1Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
