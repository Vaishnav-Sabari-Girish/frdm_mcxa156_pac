#[doc = "Register `R_A_MISR0` reader"]
pub type R = crate::R<RAMisr0Spec>;
#[doc = "Field `ADRSIG0` reader - Address Signature"]
pub type Adrsig0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Address Signature"]
    #[inline(always)]
    pub fn adrsig0(&self) -> Adrsig0R {
        Adrsig0R::new(self.bits)
    }
}
#[doc = "BIST Address MISR 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r_a_misr0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RAMisr0Spec;
impl crate::RegisterSpec for RAMisr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r_a_misr0::R`](R) reader structure"]
impl crate::Readable for RAMisr0Spec {}
#[doc = "`reset()` method sets R_A_MISR0 to value 0"]
impl crate::Resettable for RAMisr0Spec {}
