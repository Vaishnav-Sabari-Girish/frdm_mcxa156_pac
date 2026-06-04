#[doc = "Register `MAX_PULSE_CNT` reader"]
pub type R = crate::R<MaxPulseCntSpec>;
#[doc = "Register `MAX_PULSE_CNT` writer"]
pub type W = crate::W<MaxPulseCntSpec>;
#[doc = "Field `LAST_PCNT` reader - Last SMW Operation's Pulse Count"]
pub type LastPcntR = crate::FieldReader<u16>;
#[doc = "Field `LAST_PCNT` writer - Last SMW Operation's Pulse Count"]
pub type LastPcntW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `MAX_ERS_CNT` reader - Maximum Erase Pulse Count"]
pub type MaxErsCntR = crate::FieldReader<u16>;
#[doc = "Field `MAX_ERS_CNT` writer - Maximum Erase Pulse Count"]
pub type MaxErsCntW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `MAX_PGM_CNT` reader - Maximum Program Pulse Count"]
pub type MaxPgmCntR = crate::FieldReader;
#[doc = "Field `MAX_PGM_CNT` writer - Maximum Program Pulse Count"]
pub type MaxPgmCntW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:8 - Last SMW Operation's Pulse Count"]
    #[inline(always)]
    pub fn last_pcnt(&self) -> LastPcntR {
        LastPcntR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:24 - Maximum Erase Pulse Count"]
    #[inline(always)]
    pub fn max_ers_cnt(&self) -> MaxErsCntR {
        MaxErsCntR::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bits 27:31 - Maximum Program Pulse Count"]
    #[inline(always)]
    pub fn max_pgm_cnt(&self) -> MaxPgmCntR {
        MaxPgmCntR::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - Last SMW Operation's Pulse Count"]
    #[inline(always)]
    pub fn last_pcnt(&mut self) -> LastPcntW<'_, MaxPulseCntSpec> {
        LastPcntW::new(self, 0)
    }
    #[doc = "Bits 16:24 - Maximum Erase Pulse Count"]
    #[inline(always)]
    pub fn max_ers_cnt(&mut self) -> MaxErsCntW<'_, MaxPulseCntSpec> {
        MaxErsCntW::new(self, 16)
    }
    #[doc = "Bits 27:31 - Maximum Program Pulse Count"]
    #[inline(always)]
    pub fn max_pgm_cnt(&mut self) -> MaxPgmCntW<'_, MaxPulseCntSpec> {
        MaxPgmCntW::new(self, 27)
    }
}
#[doc = "Maximum Pulse Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`max_pulse_cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`max_pulse_cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MaxPulseCntSpec;
impl crate::RegisterSpec for MaxPulseCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`max_pulse_cnt::R`](R) reader structure"]
impl crate::Readable for MaxPulseCntSpec {}
#[doc = "`write(|w| ..)` method takes [`max_pulse_cnt::W`](W) writer structure"]
impl crate::Writable for MaxPulseCntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MAX_PULSE_CNT to value 0"]
impl crate::Resettable for MaxPulseCntSpec {}
