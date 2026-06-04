#[doc = "Register `FT_STATE_B` reader"]
pub type R = crate::R<FtStateBSpec>;
#[doc = "Field `FT_STATE_B` reader - FT_STATE_B"]
pub type FtStateBR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - FT_STATE_B"]
    #[inline(always)]
    pub fn ft_state_b(&self) -> FtStateBR {
        FtStateBR::new(self.bits)
    }
}
#[doc = "FT_STATE_B\n\nYou can [`read`](crate::Reg::read) this register and get [`ft_state_b::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FtStateBSpec;
impl crate::RegisterSpec for FtStateBSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ft_state_b::R`](R) reader structure"]
impl crate::Readable for FtStateBSpec {}
#[doc = "`reset()` method sets FT_STATE_B to value 0"]
impl crate::Resettable for FtStateBSpec {}
