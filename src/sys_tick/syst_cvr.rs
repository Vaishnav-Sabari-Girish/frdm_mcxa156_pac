#[doc = "Register `SYST_CVR` reader"]
pub type R = crate::R<SystCvrSpec>;
#[doc = "Register `SYST_CVR` writer"]
pub type W = crate::W<SystCvrSpec>;
#[doc = "Field `CURRENT` reader - Reads current counter value at the time the register is accessed. Any write to the register clears the register to 0."]
pub type CurrentR = crate::FieldReader<u32>;
#[doc = "Field `CURRENT` writer - Reads current counter value at the time the register is accessed. Any write to the register clears the register to 0."]
pub type CurrentW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Reads current counter value at the time the register is accessed. Any write to the register clears the register to 0."]
    #[inline(always)]
    pub fn current(&self) -> CurrentR {
        CurrentR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Reads current counter value at the time the register is accessed. Any write to the register clears the register to 0."]
    #[inline(always)]
    pub fn current(&mut self) -> CurrentW<'_, SystCvrSpec> {
        CurrentW::new(self, 0)
    }
}
#[doc = "SysTick Current Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`syst_cvr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syst_cvr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SystCvrSpec;
impl crate::RegisterSpec for SystCvrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syst_cvr::R`](R) reader structure"]
impl crate::Readable for SystCvrSpec {}
#[doc = "`write(|w| ..)` method takes [`syst_cvr::W`](W) writer structure"]
impl crate::Writable for SystCvrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYST_CVR to value 0"]
impl crate::Resettable for SystCvrSpec {}
