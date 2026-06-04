#[doc = "Register `R_DATA_CTRL1_EX` reader"]
pub type R = crate::R<RDataCtrl1ExSpec>;
#[doc = "Register `R_DATA_CTRL1_EX` writer"]
pub type W = crate::W<RDataCtrl1ExSpec>;
#[doc = "Field `DATA1X` reader - BIST Data 1 High"]
pub type Data1xR = crate::FieldReader;
#[doc = "Field `DATA1X` writer - BIST Data 1 High"]
pub type Data1xW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - BIST Data 1 High"]
    #[inline(always)]
    pub fn data1x(&self) -> Data1xR {
        Data1xR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - BIST Data 1 High"]
    #[inline(always)]
    pub fn data1x(&mut self) -> Data1xW<'_, RDataCtrl1ExSpec> {
        Data1xW::new(self, 0)
    }
}
#[doc = "BIST Data Control 1 Extension Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r_data_ctrl1_ex::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r_data_ctrl1_ex::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RDataCtrl1ExSpec;
impl crate::RegisterSpec for RDataCtrl1ExSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r_data_ctrl1_ex::R`](R) reader structure"]
impl crate::Readable for RDataCtrl1ExSpec {}
#[doc = "`write(|w| ..)` method takes [`r_data_ctrl1_ex::W`](W) writer structure"]
impl crate::Writable for RDataCtrl1ExSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R_DATA_CTRL1_EX to value 0"]
impl crate::Resettable for RDataCtrl1ExSpec {}
