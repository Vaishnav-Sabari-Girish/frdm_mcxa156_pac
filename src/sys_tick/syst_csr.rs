#[doc = "Register `SYST_CSR` reader"]
pub type R = crate::R<SystCsrSpec>;
#[doc = "Register `SYST_CSR` writer"]
pub type W = crate::W<SystCsrSpec>;
#[doc = "Enable/disable systick counter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enable {
    #[doc = "0: counter disabled"]
    CounterDisabled = 0,
    #[doc = "1: counter enabled"]
    CounterEnabled = 1,
}
impl From<Enable> for bool {
    #[inline(always)]
    fn from(variant: Enable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` reader - Enable/disable systick counter"]
pub type EnableR = crate::BitReader<Enable>;
impl EnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enable {
        match self.bits {
            false => Enable::CounterDisabled,
            true => Enable::CounterEnabled,
        }
    }
    #[doc = "counter disabled"]
    #[inline(always)]
    pub fn is_counter_disabled(&self) -> bool {
        *self == Enable::CounterDisabled
    }
    #[doc = "counter enabled"]
    #[inline(always)]
    pub fn is_counter_enabled(&self) -> bool {
        *self == Enable::CounterEnabled
    }
}
#[doc = "Field `ENABLE` writer - Enable/disable systick counter"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG, Enable>;
impl<'a, REG> EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "counter disabled"]
    #[inline(always)]
    pub fn counter_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::CounterDisabled)
    }
    #[doc = "counter enabled"]
    #[inline(always)]
    pub fn counter_enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::CounterEnabled)
    }
}
#[doc = "Enable Systick interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tickint {
    #[doc = "0: counting down to 0 does not assert the SysTick exception request"]
    InterruptDisabled = 0,
    #[doc = "1: counting down to 0 asserts the SysTick exception request"]
    InterruptEnabled = 1,
}
impl From<Tickint> for bool {
    #[inline(always)]
    fn from(variant: Tickint) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TICKINT` reader - Enable Systick interrupt."]
pub type TickintR = crate::BitReader<Tickint>;
impl TickintR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tickint {
        match self.bits {
            false => Tickint::InterruptDisabled,
            true => Tickint::InterruptEnabled,
        }
    }
    #[doc = "counting down to 0 does not assert the SysTick exception request"]
    #[inline(always)]
    pub fn is_interrupt_disabled(&self) -> bool {
        *self == Tickint::InterruptDisabled
    }
    #[doc = "counting down to 0 asserts the SysTick exception request"]
    #[inline(always)]
    pub fn is_interrupt_enabled(&self) -> bool {
        *self == Tickint::InterruptEnabled
    }
}
#[doc = "Field `TICKINT` writer - Enable Systick interrupt."]
pub type TickintW<'a, REG> = crate::BitWriter<'a, REG, Tickint>;
impl<'a, REG> TickintW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "counting down to 0 does not assert the SysTick exception request"]
    #[inline(always)]
    pub fn interrupt_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Tickint::InterruptDisabled)
    }
    #[doc = "counting down to 0 asserts the SysTick exception request"]
    #[inline(always)]
    pub fn interrupt_enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Tickint::InterruptEnabled)
    }
}
#[doc = "Clock source selection.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clksource {
    #[doc = "0: external clock"]
    ExternalClock = 0,
    #[doc = "1: processor clock"]
    ProcessorClock = 1,
}
impl From<Clksource> for bool {
    #[inline(always)]
    fn from(variant: Clksource) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKSOURCE` reader - Clock source selection."]
pub type ClksourceR = crate::BitReader<Clksource>;
impl ClksourceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clksource {
        match self.bits {
            false => Clksource::ExternalClock,
            true => Clksource::ProcessorClock,
        }
    }
    #[doc = "external clock"]
    #[inline(always)]
    pub fn is_external_clock(&self) -> bool {
        *self == Clksource::ExternalClock
    }
    #[doc = "processor clock"]
    #[inline(always)]
    pub fn is_processor_clock(&self) -> bool {
        *self == Clksource::ProcessorClock
    }
}
#[doc = "Field `CLKSOURCE` writer - Clock source selection."]
pub type ClksourceW<'a, REG> = crate::BitWriter<'a, REG, Clksource>;
impl<'a, REG> ClksourceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "external clock"]
    #[inline(always)]
    pub fn external_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Clksource::ExternalClock)
    }
    #[doc = "processor clock"]
    #[inline(always)]
    pub fn processor_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Clksource::ProcessorClock)
    }
}
#[doc = "Field `COUNTFLAG` reader - Returns 1 if timer counted to 0 since the last read of this register."]
pub type CountflagR = crate::BitReader;
#[doc = "Field `COUNTFLAG` writer - Returns 1 if timer counted to 0 since the last read of this register."]
pub type CountflagW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable/disable systick counter"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Systick interrupt."]
    #[inline(always)]
    pub fn tickint(&self) -> TickintR {
        TickintR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clock source selection."]
    #[inline(always)]
    pub fn clksource(&self) -> ClksourceR {
        ClksourceR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 16 - Returns 1 if timer counted to 0 since the last read of this register."]
    #[inline(always)]
    pub fn countflag(&self) -> CountflagR {
        CountflagR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable/disable systick counter"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, SystCsrSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Systick interrupt."]
    #[inline(always)]
    pub fn tickint(&mut self) -> TickintW<'_, SystCsrSpec> {
        TickintW::new(self, 1)
    }
    #[doc = "Bit 2 - Clock source selection."]
    #[inline(always)]
    pub fn clksource(&mut self) -> ClksourceW<'_, SystCsrSpec> {
        ClksourceW::new(self, 2)
    }
    #[doc = "Bit 16 - Returns 1 if timer counted to 0 since the last read of this register."]
    #[inline(always)]
    pub fn countflag(&mut self) -> CountflagW<'_, SystCsrSpec> {
        CountflagW::new(self, 16)
    }
}
#[doc = "SysTick Control and Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`syst_csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syst_csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SystCsrSpec;
impl crate::RegisterSpec for SystCsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syst_csr::R`](R) reader structure"]
impl crate::Readable for SystCsrSpec {}
#[doc = "`write(|w| ..)` method takes [`syst_csr::W`](W) writer structure"]
impl crate::Writable for SystCsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYST_CSR to value 0x04"]
impl crate::Resettable for SystCsrSpec {
    const RESET_VALUE: u32 = 0x04;
}
