#[doc = "Register `R_DATA_CTRL1` reader"]
pub type R = crate::R<RDataCtrl1Spec>;
#[doc = "Register `R_DATA_CTRL1` writer"]
pub type W = crate::W<RDataCtrl1Spec>;
#[doc = "Field `DATA1` reader - BIST Data 1 Low"]
pub type Data1R = crate::FieldReader<u32>;
#[doc = "Field `DATA1` writer - BIST Data 1 Low"]
pub type Data1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - BIST Data 1 Low"]
    #[inline(always)]
    pub fn data1(&self) -> Data1R {
        Data1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - BIST Data 1 Low"]
    #[inline(always)]
    pub fn data1(&mut self) -> Data1W<'_, RDataCtrl1Spec> {
        Data1W::new(self, 0)
    }
}
#[doc = "BIST Data Control 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r_data_ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r_data_ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RDataCtrl1Spec;
impl crate::RegisterSpec for RDataCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r_data_ctrl1::R`](R) reader structure"]
impl crate::Readable for RDataCtrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`r_data_ctrl1::W`](W) writer structure"]
impl crate::Writable for RDataCtrl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R_DATA_CTRL1 to value 0"]
impl crate::Resettable for RDataCtrl1Spec {}
