#[doc = "Register `R_TEST_CTRL` reader"]
pub type R = crate::R<RTestCtrlSpec>;
#[doc = "Register `R_TEST_CTRL` writer"]
pub type W = crate::W<RTestCtrlSpec>;
#[doc = "BIST Busy Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Busy {
    #[doc = "0: BIST is idle"]
    Zz333 = 0,
    #[doc = "1: BIST is busy"]
    Zz334 = 1,
}
impl From<Busy> for bool {
    #[inline(always)]
    fn from(variant: Busy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSY` reader - BIST Busy Status"]
pub type BusyR = crate::BitReader<Busy>;
impl BusyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Busy {
        match self.bits {
            false => Busy::Zz333,
            true => Busy::Zz334,
        }
    }
    #[doc = "BIST is idle"]
    #[inline(always)]
    pub fn is_zz333(&self) -> bool {
        *self == Busy::Zz333
    }
    #[doc = "BIST is busy"]
    #[inline(always)]
    pub fn is_zz334(&self) -> bool {
        *self == Busy::Zz334
    }
}
#[doc = "Field `DEBUG` reader - BIST Debug Status"]
pub type DebugR = crate::BitReader;
#[doc = "BIST Status 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status0 {
    #[doc = "0: BIST test passed on flash block 0"]
    Zz331 = 0,
    #[doc = "1: BIST test failed on flash block 0"]
    Zz332 = 1,
}
impl From<Status0> for bool {
    #[inline(always)]
    fn from(variant: Status0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STATUS0` reader - BIST Status 0"]
pub type Status0R = crate::BitReader<Status0>;
impl Status0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Status0 {
        match self.bits {
            false => Status0::Zz331,
            true => Status0::Zz332,
        }
    }
    #[doc = "BIST test passed on flash block 0"]
    #[inline(always)]
    pub fn is_zz331(&self) -> bool {
        *self == Status0::Zz331
    }
    #[doc = "BIST test failed on flash block 0"]
    #[inline(always)]
    pub fn is_zz332(&self) -> bool {
        *self == Status0::Zz332
    }
}
#[doc = "BIST status 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status1 {
    #[doc = "0: BIST test passed on flash block 1"]
    Zz329 = 0,
    #[doc = "1: BIST test failed on flash block 1"]
    Zz330 = 1,
}
impl From<Status1> for bool {
    #[inline(always)]
    fn from(variant: Status1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STATUS1` reader - BIST status 1"]
pub type Status1R = crate::BitReader<Status1>;
impl Status1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Status1 {
        match self.bits {
            false => Status1::Zz329,
            true => Status1::Zz330,
        }
    }
    #[doc = "BIST test passed on flash block 1"]
    #[inline(always)]
    pub fn is_zz329(&self) -> bool {
        *self == Status1::Zz329
    }
    #[doc = "BIST test failed on flash block 1"]
    #[inline(always)]
    pub fn is_zz330(&self) -> bool {
        *self == Status1::Zz330
    }
}
#[doc = "Field `DEBUGRUN` reader - BIST Continue Debug Run"]
pub type DebugrunR = crate::BitReader;
#[doc = "Field `DEBUGRUN` writer - BIST Continue Debug Run"]
pub type DebugrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STARTRUN` reader - Run New BIST Operation"]
pub type StartrunR = crate::BitReader;
#[doc = "Field `STARTRUN` writer - Run New BIST Operation"]
pub type StartrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDINDEX` reader - BIST Command Index (code)"]
pub type CmdindexR = crate::FieldReader<u16>;
#[doc = "Field `CMDINDEX` writer - BIST Command Index (code)"]
pub type CmdindexW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `DISABLE_IP1` reader - BIST Disable IP1"]
pub type DisableIp1R = crate::BitReader;
#[doc = "Field `DISABLE_IP1` writer - BIST Disable IP1"]
pub type DisableIp1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - BIST Busy Status"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BIST Debug Status"]
    #[inline(always)]
    pub fn debug(&self) -> DebugR {
        DebugR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BIST Status 0"]
    #[inline(always)]
    pub fn status0(&self) -> Status0R {
        Status0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - BIST status 1"]
    #[inline(always)]
    pub fn status1(&self) -> Status1R {
        Status1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - BIST Continue Debug Run"]
    #[inline(always)]
    pub fn debugrun(&self) -> DebugrunR {
        DebugrunR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Run New BIST Operation"]
    #[inline(always)]
    pub fn startrun(&self) -> StartrunR {
        StartrunR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:15 - BIST Command Index (code)"]
    #[inline(always)]
    pub fn cmdindex(&self) -> CmdindexR {
        CmdindexR::new(((self.bits >> 6) & 0x03ff) as u16)
    }
    #[doc = "Bit 16 - BIST Disable IP1"]
    #[inline(always)]
    pub fn disable_ip1(&self) -> DisableIp1R {
        DisableIp1R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - BIST Continue Debug Run"]
    #[inline(always)]
    pub fn debugrun(&mut self) -> DebugrunW<'_, RTestCtrlSpec> {
        DebugrunW::new(self, 4)
    }
    #[doc = "Bit 5 - Run New BIST Operation"]
    #[inline(always)]
    pub fn startrun(&mut self) -> StartrunW<'_, RTestCtrlSpec> {
        StartrunW::new(self, 5)
    }
    #[doc = "Bits 6:15 - BIST Command Index (code)"]
    #[inline(always)]
    pub fn cmdindex(&mut self) -> CmdindexW<'_, RTestCtrlSpec> {
        CmdindexW::new(self, 6)
    }
    #[doc = "Bit 16 - BIST Disable IP1"]
    #[inline(always)]
    pub fn disable_ip1(&mut self) -> DisableIp1W<'_, RTestCtrlSpec> {
        DisableIp1W::new(self, 16)
    }
}
#[doc = "BIST Test Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r_test_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r_test_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTestCtrlSpec;
impl crate::RegisterSpec for RTestCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r_test_ctrl::R`](R) reader structure"]
impl crate::Readable for RTestCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`r_test_ctrl::W`](W) writer structure"]
impl crate::Writable for RTestCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R_TEST_CTRL to value 0"]
impl crate::Resettable for RTestCtrlSpec {}
