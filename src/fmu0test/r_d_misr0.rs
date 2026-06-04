#[doc = "Register `R_D_MISR0` reader"]
pub type R = crate::R<RDMisr0Spec>;
#[doc = "Field `DATASIG0` reader - Data Signature"]
pub type Datasig0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Data Signature"]
    #[inline(always)]
    pub fn datasig0(&self) -> Datasig0R {
        Datasig0R::new(self.bits)
    }
}
#[doc = "BIST DIN MISR 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r_d_misr0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RDMisr0Spec;
impl crate::RegisterSpec for RDMisr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r_d_misr0::R`](R) reader structure"]
impl crate::Readable for RDMisr0Spec {}
#[doc = "`reset()` method sets R_D_MISR0 to value 0"]
impl crate::Resettable for RDMisr0Spec {}
