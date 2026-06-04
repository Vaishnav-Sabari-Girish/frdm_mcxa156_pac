#[doc = "Register `R_REPAIR0_0` reader"]
pub type R = crate::R<RRepair0_0Spec>;
#[doc = "Control Repair 0 in Block 0.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rdis0_0 {
    #[doc = "0: Repair address is valid"]
    Zz337 = 0,
    #[doc = "1: Repair address is not valid"]
    Zz338 = 1,
}
impl From<Rdis0_0> for bool {
    #[inline(always)]
    fn from(variant: Rdis0_0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDIS0_0` reader - Control Repair 0 in Block 0."]
pub type Rdis0_0R = crate::BitReader<Rdis0_0>;
impl Rdis0_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rdis0_0 {
        match self.bits {
            false => Rdis0_0::Zz337,
            true => Rdis0_0::Zz338,
        }
    }
    #[doc = "Repair address is valid"]
    #[inline(always)]
    pub fn is_zz337(&self) -> bool {
        *self == Rdis0_0::Zz337
    }
    #[doc = "Repair address is not valid"]
    #[inline(always)]
    pub fn is_zz338(&self) -> bool {
        *self == Rdis0_0::Zz338
    }
}
#[doc = "Field `RADR0_0` reader - XADR for Repair 0 in Block 0"]
pub type Radr0_0R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Control Repair 0 in Block 0."]
    #[inline(always)]
    pub fn rdis0_0(&self) -> Rdis0_0R {
        Rdis0_0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:8 - XADR for Repair 0 in Block 0"]
    #[inline(always)]
    pub fn radr0_0(&self) -> Radr0_0R {
        Radr0_0R::new(((self.bits >> 1) & 0xff) as u8)
    }
}
#[doc = "BIST Repair 0 for Block 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r_repair0_0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RRepair0_0Spec;
impl crate::RegisterSpec for RRepair0_0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r_repair0_0::R`](R) reader structure"]
impl crate::Readable for RRepair0_0Spec {}
#[doc = "`reset()` method sets R_REPAIR0_0 to value 0x01ff"]
impl crate::Resettable for RRepair0_0Spec {
    const RESET_VALUE: u32 = 0x01ff;
}
