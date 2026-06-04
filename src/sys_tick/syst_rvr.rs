#[doc = "Register `SYST_RVR` reader"]
pub type R = crate::R<SystRvrSpec>;
#[doc = "Register `SYST_RVR` writer"]
pub type W = crate::W<SystRvrSpec>;
#[doc = "Field `RELOAD` reader - Value to load into the SysTick Current Value Register when the counter reaches 0"]
pub type ReloadR = crate::FieldReader<u32>;
#[doc = "Field `RELOAD` writer - Value to load into the SysTick Current Value Register when the counter reaches 0"]
pub type ReloadW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Value to load into the SysTick Current Value Register when the counter reaches 0"]
    #[inline(always)]
    pub fn reload(&self) -> ReloadR {
        ReloadR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Value to load into the SysTick Current Value Register when the counter reaches 0"]
    #[inline(always)]
    pub fn reload(&mut self) -> ReloadW<'_, SystRvrSpec> {
        ReloadW::new(self, 0)
    }
}
#[doc = "SysTick Reload Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`syst_rvr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syst_rvr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SystRvrSpec;
impl crate::RegisterSpec for SystRvrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syst_rvr::R`](R) reader structure"]
impl crate::Readable for SystRvrSpec {}
#[doc = "`write(|w| ..)` method takes [`syst_rvr::W`](W) writer structure"]
impl crate::Writable for SystRvrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYST_RVR to value 0"]
impl crate::Resettable for SystRvrSpec {}
