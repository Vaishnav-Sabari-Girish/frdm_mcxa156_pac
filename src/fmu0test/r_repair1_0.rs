#[doc = "Register `R_REPAIR1_0` reader"]
pub type R = crate::R<RRepair1_0Spec>;
#[doc = "Control Repair 0 in Block 1.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rdis1_0 {
    #[doc = "0: Repair address is valid"]
    Zz341 = 0,
    #[doc = "1: Repair address is not valid"]
    Zz342 = 1,
}
impl From<Rdis1_0> for bool {
    #[inline(always)]
    fn from(variant: Rdis1_0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDIS1_0` reader - Control Repair 0 in Block 1."]
pub type Rdis1_0R = crate::BitReader<Rdis1_0>;
impl Rdis1_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rdis1_0 {
        match self.bits {
            false => Rdis1_0::Zz341,
            true => Rdis1_0::Zz342,
        }
    }
    #[doc = "Repair address is valid"]
    #[inline(always)]
    pub fn is_zz341(&self) -> bool {
        *self == Rdis1_0::Zz341
    }
    #[doc = "Repair address is not valid"]
    #[inline(always)]
    pub fn is_zz342(&self) -> bool {
        *self == Rdis1_0::Zz342
    }
}
#[doc = "Field `RADR1_0` reader - XADR for Repair 0 in Block 1."]
pub type Radr1_0R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Control Repair 0 in Block 1."]
    #[inline(always)]
    pub fn rdis1_0(&self) -> Rdis1_0R {
        Rdis1_0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:8 - XADR for Repair 0 in Block 1."]
    #[inline(always)]
    pub fn radr1_0(&self) -> Radr1_0R {
        Radr1_0R::new(((self.bits >> 1) & 0xff) as u8)
    }
}
#[doc = "BIST Repair 0 Block 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r_repair1_0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RRepair1_0Spec;
impl crate::RegisterSpec for RRepair1_0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r_repair1_0::R`](R) reader structure"]
impl crate::Readable for RRepair1_0Spec {}
#[doc = "`reset()` method sets R_REPAIR1_0 to value 0x01ff"]
impl crate::Resettable for RRepair1_0Spec {
    const RESET_VALUE: u32 = 0x01ff;
}
