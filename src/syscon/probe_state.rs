#[doc = "Register `PROBE_STATE` reader"]
pub type R = crate::R<ProbeStateSpec>;
#[doc = "Field `PROBE_STATE` reader - PROBE_STATE"]
pub type ProbeStateR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PROBE_STATE"]
    #[inline(always)]
    pub fn probe_state(&self) -> ProbeStateR {
        ProbeStateR::new(self.bits)
    }
}
#[doc = "PROBE_STATE\n\nYou can [`read`](crate::Reg::read) this register and get [`probe_state::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ProbeStateSpec;
impl crate::RegisterSpec for ProbeStateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`probe_state::R`](R) reader structure"]
impl crate::Readable for ProbeStateSpec {}
#[doc = "`reset()` method sets PROBE_STATE to value 0"]
impl crate::Resettable for ProbeStateSpec {}
