#[doc = "Register `PDC1` reader"]
pub type R = crate::R<Pdc1Spec>;
#[doc = "Register `PDC1` writer"]
pub type W = crate::W<Pdc1Spec>;
#[doc = "Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Reserved0 {
    #[doc = "0: Not supported"]
    Interrupt = 0,
    #[doc = "1: Not supported"]
    DmaReq = 1,
    #[doc = "2: Not supported"]
    Trigger = 2,
    #[doc = "3: Not supported"]
    Res = 3,
}
impl From<Reserved0> for u8 {
    #[inline(always)]
    fn from(variant: Reserved0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Reserved0 {
    type Ux = u8;
}
impl crate::IsEnum for Reserved0 {}
#[doc = "Field `Reserved0` reader - Reserved"]
pub type Reserved0R = crate::FieldReader<Reserved0>;
impl Reserved0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Reserved0 {
        match self.bits {
            0 => Reserved0::Interrupt,
            1 => Reserved0::DmaReq,
            2 => Reserved0::Trigger,
            3 => Reserved0::Res,
            _ => unreachable!(),
        }
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == Reserved0::Interrupt
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_dma_req(&self) -> bool {
        *self == Reserved0::DmaReq
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == Reserved0::Trigger
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_res(&self) -> bool {
        *self == Reserved0::Res
    }
}
#[doc = "Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Reserved1 {
    #[doc = "0: Not supported"]
    Interrupt = 0,
    #[doc = "1: Not supported"]
    DmaReq = 1,
    #[doc = "2: Not supported"]
    Trigger = 2,
    #[doc = "3: Not supported"]
    Res = 3,
}
impl From<Reserved1> for u8 {
    #[inline(always)]
    fn from(variant: Reserved1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Reserved1 {
    type Ux = u8;
}
impl crate::IsEnum for Reserved1 {}
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader<Reserved1>;
impl Reserved1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Reserved1 {
        match self.bits {
            0 => Reserved1::Interrupt,
            1 => Reserved1::DmaReq,
            2 => Reserved1::Trigger,
            3 => Reserved1::Res,
            _ => unreachable!(),
        }
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == Reserved1::Interrupt
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_dma_req(&self) -> bool {
        *self == Reserved1::DmaReq
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == Reserved1::Trigger
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_res(&self) -> bool {
        *self == Reserved1::Res
    }
}
#[doc = "Wake-up Pin Configuration for WUU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wupdc2 {
    #[doc = "0: Interrupt"]
    Interrupt = 0,
    #[doc = "1: DMA request"]
    DmaReq = 1,
    #[doc = "2: Trigger event"]
    Trigger = 2,
    #[doc = "3: Reserved"]
    Res = 3,
}
impl From<Wupdc2> for u8 {
    #[inline(always)]
    fn from(variant: Wupdc2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wupdc2 {
    type Ux = u8;
}
impl crate::IsEnum for Wupdc2 {}
#[doc = "Field `WUPDC2` reader - Wake-up Pin Configuration for WUU_Pn"]
pub type Wupdc2R = crate::FieldReader<Wupdc2>;
impl Wupdc2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wupdc2 {
        match self.bits {
            0 => Wupdc2::Interrupt,
            1 => Wupdc2::DmaReq,
            2 => Wupdc2::Trigger,
            3 => Wupdc2::Res,
            _ => unreachable!(),
        }
    }
    #[doc = "Interrupt"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == Wupdc2::Interrupt
    }
    #[doc = "DMA request"]
    #[inline(always)]
    pub fn is_dma_req(&self) -> bool {
        *self == Wupdc2::DmaReq
    }
    #[doc = "Trigger event"]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == Wupdc2::Trigger
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_res(&self) -> bool {
        *self == Wupdc2::Res
    }
}
#[doc = "Field `WUPDC2` writer - Wake-up Pin Configuration for WUU_Pn"]
pub type Wupdc2W<'a, REG> = crate::FieldWriter<'a, REG, 2, Wupdc2, crate::Safe>;
impl<'a, REG> Wupdc2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupt"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut crate::W<REG> {
        self.variant(Wupdc2::Interrupt)
    }
    #[doc = "DMA request"]
    #[inline(always)]
    pub fn dma_req(self) -> &'a mut crate::W<REG> {
        self.variant(Wupdc2::DmaReq)
    }
    #[doc = "Trigger event"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Wupdc2::Trigger)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn res(self) -> &'a mut crate::W<REG> {
        self.variant(Wupdc2::Res)
    }
}
#[doc = "Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Reserved3 {
    #[doc = "0: Not supported"]
    Interrupt = 0,
    #[doc = "1: Not supported"]
    DmaReq = 1,
    #[doc = "2: Not supported"]
    Trigger = 2,
    #[doc = "3: Not supported"]
    Res = 3,
}
impl From<Reserved3> for u8 {
    #[inline(always)]
    fn from(variant: Reserved3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Reserved3 {
    type Ux = u8;
}
impl crate::IsEnum for Reserved3 {}
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::FieldReader<Reserved3>;
impl Reserved3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Reserved3 {
        match self.bits {
            0 => Reserved3::Interrupt,
            1 => Reserved3::DmaReq,
            2 => Reserved3::Trigger,
            3 => Reserved3::Res,
            _ => unreachable!(),
        }
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == Reserved3::Interrupt
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_dma_req(&self) -> bool {
        *self == Reserved3::DmaReq
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == Reserved3::Trigger
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_res(&self) -> bool {
        *self == Reserved3::Res
    }
}
#[doc = "Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Reserved4 {
    #[doc = "0: Not supported"]
    Interrupt = 0,
    #[doc = "1: Not supported"]
    DmaReq = 1,
    #[doc = "2: Not supported"]
    Trigger = 2,
    #[doc = "3: Not supported"]
    Res = 3,
}
impl From<Reserved4> for u8 {
    #[inline(always)]
    fn from(variant: Reserved4) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Reserved4 {
    type Ux = u8;
}
impl crate::IsEnum for Reserved4 {}
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::FieldReader<Reserved4>;
impl Reserved4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Reserved4 {
        match self.bits {
            0 => Reserved4::Interrupt,
            1 => Reserved4::DmaReq,
            2 => Reserved4::Trigger,
            3 => Reserved4::Res,
            _ => unreachable!(),
        }
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == Reserved4::Interrupt
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_dma_req(&self) -> bool {
        *self == Reserved4::DmaReq
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == Reserved4::Trigger
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_res(&self) -> bool {
        *self == Reserved4::Res
    }
}
#[doc = "Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Reserved5 {
    #[doc = "0: Not supported"]
    Interrupt = 0,
    #[doc = "1: Not supported"]
    DmaReq = 1,
    #[doc = "2: Not supported"]
    Trigger = 2,
    #[doc = "3: Not supported"]
    Res = 3,
}
impl From<Reserved5> for u8 {
    #[inline(always)]
    fn from(variant: Reserved5) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Reserved5 {
    type Ux = u8;
}
impl crate::IsEnum for Reserved5 {}
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::FieldReader<Reserved5>;
impl Reserved5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Reserved5 {
        match self.bits {
            0 => Reserved5::Interrupt,
            1 => Reserved5::DmaReq,
            2 => Reserved5::Trigger,
            3 => Reserved5::Res,
            _ => unreachable!(),
        }
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == Reserved5::Interrupt
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_dma_req(&self) -> bool {
        *self == Reserved5::DmaReq
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == Reserved5::Trigger
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_res(&self) -> bool {
        *self == Reserved5::Res
    }
}
#[doc = "Wake-up Pin Configuration for WUU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wupdc6 {
    #[doc = "0: Interrupt"]
    Interrupt = 0,
    #[doc = "1: DMA request"]
    DmaReq = 1,
    #[doc = "2: Trigger event"]
    Trigger = 2,
    #[doc = "3: Reserved"]
    Res = 3,
}
impl From<Wupdc6> for u8 {
    #[inline(always)]
    fn from(variant: Wupdc6) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wupdc6 {
    type Ux = u8;
}
impl crate::IsEnum for Wupdc6 {}
#[doc = "Field `WUPDC6` reader - Wake-up Pin Configuration for WUU_Pn"]
pub type Wupdc6R = crate::FieldReader<Wupdc6>;
impl Wupdc6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wupdc6 {
        match self.bits {
            0 => Wupdc6::Interrupt,
            1 => Wupdc6::DmaReq,
            2 => Wupdc6::Trigger,
            3 => Wupdc6::Res,
            _ => unreachable!(),
        }
    }
    #[doc = "Interrupt"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == Wupdc6::Interrupt
    }
    #[doc = "DMA request"]
    #[inline(always)]
    pub fn is_dma_req(&self) -> bool {
        *self == Wupdc6::DmaReq
    }
    #[doc = "Trigger event"]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == Wupdc6::Trigger
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_res(&self) -> bool {
        *self == Wupdc6::Res
    }
}
#[doc = "Field `WUPDC6` writer - Wake-up Pin Configuration for WUU_Pn"]
pub type Wupdc6W<'a, REG> = crate::FieldWriter<'a, REG, 2, Wupdc6, crate::Safe>;
impl<'a, REG> Wupdc6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupt"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut crate::W<REG> {
        self.variant(Wupdc6::Interrupt)
    }
    #[doc = "DMA request"]
    #[inline(always)]
    pub fn dma_req(self) -> &'a mut crate::W<REG> {
        self.variant(Wupdc6::DmaReq)
    }
    #[doc = "Trigger event"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Wupdc6::Trigger)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn res(self) -> &'a mut crate::W<REG> {
        self.variant(Wupdc6::Res)
    }
}
#[doc = "Wake-up Pin Configuration for WUU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wupdc7 {
    #[doc = "0: Interrupt"]
    Interrupt = 0,
    #[doc = "1: DMA request"]
    DmaReq = 1,
    #[doc = "2: Trigger event"]
    Trigger = 2,
    #[doc = "3: Reserved"]
    Res = 3,
}
impl From<Wupdc7> for u8 {
    #[inline(always)]
    fn from(variant: Wupdc7) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wupdc7 {
    type Ux = u8;
}
impl crate::IsEnum for Wupdc7 {}
#[doc = "Field `WUPDC7` reader - Wake-up Pin Configuration for WUU_Pn"]
pub type Wupdc7R = crate::FieldReader<Wupdc7>;
impl Wupdc7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wupdc7 {
        match self.bits {
            0 => Wupdc7::Interrupt,
            1 => Wupdc7::DmaReq,
            2 => Wupdc7::Trigger,
            3 => Wupdc7::Res,
            _ => unreachable!(),
        }
    }
    #[doc = "Interrupt"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == Wupdc7::Interrupt
    }
    #[doc = "DMA request"]
    #[inline(always)]
    pub fn is_dma_req(&self) -> bool {
        *self == Wupdc7::DmaReq
    }
    #[doc = "Trigger event"]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == Wupdc7::Trigger
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_res(&self) -> bool {
        *self == Wupdc7::Res
    }
}
#[doc = "Field `WUPDC7` writer - Wake-up Pin Configuration for WUU_Pn"]
pub type Wupdc7W<'a, REG> = crate::FieldWriter<'a, REG, 2, Wupdc7, crate::Safe>;
impl<'a, REG> Wupdc7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupt"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut crate::W<REG> {
        self.variant(Wupdc7::Interrupt)
    }
    #[doc = "DMA request"]
    #[inline(always)]
    pub fn dma_req(self) -> &'a mut crate::W<REG> {
        self.variant(Wupdc7::DmaReq)
    }
    #[doc = "Trigger event"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Wupdc7::Trigger)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn res(self) -> &'a mut crate::W<REG> {
        self.variant(Wupdc7::Res)
    }
}
#[doc = "Wake-up Pin Configuration for WUU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wupdc8 {
    #[doc = "0: Interrupt"]
    Interrupt = 0,
    #[doc = "1: DMA request"]
    DmaReq = 1,
    #[doc = "2: Trigger event"]
    Trigger = 2,
    #[doc = "3: Reserved"]
    Res = 3,
}
impl From<Wupdc8> for u8 {
    #[inline(always)]
    fn from(variant: Wupdc8) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wupdc8 {
    type Ux = u8;
}
impl crate::IsEnum for Wupdc8 {}
#[doc = "Field `WUPDC8` reader - Wake-up Pin Configuration for WUU_Pn"]
pub type Wupdc8R = crate::FieldReader<Wupdc8>;
impl Wupdc8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wupdc8 {
        match self.bits {
            0 => Wupdc8::Interrupt,
            1 => Wupdc8::DmaReq,
            2 => Wupdc8::Trigger,
            3 => Wupdc8::Res,
            _ => unreachable!(),
        }
    }
    #[doc = "Interrupt"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == Wupdc8::Interrupt
    }
    #[doc = "DMA request"]
    #[inline(always)]
    pub fn is_dma_req(&self) -> bool {
        *self == Wupdc8::DmaReq
    }
    #[doc = "Trigger event"]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == Wupdc8::Trigger
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_res(&self) -> bool {
        *self == Wupdc8::Res
    }
}
#[doc = "Field `WUPDC8` writer - Wake-up Pin Configuration for WUU_Pn"]
pub type Wupdc8W<'a, REG> = crate::FieldWriter<'a, REG, 2, Wupdc8, crate::Safe>;
impl<'a, REG> Wupdc8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupt"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut crate::W<REG> {
        self.variant(Wupdc8::Interrupt)
    }
    #[doc = "DMA request"]
    #[inline(always)]
    pub fn dma_req(self) -> &'a mut crate::W<REG> {
        self.variant(Wupdc8::DmaReq)
    }
    #[doc = "Trigger event"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Wupdc8::Trigger)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn res(self) -> &'a mut crate::W<REG> {
        self.variant(Wupdc8::Res)
    }
}
#[doc = "Wake-up Pin Configuration for WUU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wupdc9 {
    #[doc = "0: Interrupt"]
    Interrupt = 0,
    #[doc = "1: DMA request"]
    DmaReq = 1,
    #[doc = "2: Trigger event"]
    Trigger = 2,
    #[doc = "3: Reserved"]
    Res = 3,
}
impl From<Wupdc9> for u8 {
    #[inline(always)]
    fn from(variant: Wupdc9) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wupdc9 {
    type Ux = u8;
}
impl crate::IsEnum for Wupdc9 {}
#[doc = "Field `WUPDC9` reader - Wake-up Pin Configuration for WUU_Pn"]
pub type Wupdc9R = crate::FieldReader<Wupdc9>;
impl Wupdc9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wupdc9 {
        match self.bits {
            0 => Wupdc9::Interrupt,
            1 => Wupdc9::DmaReq,
            2 => Wupdc9::Trigger,
            3 => Wupdc9::Res,
            _ => unreachable!(),
        }
    }
    #[doc = "Interrupt"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == Wupdc9::Interrupt
    }
    #[doc = "DMA request"]
    #[inline(always)]
    pub fn is_dma_req(&self) -> bool {
        *self == Wupdc9::DmaReq
    }
    #[doc = "Trigger event"]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == Wupdc9::Trigger
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_res(&self) -> bool {
        *self == Wupdc9::Res
    }
}
#[doc = "Field `WUPDC9` writer - Wake-up Pin Configuration for WUU_Pn"]
pub type Wupdc9W<'a, REG> = crate::FieldWriter<'a, REG, 2, Wupdc9, crate::Safe>;
impl<'a, REG> Wupdc9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupt"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut crate::W<REG> {
        self.variant(Wupdc9::Interrupt)
    }
    #[doc = "DMA request"]
    #[inline(always)]
    pub fn dma_req(self) -> &'a mut crate::W<REG> {
        self.variant(Wupdc9::DmaReq)
    }
    #[doc = "Trigger event"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Wupdc9::Trigger)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn res(self) -> &'a mut crate::W<REG> {
        self.variant(Wupdc9::Res)
    }
}
#[doc = "Wake-up Pin Configuration for WUU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wupdc10 {
    #[doc = "0: Interrupt"]
    Interrupt = 0,
    #[doc = "1: DMA request"]
    DmaReq = 1,
    #[doc = "2: Trigger event"]
    Trigger = 2,
    #[doc = "3: Reserved"]
    Res = 3,
}
impl From<Wupdc10> for u8 {
    #[inline(always)]
    fn from(variant: Wupdc10) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wupdc10 {
    type Ux = u8;
}
impl crate::IsEnum for Wupdc10 {}
#[doc = "Field `WUPDC10` reader - Wake-up Pin Configuration for WUU_Pn"]
pub type Wupdc10R = crate::FieldReader<Wupdc10>;
impl Wupdc10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wupdc10 {
        match self.bits {
            0 => Wupdc10::Interrupt,
            1 => Wupdc10::DmaReq,
            2 => Wupdc10::Trigger,
            3 => Wupdc10::Res,
            _ => unreachable!(),
        }
    }
    #[doc = "Interrupt"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == Wupdc10::Interrupt
    }
    #[doc = "DMA request"]
    #[inline(always)]
    pub fn is_dma_req(&self) -> bool {
        *self == Wupdc10::DmaReq
    }
    #[doc = "Trigger event"]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == Wupdc10::Trigger
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_res(&self) -> bool {
        *self == Wupdc10::Res
    }
}
#[doc = "Field `WUPDC10` writer - Wake-up Pin Configuration for WUU_Pn"]
pub type Wupdc10W<'a, REG> = crate::FieldWriter<'a, REG, 2, Wupdc10, crate::Safe>;
impl<'a, REG> Wupdc10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupt"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut crate::W<REG> {
        self.variant(Wupdc10::Interrupt)
    }
    #[doc = "DMA request"]
    #[inline(always)]
    pub fn dma_req(self) -> &'a mut crate::W<REG> {
        self.variant(Wupdc10::DmaReq)
    }
    #[doc = "Trigger event"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Wupdc10::Trigger)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn res(self) -> &'a mut crate::W<REG> {
        self.variant(Wupdc10::Res)
    }
}
#[doc = "Wake-up Pin Configuration for WUU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wupdc11 {
    #[doc = "0: Interrupt"]
    Interrupt = 0,
    #[doc = "1: DMA request"]
    DmaReq = 1,
    #[doc = "2: Trigger event"]
    Trigger = 2,
    #[doc = "3: Reserved"]
    Res = 3,
}
impl From<Wupdc11> for u8 {
    #[inline(always)]
    fn from(variant: Wupdc11) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wupdc11 {
    type Ux = u8;
}
impl crate::IsEnum for Wupdc11 {}
#[doc = "Field `WUPDC11` reader - Wake-up Pin Configuration for WUU_Pn"]
pub type Wupdc11R = crate::FieldReader<Wupdc11>;
impl Wupdc11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wupdc11 {
        match self.bits {
            0 => Wupdc11::Interrupt,
            1 => Wupdc11::DmaReq,
            2 => Wupdc11::Trigger,
            3 => Wupdc11::Res,
            _ => unreachable!(),
        }
    }
    #[doc = "Interrupt"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == Wupdc11::Interrupt
    }
    #[doc = "DMA request"]
    #[inline(always)]
    pub fn is_dma_req(&self) -> bool {
        *self == Wupdc11::DmaReq
    }
    #[doc = "Trigger event"]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == Wupdc11::Trigger
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_res(&self) -> bool {
        *self == Wupdc11::Res
    }
}
#[doc = "Field `WUPDC11` writer - Wake-up Pin Configuration for WUU_Pn"]
pub type Wupdc11W<'a, REG> = crate::FieldWriter<'a, REG, 2, Wupdc11, crate::Safe>;
impl<'a, REG> Wupdc11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupt"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut crate::W<REG> {
        self.variant(Wupdc11::Interrupt)
    }
    #[doc = "DMA request"]
    #[inline(always)]
    pub fn dma_req(self) -> &'a mut crate::W<REG> {
        self.variant(Wupdc11::DmaReq)
    }
    #[doc = "Trigger event"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Wupdc11::Trigger)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn res(self) -> &'a mut crate::W<REG> {
        self.variant(Wupdc11::Res)
    }
}
#[doc = "Wake-up Pin Configuration for WUU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wupdc12 {
    #[doc = "0: Interrupt"]
    Interrupt = 0,
    #[doc = "1: DMA request"]
    DmaReq = 1,
    #[doc = "2: Trigger event"]
    Trigger = 2,
    #[doc = "3: Reserved"]
    Res = 3,
}
impl From<Wupdc12> for u8 {
    #[inline(always)]
    fn from(variant: Wupdc12) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wupdc12 {
    type Ux = u8;
}
impl crate::IsEnum for Wupdc12 {}
#[doc = "Field `WUPDC12` reader - Wake-up Pin Configuration for WUU_Pn"]
pub type Wupdc12R = crate::FieldReader<Wupdc12>;
impl Wupdc12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wupdc12 {
        match self.bits {
            0 => Wupdc12::Interrupt,
            1 => Wupdc12::DmaReq,
            2 => Wupdc12::Trigger,
            3 => Wupdc12::Res,
            _ => unreachable!(),
        }
    }
    #[doc = "Interrupt"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == Wupdc12::Interrupt
    }
    #[doc = "DMA request"]
    #[inline(always)]
    pub fn is_dma_req(&self) -> bool {
        *self == Wupdc12::DmaReq
    }
    #[doc = "Trigger event"]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == Wupdc12::Trigger
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_res(&self) -> bool {
        *self == Wupdc12::Res
    }
}
#[doc = "Field `WUPDC12` writer - Wake-up Pin Configuration for WUU_Pn"]
pub type Wupdc12W<'a, REG> = crate::FieldWriter<'a, REG, 2, Wupdc12, crate::Safe>;
impl<'a, REG> Wupdc12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupt"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut crate::W<REG> {
        self.variant(Wupdc12::Interrupt)
    }
    #[doc = "DMA request"]
    #[inline(always)]
    pub fn dma_req(self) -> &'a mut crate::W<REG> {
        self.variant(Wupdc12::DmaReq)
    }
    #[doc = "Trigger event"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Wupdc12::Trigger)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn res(self) -> &'a mut crate::W<REG> {
        self.variant(Wupdc12::Res)
    }
}
#[doc = "Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Reserved13 {
    #[doc = "0: Not supported"]
    Interrupt = 0,
    #[doc = "1: Not supported"]
    DmaReq = 1,
    #[doc = "2: Not supported"]
    Trigger = 2,
    #[doc = "3: Not supported"]
    Res = 3,
}
impl From<Reserved13> for u8 {
    #[inline(always)]
    fn from(variant: Reserved13) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Reserved13 {
    type Ux = u8;
}
impl crate::IsEnum for Reserved13 {}
#[doc = "Field `Reserved13` reader - Reserved"]
pub type Reserved13R = crate::FieldReader<Reserved13>;
impl Reserved13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Reserved13 {
        match self.bits {
            0 => Reserved13::Interrupt,
            1 => Reserved13::DmaReq,
            2 => Reserved13::Trigger,
            3 => Reserved13::Res,
            _ => unreachable!(),
        }
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == Reserved13::Interrupt
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_dma_req(&self) -> bool {
        *self == Reserved13::DmaReq
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == Reserved13::Trigger
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_res(&self) -> bool {
        *self == Reserved13::Res
    }
}
#[doc = "Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Reserved14 {
    #[doc = "0: Not supported"]
    Interrupt = 0,
    #[doc = "1: Not supported"]
    DmaReq = 1,
    #[doc = "2: Not supported"]
    Trigger = 2,
    #[doc = "3: Not supported"]
    Res = 3,
}
impl From<Reserved14> for u8 {
    #[inline(always)]
    fn from(variant: Reserved14) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Reserved14 {
    type Ux = u8;
}
impl crate::IsEnum for Reserved14 {}
#[doc = "Field `Reserved14` reader - Reserved"]
pub type Reserved14R = crate::FieldReader<Reserved14>;
impl Reserved14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Reserved14 {
        match self.bits {
            0 => Reserved14::Interrupt,
            1 => Reserved14::DmaReq,
            2 => Reserved14::Trigger,
            3 => Reserved14::Res,
            _ => unreachable!(),
        }
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == Reserved14::Interrupt
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_dma_req(&self) -> bool {
        *self == Reserved14::DmaReq
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == Reserved14::Trigger
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_res(&self) -> bool {
        *self == Reserved14::Res
    }
}
#[doc = "Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Reserved15 {
    #[doc = "0: Not supported"]
    Interrupt = 0,
    #[doc = "1: Not supported"]
    DmaReq = 1,
    #[doc = "2: Not supported"]
    Trigger = 2,
    #[doc = "3: Not supported"]
    Res = 3,
}
impl From<Reserved15> for u8 {
    #[inline(always)]
    fn from(variant: Reserved15) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Reserved15 {
    type Ux = u8;
}
impl crate::IsEnum for Reserved15 {}
#[doc = "Field `Reserved15` reader - Reserved"]
pub type Reserved15R = crate::FieldReader<Reserved15>;
impl Reserved15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Reserved15 {
        match self.bits {
            0 => Reserved15::Interrupt,
            1 => Reserved15::DmaReq,
            2 => Reserved15::Trigger,
            3 => Reserved15::Res,
            _ => unreachable!(),
        }
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == Reserved15::Interrupt
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_dma_req(&self) -> bool {
        *self == Reserved15::DmaReq
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == Reserved15::Trigger
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn is_res(&self) -> bool {
        *self == Reserved15::Res
    }
}
impl R {
    #[doc = "Bits 0:1 - Reserved"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Wake-up Pin Configuration for WUU_Pn"]
    #[inline(always)]
    pub fn wupdc2(&self) -> Wupdc2R {
        Wupdc2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Wake-up Pin Configuration for WUU_Pn"]
    #[inline(always)]
    pub fn wupdc6(&self) -> Wupdc6R {
        Wupdc6R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Wake-up Pin Configuration for WUU_Pn"]
    #[inline(always)]
    pub fn wupdc7(&self) -> Wupdc7R {
        Wupdc7R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Wake-up Pin Configuration for WUU_Pn"]
    #[inline(always)]
    pub fn wupdc8(&self) -> Wupdc8R {
        Wupdc8R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Wake-up Pin Configuration for WUU_Pn"]
    #[inline(always)]
    pub fn wupdc9(&self) -> Wupdc9R {
        Wupdc9R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Wake-up Pin Configuration for WUU_Pn"]
    #[inline(always)]
    pub fn wupdc10(&self) -> Wupdc10R {
        Wupdc10R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Wake-up Pin Configuration for WUU_Pn"]
    #[inline(always)]
    pub fn wupdc11(&self) -> Wupdc11R {
        Wupdc11R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Wake-up Pin Configuration for WUU_Pn"]
    #[inline(always)]
    pub fn wupdc12(&self) -> Wupdc12R {
        Wupdc12R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Reserved"]
    #[inline(always)]
    pub fn reserved13(&self) -> Reserved13R {
        Reserved13R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Reserved"]
    #[inline(always)]
    pub fn reserved14(&self) -> Reserved14R {
        Reserved14R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Reserved"]
    #[inline(always)]
    pub fn reserved15(&self) -> Reserved15R {
        Reserved15R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 4:5 - Wake-up Pin Configuration for WUU_Pn"]
    #[inline(always)]
    pub fn wupdc2(&mut self) -> Wupdc2W<'_, Pdc1Spec> {
        Wupdc2W::new(self, 4)
    }
    #[doc = "Bits 12:13 - Wake-up Pin Configuration for WUU_Pn"]
    #[inline(always)]
    pub fn wupdc6(&mut self) -> Wupdc6W<'_, Pdc1Spec> {
        Wupdc6W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Wake-up Pin Configuration for WUU_Pn"]
    #[inline(always)]
    pub fn wupdc7(&mut self) -> Wupdc7W<'_, Pdc1Spec> {
        Wupdc7W::new(self, 14)
    }
    #[doc = "Bits 16:17 - Wake-up Pin Configuration for WUU_Pn"]
    #[inline(always)]
    pub fn wupdc8(&mut self) -> Wupdc8W<'_, Pdc1Spec> {
        Wupdc8W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Wake-up Pin Configuration for WUU_Pn"]
    #[inline(always)]
    pub fn wupdc9(&mut self) -> Wupdc9W<'_, Pdc1Spec> {
        Wupdc9W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Wake-up Pin Configuration for WUU_Pn"]
    #[inline(always)]
    pub fn wupdc10(&mut self) -> Wupdc10W<'_, Pdc1Spec> {
        Wupdc10W::new(self, 20)
    }
    #[doc = "Bits 22:23 - Wake-up Pin Configuration for WUU_Pn"]
    #[inline(always)]
    pub fn wupdc11(&mut self) -> Wupdc11W<'_, Pdc1Spec> {
        Wupdc11W::new(self, 22)
    }
    #[doc = "Bits 24:25 - Wake-up Pin Configuration for WUU_Pn"]
    #[inline(always)]
    pub fn wupdc12(&mut self) -> Wupdc12W<'_, Pdc1Spec> {
        Wupdc12W::new(self, 24)
    }
}
#[doc = "Pin DMA/Trigger Configuration 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pdc1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdc1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pdc1Spec;
impl crate::RegisterSpec for Pdc1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdc1::R`](R) reader structure"]
impl crate::Readable for Pdc1Spec {}
#[doc = "`write(|w| ..)` method takes [`pdc1::W`](W) writer structure"]
impl crate::Writable for Pdc1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PDC1 to value 0"]
impl crate::Resettable for Pdc1Spec {}
