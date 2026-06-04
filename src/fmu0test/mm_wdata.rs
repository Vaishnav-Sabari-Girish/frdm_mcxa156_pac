#[doc = "Register `MM_WDATA` reader"]
pub type R = crate::R<MmWdataSpec>;
#[doc = "Register `MM_WDATA` writer"]
pub type W = crate::W<MmWdataSpec>;
#[doc = "Field `MM_WDATA` reader - Memory Map Write Data"]
pub type MmWdataR = crate::FieldReader<u32>;
#[doc = "Field `MM_WDATA` writer - Memory Map Write Data"]
pub type MmWdataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Memory Map Write Data"]
    #[inline(always)]
    pub fn mm_wdata(&self) -> MmWdataR {
        MmWdataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Memory Map Write Data"]
    #[inline(always)]
    pub fn mm_wdata(&mut self) -> MmWdataW<'_, MmWdataSpec> {
        MmWdataW::new(self, 0)
    }
}
#[doc = "Memory Map Write Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mm_wdata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mm_wdata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmWdataSpec;
impl crate::RegisterSpec for MmWdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mm_wdata::R`](R) reader structure"]
impl crate::Readable for MmWdataSpec {}
#[doc = "`write(|w| ..)` method takes [`mm_wdata::W`](W) writer structure"]
impl crate::Writable for MmWdataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MM_WDATA to value 0"]
impl crate::Resettable for MmWdataSpec {}
