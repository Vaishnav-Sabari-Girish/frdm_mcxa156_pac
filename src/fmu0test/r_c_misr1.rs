#[doc = "Register `R_C_MISR1` reader"]
pub type R = crate::R<RCMisr1Spec>;
#[doc = "Field `CTRLSIG1` reader - MISR Control Signature High"]
pub type Ctrlsig1R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - MISR Control Signature High"]
    #[inline(always)]
    pub fn ctrlsig1(&self) -> Ctrlsig1R {
        Ctrlsig1R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "BIST Control MISR 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r_c_misr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCMisr1Spec;
impl crate::RegisterSpec for RCMisr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r_c_misr1::R`](R) reader structure"]
impl crate::Readable for RCMisr1Spec {}
#[doc = "`reset()` method sets R_C_MISR1 to value 0"]
impl crate::Resettable for RCMisr1Spec {}
