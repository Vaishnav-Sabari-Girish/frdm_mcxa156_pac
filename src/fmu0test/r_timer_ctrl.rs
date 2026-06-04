#[doc = "Register `R_TIMER_CTRL` reader"]
pub type R = crate::R<RTimerCtrlSpec>;
#[doc = "Register `R_TIMER_CTRL` writer"]
pub type W = crate::W<RTimerCtrlSpec>;
#[doc = "Tnvs Time Unit\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tnvsunit {
    #[doc = "0: Clock cycles"]
    Zz321 = 0,
    #[doc = "1: 0.5 usec"]
    Zz322 = 1,
    #[doc = "2: 1 usec"]
    Zz323 = 2,
    #[doc = "3: 10 usec"]
    Zz324 = 3,
    #[doc = "4: 100 usec"]
    Zz325 = 4,
    #[doc = "5: 1 msec"]
    Zz326 = 5,
    #[doc = "6: 10 msec"]
    Zz327 = 6,
    #[doc = "7: 100 msec"]
    Zz328 = 7,
}
impl From<Tnvsunit> for u8 {
    #[inline(always)]
    fn from(variant: Tnvsunit) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tnvsunit {
    type Ux = u8;
}
impl crate::IsEnum for Tnvsunit {}
#[doc = "Field `TNVSUNIT` reader - Tnvs Time Unit"]
pub type TnvsunitR = crate::FieldReader<Tnvsunit>;
impl TnvsunitR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tnvsunit {
        match self.bits {
            0 => Tnvsunit::Zz321,
            1 => Tnvsunit::Zz322,
            2 => Tnvsunit::Zz323,
            3 => Tnvsunit::Zz324,
            4 => Tnvsunit::Zz325,
            5 => Tnvsunit::Zz326,
            6 => Tnvsunit::Zz327,
            7 => Tnvsunit::Zz328,
            _ => unreachable!(),
        }
    }
    #[doc = "Clock cycles"]
    #[inline(always)]
    pub fn is_zz321(&self) -> bool {
        *self == Tnvsunit::Zz321
    }
    #[doc = "0.5 usec"]
    #[inline(always)]
    pub fn is_zz322(&self) -> bool {
        *self == Tnvsunit::Zz322
    }
    #[doc = "1 usec"]
    #[inline(always)]
    pub fn is_zz323(&self) -> bool {
        *self == Tnvsunit::Zz323
    }
    #[doc = "10 usec"]
    #[inline(always)]
    pub fn is_zz324(&self) -> bool {
        *self == Tnvsunit::Zz324
    }
    #[doc = "100 usec"]
    #[inline(always)]
    pub fn is_zz325(&self) -> bool {
        *self == Tnvsunit::Zz325
    }
    #[doc = "1 msec"]
    #[inline(always)]
    pub fn is_zz326(&self) -> bool {
        *self == Tnvsunit::Zz326
    }
    #[doc = "10 msec"]
    #[inline(always)]
    pub fn is_zz327(&self) -> bool {
        *self == Tnvsunit::Zz327
    }
    #[doc = "100 msec"]
    #[inline(always)]
    pub fn is_zz328(&self) -> bool {
        *self == Tnvsunit::Zz328
    }
}
#[doc = "Field `TNVSUNIT` writer - Tnvs Time Unit"]
pub type TnvsunitW<'a, REG> = crate::FieldWriter<'a, REG, 3, Tnvsunit, crate::Safe>;
impl<'a, REG> TnvsunitW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Clock cycles"]
    #[inline(always)]
    pub fn zz321(self) -> &'a mut crate::W<REG> {
        self.variant(Tnvsunit::Zz321)
    }
    #[doc = "0.5 usec"]
    #[inline(always)]
    pub fn zz322(self) -> &'a mut crate::W<REG> {
        self.variant(Tnvsunit::Zz322)
    }
    #[doc = "1 usec"]
    #[inline(always)]
    pub fn zz323(self) -> &'a mut crate::W<REG> {
        self.variant(Tnvsunit::Zz323)
    }
    #[doc = "10 usec"]
    #[inline(always)]
    pub fn zz324(self) -> &'a mut crate::W<REG> {
        self.variant(Tnvsunit::Zz324)
    }
    #[doc = "100 usec"]
    #[inline(always)]
    pub fn zz325(self) -> &'a mut crate::W<REG> {
        self.variant(Tnvsunit::Zz325)
    }
    #[doc = "1 msec"]
    #[inline(always)]
    pub fn zz326(self) -> &'a mut crate::W<REG> {
        self.variant(Tnvsunit::Zz326)
    }
    #[doc = "10 msec"]
    #[inline(always)]
    pub fn zz327(self) -> &'a mut crate::W<REG> {
        self.variant(Tnvsunit::Zz327)
    }
    #[doc = "100 msec"]
    #[inline(always)]
    pub fn zz328(self) -> &'a mut crate::W<REG> {
        self.variant(Tnvsunit::Zz328)
    }
}
#[doc = "Field `TNVSDLY` reader - Tnvs Time Delay Scalar"]
pub type TnvsdlyR = crate::FieldReader;
#[doc = "Field `TNVSDLY` writer - Tnvs Time Delay Scalar"]
pub type TnvsdlyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Tnvh Time Unit\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tnvhunit {
    #[doc = "0: Clock cycles"]
    Zz313 = 0,
    #[doc = "1: 0.5 usec"]
    Zz314 = 1,
    #[doc = "2: 1 usec"]
    Zz315 = 2,
    #[doc = "3: 10 usec"]
    Zz316 = 3,
    #[doc = "4: 100 usec"]
    Zz317 = 4,
    #[doc = "5: 1 msec"]
    Zz318 = 5,
    #[doc = "6: 10 msec"]
    Zz319 = 6,
    #[doc = "7: 100 msec"]
    Zz320 = 7,
}
impl From<Tnvhunit> for u8 {
    #[inline(always)]
    fn from(variant: Tnvhunit) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tnvhunit {
    type Ux = u8;
}
impl crate::IsEnum for Tnvhunit {}
#[doc = "Field `TNVHUNIT` reader - Tnvh Time Unit"]
pub type TnvhunitR = crate::FieldReader<Tnvhunit>;
impl TnvhunitR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tnvhunit {
        match self.bits {
            0 => Tnvhunit::Zz313,
            1 => Tnvhunit::Zz314,
            2 => Tnvhunit::Zz315,
            3 => Tnvhunit::Zz316,
            4 => Tnvhunit::Zz317,
            5 => Tnvhunit::Zz318,
            6 => Tnvhunit::Zz319,
            7 => Tnvhunit::Zz320,
            _ => unreachable!(),
        }
    }
    #[doc = "Clock cycles"]
    #[inline(always)]
    pub fn is_zz313(&self) -> bool {
        *self == Tnvhunit::Zz313
    }
    #[doc = "0.5 usec"]
    #[inline(always)]
    pub fn is_zz314(&self) -> bool {
        *self == Tnvhunit::Zz314
    }
    #[doc = "1 usec"]
    #[inline(always)]
    pub fn is_zz315(&self) -> bool {
        *self == Tnvhunit::Zz315
    }
    #[doc = "10 usec"]
    #[inline(always)]
    pub fn is_zz316(&self) -> bool {
        *self == Tnvhunit::Zz316
    }
    #[doc = "100 usec"]
    #[inline(always)]
    pub fn is_zz317(&self) -> bool {
        *self == Tnvhunit::Zz317
    }
    #[doc = "1 msec"]
    #[inline(always)]
    pub fn is_zz318(&self) -> bool {
        *self == Tnvhunit::Zz318
    }
    #[doc = "10 msec"]
    #[inline(always)]
    pub fn is_zz319(&self) -> bool {
        *self == Tnvhunit::Zz319
    }
    #[doc = "100 msec"]
    #[inline(always)]
    pub fn is_zz320(&self) -> bool {
        *self == Tnvhunit::Zz320
    }
}
#[doc = "Field `TNVHUNIT` writer - Tnvh Time Unit"]
pub type TnvhunitW<'a, REG> = crate::FieldWriter<'a, REG, 3, Tnvhunit, crate::Safe>;
impl<'a, REG> TnvhunitW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Clock cycles"]
    #[inline(always)]
    pub fn zz313(self) -> &'a mut crate::W<REG> {
        self.variant(Tnvhunit::Zz313)
    }
    #[doc = "0.5 usec"]
    #[inline(always)]
    pub fn zz314(self) -> &'a mut crate::W<REG> {
        self.variant(Tnvhunit::Zz314)
    }
    #[doc = "1 usec"]
    #[inline(always)]
    pub fn zz315(self) -> &'a mut crate::W<REG> {
        self.variant(Tnvhunit::Zz315)
    }
    #[doc = "10 usec"]
    #[inline(always)]
    pub fn zz316(self) -> &'a mut crate::W<REG> {
        self.variant(Tnvhunit::Zz316)
    }
    #[doc = "100 usec"]
    #[inline(always)]
    pub fn zz317(self) -> &'a mut crate::W<REG> {
        self.variant(Tnvhunit::Zz317)
    }
    #[doc = "1 msec"]
    #[inline(always)]
    pub fn zz318(self) -> &'a mut crate::W<REG> {
        self.variant(Tnvhunit::Zz318)
    }
    #[doc = "10 msec"]
    #[inline(always)]
    pub fn zz319(self) -> &'a mut crate::W<REG> {
        self.variant(Tnvhunit::Zz319)
    }
    #[doc = "100 msec"]
    #[inline(always)]
    pub fn zz320(self) -> &'a mut crate::W<REG> {
        self.variant(Tnvhunit::Zz320)
    }
}
#[doc = "Field `TNVHDLY` reader - Tnvh Time Delay Scalar"]
pub type TnvhdlyR = crate::FieldReader;
#[doc = "Field `TNVHDLY` writer - Tnvh Time Delay Scalar"]
pub type TnvhdlyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Tpgs Time Unit\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tpgsunit {
    #[doc = "0: Clock cycles"]
    Zz305 = 0,
    #[doc = "1: 0.5 usec"]
    Zz306 = 1,
    #[doc = "2: 1 usec"]
    Zz307 = 2,
    #[doc = "3: 10 usec"]
    Zz308 = 3,
    #[doc = "4: 100 usec"]
    Zz309 = 4,
    #[doc = "5: 1 msec"]
    Zz310 = 5,
    #[doc = "6: 10 msec"]
    Zz311 = 6,
    #[doc = "7: 100 msec"]
    Zz312 = 7,
}
impl From<Tpgsunit> for u8 {
    #[inline(always)]
    fn from(variant: Tpgsunit) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tpgsunit {
    type Ux = u8;
}
impl crate::IsEnum for Tpgsunit {}
#[doc = "Field `TPGSUNIT` reader - Tpgs Time Unit"]
pub type TpgsunitR = crate::FieldReader<Tpgsunit>;
impl TpgsunitR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tpgsunit {
        match self.bits {
            0 => Tpgsunit::Zz305,
            1 => Tpgsunit::Zz306,
            2 => Tpgsunit::Zz307,
            3 => Tpgsunit::Zz308,
            4 => Tpgsunit::Zz309,
            5 => Tpgsunit::Zz310,
            6 => Tpgsunit::Zz311,
            7 => Tpgsunit::Zz312,
            _ => unreachable!(),
        }
    }
    #[doc = "Clock cycles"]
    #[inline(always)]
    pub fn is_zz305(&self) -> bool {
        *self == Tpgsunit::Zz305
    }
    #[doc = "0.5 usec"]
    #[inline(always)]
    pub fn is_zz306(&self) -> bool {
        *self == Tpgsunit::Zz306
    }
    #[doc = "1 usec"]
    #[inline(always)]
    pub fn is_zz307(&self) -> bool {
        *self == Tpgsunit::Zz307
    }
    #[doc = "10 usec"]
    #[inline(always)]
    pub fn is_zz308(&self) -> bool {
        *self == Tpgsunit::Zz308
    }
    #[doc = "100 usec"]
    #[inline(always)]
    pub fn is_zz309(&self) -> bool {
        *self == Tpgsunit::Zz309
    }
    #[doc = "1 msec"]
    #[inline(always)]
    pub fn is_zz310(&self) -> bool {
        *self == Tpgsunit::Zz310
    }
    #[doc = "10 msec"]
    #[inline(always)]
    pub fn is_zz311(&self) -> bool {
        *self == Tpgsunit::Zz311
    }
    #[doc = "100 msec"]
    #[inline(always)]
    pub fn is_zz312(&self) -> bool {
        *self == Tpgsunit::Zz312
    }
}
#[doc = "Field `TPGSUNIT` writer - Tpgs Time Unit"]
pub type TpgsunitW<'a, REG> = crate::FieldWriter<'a, REG, 3, Tpgsunit, crate::Safe>;
impl<'a, REG> TpgsunitW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Clock cycles"]
    #[inline(always)]
    pub fn zz305(self) -> &'a mut crate::W<REG> {
        self.variant(Tpgsunit::Zz305)
    }
    #[doc = "0.5 usec"]
    #[inline(always)]
    pub fn zz306(self) -> &'a mut crate::W<REG> {
        self.variant(Tpgsunit::Zz306)
    }
    #[doc = "1 usec"]
    #[inline(always)]
    pub fn zz307(self) -> &'a mut crate::W<REG> {
        self.variant(Tpgsunit::Zz307)
    }
    #[doc = "10 usec"]
    #[inline(always)]
    pub fn zz308(self) -> &'a mut crate::W<REG> {
        self.variant(Tpgsunit::Zz308)
    }
    #[doc = "100 usec"]
    #[inline(always)]
    pub fn zz309(self) -> &'a mut crate::W<REG> {
        self.variant(Tpgsunit::Zz309)
    }
    #[doc = "1 msec"]
    #[inline(always)]
    pub fn zz310(self) -> &'a mut crate::W<REG> {
        self.variant(Tpgsunit::Zz310)
    }
    #[doc = "10 msec"]
    #[inline(always)]
    pub fn zz311(self) -> &'a mut crate::W<REG> {
        self.variant(Tpgsunit::Zz311)
    }
    #[doc = "100 msec"]
    #[inline(always)]
    pub fn zz312(self) -> &'a mut crate::W<REG> {
        self.variant(Tpgsunit::Zz312)
    }
}
#[doc = "Field `TPGSDLY` reader - Tpgs Time Delay Scalar"]
pub type TpgsdlyR = crate::FieldReader;
#[doc = "Field `TPGSDLY` writer - Tpgs Time Delay Scalar"]
pub type TpgsdlyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Trcv Time Unit\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Trcvunit {
    #[doc = "0: Clock cycles"]
    Zz297 = 0,
    #[doc = "1: 0.5 usec"]
    Zz298 = 1,
    #[doc = "2: 1 usec"]
    Zz299 = 2,
    #[doc = "3: 10 usec"]
    Zz300 = 3,
    #[doc = "4: 100 usec"]
    Zz301 = 4,
    #[doc = "5: 1 msec"]
    Zz302 = 5,
    #[doc = "6: 10 msec"]
    Zz303 = 6,
    #[doc = "7: 100 msec"]
    Zz304 = 7,
}
impl From<Trcvunit> for u8 {
    #[inline(always)]
    fn from(variant: Trcvunit) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Trcvunit {
    type Ux = u8;
}
impl crate::IsEnum for Trcvunit {}
#[doc = "Field `TRCVUNIT` reader - Trcv Time Unit"]
pub type TrcvunitR = crate::FieldReader<Trcvunit>;
impl TrcvunitR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trcvunit {
        match self.bits {
            0 => Trcvunit::Zz297,
            1 => Trcvunit::Zz298,
            2 => Trcvunit::Zz299,
            3 => Trcvunit::Zz300,
            4 => Trcvunit::Zz301,
            5 => Trcvunit::Zz302,
            6 => Trcvunit::Zz303,
            7 => Trcvunit::Zz304,
            _ => unreachable!(),
        }
    }
    #[doc = "Clock cycles"]
    #[inline(always)]
    pub fn is_zz297(&self) -> bool {
        *self == Trcvunit::Zz297
    }
    #[doc = "0.5 usec"]
    #[inline(always)]
    pub fn is_zz298(&self) -> bool {
        *self == Trcvunit::Zz298
    }
    #[doc = "1 usec"]
    #[inline(always)]
    pub fn is_zz299(&self) -> bool {
        *self == Trcvunit::Zz299
    }
    #[doc = "10 usec"]
    #[inline(always)]
    pub fn is_zz300(&self) -> bool {
        *self == Trcvunit::Zz300
    }
    #[doc = "100 usec"]
    #[inline(always)]
    pub fn is_zz301(&self) -> bool {
        *self == Trcvunit::Zz301
    }
    #[doc = "1 msec"]
    #[inline(always)]
    pub fn is_zz302(&self) -> bool {
        *self == Trcvunit::Zz302
    }
    #[doc = "10 msec"]
    #[inline(always)]
    pub fn is_zz303(&self) -> bool {
        *self == Trcvunit::Zz303
    }
    #[doc = "100 msec"]
    #[inline(always)]
    pub fn is_zz304(&self) -> bool {
        *self == Trcvunit::Zz304
    }
}
#[doc = "Field `TRCVUNIT` writer - Trcv Time Unit"]
pub type TrcvunitW<'a, REG> = crate::FieldWriter<'a, REG, 3, Trcvunit, crate::Safe>;
impl<'a, REG> TrcvunitW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Clock cycles"]
    #[inline(always)]
    pub fn zz297(self) -> &'a mut crate::W<REG> {
        self.variant(Trcvunit::Zz297)
    }
    #[doc = "0.5 usec"]
    #[inline(always)]
    pub fn zz298(self) -> &'a mut crate::W<REG> {
        self.variant(Trcvunit::Zz298)
    }
    #[doc = "1 usec"]
    #[inline(always)]
    pub fn zz299(self) -> &'a mut crate::W<REG> {
        self.variant(Trcvunit::Zz299)
    }
    #[doc = "10 usec"]
    #[inline(always)]
    pub fn zz300(self) -> &'a mut crate::W<REG> {
        self.variant(Trcvunit::Zz300)
    }
    #[doc = "100 usec"]
    #[inline(always)]
    pub fn zz301(self) -> &'a mut crate::W<REG> {
        self.variant(Trcvunit::Zz301)
    }
    #[doc = "1 msec"]
    #[inline(always)]
    pub fn zz302(self) -> &'a mut crate::W<REG> {
        self.variant(Trcvunit::Zz302)
    }
    #[doc = "10 msec"]
    #[inline(always)]
    pub fn zz303(self) -> &'a mut crate::W<REG> {
        self.variant(Trcvunit::Zz303)
    }
    #[doc = "100 msec"]
    #[inline(always)]
    pub fn zz304(self) -> &'a mut crate::W<REG> {
        self.variant(Trcvunit::Zz304)
    }
}
#[doc = "Field `TRCVDLY` reader - Trcv Time Delay Scalar"]
pub type TrcvdlyR = crate::FieldReader;
#[doc = "Field `TRCVDLY` writer - Trcv Time Delay Scalar"]
pub type TrcvdlyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Tlvs Time Unit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tlvsunit {
    #[doc = "0: Clock cycles"]
    Zz289 = 0,
    #[doc = "1: 0.5 usec"]
    Zz290 = 1,
    #[doc = "2: 1 usec"]
    Zz291 = 2,
    #[doc = "3: 10 usec"]
    Zz292 = 3,
    #[doc = "4: 100 usec"]
    Zz293 = 4,
    #[doc = "5: 1 msec"]
    Zz294 = 5,
    #[doc = "6: 10 msec"]
    Zz295 = 6,
    #[doc = "7: 100 msec"]
    Zz296 = 7,
}
impl From<Tlvsunit> for u8 {
    #[inline(always)]
    fn from(variant: Tlvsunit) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tlvsunit {
    type Ux = u8;
}
impl crate::IsEnum for Tlvsunit {}
#[doc = "Field `TLVSUNIT` reader - Tlvs Time Unit"]
pub type TlvsunitR = crate::FieldReader<Tlvsunit>;
impl TlvsunitR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tlvsunit {
        match self.bits {
            0 => Tlvsunit::Zz289,
            1 => Tlvsunit::Zz290,
            2 => Tlvsunit::Zz291,
            3 => Tlvsunit::Zz292,
            4 => Tlvsunit::Zz293,
            5 => Tlvsunit::Zz294,
            6 => Tlvsunit::Zz295,
            7 => Tlvsunit::Zz296,
            _ => unreachable!(),
        }
    }
    #[doc = "Clock cycles"]
    #[inline(always)]
    pub fn is_zz289(&self) -> bool {
        *self == Tlvsunit::Zz289
    }
    #[doc = "0.5 usec"]
    #[inline(always)]
    pub fn is_zz290(&self) -> bool {
        *self == Tlvsunit::Zz290
    }
    #[doc = "1 usec"]
    #[inline(always)]
    pub fn is_zz291(&self) -> bool {
        *self == Tlvsunit::Zz291
    }
    #[doc = "10 usec"]
    #[inline(always)]
    pub fn is_zz292(&self) -> bool {
        *self == Tlvsunit::Zz292
    }
    #[doc = "100 usec"]
    #[inline(always)]
    pub fn is_zz293(&self) -> bool {
        *self == Tlvsunit::Zz293
    }
    #[doc = "1 msec"]
    #[inline(always)]
    pub fn is_zz294(&self) -> bool {
        *self == Tlvsunit::Zz294
    }
    #[doc = "10 msec"]
    #[inline(always)]
    pub fn is_zz295(&self) -> bool {
        *self == Tlvsunit::Zz295
    }
    #[doc = "100 msec"]
    #[inline(always)]
    pub fn is_zz296(&self) -> bool {
        *self == Tlvsunit::Zz296
    }
}
#[doc = "Field `TLVSUNIT` writer - Tlvs Time Unit"]
pub type TlvsunitW<'a, REG> = crate::FieldWriter<'a, REG, 3, Tlvsunit, crate::Safe>;
impl<'a, REG> TlvsunitW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Clock cycles"]
    #[inline(always)]
    pub fn zz289(self) -> &'a mut crate::W<REG> {
        self.variant(Tlvsunit::Zz289)
    }
    #[doc = "0.5 usec"]
    #[inline(always)]
    pub fn zz290(self) -> &'a mut crate::W<REG> {
        self.variant(Tlvsunit::Zz290)
    }
    #[doc = "1 usec"]
    #[inline(always)]
    pub fn zz291(self) -> &'a mut crate::W<REG> {
        self.variant(Tlvsunit::Zz291)
    }
    #[doc = "10 usec"]
    #[inline(always)]
    pub fn zz292(self) -> &'a mut crate::W<REG> {
        self.variant(Tlvsunit::Zz292)
    }
    #[doc = "100 usec"]
    #[inline(always)]
    pub fn zz293(self) -> &'a mut crate::W<REG> {
        self.variant(Tlvsunit::Zz293)
    }
    #[doc = "1 msec"]
    #[inline(always)]
    pub fn zz294(self) -> &'a mut crate::W<REG> {
        self.variant(Tlvsunit::Zz294)
    }
    #[doc = "10 msec"]
    #[inline(always)]
    pub fn zz295(self) -> &'a mut crate::W<REG> {
        self.variant(Tlvsunit::Zz295)
    }
    #[doc = "100 msec"]
    #[inline(always)]
    pub fn zz296(self) -> &'a mut crate::W<REG> {
        self.variant(Tlvsunit::Zz296)
    }
}
#[doc = "Field `TLVSDLY_L` reader - Tlvs Time Delay Scalar Low"]
pub type TlvsdlyLR = crate::BitReader;
#[doc = "Field `TLVSDLY_L` writer - Tlvs Time Delay Scalar Low"]
pub type TlvsdlyLW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Tnvs Time Unit"]
    #[inline(always)]
    pub fn tnvsunit(&self) -> TnvsunitR {
        TnvsunitR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:6 - Tnvs Time Delay Scalar"]
    #[inline(always)]
    pub fn tnvsdly(&self) -> TnvsdlyR {
        TnvsdlyR::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bits 7:9 - Tnvh Time Unit"]
    #[inline(always)]
    pub fn tnvhunit(&self) -> TnvhunitR {
        TnvhunitR::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bits 10:13 - Tnvh Time Delay Scalar"]
    #[inline(always)]
    pub fn tnvhdly(&self) -> TnvhdlyR {
        TnvhdlyR::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bits 14:16 - Tpgs Time Unit"]
    #[inline(always)]
    pub fn tpgsunit(&self) -> TpgsunitR {
        TpgsunitR::new(((self.bits >> 14) & 7) as u8)
    }
    #[doc = "Bits 17:20 - Tpgs Time Delay Scalar"]
    #[inline(always)]
    pub fn tpgsdly(&self) -> TpgsdlyR {
        TpgsdlyR::new(((self.bits >> 17) & 0x0f) as u8)
    }
    #[doc = "Bits 21:23 - Trcv Time Unit"]
    #[inline(always)]
    pub fn trcvunit(&self) -> TrcvunitR {
        TrcvunitR::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:27 - Trcv Time Delay Scalar"]
    #[inline(always)]
    pub fn trcvdly(&self) -> TrcvdlyR {
        TrcvdlyR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:30 - Tlvs Time Unit"]
    #[inline(always)]
    pub fn tlvsunit(&self) -> TlvsunitR {
        TlvsunitR::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - Tlvs Time Delay Scalar Low"]
    #[inline(always)]
    pub fn tlvsdly_l(&self) -> TlvsdlyLR {
        TlvsdlyLR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Tnvs Time Unit"]
    #[inline(always)]
    pub fn tnvsunit(&mut self) -> TnvsunitW<'_, RTimerCtrlSpec> {
        TnvsunitW::new(self, 0)
    }
    #[doc = "Bits 3:6 - Tnvs Time Delay Scalar"]
    #[inline(always)]
    pub fn tnvsdly(&mut self) -> TnvsdlyW<'_, RTimerCtrlSpec> {
        TnvsdlyW::new(self, 3)
    }
    #[doc = "Bits 7:9 - Tnvh Time Unit"]
    #[inline(always)]
    pub fn tnvhunit(&mut self) -> TnvhunitW<'_, RTimerCtrlSpec> {
        TnvhunitW::new(self, 7)
    }
    #[doc = "Bits 10:13 - Tnvh Time Delay Scalar"]
    #[inline(always)]
    pub fn tnvhdly(&mut self) -> TnvhdlyW<'_, RTimerCtrlSpec> {
        TnvhdlyW::new(self, 10)
    }
    #[doc = "Bits 14:16 - Tpgs Time Unit"]
    #[inline(always)]
    pub fn tpgsunit(&mut self) -> TpgsunitW<'_, RTimerCtrlSpec> {
        TpgsunitW::new(self, 14)
    }
    #[doc = "Bits 17:20 - Tpgs Time Delay Scalar"]
    #[inline(always)]
    pub fn tpgsdly(&mut self) -> TpgsdlyW<'_, RTimerCtrlSpec> {
        TpgsdlyW::new(self, 17)
    }
    #[doc = "Bits 21:23 - Trcv Time Unit"]
    #[inline(always)]
    pub fn trcvunit(&mut self) -> TrcvunitW<'_, RTimerCtrlSpec> {
        TrcvunitW::new(self, 21)
    }
    #[doc = "Bits 24:27 - Trcv Time Delay Scalar"]
    #[inline(always)]
    pub fn trcvdly(&mut self) -> TrcvdlyW<'_, RTimerCtrlSpec> {
        TrcvdlyW::new(self, 24)
    }
    #[doc = "Bits 28:30 - Tlvs Time Unit"]
    #[inline(always)]
    pub fn tlvsunit(&mut self) -> TlvsunitW<'_, RTimerCtrlSpec> {
        TlvsunitW::new(self, 28)
    }
    #[doc = "Bit 31 - Tlvs Time Delay Scalar Low"]
    #[inline(always)]
    pub fn tlvsdly_l(&mut self) -> TlvsdlyLW<'_, RTimerCtrlSpec> {
        TlvsdlyLW::new(self, 31)
    }
}
#[doc = "BIST Timer Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r_timer_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r_timer_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTimerCtrlSpec;
impl crate::RegisterSpec for RTimerCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r_timer_ctrl::R`](R) reader structure"]
impl crate::Readable for RTimerCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`r_timer_ctrl::W`](W) writer structure"]
impl crate::Writable for RTimerCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R_TIMER_CTRL to value 0x9a44_9542"]
impl crate::Resettable for RTimerCtrlSpec {
    const RESET_VALUE: u32 = 0x9a44_9542;
}
