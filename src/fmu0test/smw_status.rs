#[doc = "Register `SMW_STATUS` reader"]
pub type R = crate::R<SmwStatusSpec>;
#[doc = "SMW Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SmwErr {
    #[doc = "0: Error not detected"]
    Zz205 = 0,
    #[doc = "1: Error detected"]
    Zz206 = 1,
}
impl From<SmwErr> for bool {
    #[inline(always)]
    fn from(variant: SmwErr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMW_ERR` reader - SMW Error"]
pub type SmwErrR = crate::BitReader<SmwErr>;
impl SmwErrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SmwErr {
        match self.bits {
            false => SmwErr::Zz205,
            true => SmwErr::Zz206,
        }
    }
    #[doc = "Error not detected"]
    #[inline(always)]
    pub fn is_zz205(&self) -> bool {
        *self == SmwErr::Zz205
    }
    #[doc = "Error detected"]
    #[inline(always)]
    pub fn is_zz206(&self) -> bool {
        *self == SmwErr::Zz206
    }
}
#[doc = "SMW Busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SmwBusy {
    #[doc = "0: SMW command not active"]
    Zz203 = 0,
    #[doc = "1: SMW command is active"]
    Zz204 = 1,
}
impl From<SmwBusy> for bool {
    #[inline(always)]
    fn from(variant: SmwBusy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMW_BUSY` reader - SMW Busy"]
pub type SmwBusyR = crate::BitReader<SmwBusy>;
impl SmwBusyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SmwBusy {
        match self.bits {
            false => SmwBusy::Zz203,
            true => SmwBusy::Zz204,
        }
    }
    #[doc = "SMW command not active"]
    #[inline(always)]
    pub fn is_zz203(&self) -> bool {
        *self == SmwBusy::Zz203
    }
    #[doc = "SMW command is active"]
    #[inline(always)]
    pub fn is_zz204(&self) -> bool {
        *self == SmwBusy::Zz204
    }
}
#[doc = "BIST Busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BistBusy {
    #[doc = "0: BIST Command not active"]
    Zz201 = 0,
    #[doc = "1: BIST Command is active"]
    Zz202 = 1,
}
impl From<BistBusy> for bool {
    #[inline(always)]
    fn from(variant: BistBusy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BIST_BUSY` reader - BIST Busy"]
pub type BistBusyR = crate::BitReader<BistBusy>;
impl BistBusyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BistBusy {
        match self.bits {
            false => BistBusy::Zz201,
            true => BistBusy::Zz202,
        }
    }
    #[doc = "BIST Command not active"]
    #[inline(always)]
    pub fn is_zz201(&self) -> bool {
        *self == BistBusy::Zz201
    }
    #[doc = "BIST Command is active"]
    #[inline(always)]
    pub fn is_zz202(&self) -> bool {
        *self == BistBusy::Zz202
    }
}
impl R {
    #[doc = "Bit 0 - SMW Error"]
    #[inline(always)]
    pub fn smw_err(&self) -> SmwErrR {
        SmwErrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SMW Busy"]
    #[inline(always)]
    pub fn smw_busy(&self) -> SmwBusyR {
        SmwBusyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BIST Busy"]
    #[inline(always)]
    pub fn bist_busy(&self) -> BistBusyR {
        BistBusyR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "SMW Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`smw_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmwStatusSpec;
impl crate::RegisterSpec for SmwStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smw_status::R`](R) reader structure"]
impl crate::Readable for SmwStatusSpec {}
#[doc = "`reset()` method sets SMW_STATUS to value 0"]
impl crate::Resettable for SmwStatusSpec {}
