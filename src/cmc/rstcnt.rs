#[doc = "Register `RSTCNT` reader"]
pub type R = crate::R<RstcntSpec>;
#[doc = "Field `COUNT` reader - Count"]
pub type CountR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Count"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Reset Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rstcnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RstcntSpec;
impl crate::RegisterSpec for RstcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rstcnt::R`](R) reader structure"]
impl crate::Readable for RstcntSpec {}
#[doc = "`reset()` method sets RSTCNT to value 0"]
impl crate::Resettable for RstcntSpec {}
