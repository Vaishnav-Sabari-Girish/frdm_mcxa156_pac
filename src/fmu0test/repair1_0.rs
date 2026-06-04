#[doc = "Register `REPAIR1_0` reader"]
pub type R = crate::R<Repair1_0Spec>;
#[doc = "Register `REPAIR1_0` writer"]
pub type W = crate::W<Repair1_0Spec>;
#[doc = "RDIS1_0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rdis1_0 {
    #[doc = "0: Repair address is valid"]
    Zz409 = 0,
    #[doc = "1: Repair address is not valid"]
    Zz410 = 1,
}
impl From<Rdis1_0> for bool {
    #[inline(always)]
    fn from(variant: Rdis1_0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDIS1_0` reader - RDIS1_0"]
pub type Rdis1_0R = crate::BitReader<Rdis1_0>;
impl Rdis1_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rdis1_0 {
        match self.bits {
            false => Rdis1_0::Zz409,
            true => Rdis1_0::Zz410,
        }
    }
    #[doc = "Repair address is valid"]
    #[inline(always)]
    pub fn is_zz409(&self) -> bool {
        *self == Rdis1_0::Zz409
    }
    #[doc = "Repair address is not valid"]
    #[inline(always)]
    pub fn is_zz410(&self) -> bool {
        *self == Rdis1_0::Zz410
    }
}
#[doc = "Field `RDIS1_0` writer - RDIS1_0"]
pub type Rdis1_0W<'a, REG> = crate::BitWriter<'a, REG, Rdis1_0>;
impl<'a, REG> Rdis1_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Repair address is valid"]
    #[inline(always)]
    pub fn zz409(self) -> &'a mut crate::W<REG> {
        self.variant(Rdis1_0::Zz409)
    }
    #[doc = "Repair address is not valid"]
    #[inline(always)]
    pub fn zz410(self) -> &'a mut crate::W<REG> {
        self.variant(Rdis1_0::Zz410)
    }
}
#[doc = "Field `RADR1_0` reader - RADR1_0"]
pub type Radr1_0R = crate::FieldReader;
#[doc = "Field `RADR1_0` writer - RADR1_0"]
pub type Radr1_0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - RDIS1_0"]
    #[inline(always)]
    pub fn rdis1_0(&self) -> Rdis1_0R {
        Rdis1_0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:8 - RADR1_0"]
    #[inline(always)]
    pub fn radr1_0(&self) -> Radr1_0R {
        Radr1_0R::new(((self.bits >> 1) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - RDIS1_0"]
    #[inline(always)]
    pub fn rdis1_0(&mut self) -> Rdis1_0W<'_, Repair1_0Spec> {
        Rdis1_0W::new(self, 0)
    }
    #[doc = "Bits 1:8 - RADR1_0"]
    #[inline(always)]
    pub fn radr1_0(&mut self) -> Radr1_0W<'_, Repair1_0Spec> {
        Radr1_0W::new(self, 1)
    }
}
#[doc = "FMU Repair 0 Block 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`repair1_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`repair1_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Repair1_0Spec;
impl crate::RegisterSpec for Repair1_0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`repair1_0::R`](R) reader structure"]
impl crate::Readable for Repair1_0Spec {}
#[doc = "`write(|w| ..)` method takes [`repair1_0::W`](W) writer structure"]
impl crate::Writable for Repair1_0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REPAIR1_0 to value 0x01ff"]
impl crate::Resettable for Repair1_0Spec {
    const RESET_VALUE: u32 = 0x01ff;
}
