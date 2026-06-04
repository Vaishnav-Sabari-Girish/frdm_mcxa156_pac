#[doc = "Register `ERS_PULSE_CNT` reader"]
pub type R = crate::R<ErsPulseCntSpec>;
#[doc = "Register `ERS_PULSE_CNT` writer"]
pub type W = crate::W<ErsPulseCntSpec>;
#[doc = "Field `ERS_CNT0` reader - Block 0 Erase Pulse Count"]
pub type ErsCnt0R = crate::FieldReader<u16>;
#[doc = "Field `ERS_CNT0` writer - Block 0 Erase Pulse Count"]
pub type ErsCnt0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ERS_CNT1` reader - Block 1 Erase Pulse Count"]
pub type ErsCnt1R = crate::FieldReader<u16>;
#[doc = "Field `ERS_CNT1` writer - Block 1 Erase Pulse Count"]
pub type ErsCnt1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Block 0 Erase Pulse Count"]
    #[inline(always)]
    pub fn ers_cnt0(&self) -> ErsCnt0R {
        ErsCnt0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Block 1 Erase Pulse Count"]
    #[inline(always)]
    pub fn ers_cnt1(&self) -> ErsCnt1R {
        ErsCnt1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Block 0 Erase Pulse Count"]
    #[inline(always)]
    pub fn ers_cnt0(&mut self) -> ErsCnt0W<'_, ErsPulseCntSpec> {
        ErsCnt0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Block 1 Erase Pulse Count"]
    #[inline(always)]
    pub fn ers_cnt1(&mut self) -> ErsCnt1W<'_, ErsPulseCntSpec> {
        ErsCnt1W::new(self, 16)
    }
}
#[doc = "Erase Pulse Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ers_pulse_cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ers_pulse_cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErsPulseCntSpec;
impl crate::RegisterSpec for ErsPulseCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ers_pulse_cnt::R`](R) reader structure"]
impl crate::Readable for ErsPulseCntSpec {}
#[doc = "`write(|w| ..)` method takes [`ers_pulse_cnt::W`](W) writer structure"]
impl crate::Writable for ErsPulseCntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ERS_PULSE_CNT to value 0"]
impl crate::Resettable for ErsPulseCntSpec {}
