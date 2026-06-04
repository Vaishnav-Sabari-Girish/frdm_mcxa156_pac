#[doc = "Register `R_SMW_QUERY` reader"]
pub type R = crate::R<RSmwQuerySpec>;
#[doc = "Field `SMWLOOP` reader - SMW Total Loop Count"]
pub type SmwloopR = crate::FieldReader<u16>;
#[doc = "Field `SMWLAST` reader - SMW Last Voltage Setting"]
pub type SmwlastR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - SMW Total Loop Count"]
    #[inline(always)]
    pub fn smwloop(&self) -> SmwloopR {
        SmwloopR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:18 - SMW Last Voltage Setting"]
    #[inline(always)]
    pub fn smwlast(&self) -> SmwlastR {
        SmwlastR::new(((self.bits >> 10) & 0x01ff) as u16)
    }
}
#[doc = "BIST SMW Query Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r_smw_query::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RSmwQuerySpec;
impl crate::RegisterSpec for RSmwQuerySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r_smw_query::R`](R) reader structure"]
impl crate::Readable for RSmwQuerySpec {}
#[doc = "`reset()` method sets R_SMW_QUERY to value 0"]
impl crate::Resettable for RSmwQuerySpec {}
