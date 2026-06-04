#[doc = "Register `ATX_PIN_CTRL` reader"]
pub type R = crate::R<AtxPinCtrlSpec>;
#[doc = "Register `ATX_PIN_CTRL` writer"]
pub type W = crate::W<AtxPinCtrlSpec>;
#[doc = "TM to ATX\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TmToAtx {
    #[doc = "1: TM\\[0\\] to ATX0"]
    Zz441 = 1,
    #[doc = "2: TM\\[1\\] to ATX0"]
    Zz442 = 2,
    #[doc = "4: TM\\[2\\] to ATX0"]
    Zz443 = 4,
    #[doc = "8: TM\\[3\\] to ATX0"]
    Zz444 = 8,
    #[doc = "16: TM\\[0\\] to ATX1"]
    Zz445 = 16,
    #[doc = "32: TM\\[1\\] to ATX1"]
    Zz446 = 32,
    #[doc = "64: TM\\[2\\] to ATX1"]
    Zz447 = 64,
    #[doc = "128: TM\\[3\\] to ATX1"]
    Zz448 = 128,
}
impl From<TmToAtx> for u8 {
    #[inline(always)]
    fn from(variant: TmToAtx) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TmToAtx {
    type Ux = u8;
}
impl crate::IsEnum for TmToAtx {}
#[doc = "Field `TM_TO_ATX` reader - TM to ATX"]
pub type TmToAtxR = crate::FieldReader<TmToAtx>;
impl TmToAtxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TmToAtx> {
        match self.bits {
            1 => Some(TmToAtx::Zz441),
            2 => Some(TmToAtx::Zz442),
            4 => Some(TmToAtx::Zz443),
            8 => Some(TmToAtx::Zz444),
            16 => Some(TmToAtx::Zz445),
            32 => Some(TmToAtx::Zz446),
            64 => Some(TmToAtx::Zz447),
            128 => Some(TmToAtx::Zz448),
            _ => None,
        }
    }
    #[doc = "TM\\[0\\] to ATX0"]
    #[inline(always)]
    pub fn is_zz441(&self) -> bool {
        *self == TmToAtx::Zz441
    }
    #[doc = "TM\\[1\\] to ATX0"]
    #[inline(always)]
    pub fn is_zz442(&self) -> bool {
        *self == TmToAtx::Zz442
    }
    #[doc = "TM\\[2\\] to ATX0"]
    #[inline(always)]
    pub fn is_zz443(&self) -> bool {
        *self == TmToAtx::Zz443
    }
    #[doc = "TM\\[3\\] to ATX0"]
    #[inline(always)]
    pub fn is_zz444(&self) -> bool {
        *self == TmToAtx::Zz444
    }
    #[doc = "TM\\[0\\] to ATX1"]
    #[inline(always)]
    pub fn is_zz445(&self) -> bool {
        *self == TmToAtx::Zz445
    }
    #[doc = "TM\\[1\\] to ATX1"]
    #[inline(always)]
    pub fn is_zz446(&self) -> bool {
        *self == TmToAtx::Zz446
    }
    #[doc = "TM\\[2\\] to ATX1"]
    #[inline(always)]
    pub fn is_zz447(&self) -> bool {
        *self == TmToAtx::Zz447
    }
    #[doc = "TM\\[3\\] to ATX1"]
    #[inline(always)]
    pub fn is_zz448(&self) -> bool {
        *self == TmToAtx::Zz448
    }
}
#[doc = "Field `TM_TO_ATX` writer - TM to ATX"]
pub type TmToAtxW<'a, REG> = crate::FieldWriter<'a, REG, 8, TmToAtx>;
impl<'a, REG> TmToAtxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TM\\[0\\] to ATX0"]
    #[inline(always)]
    pub fn zz441(self) -> &'a mut crate::W<REG> {
        self.variant(TmToAtx::Zz441)
    }
    #[doc = "TM\\[1\\] to ATX0"]
    #[inline(always)]
    pub fn zz442(self) -> &'a mut crate::W<REG> {
        self.variant(TmToAtx::Zz442)
    }
    #[doc = "TM\\[2\\] to ATX0"]
    #[inline(always)]
    pub fn zz443(self) -> &'a mut crate::W<REG> {
        self.variant(TmToAtx::Zz443)
    }
    #[doc = "TM\\[3\\] to ATX0"]
    #[inline(always)]
    pub fn zz444(self) -> &'a mut crate::W<REG> {
        self.variant(TmToAtx::Zz444)
    }
    #[doc = "TM\\[0\\] to ATX1"]
    #[inline(always)]
    pub fn zz445(self) -> &'a mut crate::W<REG> {
        self.variant(TmToAtx::Zz445)
    }
    #[doc = "TM\\[1\\] to ATX1"]
    #[inline(always)]
    pub fn zz446(self) -> &'a mut crate::W<REG> {
        self.variant(TmToAtx::Zz446)
    }
    #[doc = "TM\\[2\\] to ATX1"]
    #[inline(always)]
    pub fn zz447(self) -> &'a mut crate::W<REG> {
        self.variant(TmToAtx::Zz447)
    }
    #[doc = "TM\\[3\\] to ATX1"]
    #[inline(always)]
    pub fn zz448(self) -> &'a mut crate::W<REG> {
        self.variant(TmToAtx::Zz448)
    }
}
impl R {
    #[doc = "Bits 0:7 - TM to ATX"]
    #[inline(always)]
    pub fn tm_to_atx(&self) -> TmToAtxR {
        TmToAtxR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - TM to ATX"]
    #[inline(always)]
    pub fn tm_to_atx(&mut self) -> TmToAtxW<'_, AtxPinCtrlSpec> {
        TmToAtxW::new(self, 0)
    }
}
#[doc = "ATX Pin Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`atx_pin_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`atx_pin_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AtxPinCtrlSpec;
impl crate::RegisterSpec for AtxPinCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`atx_pin_ctrl::R`](R) reader structure"]
impl crate::Readable for AtxPinCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`atx_pin_ctrl::W`](W) writer structure"]
impl crate::Writable for AtxPinCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ATX_PIN_CTRL to value 0"]
impl crate::Resettable for AtxPinCtrlSpec {}
