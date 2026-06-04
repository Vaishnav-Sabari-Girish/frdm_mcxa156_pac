#[doc = "Register `OTGCTL` reader"]
pub type R = crate::R<OtgctlSpec>;
#[doc = "Register `OTGCTL` writer"]
pub type W = crate::W<OtgctlSpec>;
#[doc = "D+ Data Line Pullup Resistor Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dphigh {
    #[doc = "0: Disable"]
    DisDpPullup = 0,
    #[doc = "1: Enable"]
    EnDpPullup = 1,
}
impl From<Dphigh> for bool {
    #[inline(always)]
    fn from(variant: Dphigh) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DPHIGH` reader - D+ Data Line Pullup Resistor Enable"]
pub type DphighR = crate::BitReader<Dphigh>;
impl DphighR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dphigh {
        match self.bits {
            false => Dphigh::DisDpPullup,
            true => Dphigh::EnDpPullup,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_dis_dp_pullup(&self) -> bool {
        *self == Dphigh::DisDpPullup
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_en_dp_pullup(&self) -> bool {
        *self == Dphigh::EnDpPullup
    }
}
#[doc = "Field `DPHIGH` writer - D+ Data Line Pullup Resistor Enable"]
pub type DphighW<'a, REG> = crate::BitWriter<'a, REG, Dphigh>;
impl<'a, REG> DphighW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn dis_dp_pullup(self) -> &'a mut crate::W<REG> {
        self.variant(Dphigh::DisDpPullup)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn en_dp_pullup(self) -> &'a mut crate::W<REG> {
        self.variant(Dphigh::EnDpPullup)
    }
}
impl R {
    #[doc = "Bit 7 - D+ Data Line Pullup Resistor Enable"]
    #[inline(always)]
    pub fn dphigh(&self) -> DphighR {
        DphighR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - D+ Data Line Pullup Resistor Enable"]
    #[inline(always)]
    pub fn dphigh(&mut self) -> DphighW<'_, OtgctlSpec> {
        DphighW::new(self, 7)
    }
}
#[doc = "OTG Control\n\nYou can [`read`](crate::Reg::read) this register and get [`otgctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otgctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtgctlSpec;
impl crate::RegisterSpec for OtgctlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`otgctl::R`](R) reader structure"]
impl crate::Readable for OtgctlSpec {}
#[doc = "`write(|w| ..)` method takes [`otgctl::W`](W) writer structure"]
impl crate::Writable for OtgctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTGCTL to value 0"]
impl crate::Resettable for OtgctlSpec {}
