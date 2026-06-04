#[doc = "Register `ENDPT` reader"]
pub type R = crate::R<EndptSpec>;
#[doc = "Register `ENDPT` writer"]
pub type W = crate::W<EndptSpec>;
#[doc = "Field `EPHSHK` reader - Endpoint Handshaking Enable"]
pub type EphshkR = crate::BitReader;
#[doc = "Field `EPHSHK` writer - Endpoint Handshaking Enable"]
pub type EphshkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPSTALL` reader - Endpoint Stalled"]
pub type EpstallR = crate::BitReader;
#[doc = "Field `EPSTALL` writer - Endpoint Stalled"]
pub type EpstallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPTXEN` reader - Endpoint for TX transfers enable"]
pub type EptxenR = crate::BitReader;
#[doc = "Field `EPTXEN` writer - Endpoint for TX transfers enable"]
pub type EptxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPRXEN` reader - Endpoint for RX transfers enable"]
pub type EprxenR = crate::BitReader;
#[doc = "Field `EPRXEN` writer - Endpoint for RX transfers enable"]
pub type EprxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Control Transfer Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Epctldis {
    #[doc = "0: Enable"]
    Enable = 0,
    #[doc = "1: Disable"]
    Disable = 1,
}
impl From<Epctldis> for bool {
    #[inline(always)]
    fn from(variant: Epctldis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPCTLDIS` reader - Control Transfer Disable"]
pub type EpctldisR = crate::BitReader<Epctldis>;
impl EpctldisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Epctldis {
        match self.bits {
            false => Epctldis::Enable,
            true => Epctldis::Disable,
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Epctldis::Enable
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Epctldis::Disable
    }
}
#[doc = "Field `EPCTLDIS` writer - Control Transfer Disable"]
pub type EpctldisW<'a, REG> = crate::BitWriter<'a, REG, Epctldis>;
impl<'a, REG> EpctldisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Epctldis::Enable)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Epctldis::Disable)
    }
}
impl R {
    #[doc = "Bit 0 - Endpoint Handshaking Enable"]
    #[inline(always)]
    pub fn ephshk(&self) -> EphshkR {
        EphshkR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint Stalled"]
    #[inline(always)]
    pub fn epstall(&self) -> EpstallR {
        EpstallR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Endpoint for TX transfers enable"]
    #[inline(always)]
    pub fn eptxen(&self) -> EptxenR {
        EptxenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Endpoint for RX transfers enable"]
    #[inline(always)]
    pub fn eprxen(&self) -> EprxenR {
        EprxenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Control Transfer Disable"]
    #[inline(always)]
    pub fn epctldis(&self) -> EpctldisR {
        EpctldisR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Endpoint Handshaking Enable"]
    #[inline(always)]
    pub fn ephshk(&mut self) -> EphshkW<'_, EndptSpec> {
        EphshkW::new(self, 0)
    }
    #[doc = "Bit 1 - Endpoint Stalled"]
    #[inline(always)]
    pub fn epstall(&mut self) -> EpstallW<'_, EndptSpec> {
        EpstallW::new(self, 1)
    }
    #[doc = "Bit 2 - Endpoint for TX transfers enable"]
    #[inline(always)]
    pub fn eptxen(&mut self) -> EptxenW<'_, EndptSpec> {
        EptxenW::new(self, 2)
    }
    #[doc = "Bit 3 - Endpoint for RX transfers enable"]
    #[inline(always)]
    pub fn eprxen(&mut self) -> EprxenW<'_, EndptSpec> {
        EprxenW::new(self, 3)
    }
    #[doc = "Bit 4 - Control Transfer Disable"]
    #[inline(always)]
    pub fn epctldis(&mut self) -> EpctldisW<'_, EndptSpec> {
        EpctldisW::new(self, 4)
    }
}
#[doc = "Endpoint Control\n\nYou can [`read`](crate::Reg::read) this register and get [`endpt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`endpt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EndptSpec;
impl crate::RegisterSpec for EndptSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`endpt::R`](R) reader structure"]
impl crate::Readable for EndptSpec {}
#[doc = "`write(|w| ..)` method takes [`endpt::W`](W) writer structure"]
impl crate::Writable for EndptSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ENDPT to value 0"]
impl crate::Resettable for EndptSpec {}
