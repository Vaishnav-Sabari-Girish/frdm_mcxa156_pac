#[doc = "Register `REPAIR1_1` reader"]
pub type R = crate::R<Repair1_1Spec>;
#[doc = "Register `REPAIR1_1` writer"]
pub type W = crate::W<Repair1_1Spec>;
#[doc = "RDIS1_1\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rdis1_1 {
    #[doc = "0: Repair address is valid"]
    Zz411 = 0,
    #[doc = "1: Repair address is not valid"]
    Zz412 = 1,
}
impl From<Rdis1_1> for bool {
    #[inline(always)]
    fn from(variant: Rdis1_1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDIS1_1` reader - RDIS1_1"]
pub type Rdis1_1R = crate::BitReader<Rdis1_1>;
impl Rdis1_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rdis1_1 {
        match self.bits {
            false => Rdis1_1::Zz411,
            true => Rdis1_1::Zz412,
        }
    }
    #[doc = "Repair address is valid"]
    #[inline(always)]
    pub fn is_zz411(&self) -> bool {
        *self == Rdis1_1::Zz411
    }
    #[doc = "Repair address is not valid"]
    #[inline(always)]
    pub fn is_zz412(&self) -> bool {
        *self == Rdis1_1::Zz412
    }
}
#[doc = "Field `RDIS1_1` writer - RDIS1_1"]
pub type Rdis1_1W<'a, REG> = crate::BitWriter<'a, REG, Rdis1_1>;
impl<'a, REG> Rdis1_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Repair address is valid"]
    #[inline(always)]
    pub fn zz411(self) -> &'a mut crate::W<REG> {
        self.variant(Rdis1_1::Zz411)
    }
    #[doc = "Repair address is not valid"]
    #[inline(always)]
    pub fn zz412(self) -> &'a mut crate::W<REG> {
        self.variant(Rdis1_1::Zz412)
    }
}
#[doc = "Field `RADR1_1` reader - RADR1_1"]
pub type Radr1_1R = crate::FieldReader;
#[doc = "Field `RADR1_1` writer - RADR1_1"]
pub type Radr1_1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - RDIS1_1"]
    #[inline(always)]
    pub fn rdis1_1(&self) -> Rdis1_1R {
        Rdis1_1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:8 - RADR1_1"]
    #[inline(always)]
    pub fn radr1_1(&self) -> Radr1_1R {
        Radr1_1R::new(((self.bits >> 1) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - RDIS1_1"]
    #[inline(always)]
    pub fn rdis1_1(&mut self) -> Rdis1_1W<'_, Repair1_1Spec> {
        Rdis1_1W::new(self, 0)
    }
    #[doc = "Bits 1:8 - RADR1_1"]
    #[inline(always)]
    pub fn radr1_1(&mut self) -> Radr1_1W<'_, Repair1_1Spec> {
        Radr1_1W::new(self, 1)
    }
}
#[doc = "FMU Repair 1 Block 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`repair1_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`repair1_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Repair1_1Spec;
impl crate::RegisterSpec for Repair1_1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`repair1_1::R`](R) reader structure"]
impl crate::Readable for Repair1_1Spec {}
#[doc = "`write(|w| ..)` method takes [`repair1_1::W`](W) writer structure"]
impl crate::Writable for Repair1_1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REPAIR1_1 to value 0x01ff"]
impl crate::Resettable for Repair1_1Spec {
    const RESET_VALUE: u32 = 0x01ff;
}
