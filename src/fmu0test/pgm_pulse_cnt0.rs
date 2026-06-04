#[doc = "Register `PGM_PULSE_CNT0` reader"]
pub type R = crate::R<PgmPulseCnt0Spec>;
#[doc = "Register `PGM_PULSE_CNT0` writer"]
pub type W = crate::W<PgmPulseCnt0Spec>;
#[doc = "Field `PGM_CNT0` reader - Program Pulse Count"]
pub type PgmCnt0R = crate::FieldReader<u32>;
#[doc = "Field `PGM_CNT0` writer - Program Pulse Count"]
pub type PgmCnt0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Program Pulse Count"]
    #[inline(always)]
    pub fn pgm_cnt0(&self) -> PgmCnt0R {
        PgmCnt0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Program Pulse Count"]
    #[inline(always)]
    pub fn pgm_cnt0(&mut self) -> PgmCnt0W<'_, PgmPulseCnt0Spec> {
        PgmCnt0W::new(self, 0)
    }
}
#[doc = "Block 0 Program Pulse Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pgm_pulse_cnt0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pgm_pulse_cnt0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PgmPulseCnt0Spec;
impl crate::RegisterSpec for PgmPulseCnt0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pgm_pulse_cnt0::R`](R) reader structure"]
impl crate::Readable for PgmPulseCnt0Spec {}
#[doc = "`write(|w| ..)` method takes [`pgm_pulse_cnt0::W`](W) writer structure"]
impl crate::Writable for PgmPulseCnt0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PGM_PULSE_CNT0 to value 0"]
impl crate::Resettable for PgmPulseCnt0Spec {}
