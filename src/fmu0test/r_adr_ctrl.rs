#[doc = "Register `R_ADR_CTRL` reader"]
pub type R = crate::R<RAdrCtrlSpec>;
#[doc = "Register `R_ADR_CTRL` writer"]
pub type W = crate::W<RAdrCtrlSpec>;
#[doc = "Data Group Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Grpsel {
    #[doc = "0: Select no data"]
    Zz270 = 0,
    #[doc = "1: Select data slice \\[34:0\\]"]
    Zz271 = 1,
    #[doc = "2: Select data slice \\[69:35\\]"]
    Zz272 = 2,
    #[doc = "4: Select data slice \\[104:70\\]"]
    Zz273 = 4,
    #[doc = "8: Select data slice \\[136:105\\]"]
    Zz274 = 8,
    #[doc = "15: Select data \\[136:0\\]"]
    Zz275 = 15,
}
impl From<Grpsel> for u8 {
    #[inline(always)]
    fn from(variant: Grpsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Grpsel {
    type Ux = u8;
}
impl crate::IsEnum for Grpsel {}
#[doc = "Field `GRPSEL` reader - Data Group Select"]
pub type GrpselR = crate::FieldReader<Grpsel>;
impl GrpselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Grpsel> {
        match self.bits {
            0 => Some(Grpsel::Zz270),
            1 => Some(Grpsel::Zz271),
            2 => Some(Grpsel::Zz272),
            4 => Some(Grpsel::Zz273),
            8 => Some(Grpsel::Zz274),
            15 => Some(Grpsel::Zz275),
            _ => None,
        }
    }
    #[doc = "Select no data"]
    #[inline(always)]
    pub fn is_zz270(&self) -> bool {
        *self == Grpsel::Zz270
    }
    #[doc = "Select data slice \\[34:0\\]"]
    #[inline(always)]
    pub fn is_zz271(&self) -> bool {
        *self == Grpsel::Zz271
    }
    #[doc = "Select data slice \\[69:35\\]"]
    #[inline(always)]
    pub fn is_zz272(&self) -> bool {
        *self == Grpsel::Zz272
    }
    #[doc = "Select data slice \\[104:70\\]"]
    #[inline(always)]
    pub fn is_zz273(&self) -> bool {
        *self == Grpsel::Zz273
    }
    #[doc = "Select data slice \\[136:105\\]"]
    #[inline(always)]
    pub fn is_zz274(&self) -> bool {
        *self == Grpsel::Zz274
    }
    #[doc = "Select data \\[136:0\\]"]
    #[inline(always)]
    pub fn is_zz275(&self) -> bool {
        *self == Grpsel::Zz275
    }
}
#[doc = "Field `GRPSEL` writer - Data Group Select"]
pub type GrpselW<'a, REG> = crate::FieldWriter<'a, REG, 4, Grpsel>;
impl<'a, REG> GrpselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select no data"]
    #[inline(always)]
    pub fn zz270(self) -> &'a mut crate::W<REG> {
        self.variant(Grpsel::Zz270)
    }
    #[doc = "Select data slice \\[34:0\\]"]
    #[inline(always)]
    pub fn zz271(self) -> &'a mut crate::W<REG> {
        self.variant(Grpsel::Zz271)
    }
    #[doc = "Select data slice \\[69:35\\]"]
    #[inline(always)]
    pub fn zz272(self) -> &'a mut crate::W<REG> {
        self.variant(Grpsel::Zz272)
    }
    #[doc = "Select data slice \\[104:70\\]"]
    #[inline(always)]
    pub fn zz273(self) -> &'a mut crate::W<REG> {
        self.variant(Grpsel::Zz273)
    }
    #[doc = "Select data slice \\[136:105\\]"]
    #[inline(always)]
    pub fn zz274(self) -> &'a mut crate::W<REG> {
        self.variant(Grpsel::Zz274)
    }
    #[doc = "Select data \\[136:0\\]"]
    #[inline(always)]
    pub fn zz275(self) -> &'a mut crate::W<REG> {
        self.variant(Grpsel::Zz275)
    }
}
#[doc = "Field `XADR` reader - BIST XADR"]
pub type XadrR = crate::FieldReader<u16>;
#[doc = "Field `XADR` writer - BIST XADR"]
pub type XadrW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `YADR` reader - BIST YADR"]
pub type YadrR = crate::FieldReader;
#[doc = "Field `YADR` writer - BIST YADR"]
pub type YadrW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Program Attribute\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ProgAttr {
    #[doc = "0: One YE pulse will program one data slice group"]
    Zz262 = 0,
    #[doc = "1: One YE pulse will program two data slice groups"]
    Zz263 = 1,
    #[doc = "2: One YE pulse will program three data slice groups (reserved)"]
    Zz264 = 2,
    #[doc = "3: One YE pulse will program four data slice groups"]
    Zz265 = 3,
    #[doc = "4: One YE pulse will program five data slice groups (reserved)"]
    Zz266 = 4,
    #[doc = "5: One YE pulse will program six data slice groups (reserved)"]
    Zz267 = 5,
    #[doc = "6: One YE pulse will program seven data slice groups (reserved)"]
    Zz268 = 6,
    #[doc = "7: One YE pulse will program eight data slice groups (reserved)"]
    Zz269 = 7,
}
impl From<ProgAttr> for u8 {
    #[inline(always)]
    fn from(variant: ProgAttr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ProgAttr {
    type Ux = u8;
}
impl crate::IsEnum for ProgAttr {}
#[doc = "Field `PROG_ATTR` reader - Program Attribute"]
pub type ProgAttrR = crate::FieldReader<ProgAttr>;
impl ProgAttrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ProgAttr {
        match self.bits {
            0 => ProgAttr::Zz262,
            1 => ProgAttr::Zz263,
            2 => ProgAttr::Zz264,
            3 => ProgAttr::Zz265,
            4 => ProgAttr::Zz266,
            5 => ProgAttr::Zz267,
            6 => ProgAttr::Zz268,
            7 => ProgAttr::Zz269,
            _ => unreachable!(),
        }
    }
    #[doc = "One YE pulse will program one data slice group"]
    #[inline(always)]
    pub fn is_zz262(&self) -> bool {
        *self == ProgAttr::Zz262
    }
    #[doc = "One YE pulse will program two data slice groups"]
    #[inline(always)]
    pub fn is_zz263(&self) -> bool {
        *self == ProgAttr::Zz263
    }
    #[doc = "One YE pulse will program three data slice groups (reserved)"]
    #[inline(always)]
    pub fn is_zz264(&self) -> bool {
        *self == ProgAttr::Zz264
    }
    #[doc = "One YE pulse will program four data slice groups"]
    #[inline(always)]
    pub fn is_zz265(&self) -> bool {
        *self == ProgAttr::Zz265
    }
    #[doc = "One YE pulse will program five data slice groups (reserved)"]
    #[inline(always)]
    pub fn is_zz266(&self) -> bool {
        *self == ProgAttr::Zz266
    }
    #[doc = "One YE pulse will program six data slice groups (reserved)"]
    #[inline(always)]
    pub fn is_zz267(&self) -> bool {
        *self == ProgAttr::Zz267
    }
    #[doc = "One YE pulse will program seven data slice groups (reserved)"]
    #[inline(always)]
    pub fn is_zz268(&self) -> bool {
        *self == ProgAttr::Zz268
    }
    #[doc = "One YE pulse will program eight data slice groups (reserved)"]
    #[inline(always)]
    pub fn is_zz269(&self) -> bool {
        *self == ProgAttr::Zz269
    }
}
#[doc = "Field `PROG_ATTR` writer - Program Attribute"]
pub type ProgAttrW<'a, REG> = crate::FieldWriter<'a, REG, 3, ProgAttr, crate::Safe>;
impl<'a, REG> ProgAttrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "One YE pulse will program one data slice group"]
    #[inline(always)]
    pub fn zz262(self) -> &'a mut crate::W<REG> {
        self.variant(ProgAttr::Zz262)
    }
    #[doc = "One YE pulse will program two data slice groups"]
    #[inline(always)]
    pub fn zz263(self) -> &'a mut crate::W<REG> {
        self.variant(ProgAttr::Zz263)
    }
    #[doc = "One YE pulse will program three data slice groups (reserved)"]
    #[inline(always)]
    pub fn zz264(self) -> &'a mut crate::W<REG> {
        self.variant(ProgAttr::Zz264)
    }
    #[doc = "One YE pulse will program four data slice groups"]
    #[inline(always)]
    pub fn zz265(self) -> &'a mut crate::W<REG> {
        self.variant(ProgAttr::Zz265)
    }
    #[doc = "One YE pulse will program five data slice groups (reserved)"]
    #[inline(always)]
    pub fn zz266(self) -> &'a mut crate::W<REG> {
        self.variant(ProgAttr::Zz266)
    }
    #[doc = "One YE pulse will program six data slice groups (reserved)"]
    #[inline(always)]
    pub fn zz267(self) -> &'a mut crate::W<REG> {
        self.variant(ProgAttr::Zz267)
    }
    #[doc = "One YE pulse will program seven data slice groups (reserved)"]
    #[inline(always)]
    pub fn zz268(self) -> &'a mut crate::W<REG> {
        self.variant(ProgAttr::Zz268)
    }
    #[doc = "One YE pulse will program eight data slice groups (reserved)"]
    #[inline(always)]
    pub fn zz269(self) -> &'a mut crate::W<REG> {
        self.variant(ProgAttr::Zz269)
    }
}
impl R {
    #[doc = "Bits 0:3 - Data Group Select"]
    #[inline(always)]
    pub fn grpsel(&self) -> GrpselR {
        GrpselR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:15 - BIST XADR"]
    #[inline(always)]
    pub fn xadr(&self) -> XadrR {
        XadrR::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 16:20 - BIST YADR"]
    #[inline(always)]
    pub fn yadr(&self) -> YadrR {
        YadrR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:23 - Program Attribute"]
    #[inline(always)]
    pub fn prog_attr(&self) -> ProgAttrR {
        ProgAttrR::new(((self.bits >> 21) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Data Group Select"]
    #[inline(always)]
    pub fn grpsel(&mut self) -> GrpselW<'_, RAdrCtrlSpec> {
        GrpselW::new(self, 0)
    }
    #[doc = "Bits 4:15 - BIST XADR"]
    #[inline(always)]
    pub fn xadr(&mut self) -> XadrW<'_, RAdrCtrlSpec> {
        XadrW::new(self, 4)
    }
    #[doc = "Bits 16:20 - BIST YADR"]
    #[inline(always)]
    pub fn yadr(&mut self) -> YadrW<'_, RAdrCtrlSpec> {
        YadrW::new(self, 16)
    }
    #[doc = "Bits 21:23 - Program Attribute"]
    #[inline(always)]
    pub fn prog_attr(&mut self) -> ProgAttrW<'_, RAdrCtrlSpec> {
        ProgAttrW::new(self, 21)
    }
}
#[doc = "BIST Address Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r_adr_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r_adr_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RAdrCtrlSpec;
impl crate::RegisterSpec for RAdrCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r_adr_ctrl::R`](R) reader structure"]
impl crate::Readable for RAdrCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`r_adr_ctrl::W`](W) writer structure"]
impl crate::Writable for RAdrCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R_ADR_CTRL to value 0"]
impl crate::Resettable for RAdrCtrlSpec {}
