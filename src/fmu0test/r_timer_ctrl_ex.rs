#[doc = "Register `R_TIMER_CTRL_EX` reader"]
pub type R = crate::R<RTimerCtrlExSpec>;
#[doc = "Register `R_TIMER_CTRL_EX` writer"]
pub type W = crate::W<RTimerCtrlExSpec>;
#[doc = "Field `TLVSDLY_H` reader - Tlvs Time Delay Scalar High"]
pub type TlvsdlyHR = crate::FieldReader;
#[doc = "Field `TLVSDLY_H` writer - Tlvs Time Delay Scalar High"]
pub type TlvsdlyHW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Tlvs Time Delay Scalar High"]
    #[inline(always)]
    pub fn tlvsdly_h(&self) -> TlvsdlyHR {
        TlvsdlyHR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Tlvs Time Delay Scalar High"]
    #[inline(always)]
    pub fn tlvsdly_h(&mut self) -> TlvsdlyHW<'_, RTimerCtrlExSpec> {
        TlvsdlyHW::new(self, 0)
    }
}
#[doc = "BIST Timer Control Extension Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r_timer_ctrl_ex::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r_timer_ctrl_ex::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTimerCtrlExSpec;
impl crate::RegisterSpec for RTimerCtrlExSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r_timer_ctrl_ex::R`](R) reader structure"]
impl crate::Readable for RTimerCtrlExSpec {}
#[doc = "`write(|w| ..)` method takes [`r_timer_ctrl_ex::W`](W) writer structure"]
impl crate::Writable for RTimerCtrlExSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R_TIMER_CTRL_EX to value 0x01"]
impl crate::Resettable for RTimerCtrlExSpec {
    const RESET_VALUE: u32 = 0x01;
}
