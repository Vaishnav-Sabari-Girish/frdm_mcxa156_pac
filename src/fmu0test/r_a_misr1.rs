#[doc = "Register `R_A_MISR1` reader"]
pub type R = crate::R<RAMisr1Spec>;
#[doc = "Field `ADRSIG1` reader - MISR Address Signature High"]
pub type Adrsig1R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - MISR Address Signature High"]
    #[inline(always)]
    pub fn adrsig1(&self) -> Adrsig1R {
        Adrsig1R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "BIST Address MISR 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r_a_misr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RAMisr1Spec;
impl crate::RegisterSpec for RAMisr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r_a_misr1::R`](R) reader structure"]
impl crate::Readable for RAMisr1Spec {}
#[doc = "`reset()` method sets R_A_MISR1 to value 0"]
impl crate::Resettable for RAMisr1Spec {}
