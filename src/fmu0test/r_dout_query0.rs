#[doc = "Register `R_DOUT_QUERY0` reader"]
pub type R = crate::R<RDoutQuery0Spec>;
#[doc = "Field `DOUTFAIL` reader - Failing DOUT Low"]
pub type DoutfailR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Failing DOUT Low"]
    #[inline(always)]
    pub fn doutfail(&self) -> DoutfailR {
        DoutfailR::new(self.bits)
    }
}
#[doc = "BIST DOUT Query 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r_dout_query0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RDoutQuery0Spec;
impl crate::RegisterSpec for RDoutQuery0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r_dout_query0::R`](R) reader structure"]
impl crate::Readable for RDoutQuery0Spec {}
#[doc = "`reset()` method sets R_DOUT_QUERY0 to value 0"]
impl crate::Resettable for RDoutQuery0Spec {}
