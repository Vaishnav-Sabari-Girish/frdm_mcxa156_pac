#[doc = "Register `R_DATA_CTRL3` reader"]
pub type R = crate::R<RDataCtrl3Spec>;
#[doc = "Register `R_DATA_CTRL3` writer"]
pub type W = crate::W<RDataCtrl3Spec>;
#[doc = "Field `DATA3` reader - BIST Data 3 Low"]
pub type Data3R = crate::FieldReader<u32>;
#[doc = "Field `DATA3` writer - BIST Data 3 Low"]
pub type Data3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - BIST Data 3 Low"]
    #[inline(always)]
    pub fn data3(&self) -> Data3R {
        Data3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - BIST Data 3 Low"]
    #[inline(always)]
    pub fn data3(&mut self) -> Data3W<'_, RDataCtrl3Spec> {
        Data3W::new(self, 0)
    }
}
#[doc = "BIST Data Control 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r_data_ctrl3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r_data_ctrl3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RDataCtrl3Spec;
impl crate::RegisterSpec for RDataCtrl3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r_data_ctrl3::R`](R) reader structure"]
impl crate::Readable for RDataCtrl3Spec {}
#[doc = "`write(|w| ..)` method takes [`r_data_ctrl3::W`](W) writer structure"]
impl crate::Writable for RDataCtrl3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R_DATA_CTRL3 to value 0"]
impl crate::Resettable for RDataCtrl3Spec {}
