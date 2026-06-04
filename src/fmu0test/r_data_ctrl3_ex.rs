#[doc = "Register `R_DATA_CTRL3_EX` reader"]
pub type R = crate::R<RDataCtrl3ExSpec>;
#[doc = "Register `R_DATA_CTRL3_EX` writer"]
pub type W = crate::W<RDataCtrl3ExSpec>;
#[doc = "Field `DATA3X` reader - BIST Data 3 High"]
pub type Data3xR = crate::FieldReader;
#[doc = "Field `DATA3X` writer - BIST Data 3 High"]
pub type Data3xW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - BIST Data 3 High"]
    #[inline(always)]
    pub fn data3x(&self) -> Data3xR {
        Data3xR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - BIST Data 3 High"]
    #[inline(always)]
    pub fn data3x(&mut self) -> Data3xW<'_, RDataCtrl3ExSpec> {
        Data3xW::new(self, 0)
    }
}
#[doc = "BIST Data Control 3 Extension Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r_data_ctrl3_ex::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r_data_ctrl3_ex::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RDataCtrl3ExSpec;
impl crate::RegisterSpec for RDataCtrl3ExSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r_data_ctrl3_ex::R`](R) reader structure"]
impl crate::Readable for RDataCtrl3ExSpec {}
#[doc = "`write(|w| ..)` method takes [`r_data_ctrl3_ex::W`](W) writer structure"]
impl crate::Writable for RDataCtrl3ExSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R_DATA_CTRL3_EX to value 0"]
impl crate::Resettable for RDataCtrl3ExSpec {}
