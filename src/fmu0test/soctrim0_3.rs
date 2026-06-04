#[doc = "Register `SOCTRIM0_3` reader"]
pub type R = crate::R<Soctrim0_3Spec>;
#[doc = "Register `SOCTRIM0_3` writer"]
pub type W = crate::W<Soctrim0_3Spec>;
#[doc = "Field `TRIM0_3` reader - TRIM0_3"]
pub type Trim0_3R = crate::FieldReader<u32>;
#[doc = "Field `TRIM0_3` writer - TRIM0_3"]
pub type Trim0_3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - TRIM0_3"]
    #[inline(always)]
    pub fn trim0_3(&self) -> Trim0_3R {
        Trim0_3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - TRIM0_3"]
    #[inline(always)]
    pub fn trim0_3(&mut self) -> Trim0_3W<'_, Soctrim0_3Spec> {
        Trim0_3W::new(self, 0)
    }
}
#[doc = "SoC Trim Phrase 0 Word 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`soctrim0_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soctrim0_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Soctrim0_3Spec;
impl crate::RegisterSpec for Soctrim0_3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`soctrim0_3::R`](R) reader structure"]
impl crate::Readable for Soctrim0_3Spec {}
#[doc = "`write(|w| ..)` method takes [`soctrim0_3::W`](W) writer structure"]
impl crate::Writable for Soctrim0_3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SOCTRIM0_3 to value 0xffff_ffff"]
impl crate::Resettable for Soctrim0_3Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
