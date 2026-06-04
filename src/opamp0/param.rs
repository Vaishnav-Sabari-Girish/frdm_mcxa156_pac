#[doc = "Register `PARAM` reader"]
pub type R = crate::R<ParamSpec>;
#[doc = "PGA Function Option\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PgaFunction {
    #[doc = "0: Core amplifier enabled"]
    CoreAmp = 0,
    #[doc = "1: PGA function enabled"]
    Pga = 1,
}
impl From<PgaFunction> for bool {
    #[inline(always)]
    fn from(variant: PgaFunction) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PGA_FUNCTION` reader - PGA Function Option"]
pub type PgaFunctionR = crate::BitReader<PgaFunction>;
impl PgaFunctionR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PgaFunction {
        match self.bits {
            false => PgaFunction::CoreAmp,
            true => PgaFunction::Pga,
        }
    }
    #[doc = "Core amplifier enabled"]
    #[inline(always)]
    pub fn is_core_amp(&self) -> bool {
        *self == PgaFunction::CoreAmp
    }
    #[doc = "PGA function enabled"]
    #[inline(always)]
    pub fn is_pga(&self) -> bool {
        *self == PgaFunction::Pga
    }
}
impl R {
    #[doc = "Bit 0 - PGA Function Option"]
    #[inline(always)]
    pub fn pga_function(&self) -> PgaFunctionR {
        PgaFunctionR::new((self.bits & 1) != 0)
    }
}
#[doc = "Parameter\n\nYou can [`read`](crate::Reg::read) this register and get [`param::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ParamSpec;
impl crate::RegisterSpec for ParamSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`param::R`](R) reader structure"]
impl crate::Readable for ParamSpec {}
#[doc = "`reset()` method sets PARAM to value 0x01"]
impl crate::Resettable for ParamSpec {
    const RESET_VALUE: u32 = 0x01;
}
