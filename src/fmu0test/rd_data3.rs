#[doc = "Register `RD_DATA3` reader"]
pub type R = crate::R<RdData3Spec>;
#[doc = "Register `RD_DATA3` writer"]
pub type W = crate::W<RdData3Spec>;
#[doc = "Field `RD_DATA3` reader - Read Data 3"]
pub type RdData3R = crate::FieldReader<u32>;
#[doc = "Field `RD_DATA3` writer - Read Data 3"]
pub type RdData3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Read Data 3"]
    #[inline(always)]
    pub fn rd_data3(&self) -> RdData3R {
        RdData3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Read Data 3"]
    #[inline(always)]
    pub fn rd_data3(&mut self) -> RdData3W<'_, RdData3Spec> {
        RdData3W::new(self, 0)
    }
}
#[doc = "Read Data 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_data3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rd_data3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RdData3Spec;
impl crate::RegisterSpec for RdData3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_data3::R`](R) reader structure"]
impl crate::Readable for RdData3Spec {}
#[doc = "`write(|w| ..)` method takes [`rd_data3::W`](W) writer structure"]
impl crate::Writable for RdData3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RD_DATA3 to value 0"]
impl crate::Resettable for RdData3Spec {}
