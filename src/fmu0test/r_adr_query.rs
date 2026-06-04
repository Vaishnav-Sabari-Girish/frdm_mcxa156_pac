#[doc = "Register `R_ADR_QUERY` reader"]
pub type R = crate::R<RAdrQuerySpec>;
#[doc = "Field `YADRFAIL` reader - Failing YADR"]
pub type YadrfailR = crate::FieldReader;
#[doc = "Field `XADRFAIL` reader - Failing XADR"]
pub type XadrfailR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:4 - Failing YADR"]
    #[inline(always)]
    pub fn yadrfail(&self) -> YadrfailR {
        YadrfailR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:16 - Failing XADR"]
    #[inline(always)]
    pub fn xadrfail(&self) -> XadrfailR {
        XadrfailR::new(((self.bits >> 5) & 0x0fff) as u16)
    }
}
#[doc = "BIST Address Query Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r_adr_query::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RAdrQuerySpec;
impl crate::RegisterSpec for RAdrQuerySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r_adr_query::R`](R) reader structure"]
impl crate::Readable for RAdrQuerySpec {}
#[doc = "`reset()` method sets R_ADR_QUERY to value 0"]
impl crate::Resettable for RAdrQuerySpec {}
