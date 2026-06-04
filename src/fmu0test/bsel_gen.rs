#[doc = "Register `BSEL_GEN` reader"]
pub type R = crate::R<BselGenSpec>;
#[doc = "Field `SBSEL_GEN` reader - Generated SBSEL"]
pub type SbselGenR = crate::FieldReader;
#[doc = "Field `MBSEL_GEN` reader - Generated MBSEL"]
pub type MbselGenR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - Generated SBSEL"]
    #[inline(always)]
    pub fn sbsel_gen(&self) -> SbselGenR {
        SbselGenR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - Generated MBSEL"]
    #[inline(always)]
    pub fn mbsel_gen(&self) -> MbselGenR {
        MbselGenR::new(((self.bits >> 8) & 3) as u8)
    }
}
#[doc = "FMU Block Select Generation Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bsel_gen::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BselGenSpec;
impl crate::RegisterSpec for BselGenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bsel_gen::R`](R) reader structure"]
impl crate::Readable for BselGenSpec {}
#[doc = "`reset()` method sets BSEL_GEN to value 0x0301"]
impl crate::Resettable for BselGenSpec {
    const RESET_VALUE: u32 = 0x0301;
}
