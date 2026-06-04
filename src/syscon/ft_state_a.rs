#[doc = "Register `FT_STATE_A` reader"]
pub type R = crate::R<FtStateASpec>;
#[doc = "Field `FT_STATE_A` reader - FT_STATE_A"]
pub type FtStateAR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - FT_STATE_A"]
    #[inline(always)]
    pub fn ft_state_a(&self) -> FtStateAR {
        FtStateAR::new(self.bits)
    }
}
#[doc = "FT_STATE_A\n\nYou can [`read`](crate::Reg::read) this register and get [`ft_state_a::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FtStateASpec;
impl crate::RegisterSpec for FtStateASpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ft_state_a::R`](R) reader structure"]
impl crate::Readable for FtStateASpec {}
#[doc = "`reset()` method sets FT_STATE_A to value 0"]
impl crate::Resettable for FtStateASpec {}
