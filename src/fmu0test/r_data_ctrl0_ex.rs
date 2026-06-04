#[doc = "Register `R_DATA_CTRL0_EX` reader"]
pub type R = crate::R<RDataCtrl0ExSpec>;
#[doc = "Register `R_DATA_CTRL0_EX` writer"]
pub type W = crate::W<RDataCtrl0ExSpec>;
#[doc = "Field `DATA0X` reader - BIST Data 0 High"]
pub type Data0xR = crate::FieldReader;
#[doc = "Field `DATA0X` writer - BIST Data 0 High"]
pub type Data0xW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - BIST Data 0 High"]
    #[inline(always)]
    pub fn data0x(&self) -> Data0xR {
        Data0xR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - BIST Data 0 High"]
    #[inline(always)]
    pub fn data0x(&mut self) -> Data0xW<'_, RDataCtrl0ExSpec> {
        Data0xW::new(self, 0)
    }
}
#[doc = "BIST Data Control 0 Extension Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r_data_ctrl0_ex::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r_data_ctrl0_ex::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RDataCtrl0ExSpec;
impl crate::RegisterSpec for RDataCtrl0ExSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r_data_ctrl0_ex::R`](R) reader structure"]
impl crate::Readable for RDataCtrl0ExSpec {}
#[doc = "`write(|w| ..)` method takes [`r_data_ctrl0_ex::W`](W) writer structure"]
impl crate::Writable for RDataCtrl0ExSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R_DATA_CTRL0_EX to value 0"]
impl crate::Resettable for RDataCtrl0ExSpec {}
