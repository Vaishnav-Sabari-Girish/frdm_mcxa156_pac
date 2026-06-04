#[doc = "Register `RD_DATA0` reader"]
pub type R = crate::R<RdData0Spec>;
#[doc = "Register `RD_DATA0` writer"]
pub type W = crate::W<RdData0Spec>;
#[doc = "Field `RD_DATA0` reader - Read Data 0"]
pub type RdData0R = crate::FieldReader<u32>;
#[doc = "Field `RD_DATA0` writer - Read Data 0"]
pub type RdData0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Read Data 0"]
    #[inline(always)]
    pub fn rd_data0(&self) -> RdData0R {
        RdData0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Read Data 0"]
    #[inline(always)]
    pub fn rd_data0(&mut self) -> RdData0W<'_, RdData0Spec> {
        RdData0W::new(self, 0)
    }
}
#[doc = "Read Data 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_data0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rd_data0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RdData0Spec;
impl crate::RegisterSpec for RdData0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_data0::R`](R) reader structure"]
impl crate::Readable for RdData0Spec {}
#[doc = "`write(|w| ..)` method takes [`rd_data0::W`](W) writer structure"]
impl crate::Writable for RdData0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RD_DATA0 to value 0"]
impl crate::Resettable for RdData0Spec {}
