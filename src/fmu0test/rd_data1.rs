#[doc = "Register `RD_DATA1` reader"]
pub type R = crate::R<RdData1Spec>;
#[doc = "Register `RD_DATA1` writer"]
pub type W = crate::W<RdData1Spec>;
#[doc = "Field `RD_DATA1` reader - Read Data 1"]
pub type RdData1R = crate::FieldReader<u32>;
#[doc = "Field `RD_DATA1` writer - Read Data 1"]
pub type RdData1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Read Data 1"]
    #[inline(always)]
    pub fn rd_data1(&self) -> RdData1R {
        RdData1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Read Data 1"]
    #[inline(always)]
    pub fn rd_data1(&mut self) -> RdData1W<'_, RdData1Spec> {
        RdData1W::new(self, 0)
    }
}
#[doc = "Read Data 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_data1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rd_data1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RdData1Spec;
impl crate::RegisterSpec for RdData1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_data1::R`](R) reader structure"]
impl crate::Readable for RdData1Spec {}
#[doc = "`write(|w| ..)` method takes [`rd_data1::W`](W) writer structure"]
impl crate::Writable for RdData1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RD_DATA1 to value 0"]
impl crate::Resettable for RdData1Spec {}
