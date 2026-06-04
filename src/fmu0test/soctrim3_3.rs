#[doc = "Register `SOCTRIM3_3` reader"]
pub type R = crate::R<Soctrim3_3Spec>;
#[doc = "Register `SOCTRIM3_3` writer"]
pub type W = crate::W<Soctrim3_3Spec>;
#[doc = "Field `TRIM3_3` reader - TRIM3_3"]
pub type Trim3_3R = crate::FieldReader<u32>;
#[doc = "Field `TRIM3_3` writer - TRIM3_3"]
pub type Trim3_3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - TRIM3_3"]
    #[inline(always)]
    pub fn trim3_3(&self) -> Trim3_3R {
        Trim3_3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - TRIM3_3"]
    #[inline(always)]
    pub fn trim3_3(&mut self) -> Trim3_3W<'_, Soctrim3_3Spec> {
        Trim3_3W::new(self, 0)
    }
}
#[doc = "SoC Trim Phrase 3 Word 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`soctrim3_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soctrim3_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Soctrim3_3Spec;
impl crate::RegisterSpec for Soctrim3_3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`soctrim3_3::R`](R) reader structure"]
impl crate::Readable for Soctrim3_3Spec {}
#[doc = "`write(|w| ..)` method takes [`soctrim3_3::W`](W) writer structure"]
impl crate::Writable for Soctrim3_3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SOCTRIM3_3 to value 0xffff_ffff"]
impl crate::Resettable for Soctrim3_3Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
