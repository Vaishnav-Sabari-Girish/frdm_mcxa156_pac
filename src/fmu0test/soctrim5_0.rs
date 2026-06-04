#[doc = "Register `SOCTRIM5_0` reader"]
pub type R = crate::R<Soctrim5_0Spec>;
#[doc = "Register `SOCTRIM5_0` writer"]
pub type W = crate::W<Soctrim5_0Spec>;
#[doc = "Field `TRIM5_0` reader - TRIM5_0"]
pub type Trim5_0R = crate::FieldReader<u32>;
#[doc = "Field `TRIM5_0` writer - TRIM5_0"]
pub type Trim5_0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - TRIM5_0"]
    #[inline(always)]
    pub fn trim5_0(&self) -> Trim5_0R {
        Trim5_0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - TRIM5_0"]
    #[inline(always)]
    pub fn trim5_0(&mut self) -> Trim5_0W<'_, Soctrim5_0Spec> {
        Trim5_0W::new(self, 0)
    }
}
#[doc = "SoC Trim Phrase 5 Word 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`soctrim5_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soctrim5_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Soctrim5_0Spec;
impl crate::RegisterSpec for Soctrim5_0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`soctrim5_0::R`](R) reader structure"]
impl crate::Readable for Soctrim5_0Spec {}
#[doc = "`write(|w| ..)` method takes [`soctrim5_0::W`](W) writer structure"]
impl crate::Writable for Soctrim5_0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SOCTRIM5_0 to value 0xffff_ffff"]
impl crate::Resettable for Soctrim5_0Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
