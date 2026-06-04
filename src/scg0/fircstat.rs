#[doc = "Register `FIRCSTAT` reader"]
pub type R = crate::R<FircstatSpec>;
#[doc = "Register `FIRCSTAT` writer"]
pub type W = crate::W<FircstatSpec>;
#[doc = "Field `TRIMFINE` reader - Trim Fine"]
pub type TrimfineR = crate::FieldReader;
#[doc = "Field `TRIMFINE` writer - Trim Fine"]
pub type TrimfineW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TRIMCOAR` reader - Trim Coarse"]
pub type TrimcoarR = crate::FieldReader;
#[doc = "Field `TRIMCOAR` writer - Trim Coarse"]
pub type TrimcoarW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:7 - Trim Fine"]
    #[inline(always)]
    pub fn trimfine(&self) -> TrimfineR {
        TrimfineR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:13 - Trim Coarse"]
    #[inline(always)]
    pub fn trimcoar(&self) -> TrimcoarR {
        TrimcoarR::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Trim Fine"]
    #[inline(always)]
    pub fn trimfine(&mut self) -> TrimfineW<'_, FircstatSpec> {
        TrimfineW::new(self, 0)
    }
    #[doc = "Bits 8:13 - Trim Coarse"]
    #[inline(always)]
    pub fn trimcoar(&mut self) -> TrimcoarW<'_, FircstatSpec> {
        TrimcoarW::new(self, 8)
    }
}
#[doc = "FIRC Auto-trimming Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fircstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fircstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FircstatSpec;
impl crate::RegisterSpec for FircstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fircstat::R`](R) reader structure"]
impl crate::Readable for FircstatSpec {}
#[doc = "`write(|w| ..)` method takes [`fircstat::W`](W) writer structure"]
impl crate::Writable for FircstatSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIRCSTAT to value 0"]
impl crate::Resettable for FircstatSpec {}
