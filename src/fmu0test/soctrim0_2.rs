#[doc = "Register `SOCTRIM0_2` reader"]
pub type R = crate::R<Soctrim0_2Spec>;
#[doc = "Register `SOCTRIM0_2` writer"]
pub type W = crate::W<Soctrim0_2Spec>;
#[doc = "Field `TRIM0_2` reader - TRIM0_2"]
pub type Trim0_2R = crate::FieldReader<u32>;
#[doc = "Field `TRIM0_2` writer - TRIM0_2"]
pub type Trim0_2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - TRIM0_2"]
    #[inline(always)]
    pub fn trim0_2(&self) -> Trim0_2R {
        Trim0_2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - TRIM0_2"]
    #[inline(always)]
    pub fn trim0_2(&mut self) -> Trim0_2W<'_, Soctrim0_2Spec> {
        Trim0_2W::new(self, 0)
    }
}
#[doc = "SoC Trim Phrase 0 Word 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`soctrim0_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soctrim0_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Soctrim0_2Spec;
impl crate::RegisterSpec for Soctrim0_2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`soctrim0_2::R`](R) reader structure"]
impl crate::Readable for Soctrim0_2Spec {}
#[doc = "`write(|w| ..)` method takes [`soctrim0_2::W`](W) writer structure"]
impl crate::Writable for Soctrim0_2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SOCTRIM0_2 to value 0xffff_ffff"]
impl crate::Resettable for Soctrim0_2Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
