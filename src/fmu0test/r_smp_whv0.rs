#[doc = "Register `R_SMP_WHV0` reader"]
pub type R = crate::R<RSmpWhv0Spec>;
#[doc = "Register `R_SMP_WHV0` writer"]
pub type W = crate::W<RSmpWhv0Spec>;
#[doc = "Field `SMPWHV0` reader - SMP WHV Parameter Set 0"]
pub type Smpwhv0R = crate::FieldReader<u32>;
#[doc = "Field `SMPWHV0` writer - SMP WHV Parameter Set 0"]
pub type Smpwhv0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SMP WHV Parameter Set 0"]
    #[inline(always)]
    pub fn smpwhv0(&self) -> Smpwhv0R {
        Smpwhv0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SMP WHV Parameter Set 0"]
    #[inline(always)]
    pub fn smpwhv0(&mut self) -> Smpwhv0W<'_, RSmpWhv0Spec> {
        Smpwhv0W::new(self, 0)
    }
}
#[doc = "BIST SMP WHV Setting 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r_smp_whv0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r_smp_whv0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RSmpWhv0Spec;
impl crate::RegisterSpec for RSmpWhv0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r_smp_whv0::R`](R) reader structure"]
impl crate::Readable for RSmpWhv0Spec {}
#[doc = "`write(|w| ..)` method takes [`r_smp_whv0::W`](W) writer structure"]
impl crate::Writable for RSmpWhv0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R_SMP_WHV0 to value 0x7777_7765"]
impl crate::Resettable for RSmpWhv0Spec {
    const RESET_VALUE: u32 = 0x7777_7765;
}
