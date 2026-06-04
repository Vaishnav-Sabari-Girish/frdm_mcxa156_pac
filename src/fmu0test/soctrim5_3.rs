#[doc = "Register `SOCTRIM5_3` reader"]
pub type R = crate::R<Soctrim5_3Spec>;
#[doc = "Register `SOCTRIM5_3` writer"]
pub type W = crate::W<Soctrim5_3Spec>;
#[doc = "Field `TRIM5_3` reader - TRIM5_3"]
pub type Trim5_3R = crate::FieldReader<u32>;
#[doc = "Field `TRIM5_3` writer - TRIM5_3"]
pub type Trim5_3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - TRIM5_3"]
    #[inline(always)]
    pub fn trim5_3(&self) -> Trim5_3R {
        Trim5_3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - TRIM5_3"]
    #[inline(always)]
    pub fn trim5_3(&mut self) -> Trim5_3W<'_, Soctrim5_3Spec> {
        Trim5_3W::new(self, 0)
    }
}
#[doc = "SoC Trim Phrase 5 Word 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`soctrim5_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soctrim5_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Soctrim5_3Spec;
impl crate::RegisterSpec for Soctrim5_3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`soctrim5_3::R`](R) reader structure"]
impl crate::Readable for Soctrim5_3Spec {}
#[doc = "`write(|w| ..)` method takes [`soctrim5_3::W`](W) writer structure"]
impl crate::Writable for Soctrim5_3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SOCTRIM5_3 to value 0xffff_ffff"]
impl crate::Resettable for Soctrim5_3Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
