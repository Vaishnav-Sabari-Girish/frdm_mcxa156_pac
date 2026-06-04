#[doc = "Register `R_REPAIR1_1` reader"]
pub type R = crate::R<RRepair1_1Spec>;
#[doc = "Control Repair 1 in Block 1.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rdis1_1 {
    #[doc = "0: Repair address is valid"]
    Zz343 = 0,
    #[doc = "1: Repair address is not valid"]
    Zz344 = 1,
}
impl From<Rdis1_1> for bool {
    #[inline(always)]
    fn from(variant: Rdis1_1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDIS1_1` reader - Control Repair 1 in Block 1."]
pub type Rdis1_1R = crate::BitReader<Rdis1_1>;
impl Rdis1_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rdis1_1 {
        match self.bits {
            false => Rdis1_1::Zz343,
            true => Rdis1_1::Zz344,
        }
    }
    #[doc = "Repair address is valid"]
    #[inline(always)]
    pub fn is_zz343(&self) -> bool {
        *self == Rdis1_1::Zz343
    }
    #[doc = "Repair address is not valid"]
    #[inline(always)]
    pub fn is_zz344(&self) -> bool {
        *self == Rdis1_1::Zz344
    }
}
#[doc = "Field `RADR1_1` reader - XADR for Repair 1 in Block 1."]
pub type Radr1_1R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Control Repair 1 in Block 1."]
    #[inline(always)]
    pub fn rdis1_1(&self) -> Rdis1_1R {
        Rdis1_1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:8 - XADR for Repair 1 in Block 1."]
    #[inline(always)]
    pub fn radr1_1(&self) -> Radr1_1R {
        Radr1_1R::new(((self.bits >> 1) & 0xff) as u8)
    }
}
#[doc = "BIST Repair 1 Block 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r_repair1_1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RRepair1_1Spec;
impl crate::RegisterSpec for RRepair1_1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r_repair1_1::R`](R) reader structure"]
impl crate::Readable for RRepair1_1Spec {}
#[doc = "`reset()` method sets R_REPAIR1_1 to value 0x01ff"]
impl crate::Resettable for RRepair1_1Spec {
    const RESET_VALUE: u32 = 0x01ff;
}
