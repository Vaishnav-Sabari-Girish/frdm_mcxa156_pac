#[doc = "Register `R_DATA_CTRL0` reader"]
pub type R = crate::R<RDataCtrl0Spec>;
#[doc = "Register `R_DATA_CTRL0` writer"]
pub type W = crate::W<RDataCtrl0Spec>;
#[doc = "Field `DATA0` reader - BIST Data 0 Low"]
pub type Data0R = crate::FieldReader<u32>;
#[doc = "Field `DATA0` writer - BIST Data 0 Low"]
pub type Data0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - BIST Data 0 Low"]
    #[inline(always)]
    pub fn data0(&self) -> Data0R {
        Data0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - BIST Data 0 Low"]
    #[inline(always)]
    pub fn data0(&mut self) -> Data0W<'_, RDataCtrl0Spec> {
        Data0W::new(self, 0)
    }
}
#[doc = "BIST Data Control 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r_data_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r_data_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RDataCtrl0Spec;
impl crate::RegisterSpec for RDataCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r_data_ctrl0::R`](R) reader structure"]
impl crate::Readable for RDataCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`r_data_ctrl0::W`](W) writer structure"]
impl crate::Writable for RDataCtrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R_DATA_CTRL0 to value 0"]
impl crate::Resettable for RDataCtrl0Spec {}
