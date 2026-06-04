#[doc = "Register `PGM_PULSE_CNT1` reader"]
pub type R = crate::R<PgmPulseCnt1Spec>;
#[doc = "Register `PGM_PULSE_CNT1` writer"]
pub type W = crate::W<PgmPulseCnt1Spec>;
#[doc = "Field `PGM_CNT1` reader - Program Pulse Count"]
pub type PgmCnt1R = crate::FieldReader<u32>;
#[doc = "Field `PGM_CNT1` writer - Program Pulse Count"]
pub type PgmCnt1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Program Pulse Count"]
    #[inline(always)]
    pub fn pgm_cnt1(&self) -> PgmCnt1R {
        PgmCnt1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Program Pulse Count"]
    #[inline(always)]
    pub fn pgm_cnt1(&mut self) -> PgmCnt1W<'_, PgmPulseCnt1Spec> {
        PgmCnt1W::new(self, 0)
    }
}
#[doc = "Block 1 Program Pulse Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pgm_pulse_cnt1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pgm_pulse_cnt1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PgmPulseCnt1Spec;
impl crate::RegisterSpec for PgmPulseCnt1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pgm_pulse_cnt1::R`](R) reader structure"]
impl crate::Readable for PgmPulseCnt1Spec {}
#[doc = "`write(|w| ..)` method takes [`pgm_pulse_cnt1::W`](W) writer structure"]
impl crate::Writable for PgmPulseCnt1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PGM_PULSE_CNT1 to value 0"]
impl crate::Resettable for PgmPulseCnt1Spec {}
