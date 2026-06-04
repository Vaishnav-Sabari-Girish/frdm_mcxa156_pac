#[doc = "Register `R_PIN_CTRL` reader"]
pub type R = crate::R<RPinCtrlSpec>;
#[doc = "Register `R_PIN_CTRL` writer"]
pub type W = crate::W<RPinCtrlSpec>;
#[doc = "Field `MAS1` reader - Mass Erase"]
pub type Mas1R = crate::BitReader;
#[doc = "Field `MAS1` writer - Mass Erase"]
pub type Mas1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IFREN` reader - IFR Enable"]
pub type IfrenR = crate::BitReader;
#[doc = "Field `IFREN` writer - IFR Enable"]
pub type IfrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IFREN1` reader - IFR1 Enable"]
pub type Ifren1R = crate::BitReader;
#[doc = "Field `IFREN1` writer - IFR1 Enable"]
pub type Ifren1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REDEN` reader - Redundancy Block Enable"]
pub type RedenR = crate::BitReader;
#[doc = "Field `REDEN` writer - Redundancy Block Enable"]
pub type RedenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LVE` reader - Low Voltage Enable"]
pub type LveR = crate::BitReader;
#[doc = "Field `LVE` writer - Low Voltage Enable"]
pub type LveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PV` reader - Program Verify Enable"]
pub type PvR = crate::BitReader;
#[doc = "Field `PV` writer - Program Verify Enable"]
pub type PvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EV` reader - Erase Verify Enable"]
pub type EvR = crate::BitReader;
#[doc = "Field `EV` writer - Erase Verify Enable"]
pub type EvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WIPGM` reader - Program Current"]
pub type WipgmR = crate::FieldReader;
#[doc = "Field `WIPGM` writer - Program Current"]
pub type WipgmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WHV` reader - High Voltage Level"]
pub type WhvR = crate::FieldReader;
#[doc = "Field `WHV` writer - High Voltage Level"]
pub type WhvW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WMV` reader - Medium Voltage Level"]
pub type WmvR = crate::FieldReader;
#[doc = "Field `WMV` writer - Medium Voltage Level"]
pub type WmvW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `XE` reader - X Address Enable"]
pub type XeR = crate::BitReader;
#[doc = "Field `XE` writer - X Address Enable"]
pub type XeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `YE` reader - Y Address Enable"]
pub type YeR = crate::BitReader;
#[doc = "Field `YE` writer - Y Address Enable"]
pub type YeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SE` reader - Sense Amp Enable"]
pub type SeR = crate::BitReader;
#[doc = "Field `SE` writer - Sense Amp Enable"]
pub type SeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERASE` reader - Erase Mode"]
pub type EraseR = crate::BitReader;
#[doc = "Field `ERASE` writer - Erase Mode"]
pub type EraseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROG` reader - Program Mode"]
pub type ProgR = crate::BitReader;
#[doc = "Field `PROG` writer - Program Mode"]
pub type ProgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NVSTR` reader - NVM Store"]
pub type NvstrR = crate::BitReader;
#[doc = "Field `NVSTR` writer - NVM Store"]
pub type NvstrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLM` reader - Sleep Mode Enable"]
pub type SlmR = crate::BitReader;
#[doc = "Field `SLM` writer - Sleep Mode Enable"]
pub type SlmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RECALL` reader - Recall Trim Code"]
pub type RecallR = crate::BitReader;
#[doc = "Field `RECALL` writer - Recall Trim Code"]
pub type RecallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HEM` reader - HEM Control"]
pub type HemR = crate::BitReader;
#[doc = "Field `HEM` writer - HEM Control"]
pub type HemW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Mass Erase"]
    #[inline(always)]
    pub fn mas1(&self) -> Mas1R {
        Mas1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IFR Enable"]
    #[inline(always)]
    pub fn ifren(&self) -> IfrenR {
        IfrenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IFR1 Enable"]
    #[inline(always)]
    pub fn ifren1(&self) -> Ifren1R {
        Ifren1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Redundancy Block Enable"]
    #[inline(always)]
    pub fn reden(&self) -> RedenR {
        RedenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Low Voltage Enable"]
    #[inline(always)]
    pub fn lve(&self) -> LveR {
        LveR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Program Verify Enable"]
    #[inline(always)]
    pub fn pv(&self) -> PvR {
        PvR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Erase Verify Enable"]
    #[inline(always)]
    pub fn ev(&self) -> EvR {
        EvR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:8 - Program Current"]
    #[inline(always)]
    pub fn wipgm(&self) -> WipgmR {
        WipgmR::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 9:12 - High Voltage Level"]
    #[inline(always)]
    pub fn whv(&self) -> WhvR {
        WhvR::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bits 13:15 - Medium Voltage Level"]
    #[inline(always)]
    pub fn wmv(&self) -> WmvR {
        WmvR::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bit 16 - X Address Enable"]
    #[inline(always)]
    pub fn xe(&self) -> XeR {
        XeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Y Address Enable"]
    #[inline(always)]
    pub fn ye(&self) -> YeR {
        YeR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Sense Amp Enable"]
    #[inline(always)]
    pub fn se(&self) -> SeR {
        SeR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Erase Mode"]
    #[inline(always)]
    pub fn erase(&self) -> EraseR {
        EraseR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Program Mode"]
    #[inline(always)]
    pub fn prog(&self) -> ProgR {
        ProgR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - NVM Store"]
    #[inline(always)]
    pub fn nvstr(&self) -> NvstrR {
        NvstrR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Sleep Mode Enable"]
    #[inline(always)]
    pub fn slm(&self) -> SlmR {
        SlmR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Recall Trim Code"]
    #[inline(always)]
    pub fn recall(&self) -> RecallR {
        RecallR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - HEM Control"]
    #[inline(always)]
    pub fn hem(&self) -> HemR {
        HemR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mass Erase"]
    #[inline(always)]
    pub fn mas1(&mut self) -> Mas1W<'_, RPinCtrlSpec> {
        Mas1W::new(self, 0)
    }
    #[doc = "Bit 1 - IFR Enable"]
    #[inline(always)]
    pub fn ifren(&mut self) -> IfrenW<'_, RPinCtrlSpec> {
        IfrenW::new(self, 1)
    }
    #[doc = "Bit 2 - IFR1 Enable"]
    #[inline(always)]
    pub fn ifren1(&mut self) -> Ifren1W<'_, RPinCtrlSpec> {
        Ifren1W::new(self, 2)
    }
    #[doc = "Bit 3 - Redundancy Block Enable"]
    #[inline(always)]
    pub fn reden(&mut self) -> RedenW<'_, RPinCtrlSpec> {
        RedenW::new(self, 3)
    }
    #[doc = "Bit 4 - Low Voltage Enable"]
    #[inline(always)]
    pub fn lve(&mut self) -> LveW<'_, RPinCtrlSpec> {
        LveW::new(self, 4)
    }
    #[doc = "Bit 5 - Program Verify Enable"]
    #[inline(always)]
    pub fn pv(&mut self) -> PvW<'_, RPinCtrlSpec> {
        PvW::new(self, 5)
    }
    #[doc = "Bit 6 - Erase Verify Enable"]
    #[inline(always)]
    pub fn ev(&mut self) -> EvW<'_, RPinCtrlSpec> {
        EvW::new(self, 6)
    }
    #[doc = "Bits 7:8 - Program Current"]
    #[inline(always)]
    pub fn wipgm(&mut self) -> WipgmW<'_, RPinCtrlSpec> {
        WipgmW::new(self, 7)
    }
    #[doc = "Bits 9:12 - High Voltage Level"]
    #[inline(always)]
    pub fn whv(&mut self) -> WhvW<'_, RPinCtrlSpec> {
        WhvW::new(self, 9)
    }
    #[doc = "Bits 13:15 - Medium Voltage Level"]
    #[inline(always)]
    pub fn wmv(&mut self) -> WmvW<'_, RPinCtrlSpec> {
        WmvW::new(self, 13)
    }
    #[doc = "Bit 16 - X Address Enable"]
    #[inline(always)]
    pub fn xe(&mut self) -> XeW<'_, RPinCtrlSpec> {
        XeW::new(self, 16)
    }
    #[doc = "Bit 17 - Y Address Enable"]
    #[inline(always)]
    pub fn ye(&mut self) -> YeW<'_, RPinCtrlSpec> {
        YeW::new(self, 17)
    }
    #[doc = "Bit 18 - Sense Amp Enable"]
    #[inline(always)]
    pub fn se(&mut self) -> SeW<'_, RPinCtrlSpec> {
        SeW::new(self, 18)
    }
    #[doc = "Bit 19 - Erase Mode"]
    #[inline(always)]
    pub fn erase(&mut self) -> EraseW<'_, RPinCtrlSpec> {
        EraseW::new(self, 19)
    }
    #[doc = "Bit 20 - Program Mode"]
    #[inline(always)]
    pub fn prog(&mut self) -> ProgW<'_, RPinCtrlSpec> {
        ProgW::new(self, 20)
    }
    #[doc = "Bit 21 - NVM Store"]
    #[inline(always)]
    pub fn nvstr(&mut self) -> NvstrW<'_, RPinCtrlSpec> {
        NvstrW::new(self, 21)
    }
    #[doc = "Bit 22 - Sleep Mode Enable"]
    #[inline(always)]
    pub fn slm(&mut self) -> SlmW<'_, RPinCtrlSpec> {
        SlmW::new(self, 22)
    }
    #[doc = "Bit 23 - Recall Trim Code"]
    #[inline(always)]
    pub fn recall(&mut self) -> RecallW<'_, RPinCtrlSpec> {
        RecallW::new(self, 23)
    }
    #[doc = "Bit 24 - HEM Control"]
    #[inline(always)]
    pub fn hem(&mut self) -> HemW<'_, RPinCtrlSpec> {
        HemW::new(self, 24)
    }
}
#[doc = "BIST Pin Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r_pin_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r_pin_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RPinCtrlSpec;
impl crate::RegisterSpec for RPinCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r_pin_ctrl::R`](R) reader structure"]
impl crate::Readable for RPinCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`r_pin_ctrl::W`](W) writer structure"]
impl crate::Writable for RPinCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R_PIN_CTRL to value 0"]
impl crate::Resettable for RPinCtrlSpec {}
