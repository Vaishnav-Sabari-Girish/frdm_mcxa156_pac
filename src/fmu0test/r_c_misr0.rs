#[doc = "Register `R_C_MISR0` reader"]
pub type R = crate::R<RCMisr0Spec>;
#[doc = "Field `CTRLSIG0` reader - Control Signature"]
pub type Ctrlsig0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Control Signature"]
    #[inline(always)]
    pub fn ctrlsig0(&self) -> Ctrlsig0R {
        Ctrlsig0R::new(self.bits)
    }
}
#[doc = "BIST Control MISR 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r_c_misr0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCMisr0Spec;
impl crate::RegisterSpec for RCMisr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r_c_misr0::R`](R) reader structure"]
impl crate::Readable for RCMisr0Spec {}
#[doc = "`reset()` method sets R_C_MISR0 to value 0"]
impl crate::Resettable for RCMisr0Spec {}
