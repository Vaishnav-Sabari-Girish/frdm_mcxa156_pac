#[doc = "Register `SOCTRIM5_2` reader"]
pub type R = crate::R<Soctrim5_2Spec>;
#[doc = "Register `SOCTRIM5_2` writer"]
pub type W = crate::W<Soctrim5_2Spec>;
#[doc = "Field `TRIM5_2` reader - TRIM5_2"]
pub type Trim5_2R = crate::FieldReader<u32>;
#[doc = "Field `TRIM5_2` writer - TRIM5_2"]
pub type Trim5_2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - TRIM5_2"]
    #[inline(always)]
    pub fn trim5_2(&self) -> Trim5_2R {
        Trim5_2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - TRIM5_2"]
    #[inline(always)]
    pub fn trim5_2(&mut self) -> Trim5_2W<'_, Soctrim5_2Spec> {
        Trim5_2W::new(self, 0)
    }
}
#[doc = "SoC Trim Phrase 5 Word 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`soctrim5_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soctrim5_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Soctrim5_2Spec;
impl crate::RegisterSpec for Soctrim5_2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`soctrim5_2::R`](R) reader structure"]
impl crate::Readable for Soctrim5_2Spec {}
#[doc = "`write(|w| ..)` method takes [`soctrim5_2::W`](W) writer structure"]
impl crate::Writable for Soctrim5_2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SOCTRIM5_2 to value 0xffff_ffff"]
impl crate::Resettable for Soctrim5_2Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
