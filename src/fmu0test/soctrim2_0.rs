#[doc = "Register `SOCTRIM2_0` reader"]
pub type R = crate::R<Soctrim2_0Spec>;
#[doc = "Register `SOCTRIM2_0` writer"]
pub type W = crate::W<Soctrim2_0Spec>;
#[doc = "Field `TRIM2_0` reader - TRIM2_0"]
pub type Trim2_0R = crate::FieldReader<u32>;
#[doc = "Field `TRIM2_0` writer - TRIM2_0"]
pub type Trim2_0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - TRIM2_0"]
    #[inline(always)]
    pub fn trim2_0(&self) -> Trim2_0R {
        Trim2_0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - TRIM2_0"]
    #[inline(always)]
    pub fn trim2_0(&mut self) -> Trim2_0W<'_, Soctrim2_0Spec> {
        Trim2_0W::new(self, 0)
    }
}
#[doc = "SoC Trim Phrase 2 Word 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`soctrim2_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soctrim2_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Soctrim2_0Spec;
impl crate::RegisterSpec for Soctrim2_0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`soctrim2_0::R`](R) reader structure"]
impl crate::Readable for Soctrim2_0Spec {}
#[doc = "`write(|w| ..)` method takes [`soctrim2_0::W`](W) writer structure"]
impl crate::Writable for Soctrim2_0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SOCTRIM2_0 to value 0xffff_ffff"]
impl crate::Resettable for Soctrim2_0Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
