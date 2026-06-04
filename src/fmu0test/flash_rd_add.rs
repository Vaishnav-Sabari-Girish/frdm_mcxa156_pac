#[doc = "Register `FLASH_RD_ADD` reader"]
pub type R = crate::R<FlashRdAddSpec>;
#[doc = "Register `FLASH_RD_ADD` writer"]
pub type W = crate::W<FlashRdAddSpec>;
#[doc = "Field `FLASH_RD_ADD` reader - Flash Read Address"]
pub type FlashRdAddR = crate::FieldReader<u32>;
#[doc = "Field `FLASH_RD_ADD` writer - Flash Read Address"]
pub type FlashRdAddW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Flash Read Address"]
    #[inline(always)]
    pub fn flash_rd_add(&self) -> FlashRdAddR {
        FlashRdAddR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Flash Read Address"]
    #[inline(always)]
    pub fn flash_rd_add(&mut self) -> FlashRdAddW<'_, FlashRdAddSpec> {
        FlashRdAddW::new(self, 0)
    }
}
#[doc = "Flash Read Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_rd_add::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_rd_add::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashRdAddSpec;
impl crate::RegisterSpec for FlashRdAddSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_rd_add::R`](R) reader structure"]
impl crate::Readable for FlashRdAddSpec {}
#[doc = "`write(|w| ..)` method takes [`flash_rd_add::W`](W) writer structure"]
impl crate::Writable for FlashRdAddSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLASH_RD_ADD to value 0"]
impl crate::Resettable for FlashRdAddSpec {}
