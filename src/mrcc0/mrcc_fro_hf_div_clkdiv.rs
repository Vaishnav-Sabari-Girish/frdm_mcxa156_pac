#[doc = "Register `MRCC_FRO_HF_DIV_CLKDIV` reader"]
pub type R = crate::R<MrccFroHfDivClkdivSpec>;
#[doc = "Register `MRCC_FRO_HF_DIV_CLKDIV` writer"]
pub type W = crate::W<MrccFroHfDivClkdivSpec>;
#[doc = "Field `DIV` reader - Functional Clock Divider"]
pub type DivR = crate::FieldReader;
#[doc = "Field `DIV` writer - Functional Clock Divider"]
pub type DivW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Divider status flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unstab {
    #[doc = "0: Divider clock is stable"]
    On = 0,
    #[doc = "1: Clock frequency isn't stable"]
    Off = 1,
}
impl From<Unstab> for bool {
    #[inline(always)]
    fn from(variant: Unstab) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UNSTAB` reader - Divider status flag"]
pub type UnstabR = crate::BitReader<Unstab>;
impl UnstabR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Unstab {
        match self.bits {
            false => Unstab::On,
            true => Unstab::Off,
        }
    }
    #[doc = "Divider clock is stable"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Unstab::On
    }
    #[doc = "Clock frequency isn't stable"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Unstab::Off
    }
}
impl R {
    #[doc = "Bits 0:3 - Functional Clock Divider"]
    #[inline(always)]
    pub fn div(&self) -> DivR {
        DivR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 31 - Divider status flag"]
    #[inline(always)]
    pub fn unstab(&self) -> UnstabR {
        UnstabR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Functional Clock Divider"]
    #[inline(always)]
    pub fn div(&mut self) -> DivW<'_, MrccFroHfDivClkdivSpec> {
        DivW::new(self, 0)
    }
}
#[doc = "FRO_HF_DIV clock divider control\n\nYou can [`read`](crate::Reg::read) this register and get [`mrcc_fro_hf_div_clkdiv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mrcc_fro_hf_div_clkdiv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MrccFroHfDivClkdivSpec;
impl crate::RegisterSpec for MrccFroHfDivClkdivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mrcc_fro_hf_div_clkdiv::R`](R) reader structure"]
impl crate::Readable for MrccFroHfDivClkdivSpec {}
#[doc = "`write(|w| ..)` method takes [`mrcc_fro_hf_div_clkdiv::W`](W) writer structure"]
impl crate::Writable for MrccFroHfDivClkdivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MRCC_FRO_HF_DIV_CLKDIV to value 0"]
impl crate::Resettable for MrccFroHfDivClkdivSpec {}
