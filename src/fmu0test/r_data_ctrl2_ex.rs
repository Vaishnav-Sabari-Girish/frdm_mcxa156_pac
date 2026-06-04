#[doc = "Register `R_DATA_CTRL2_EX` reader"]
pub type R = crate::R<RDataCtrl2ExSpec>;
#[doc = "Register `R_DATA_CTRL2_EX` writer"]
pub type W = crate::W<RDataCtrl2ExSpec>;
#[doc = "Field `DATA2X` reader - BIST Data 2 High"]
pub type Data2xR = crate::FieldReader;
#[doc = "Field `DATA2X` writer - BIST Data 2 High"]
pub type Data2xW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - BIST Data 2 High"]
    #[inline(always)]
    pub fn data2x(&self) -> Data2xR {
        Data2xR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - BIST Data 2 High"]
    #[inline(always)]
    pub fn data2x(&mut self) -> Data2xW<'_, RDataCtrl2ExSpec> {
        Data2xW::new(self, 0)
    }
}
#[doc = "BIST Data Control 2 Extension Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r_data_ctrl2_ex::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r_data_ctrl2_ex::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RDataCtrl2ExSpec;
impl crate::RegisterSpec for RDataCtrl2ExSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r_data_ctrl2_ex::R`](R) reader structure"]
impl crate::Readable for RDataCtrl2ExSpec {}
#[doc = "`write(|w| ..)` method takes [`r_data_ctrl2_ex::W`](W) writer structure"]
impl crate::Writable for RDataCtrl2ExSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R_DATA_CTRL2_EX to value 0"]
impl crate::Resettable for RDataCtrl2ExSpec {}
