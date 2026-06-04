#[doc = "Register `R_DOUT_QUERY1` reader"]
pub type R = crate::R<RDoutQuery1Spec>;
#[doc = "Field `DOUT` reader - Failing DOUT High"]
pub type DoutR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - Failing DOUT High"]
    #[inline(always)]
    pub fn dout(&self) -> DoutR {
        DoutR::new((self.bits & 7) as u8)
    }
}
#[doc = "BIST DOUT Query 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r_dout_query1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RDoutQuery1Spec;
impl crate::RegisterSpec for RDoutQuery1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r_dout_query1::R`](R) reader structure"]
impl crate::Readable for RDoutQuery1Spec {}
#[doc = "`reset()` method sets R_DOUT_QUERY1 to value 0"]
impl crate::Resettable for RDoutQuery1Spec {}
