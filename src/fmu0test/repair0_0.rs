#[doc = "Register `REPAIR0_0` reader"]
pub type R = crate::R<Repair0_0Spec>;
#[doc = "Register `REPAIR0_0` writer"]
pub type W = crate::W<Repair0_0Spec>;
#[doc = "RDIS0_0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rdis0_0 {
    #[doc = "0: Repair address is valid"]
    Zz405 = 0,
    #[doc = "1: Repair address is not valid"]
    Zz406 = 1,
}
impl From<Rdis0_0> for bool {
    #[inline(always)]
    fn from(variant: Rdis0_0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDIS0_0` reader - RDIS0_0"]
pub type Rdis0_0R = crate::BitReader<Rdis0_0>;
impl Rdis0_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rdis0_0 {
        match self.bits {
            false => Rdis0_0::Zz405,
            true => Rdis0_0::Zz406,
        }
    }
    #[doc = "Repair address is valid"]
    #[inline(always)]
    pub fn is_zz405(&self) -> bool {
        *self == Rdis0_0::Zz405
    }
    #[doc = "Repair address is not valid"]
    #[inline(always)]
    pub fn is_zz406(&self) -> bool {
        *self == Rdis0_0::Zz406
    }
}
#[doc = "Field `RDIS0_0` writer - RDIS0_0"]
pub type Rdis0_0W<'a, REG> = crate::BitWriter<'a, REG, Rdis0_0>;
impl<'a, REG> Rdis0_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Repair address is valid"]
    #[inline(always)]
    pub fn zz405(self) -> &'a mut crate::W<REG> {
        self.variant(Rdis0_0::Zz405)
    }
    #[doc = "Repair address is not valid"]
    #[inline(always)]
    pub fn zz406(self) -> &'a mut crate::W<REG> {
        self.variant(Rdis0_0::Zz406)
    }
}
#[doc = "Field `RADR0_0` reader - RADR0_0"]
pub type Radr0_0R = crate::FieldReader;
#[doc = "Field `RADR0_0` writer - RADR0_0"]
pub type Radr0_0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - RDIS0_0"]
    #[inline(always)]
    pub fn rdis0_0(&self) -> Rdis0_0R {
        Rdis0_0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:8 - RADR0_0"]
    #[inline(always)]
    pub fn radr0_0(&self) -> Radr0_0R {
        Radr0_0R::new(((self.bits >> 1) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - RDIS0_0"]
    #[inline(always)]
    pub fn rdis0_0(&mut self) -> Rdis0_0W<'_, Repair0_0Spec> {
        Rdis0_0W::new(self, 0)
    }
    #[doc = "Bits 1:8 - RADR0_0"]
    #[inline(always)]
    pub fn radr0_0(&mut self) -> Radr0_0W<'_, Repair0_0Spec> {
        Radr0_0W::new(self, 1)
    }
}
#[doc = "FMU Repair 0 Block 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`repair0_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`repair0_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Repair0_0Spec;
impl crate::RegisterSpec for Repair0_0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`repair0_0::R`](R) reader structure"]
impl crate::Readable for Repair0_0Spec {}
#[doc = "`write(|w| ..)` method takes [`repair0_0::W`](W) writer structure"]
impl crate::Writable for Repair0_0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REPAIR0_0 to value 0x01ff"]
impl crate::Resettable for Repair0_0Spec {
    const RESET_VALUE: u32 = 0x01ff;
}
