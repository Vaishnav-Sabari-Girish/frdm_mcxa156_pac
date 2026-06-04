#[doc = "Register `SOCTRIM7_2` reader"]
pub type R = crate::R<Soctrim7_2Spec>;
#[doc = "Register `SOCTRIM7_2` writer"]
pub type W = crate::W<Soctrim7_2Spec>;
#[doc = "Field `TRIM7_2` reader - TRIM7_2"]
pub type Trim7_2R = crate::FieldReader<u32>;
#[doc = "Field `TRIM7_2` writer - TRIM7_2"]
pub type Trim7_2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - TRIM7_2"]
    #[inline(always)]
    pub fn trim7_2(&self) -> Trim7_2R {
        Trim7_2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - TRIM7_2"]
    #[inline(always)]
    pub fn trim7_2(&mut self) -> Trim7_2W<'_, Soctrim7_2Spec> {
        Trim7_2W::new(self, 0)
    }
}
#[doc = "SoC Trim Phrase 7 Word 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`soctrim7_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soctrim7_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Soctrim7_2Spec;
impl crate::RegisterSpec for Soctrim7_2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`soctrim7_2::R`](R) reader structure"]
impl crate::Readable for Soctrim7_2Spec {}
#[doc = "`write(|w| ..)` method takes [`soctrim7_2::W`](W) writer structure"]
impl crate::Writable for Soctrim7_2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SOCTRIM7_2 to value 0xffff_ffff"]
impl crate::Resettable for Soctrim7_2Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
