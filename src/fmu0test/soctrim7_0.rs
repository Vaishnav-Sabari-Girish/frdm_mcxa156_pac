#[doc = "Register `SOCTRIM7_0` reader"]
pub type R = crate::R<Soctrim7_0Spec>;
#[doc = "Register `SOCTRIM7_0` writer"]
pub type W = crate::W<Soctrim7_0Spec>;
#[doc = "Field `TRIM7_0` reader - TRIM7_0"]
pub type Trim7_0R = crate::FieldReader<u32>;
#[doc = "Field `TRIM7_0` writer - TRIM7_0"]
pub type Trim7_0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - TRIM7_0"]
    #[inline(always)]
    pub fn trim7_0(&self) -> Trim7_0R {
        Trim7_0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - TRIM7_0"]
    #[inline(always)]
    pub fn trim7_0(&mut self) -> Trim7_0W<'_, Soctrim7_0Spec> {
        Trim7_0W::new(self, 0)
    }
}
#[doc = "SoC Trim Phrase 7 Word 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`soctrim7_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soctrim7_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Soctrim7_0Spec;
impl crate::RegisterSpec for Soctrim7_0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`soctrim7_0::R`](R) reader structure"]
impl crate::Readable for Soctrim7_0Spec {}
#[doc = "`write(|w| ..)` method takes [`soctrim7_0::W`](W) writer structure"]
impl crate::Writable for Soctrim7_0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SOCTRIM7_0 to value 0xffff_ffff"]
impl crate::Resettable for Soctrim7_0Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
