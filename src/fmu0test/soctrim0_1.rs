#[doc = "Register `SOCTRIM0_1` reader"]
pub type R = crate::R<Soctrim0_1Spec>;
#[doc = "Register `SOCTRIM0_1` writer"]
pub type W = crate::W<Soctrim0_1Spec>;
#[doc = "Field `TRIM0_1` reader - TRIM0_1"]
pub type Trim0_1R = crate::FieldReader<u32>;
#[doc = "Field `TRIM0_1` writer - TRIM0_1"]
pub type Trim0_1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - TRIM0_1"]
    #[inline(always)]
    pub fn trim0_1(&self) -> Trim0_1R {
        Trim0_1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - TRIM0_1"]
    #[inline(always)]
    pub fn trim0_1(&mut self) -> Trim0_1W<'_, Soctrim0_1Spec> {
        Trim0_1W::new(self, 0)
    }
}
#[doc = "SoC Trim Phrase 0 Word 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`soctrim0_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soctrim0_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Soctrim0_1Spec;
impl crate::RegisterSpec for Soctrim0_1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`soctrim0_1::R`](R) reader structure"]
impl crate::Readable for Soctrim0_1Spec {}
#[doc = "`write(|w| ..)` method takes [`soctrim0_1::W`](W) writer structure"]
impl crate::Writable for Soctrim0_1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SOCTRIM0_1 to value 0xffff_ffff"]
impl crate::Resettable for Soctrim0_1Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
