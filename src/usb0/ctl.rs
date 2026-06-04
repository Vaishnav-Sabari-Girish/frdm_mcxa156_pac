#[doc = "Register `CTL` reader"]
pub type R = crate::R<CtlSpec>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "USB Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usbensofen {
    #[doc = "0: Disable"]
    DisUsbSof = 0,
    #[doc = "1: Enable"]
    EnUsbSof = 1,
}
impl From<Usbensofen> for bool {
    #[inline(always)]
    fn from(variant: Usbensofen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBENSOFEN` reader - USB Enable"]
pub type UsbensofenR = crate::BitReader<Usbensofen>;
impl UsbensofenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usbensofen {
        match self.bits {
            false => Usbensofen::DisUsbSof,
            true => Usbensofen::EnUsbSof,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_dis_usb_sof(&self) -> bool {
        *self == Usbensofen::DisUsbSof
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_en_usb_sof(&self) -> bool {
        *self == Usbensofen::EnUsbSof
    }
}
#[doc = "Field `USBENSOFEN` writer - USB Enable"]
pub type UsbensofenW<'a, REG> = crate::BitWriter<'a, REG, Usbensofen>;
impl<'a, REG> UsbensofenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn dis_usb_sof(self) -> &'a mut crate::W<REG> {
        self.variant(Usbensofen::DisUsbSof)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn en_usb_sof(self) -> &'a mut crate::W<REG> {
        self.variant(Usbensofen::EnUsbSof)
    }
}
#[doc = "Field `ODDRST` reader - Odd Reset"]
pub type OddrstR = crate::BitReader;
#[doc = "Field `ODDRST` writer - Odd Reset"]
pub type OddrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESUME` reader - Resume"]
pub type ResumeR = crate::BitReader;
#[doc = "Field `RESUME` writer - Resume"]
pub type ResumeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXSUSPENDTOKENBUSY` reader - TXD Suspend And Token Busy"]
pub type TxsuspendtokenbusyR = crate::BitReader;
#[doc = "Field `TXSUSPENDTOKENBUSY` writer - TXD Suspend And Token Busy"]
pub type TxsuspendtokenbusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SE0` reader - Live USB Single-Ended Zero signal"]
pub type Se0R = crate::BitReader;
#[doc = "Field `SE0` writer - Live USB Single-Ended Zero signal"]
pub type Se0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - USB Enable"]
    #[inline(always)]
    pub fn usbensofen(&self) -> UsbensofenR {
        UsbensofenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Odd Reset"]
    #[inline(always)]
    pub fn oddrst(&self) -> OddrstR {
        OddrstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Resume"]
    #[inline(always)]
    pub fn resume(&self) -> ResumeR {
        ResumeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - TXD Suspend And Token Busy"]
    #[inline(always)]
    pub fn txsuspendtokenbusy(&self) -> TxsuspendtokenbusyR {
        TxsuspendtokenbusyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Live USB Single-Ended Zero signal"]
    #[inline(always)]
    pub fn se0(&self) -> Se0R {
        Se0R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB Enable"]
    #[inline(always)]
    pub fn usbensofen(&mut self) -> UsbensofenW<'_, CtlSpec> {
        UsbensofenW::new(self, 0)
    }
    #[doc = "Bit 1 - Odd Reset"]
    #[inline(always)]
    pub fn oddrst(&mut self) -> OddrstW<'_, CtlSpec> {
        OddrstW::new(self, 1)
    }
    #[doc = "Bit 2 - Resume"]
    #[inline(always)]
    pub fn resume(&mut self) -> ResumeW<'_, CtlSpec> {
        ResumeW::new(self, 2)
    }
    #[doc = "Bit 5 - TXD Suspend And Token Busy"]
    #[inline(always)]
    pub fn txsuspendtokenbusy(&mut self) -> TxsuspendtokenbusyW<'_, CtlSpec> {
        TxsuspendtokenbusyW::new(self, 5)
    }
    #[doc = "Bit 6 - Live USB Single-Ended Zero signal"]
    #[inline(always)]
    pub fn se0(&mut self) -> Se0W<'_, CtlSpec> {
        Se0W::new(self, 6)
    }
}
#[doc = "Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlSpec;
impl crate::RegisterSpec for CtlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CtlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CtlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CtlSpec {}
