#[doc = "Register `SOCTRIM4_2` reader"]
pub type R = crate::R<Soctrim4_2Spec>;
#[doc = "Register `SOCTRIM4_2` writer"]
pub type W = crate::W<Soctrim4_2Spec>;
#[doc = "Field `TRIM4_2` reader - TRIM4_2"]
pub type Trim4_2R = crate::FieldReader<u32>;
#[doc = "Field `TRIM4_2` writer - TRIM4_2"]
pub type Trim4_2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - TRIM4_2"]
    #[inline(always)]
    pub fn trim4_2(&self) -> Trim4_2R {
        Trim4_2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - TRIM4_2"]
    #[inline(always)]
    pub fn trim4_2(&mut self) -> Trim4_2W<'_, Soctrim4_2Spec> {
        Trim4_2W::new(self, 0)
    }
}
#[doc = "SoC Trim Phrase 4 Word 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`soctrim4_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soctrim4_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Soctrim4_2Spec;
impl crate::RegisterSpec for Soctrim4_2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`soctrim4_2::R`](R) reader structure"]
impl crate::Readable for Soctrim4_2Spec {}
#[doc = "`write(|w| ..)` method takes [`soctrim4_2::W`](W) writer structure"]
impl crate::Writable for Soctrim4_2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SOCTRIM4_2 to value 0xffff_ffff"]
impl crate::Resettable for Soctrim4_2Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
