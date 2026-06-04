#[doc = "Register `SOCTRIM0_0` reader"]
pub type R = crate::R<Soctrim0_0Spec>;
#[doc = "Register `SOCTRIM0_0` writer"]
pub type W = crate::W<Soctrim0_0Spec>;
#[doc = "Field `TRIM0_0` reader - TRIM0_0"]
pub type Trim0_0R = crate::FieldReader<u32>;
#[doc = "Field `TRIM0_0` writer - TRIM0_0"]
pub type Trim0_0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - TRIM0_0"]
    #[inline(always)]
    pub fn trim0_0(&self) -> Trim0_0R {
        Trim0_0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - TRIM0_0"]
    #[inline(always)]
    pub fn trim0_0(&mut self) -> Trim0_0W<'_, Soctrim0_0Spec> {
        Trim0_0W::new(self, 0)
    }
}
#[doc = "SoC Trim Phrase 0 Word 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`soctrim0_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soctrim0_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Soctrim0_0Spec;
impl crate::RegisterSpec for Soctrim0_0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`soctrim0_0::R`](R) reader structure"]
impl crate::Readable for Soctrim0_0Spec {}
#[doc = "`write(|w| ..)` method takes [`soctrim0_0::W`](W) writer structure"]
impl crate::Writable for Soctrim0_0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SOCTRIM0_0 to value 0xffff_ffff"]
impl crate::Resettable for Soctrim0_0Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
