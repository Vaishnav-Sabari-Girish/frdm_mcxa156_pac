#[doc = "Register `OVP_PAD_STATE` reader"]
pub type R = crate::R<OvpPadStateSpec>;
#[doc = "Field `OVP_PAD_STATE` reader - OVP_PAD_STATE"]
pub type OvpPadStateR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - OVP_PAD_STATE"]
    #[inline(always)]
    pub fn ovp_pad_state(&self) -> OvpPadStateR {
        OvpPadStateR::new(self.bits)
    }
}
#[doc = "OVP_PAD_STATE\n\nYou can [`read`](crate::Reg::read) this register and get [`ovp_pad_state::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OvpPadStateSpec;
impl crate::RegisterSpec for OvpPadStateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ovp_pad_state::R`](R) reader structure"]
impl crate::Readable for OvpPadStateSpec {}
#[doc = "`reset()` method sets OVP_PAD_STATE to value 0"]
impl crate::Resettable for OvpPadStateSpec {}
