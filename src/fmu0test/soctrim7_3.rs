#[doc = "Register `SOCTRIM7_3` reader"]
pub type R = crate::R<Soctrim7_3Spec>;
#[doc = "Register `SOCTRIM7_3` writer"]
pub type W = crate::W<Soctrim7_3Spec>;
#[doc = "Field `TRIM7_3` reader - TRIM7_3"]
pub type Trim7_3R = crate::FieldReader<u32>;
#[doc = "Field `TRIM7_3` writer - TRIM7_3"]
pub type Trim7_3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - TRIM7_3"]
    #[inline(always)]
    pub fn trim7_3(&self) -> Trim7_3R {
        Trim7_3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - TRIM7_3"]
    #[inline(always)]
    pub fn trim7_3(&mut self) -> Trim7_3W<'_, Soctrim7_3Spec> {
        Trim7_3W::new(self, 0)
    }
}
#[doc = "SoC Trim Phrase 7 Word 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`soctrim7_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soctrim7_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Soctrim7_3Spec;
impl crate::RegisterSpec for Soctrim7_3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`soctrim7_3::R`](R) reader structure"]
impl crate::Readable for Soctrim7_3Spec {}
#[doc = "`write(|w| ..)` method takes [`soctrim7_3::W`](W) writer structure"]
impl crate::Writable for Soctrim7_3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SOCTRIM7_3 to value 0xffff_ffff"]
impl crate::Resettable for Soctrim7_3Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
