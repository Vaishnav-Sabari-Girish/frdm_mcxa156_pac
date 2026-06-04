#[doc = "Register `FLASH_STOP_ADD` reader"]
pub type R = crate::R<FlashStopAddSpec>;
#[doc = "Register `FLASH_STOP_ADD` writer"]
pub type W = crate::W<FlashStopAddSpec>;
#[doc = "Field `FLASH_STOP_ADD` reader - Flash Stop Address"]
pub type FlashStopAddR = crate::FieldReader<u32>;
#[doc = "Field `FLASH_STOP_ADD` writer - Flash Stop Address"]
pub type FlashStopAddW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Flash Stop Address"]
    #[inline(always)]
    pub fn flash_stop_add(&self) -> FlashStopAddR {
        FlashStopAddR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Flash Stop Address"]
    #[inline(always)]
    pub fn flash_stop_add(&mut self) -> FlashStopAddW<'_, FlashStopAddSpec> {
        FlashStopAddW::new(self, 0)
    }
}
#[doc = "Flash Stop Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_stop_add::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_stop_add::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashStopAddSpec;
impl crate::RegisterSpec for FlashStopAddSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_stop_add::R`](R) reader structure"]
impl crate::Readable for FlashStopAddSpec {}
#[doc = "`write(|w| ..)` method takes [`flash_stop_add::W`](W) writer structure"]
impl crate::Writable for FlashStopAddSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLASH_STOP_ADD to value 0"]
impl crate::Resettable for FlashStopAddSpec {}
