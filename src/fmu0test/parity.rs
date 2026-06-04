#[doc = "Register `PARITY` reader"]
pub type R = crate::R<ParitySpec>;
#[doc = "Register `PARITY` writer"]
pub type W = crate::W<ParitySpec>;
#[doc = "Field `PARITY` reader - Read data \\[136:128\\]"]
pub type ParityR = crate::FieldReader<u16>;
#[doc = "Field `PARITY` writer - Read data \\[136:128\\]"]
pub type ParityW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - Read data \\[136:128\\]"]
    #[inline(always)]
    pub fn parity(&self) -> ParityR {
        ParityR::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Read data \\[136:128\\]"]
    #[inline(always)]
    pub fn parity(&mut self) -> ParityW<'_, ParitySpec> {
        ParityW::new(self, 0)
    }
}
#[doc = "Parity Register\n\nYou can [`read`](crate::Reg::read) this register and get [`parity::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`parity::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ParitySpec;
impl crate::RegisterSpec for ParitySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`parity::R`](R) reader structure"]
impl crate::Readable for ParitySpec {}
#[doc = "`write(|w| ..)` method takes [`parity::W`](W) writer structure"]
impl crate::Writable for ParitySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PARITY to value 0"]
impl crate::Resettable for ParitySpec {}
