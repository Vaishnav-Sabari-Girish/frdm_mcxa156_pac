#[doc = "Register `R_DATA_CTRL2` reader"]
pub type R = crate::R<RDataCtrl2Spec>;
#[doc = "Register `R_DATA_CTRL2` writer"]
pub type W = crate::W<RDataCtrl2Spec>;
#[doc = "Field `DATA2` reader - BIST Data 2 Low"]
pub type Data2R = crate::FieldReader<u32>;
#[doc = "Field `DATA2` writer - BIST Data 2 Low"]
pub type Data2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - BIST Data 2 Low"]
    #[inline(always)]
    pub fn data2(&self) -> Data2R {
        Data2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - BIST Data 2 Low"]
    #[inline(always)]
    pub fn data2(&mut self) -> Data2W<'_, RDataCtrl2Spec> {
        Data2W::new(self, 0)
    }
}
#[doc = "BIST Data Control 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r_data_ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r_data_ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RDataCtrl2Spec;
impl crate::RegisterSpec for RDataCtrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r_data_ctrl2::R`](R) reader structure"]
impl crate::Readable for RDataCtrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`r_data_ctrl2::W`](W) writer structure"]
impl crate::Writable for RDataCtrl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R_DATA_CTRL2 to value 0"]
impl crate::Resettable for RDataCtrl2Spec {}
