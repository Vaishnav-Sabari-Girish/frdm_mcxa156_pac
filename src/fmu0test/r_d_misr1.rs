#[doc = "Register `R_D_MISR1` reader"]
pub type R = crate::R<RDMisr1Spec>;
#[doc = "Field `DATASIG1` reader - MISR Data Signature High"]
pub type Datasig1R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - MISR Data Signature High"]
    #[inline(always)]
    pub fn datasig1(&self) -> Datasig1R {
        Datasig1R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "BIST DIN MISR 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r_d_misr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RDMisr1Spec;
impl crate::RegisterSpec for RDMisr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r_d_misr1::R`](R) reader structure"]
impl crate::Readable for RDMisr1Spec {}
#[doc = "`reset()` method sets R_D_MISR1 to value 0"]
impl crate::Resettable for RDMisr1Spec {}
