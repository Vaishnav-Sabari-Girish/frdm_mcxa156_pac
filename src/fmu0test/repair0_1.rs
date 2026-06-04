#[doc = "Register `REPAIR0_1` reader"]
pub type R = crate::R<Repair0_1Spec>;
#[doc = "Register `REPAIR0_1` writer"]
pub type W = crate::W<Repair0_1Spec>;
#[doc = "RDIS0_1\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rdis0_1 {
    #[doc = "0: Repair address is valid"]
    Zz407 = 0,
    #[doc = "1: Repair address is not valid"]
    Zz408 = 1,
}
impl From<Rdis0_1> for bool {
    #[inline(always)]
    fn from(variant: Rdis0_1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDIS0_1` reader - RDIS0_1"]
pub type Rdis0_1R = crate::BitReader<Rdis0_1>;
impl Rdis0_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rdis0_1 {
        match self.bits {
            false => Rdis0_1::Zz407,
            true => Rdis0_1::Zz408,
        }
    }
    #[doc = "Repair address is valid"]
    #[inline(always)]
    pub fn is_zz407(&self) -> bool {
        *self == Rdis0_1::Zz407
    }
    #[doc = "Repair address is not valid"]
    #[inline(always)]
    pub fn is_zz408(&self) -> bool {
        *self == Rdis0_1::Zz408
    }
}
#[doc = "Field `RDIS0_1` writer - RDIS0_1"]
pub type Rdis0_1W<'a, REG> = crate::BitWriter<'a, REG, Rdis0_1>;
impl<'a, REG> Rdis0_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Repair address is valid"]
    #[inline(always)]
    pub fn zz407(self) -> &'a mut crate::W<REG> {
        self.variant(Rdis0_1::Zz407)
    }
    #[doc = "Repair address is not valid"]
    #[inline(always)]
    pub fn zz408(self) -> &'a mut crate::W<REG> {
        self.variant(Rdis0_1::Zz408)
    }
}
#[doc = "Field `RADR0_1` reader - RADR0_1"]
pub type Radr0_1R = crate::FieldReader;
#[doc = "Field `RADR0_1` writer - RADR0_1"]
pub type Radr0_1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - RDIS0_1"]
    #[inline(always)]
    pub fn rdis0_1(&self) -> Rdis0_1R {
        Rdis0_1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:8 - RADR0_1"]
    #[inline(always)]
    pub fn radr0_1(&self) -> Radr0_1R {
        Radr0_1R::new(((self.bits >> 1) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - RDIS0_1"]
    #[inline(always)]
    pub fn rdis0_1(&mut self) -> Rdis0_1W<'_, Repair0_1Spec> {
        Rdis0_1W::new(self, 0)
    }
    #[doc = "Bits 1:8 - RADR0_1"]
    #[inline(always)]
    pub fn radr0_1(&mut self) -> Radr0_1W<'_, Repair0_1Spec> {
        Radr0_1W::new(self, 1)
    }
}
#[doc = "FMU Repair 1 Block 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`repair0_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`repair0_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Repair0_1Spec;
impl crate::RegisterSpec for Repair0_1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`repair0_1::R`](R) reader structure"]
impl crate::Readable for Repair0_1Spec {}
#[doc = "`write(|w| ..)` method takes [`repair0_1::W`](W) writer structure"]
impl crate::Writable for Repair0_1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REPAIR0_1 to value 0x01ff"]
impl crate::Resettable for Repair0_1Spec {
    const RESET_VALUE: u32 = 0x01ff;
}
