#[doc = "Register `R_CNT_LOOP_CTRL` reader"]
pub type R = crate::R<RCntLoopCtrlSpec>;
#[doc = "Register `R_CNT_LOOP_CTRL` writer"]
pub type W = crate::W<RCntLoopCtrlSpec>;
#[doc = "Field `LOOPCNT` reader - Loop Count Control"]
pub type LoopcntR = crate::FieldReader<u16>;
#[doc = "Field `LOOPCNT` writer - Loop Count Control"]
pub type LoopcntW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Loop Option\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Loopopt {
    #[doc = "0: Loop is disabled; selected BIST operation is run once"]
    Zz284 = 0,
    #[doc = "1: Loop is enabled; XADR increments by 1 XADR increments by 1 for each new loop. Stops when total loop count meets LOOPCNT+1."]
    Zz285 = 1,
    #[doc = "2: Loop is enabled; YADR increments by 1 YADR increments by 1 for each new loop. Stops when total loop count meets LOOPCNT+1."]
    Zz286 = 2,
    #[doc = "3: Loop is enabled; XADR increments by 2 XADR increments by 2 for each new loop. Stops when total loop count meets LOOPCNT+1."]
    Zz287 = 3,
    #[doc = "4: Loop is enabled; XADR increments by sector XADR increments by 16 for each new loop. Stops when total loop count meets LOOPCNT+1."]
    Zz288 = 4,
}
impl From<Loopopt> for u8 {
    #[inline(always)]
    fn from(variant: Loopopt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Loopopt {
    type Ux = u8;
}
impl crate::IsEnum for Loopopt {}
#[doc = "Field `LOOPOPT` reader - Loop Option"]
pub type LoopoptR = crate::FieldReader<Loopopt>;
impl LoopoptR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Loopopt> {
        match self.bits {
            0 => Some(Loopopt::Zz284),
            1 => Some(Loopopt::Zz285),
            2 => Some(Loopopt::Zz286),
            3 => Some(Loopopt::Zz287),
            4 => Some(Loopopt::Zz288),
            _ => None,
        }
    }
    #[doc = "Loop is disabled; selected BIST operation is run once"]
    #[inline(always)]
    pub fn is_zz284(&self) -> bool {
        *self == Loopopt::Zz284
    }
    #[doc = "Loop is enabled; XADR increments by 1 XADR increments by 1 for each new loop. Stops when total loop count meets LOOPCNT+1."]
    #[inline(always)]
    pub fn is_zz285(&self) -> bool {
        *self == Loopopt::Zz285
    }
    #[doc = "Loop is enabled; YADR increments by 1 YADR increments by 1 for each new loop. Stops when total loop count meets LOOPCNT+1."]
    #[inline(always)]
    pub fn is_zz286(&self) -> bool {
        *self == Loopopt::Zz286
    }
    #[doc = "Loop is enabled; XADR increments by 2 XADR increments by 2 for each new loop. Stops when total loop count meets LOOPCNT+1."]
    #[inline(always)]
    pub fn is_zz287(&self) -> bool {
        *self == Loopopt::Zz287
    }
    #[doc = "Loop is enabled; XADR increments by sector XADR increments by 16 for each new loop. Stops when total loop count meets LOOPCNT+1."]
    #[inline(always)]
    pub fn is_zz288(&self) -> bool {
        *self == Loopopt::Zz288
    }
}
#[doc = "Field `LOOPOPT` writer - Loop Option"]
pub type LoopoptW<'a, REG> = crate::FieldWriter<'a, REG, 3, Loopopt>;
impl<'a, REG> LoopoptW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Loop is disabled; selected BIST operation is run once"]
    #[inline(always)]
    pub fn zz284(self) -> &'a mut crate::W<REG> {
        self.variant(Loopopt::Zz284)
    }
    #[doc = "Loop is enabled; XADR increments by 1 XADR increments by 1 for each new loop. Stops when total loop count meets LOOPCNT+1."]
    #[inline(always)]
    pub fn zz285(self) -> &'a mut crate::W<REG> {
        self.variant(Loopopt::Zz285)
    }
    #[doc = "Loop is enabled; YADR increments by 1 YADR increments by 1 for each new loop. Stops when total loop count meets LOOPCNT+1."]
    #[inline(always)]
    pub fn zz286(self) -> &'a mut crate::W<REG> {
        self.variant(Loopopt::Zz286)
    }
    #[doc = "Loop is enabled; XADR increments by 2 XADR increments by 2 for each new loop. Stops when total loop count meets LOOPCNT+1."]
    #[inline(always)]
    pub fn zz287(self) -> &'a mut crate::W<REG> {
        self.variant(Loopopt::Zz287)
    }
    #[doc = "Loop is enabled; XADR increments by sector XADR increments by 16 for each new loop. Stops when total loop count meets LOOPCNT+1."]
    #[inline(always)]
    pub fn zz288(self) -> &'a mut crate::W<REG> {
        self.variant(Loopopt::Zz288)
    }
}
#[doc = "Loop Time Unit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Loopunit {
    #[doc = "0: Clock cycles"]
    Zz276 = 0,
    #[doc = "1: 0.5 usec"]
    Zz277 = 1,
    #[doc = "2: 1 usec"]
    Zz278 = 2,
    #[doc = "3: 10 usec"]
    Zz279 = 3,
    #[doc = "4: 100 usec"]
    Zz280 = 4,
    #[doc = "5: 1 msec"]
    Zz281 = 5,
    #[doc = "6: 10 msec"]
    Zz282 = 6,
    #[doc = "7: 100 msec"]
    Zz283 = 7,
}
impl From<Loopunit> for u8 {
    #[inline(always)]
    fn from(variant: Loopunit) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Loopunit {
    type Ux = u8;
}
impl crate::IsEnum for Loopunit {}
#[doc = "Field `LOOPUNIT` reader - Loop Time Unit"]
pub type LoopunitR = crate::FieldReader<Loopunit>;
impl LoopunitR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Loopunit {
        match self.bits {
            0 => Loopunit::Zz276,
            1 => Loopunit::Zz277,
            2 => Loopunit::Zz278,
            3 => Loopunit::Zz279,
            4 => Loopunit::Zz280,
            5 => Loopunit::Zz281,
            6 => Loopunit::Zz282,
            7 => Loopunit::Zz283,
            _ => unreachable!(),
        }
    }
    #[doc = "Clock cycles"]
    #[inline(always)]
    pub fn is_zz276(&self) -> bool {
        *self == Loopunit::Zz276
    }
    #[doc = "0.5 usec"]
    #[inline(always)]
    pub fn is_zz277(&self) -> bool {
        *self == Loopunit::Zz277
    }
    #[doc = "1 usec"]
    #[inline(always)]
    pub fn is_zz278(&self) -> bool {
        *self == Loopunit::Zz278
    }
    #[doc = "10 usec"]
    #[inline(always)]
    pub fn is_zz279(&self) -> bool {
        *self == Loopunit::Zz279
    }
    #[doc = "100 usec"]
    #[inline(always)]
    pub fn is_zz280(&self) -> bool {
        *self == Loopunit::Zz280
    }
    #[doc = "1 msec"]
    #[inline(always)]
    pub fn is_zz281(&self) -> bool {
        *self == Loopunit::Zz281
    }
    #[doc = "10 msec"]
    #[inline(always)]
    pub fn is_zz282(&self) -> bool {
        *self == Loopunit::Zz282
    }
    #[doc = "100 msec"]
    #[inline(always)]
    pub fn is_zz283(&self) -> bool {
        *self == Loopunit::Zz283
    }
}
#[doc = "Field `LOOPUNIT` writer - Loop Time Unit"]
pub type LoopunitW<'a, REG> = crate::FieldWriter<'a, REG, 3, Loopunit, crate::Safe>;
impl<'a, REG> LoopunitW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Clock cycles"]
    #[inline(always)]
    pub fn zz276(self) -> &'a mut crate::W<REG> {
        self.variant(Loopunit::Zz276)
    }
    #[doc = "0.5 usec"]
    #[inline(always)]
    pub fn zz277(self) -> &'a mut crate::W<REG> {
        self.variant(Loopunit::Zz277)
    }
    #[doc = "1 usec"]
    #[inline(always)]
    pub fn zz278(self) -> &'a mut crate::W<REG> {
        self.variant(Loopunit::Zz278)
    }
    #[doc = "10 usec"]
    #[inline(always)]
    pub fn zz279(self) -> &'a mut crate::W<REG> {
        self.variant(Loopunit::Zz279)
    }
    #[doc = "100 usec"]
    #[inline(always)]
    pub fn zz280(self) -> &'a mut crate::W<REG> {
        self.variant(Loopunit::Zz280)
    }
    #[doc = "1 msec"]
    #[inline(always)]
    pub fn zz281(self) -> &'a mut crate::W<REG> {
        self.variant(Loopunit::Zz281)
    }
    #[doc = "10 msec"]
    #[inline(always)]
    pub fn zz282(self) -> &'a mut crate::W<REG> {
        self.variant(Loopunit::Zz282)
    }
    #[doc = "100 msec"]
    #[inline(always)]
    pub fn zz283(self) -> &'a mut crate::W<REG> {
        self.variant(Loopunit::Zz283)
    }
}
#[doc = "Field `LOOPDLY` reader - Loop Time Delay Scalar"]
pub type LoopdlyR = crate::FieldReader;
#[doc = "Field `LOOPDLY` writer - Loop Time Delay Scalar"]
pub type LoopdlyW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:11 - Loop Count Control"]
    #[inline(always)]
    pub fn loopcnt(&self) -> LoopcntR {
        LoopcntR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:14 - Loop Option"]
    #[inline(always)]
    pub fn loopopt(&self) -> LoopoptR {
        LoopoptR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - Loop Time Unit"]
    #[inline(always)]
    pub fn loopunit(&self) -> LoopunitR {
        LoopunitR::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:24 - Loop Time Delay Scalar"]
    #[inline(always)]
    pub fn loopdly(&self) -> LoopdlyR {
        LoopdlyR::new(((self.bits >> 18) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - Loop Count Control"]
    #[inline(always)]
    pub fn loopcnt(&mut self) -> LoopcntW<'_, RCntLoopCtrlSpec> {
        LoopcntW::new(self, 0)
    }
    #[doc = "Bits 12:14 - Loop Option"]
    #[inline(always)]
    pub fn loopopt(&mut self) -> LoopoptW<'_, RCntLoopCtrlSpec> {
        LoopoptW::new(self, 12)
    }
    #[doc = "Bits 15:17 - Loop Time Unit"]
    #[inline(always)]
    pub fn loopunit(&mut self) -> LoopunitW<'_, RCntLoopCtrlSpec> {
        LoopunitW::new(self, 15)
    }
    #[doc = "Bits 18:24 - Loop Time Delay Scalar"]
    #[inline(always)]
    pub fn loopdly(&mut self) -> LoopdlyW<'_, RCntLoopCtrlSpec> {
        LoopdlyW::new(self, 18)
    }
}
#[doc = "BIST Loop Count Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r_cnt_loop_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r_cnt_loop_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCntLoopCtrlSpec;
impl crate::RegisterSpec for RCntLoopCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r_cnt_loop_ctrl::R`](R) reader structure"]
impl crate::Readable for RCntLoopCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`r_cnt_loop_ctrl::W`](W) writer structure"]
impl crate::Writable for RCntLoopCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R_CNT_LOOP_CTRL to value 0"]
impl crate::Resettable for RCntLoopCtrlSpec {}
