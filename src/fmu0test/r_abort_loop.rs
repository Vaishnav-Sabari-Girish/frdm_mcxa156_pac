#[doc = "Register `R_ABORT_LOOP` reader"]
pub type R = crate::R<RAbortLoopSpec>;
#[doc = "Register `R_ABORT_LOOP` writer"]
pub type W = crate::W<RAbortLoopSpec>;
#[doc = "Abort Loop\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AbortLoop {
    #[doc = "0: No effect"]
    Zz335 = 0,
    #[doc = "1: Abort BIST loop commands and force the loop counter to return to 0x0"]
    Zz336 = 1,
}
impl From<AbortLoop> for bool {
    #[inline(always)]
    fn from(variant: AbortLoop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABORT_LOOP` reader - Abort Loop"]
pub type AbortLoopR = crate::BitReader<AbortLoop>;
impl AbortLoopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AbortLoop {
        match self.bits {
            false => AbortLoop::Zz335,
            true => AbortLoop::Zz336,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_zz335(&self) -> bool {
        *self == AbortLoop::Zz335
    }
    #[doc = "Abort BIST loop commands and force the loop counter to return to 0x0"]
    #[inline(always)]
    pub fn is_zz336(&self) -> bool {
        *self == AbortLoop::Zz336
    }
}
#[doc = "Field `ABORT_LOOP` writer - Abort Loop"]
pub type AbortLoopW<'a, REG> = crate::BitWriter<'a, REG, AbortLoop>;
impl<'a, REG> AbortLoopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn zz335(self) -> &'a mut crate::W<REG> {
        self.variant(AbortLoop::Zz335)
    }
    #[doc = "Abort BIST loop commands and force the loop counter to return to 0x0"]
    #[inline(always)]
    pub fn zz336(self) -> &'a mut crate::W<REG> {
        self.variant(AbortLoop::Zz336)
    }
}
impl R {
    #[doc = "Bit 0 - Abort Loop"]
    #[inline(always)]
    pub fn abort_loop(&self) -> AbortLoopR {
        AbortLoopR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Abort Loop"]
    #[inline(always)]
    pub fn abort_loop(&mut self) -> AbortLoopW<'_, RAbortLoopSpec> {
        AbortLoopW::new(self, 0)
    }
}
#[doc = "BIST Abort Loop Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r_abort_loop::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r_abort_loop::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RAbortLoopSpec;
impl crate::RegisterSpec for RAbortLoopSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r_abort_loop::R`](R) reader structure"]
impl crate::Readable for RAbortLoopSpec {}
#[doc = "`write(|w| ..)` method takes [`r_abort_loop::W`](W) writer structure"]
impl crate::Writable for RAbortLoopSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R_ABORT_LOOP to value 0"]
impl crate::Resettable for RAbortLoopSpec {}
