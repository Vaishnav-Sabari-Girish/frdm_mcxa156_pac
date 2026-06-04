#[doc = "Register `R_REPAIR0_1` reader"]
pub type R = crate::R<RRepair0_1Spec>;
#[doc = "Control Repair 1 in Block 0.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rdis0_1 {
    #[doc = "0: Repair address is valid"]
    Zz339 = 0,
    #[doc = "1: Repair address is not valid"]
    Zz340 = 1,
}
impl From<Rdis0_1> for bool {
    #[inline(always)]
    fn from(variant: Rdis0_1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDIS0_1` reader - Control Repair 1 in Block 0."]
pub type Rdis0_1R = crate::BitReader<Rdis0_1>;
impl Rdis0_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rdis0_1 {
        match self.bits {
            false => Rdis0_1::Zz339,
            true => Rdis0_1::Zz340,
        }
    }
    #[doc = "Repair address is valid"]
    #[inline(always)]
    pub fn is_zz339(&self) -> bool {
        *self == Rdis0_1::Zz339
    }
    #[doc = "Repair address is not valid"]
    #[inline(always)]
    pub fn is_zz340(&self) -> bool {
        *self == Rdis0_1::Zz340
    }
}
#[doc = "Field `RADR0_1` reader - XADR for Repair 1 in Block 0."]
pub type Radr0_1R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Control Repair 1 in Block 0."]
    #[inline(always)]
    pub fn rdis0_1(&self) -> Rdis0_1R {
        Rdis0_1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:8 - XADR for Repair 1 in Block 0."]
    #[inline(always)]
    pub fn radr0_1(&self) -> Radr0_1R {
        Radr0_1R::new(((self.bits >> 1) & 0xff) as u8)
    }
}
#[doc = "BIST Repair 1 Block 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r_repair0_1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RRepair0_1Spec;
impl crate::RegisterSpec for RRepair0_1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r_repair0_1::R`](R) reader structure"]
impl crate::Readable for RRepair0_1Spec {}
#[doc = "`reset()` method sets R_REPAIR0_1 to value 0x01ff"]
impl crate::Resettable for RRepair0_1Spec {
    const RESET_VALUE: u32 = 0x01ff;
}
