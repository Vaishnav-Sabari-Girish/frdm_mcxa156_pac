#[doc = "Register `EVD_CFG` reader"]
pub type R = crate::R<EvdCfgSpec>;
#[doc = "Register `EVD_CFG` writer"]
pub type W = crate::W<EvdCfgSpec>;
#[doc = "Field `EVDISO` reader - External Voltage Domain Isolation"]
pub type EvdisoR = crate::FieldReader;
#[doc = "Field `EVDISO` writer - External Voltage Domain Isolation"]
pub type EvdisoW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EVDLPISO` reader - External Voltage Domain Low-Power Isolation"]
pub type EvdlpisoR = crate::FieldReader;
#[doc = "Field `EVDLPISO` writer - External Voltage Domain Low-Power Isolation"]
pub type EvdlpisoW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EVDSTAT` reader - External Voltage Domain Status"]
pub type EvdstatR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - External Voltage Domain Isolation"]
    #[inline(always)]
    pub fn evdiso(&self) -> EvdisoR {
        EvdisoR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - External Voltage Domain Low-Power Isolation"]
    #[inline(always)]
    pub fn evdlpiso(&self) -> EvdlpisoR {
        EvdlpisoR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - External Voltage Domain Status"]
    #[inline(always)]
    pub fn evdstat(&self) -> EvdstatR {
        EvdstatR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - External Voltage Domain Isolation"]
    #[inline(always)]
    pub fn evdiso(&mut self) -> EvdisoW<'_, EvdCfgSpec> {
        EvdisoW::new(self, 0)
    }
    #[doc = "Bits 8:11 - External Voltage Domain Low-Power Isolation"]
    #[inline(always)]
    pub fn evdlpiso(&mut self) -> EvdlpisoW<'_, EvdCfgSpec> {
        EvdlpisoW::new(self, 8)
    }
}
#[doc = "External Voltage Domain Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`evd_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evd_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvdCfgSpec;
impl crate::RegisterSpec for EvdCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evd_cfg::R`](R) reader structure"]
impl crate::Readable for EvdCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`evd_cfg::W`](W) writer structure"]
impl crate::Writable for EvdCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVD_CFG to value 0"]
impl crate::Resettable for EvdCfgSpec {}
