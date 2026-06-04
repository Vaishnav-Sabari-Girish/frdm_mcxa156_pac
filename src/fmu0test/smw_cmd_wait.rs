#[doc = "Register `SMW_CMD_WAIT` reader"]
pub type R = crate::R<SmwCmdWaitSpec>;
#[doc = "Register `SMW_CMD_WAIT` writer"]
pub type W = crate::W<SmwCmdWaitSpec>;
#[doc = "SMW Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmd {
    #[doc = "0: IDLE"]
    Zz193 = 0,
    #[doc = "1: ABORT"]
    Zz194 = 1,
    #[doc = "2: SME2 to one-shot mass erase"]
    Zz195 = 2,
    #[doc = "3: SME3 to sector erase on selected array"]
    Zz196 = 3,
    #[doc = "4: SMP1 to program phrase or page on selected array with shot disabled on previously programmed bit"]
    Zz197 = 4,
    #[doc = "6: SMP2 to program phrase or page on selected array to repair cells of weak program after power loss"]
    Zz199 = 6,
}
impl From<Cmd> for u8 {
    #[inline(always)]
    fn from(variant: Cmd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmd {
    type Ux = u8;
}
impl crate::IsEnum for Cmd {}
#[doc = "Field `CMD` reader - SMW Command"]
pub type CmdR = crate::FieldReader<Cmd>;
impl CmdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cmd> {
        match self.bits {
            0 => Some(Cmd::Zz193),
            1 => Some(Cmd::Zz194),
            2 => Some(Cmd::Zz195),
            3 => Some(Cmd::Zz196),
            4 => Some(Cmd::Zz197),
            6 => Some(Cmd::Zz199),
            _ => None,
        }
    }
    #[doc = "IDLE"]
    #[inline(always)]
    pub fn is_zz193(&self) -> bool {
        *self == Cmd::Zz193
    }
    #[doc = "ABORT"]
    #[inline(always)]
    pub fn is_zz194(&self) -> bool {
        *self == Cmd::Zz194
    }
    #[doc = "SME2 to one-shot mass erase"]
    #[inline(always)]
    pub fn is_zz195(&self) -> bool {
        *self == Cmd::Zz195
    }
    #[doc = "SME3 to sector erase on selected array"]
    #[inline(always)]
    pub fn is_zz196(&self) -> bool {
        *self == Cmd::Zz196
    }
    #[doc = "SMP1 to program phrase or page on selected array with shot disabled on previously programmed bit"]
    #[inline(always)]
    pub fn is_zz197(&self) -> bool {
        *self == Cmd::Zz197
    }
    #[doc = "SMP2 to program phrase or page on selected array to repair cells of weak program after power loss"]
    #[inline(always)]
    pub fn is_zz199(&self) -> bool {
        *self == Cmd::Zz199
    }
}
#[doc = "Field `CMD` writer - SMW Command"]
pub type CmdW<'a, REG> = crate::FieldWriter<'a, REG, 3, Cmd>;
impl<'a, REG> CmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "IDLE"]
    #[inline(always)]
    pub fn zz193(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd::Zz193)
    }
    #[doc = "ABORT"]
    #[inline(always)]
    pub fn zz194(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd::Zz194)
    }
    #[doc = "SME2 to one-shot mass erase"]
    #[inline(always)]
    pub fn zz195(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd::Zz195)
    }
    #[doc = "SME3 to sector erase on selected array"]
    #[inline(always)]
    pub fn zz196(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd::Zz196)
    }
    #[doc = "SMP1 to program phrase or page on selected array with shot disabled on previously programmed bit"]
    #[inline(always)]
    pub fn zz197(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd::Zz197)
    }
    #[doc = "SMP2 to program phrase or page on selected array to repair cells of weak program after power loss"]
    #[inline(always)]
    pub fn zz199(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd::Zz199)
    }
}
#[doc = "SMW Wait Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WaitEn {
    #[doc = "0: Wait feature disabled"]
    Zz191 = 0,
    #[doc = "1: Wait feature enabled"]
    Zz192 = 1,
}
impl From<WaitEn> for bool {
    #[inline(always)]
    fn from(variant: WaitEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAIT_EN` reader - SMW Wait Enable"]
pub type WaitEnR = crate::BitReader<WaitEn>;
impl WaitEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WaitEn {
        match self.bits {
            false => WaitEn::Zz191,
            true => WaitEn::Zz192,
        }
    }
    #[doc = "Wait feature disabled"]
    #[inline(always)]
    pub fn is_zz191(&self) -> bool {
        *self == WaitEn::Zz191
    }
    #[doc = "Wait feature enabled"]
    #[inline(always)]
    pub fn is_zz192(&self) -> bool {
        *self == WaitEn::Zz192
    }
}
#[doc = "Field `WAIT_EN` writer - SMW Wait Enable"]
pub type WaitEnW<'a, REG> = crate::BitWriter<'a, REG, WaitEn>;
impl<'a, REG> WaitEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wait feature disabled"]
    #[inline(always)]
    pub fn zz191(self) -> &'a mut crate::W<REG> {
        self.variant(WaitEn::Zz191)
    }
    #[doc = "Wait feature enabled"]
    #[inline(always)]
    pub fn zz192(self) -> &'a mut crate::W<REG> {
        self.variant(WaitEn::Zz192)
    }
}
#[doc = "Field `WAIT_AUTO_SET` reader - SMW Wait Auto Set"]
pub type WaitAutoSetR = crate::BitReader;
#[doc = "Field `WAIT_AUTO_SET` writer - SMW Wait Auto Set"]
pub type WaitAutoSetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - SMW Command"]
    #[inline(always)]
    pub fn cmd(&self) -> CmdR {
        CmdR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - SMW Wait Enable"]
    #[inline(always)]
    pub fn wait_en(&self) -> WaitEnR {
        WaitEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SMW Wait Auto Set"]
    #[inline(always)]
    pub fn wait_auto_set(&self) -> WaitAutoSetR {
        WaitAutoSetR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - SMW Command"]
    #[inline(always)]
    pub fn cmd(&mut self) -> CmdW<'_, SmwCmdWaitSpec> {
        CmdW::new(self, 0)
    }
    #[doc = "Bit 3 - SMW Wait Enable"]
    #[inline(always)]
    pub fn wait_en(&mut self) -> WaitEnW<'_, SmwCmdWaitSpec> {
        WaitEnW::new(self, 3)
    }
    #[doc = "Bit 4 - SMW Wait Auto Set"]
    #[inline(always)]
    pub fn wait_auto_set(&mut self) -> WaitAutoSetW<'_, SmwCmdWaitSpec> {
        WaitAutoSetW::new(self, 4)
    }
}
#[doc = "SMW Command and Wait Register\n\nYou can [`read`](crate::Reg::read) this register and get [`smw_cmd_wait::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smw_cmd_wait::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmwCmdWaitSpec;
impl crate::RegisterSpec for SmwCmdWaitSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smw_cmd_wait::R`](R) reader structure"]
impl crate::Readable for SmwCmdWaitSpec {}
#[doc = "`write(|w| ..)` method takes [`smw_cmd_wait::W`](W) writer structure"]
impl crate::Writable for SmwCmdWaitSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SMW_CMD_WAIT to value 0"]
impl crate::Resettable for SmwCmdWaitSpec {}
