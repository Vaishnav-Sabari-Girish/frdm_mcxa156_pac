#[doc = "Register `SOCTRIM4_3` reader"]
pub type R = crate::R<Soctrim4_3Spec>;
#[doc = "Register `SOCTRIM4_3` writer"]
pub type W = crate::W<Soctrim4_3Spec>;
#[doc = "Field `TRIM4_3` reader - TRIM4_3"]
pub type Trim4_3R = crate::FieldReader<u32>;
#[doc = "Field `TRIM4_3` writer - TRIM4_3"]
pub type Trim4_3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - TRIM4_3"]
    #[inline(always)]
    pub fn trim4_3(&self) -> Trim4_3R {
        Trim4_3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - TRIM4_3"]
    #[inline(always)]
    pub fn trim4_3(&mut self) -> Trim4_3W<'_, Soctrim4_3Spec> {
        Trim4_3W::new(self, 0)
    }
}
#[doc = "SoC Trim Phrase 4 Word 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`soctrim4_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soctrim4_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Soctrim4_3Spec;
impl crate::RegisterSpec for Soctrim4_3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`soctrim4_3::R`](R) reader structure"]
impl crate::Readable for Soctrim4_3Spec {}
#[doc = "`write(|w| ..)` method takes [`soctrim4_3::W`](W) writer structure"]
impl crate::Writable for Soctrim4_3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SOCTRIM4_3 to value 0xffff_ffff"]
impl crate::Resettable for Soctrim4_3Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
