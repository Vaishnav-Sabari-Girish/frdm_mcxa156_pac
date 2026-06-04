#[doc = "Register `AHBMATPRIO` reader"]
pub type R = crate::R<AhbmatprioSpec>;
#[doc = "Register `AHBMATPRIO` writer"]
pub type W = crate::W<AhbmatprioSpec>;
#[doc = "CPU0 C-AHB bus master priority level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cpu0Cbus {
    #[doc = "0: level 0"]
    Level0 = 0,
    #[doc = "1: level 1"]
    Level1 = 1,
    #[doc = "2: level 2"]
    Level2 = 2,
    #[doc = "3: level 3"]
    Level3 = 3,
}
impl From<Cpu0Cbus> for u8 {
    #[inline(always)]
    fn from(variant: Cpu0Cbus) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cpu0Cbus {
    type Ux = u8;
}
impl crate::IsEnum for Cpu0Cbus {}
#[doc = "Field `CPU0_CBUS` reader - CPU0 C-AHB bus master priority level"]
pub type Cpu0CbusR = crate::FieldReader<Cpu0Cbus>;
impl Cpu0CbusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpu0Cbus {
        match self.bits {
            0 => Cpu0Cbus::Level0,
            1 => Cpu0Cbus::Level1,
            2 => Cpu0Cbus::Level2,
            3 => Cpu0Cbus::Level3,
            _ => unreachable!(),
        }
    }
    #[doc = "level 0"]
    #[inline(always)]
    pub fn is_level0(&self) -> bool {
        *self == Cpu0Cbus::Level0
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn is_level1(&self) -> bool {
        *self == Cpu0Cbus::Level1
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn is_level2(&self) -> bool {
        *self == Cpu0Cbus::Level2
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_level3(&self) -> bool {
        *self == Cpu0Cbus::Level3
    }
}
#[doc = "Field `CPU0_CBUS` writer - CPU0 C-AHB bus master priority level"]
pub type Cpu0CbusW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cpu0Cbus, crate::Safe>;
impl<'a, REG> Cpu0CbusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "level 0"]
    #[inline(always)]
    pub fn level0(self) -> &'a mut crate::W<REG> {
        self.variant(Cpu0Cbus::Level0)
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn level1(self) -> &'a mut crate::W<REG> {
        self.variant(Cpu0Cbus::Level1)
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn level2(self) -> &'a mut crate::W<REG> {
        self.variant(Cpu0Cbus::Level2)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn level3(self) -> &'a mut crate::W<REG> {
        self.variant(Cpu0Cbus::Level3)
    }
}
#[doc = "CPU0 S-AHB bus master priority level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cpu0Sbus {
    #[doc = "0: level 0"]
    Level0 = 0,
    #[doc = "1: level 1"]
    Level1 = 1,
    #[doc = "2: level 2"]
    Level2 = 2,
    #[doc = "3: level 3"]
    Level3 = 3,
}
impl From<Cpu0Sbus> for u8 {
    #[inline(always)]
    fn from(variant: Cpu0Sbus) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cpu0Sbus {
    type Ux = u8;
}
impl crate::IsEnum for Cpu0Sbus {}
#[doc = "Field `CPU0_SBUS` reader - CPU0 S-AHB bus master priority level"]
pub type Cpu0SbusR = crate::FieldReader<Cpu0Sbus>;
impl Cpu0SbusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpu0Sbus {
        match self.bits {
            0 => Cpu0Sbus::Level0,
            1 => Cpu0Sbus::Level1,
            2 => Cpu0Sbus::Level2,
            3 => Cpu0Sbus::Level3,
            _ => unreachable!(),
        }
    }
    #[doc = "level 0"]
    #[inline(always)]
    pub fn is_level0(&self) -> bool {
        *self == Cpu0Sbus::Level0
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn is_level1(&self) -> bool {
        *self == Cpu0Sbus::Level1
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn is_level2(&self) -> bool {
        *self == Cpu0Sbus::Level2
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_level3(&self) -> bool {
        *self == Cpu0Sbus::Level3
    }
}
#[doc = "Field `CPU0_SBUS` writer - CPU0 S-AHB bus master priority level"]
pub type Cpu0SbusW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cpu0Sbus, crate::Safe>;
impl<'a, REG> Cpu0SbusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "level 0"]
    #[inline(always)]
    pub fn level0(self) -> &'a mut crate::W<REG> {
        self.variant(Cpu0Sbus::Level0)
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn level1(self) -> &'a mut crate::W<REG> {
        self.variant(Cpu0Sbus::Level1)
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn level2(self) -> &'a mut crate::W<REG> {
        self.variant(Cpu0Sbus::Level2)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn level3(self) -> &'a mut crate::W<REG> {
        self.variant(Cpu0Sbus::Level3)
    }
}
#[doc = "DMA0 controller bus master priority level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dma0 {
    #[doc = "0: level 0"]
    Level0 = 0,
    #[doc = "1: level 1"]
    Level1 = 1,
    #[doc = "2: level 2"]
    Level2 = 2,
    #[doc = "3: level 3"]
    Level3 = 3,
}
impl From<Dma0> for u8 {
    #[inline(always)]
    fn from(variant: Dma0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dma0 {
    type Ux = u8;
}
impl crate::IsEnum for Dma0 {}
#[doc = "Field `DMA0` reader - DMA0 controller bus master priority level"]
pub type Dma0R = crate::FieldReader<Dma0>;
impl Dma0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dma0 {
        match self.bits {
            0 => Dma0::Level0,
            1 => Dma0::Level1,
            2 => Dma0::Level2,
            3 => Dma0::Level3,
            _ => unreachable!(),
        }
    }
    #[doc = "level 0"]
    #[inline(always)]
    pub fn is_level0(&self) -> bool {
        *self == Dma0::Level0
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn is_level1(&self) -> bool {
        *self == Dma0::Level1
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn is_level2(&self) -> bool {
        *self == Dma0::Level2
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_level3(&self) -> bool {
        *self == Dma0::Level3
    }
}
#[doc = "Field `DMA0` writer - DMA0 controller bus master priority level"]
pub type Dma0W<'a, REG> = crate::FieldWriter<'a, REG, 2, Dma0, crate::Safe>;
impl<'a, REG> Dma0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "level 0"]
    #[inline(always)]
    pub fn level0(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0::Level0)
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn level1(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0::Level1)
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn level2(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0::Level2)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn level3(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0::Level3)
    }
}
#[doc = "USB-FS bus master priority level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UsbFsEnet {
    #[doc = "0: level 0"]
    Level0 = 0,
    #[doc = "1: level 1"]
    Level1 = 1,
    #[doc = "2: level 2"]
    Level2 = 2,
    #[doc = "3: level 3"]
    Level3 = 3,
}
impl From<UsbFsEnet> for u8 {
    #[inline(always)]
    fn from(variant: UsbFsEnet) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for UsbFsEnet {
    type Ux = u8;
}
impl crate::IsEnum for UsbFsEnet {}
#[doc = "Field `USB_FS_ENET` reader - USB-FS bus master priority level"]
pub type UsbFsEnetR = crate::FieldReader<UsbFsEnet>;
impl UsbFsEnetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UsbFsEnet {
        match self.bits {
            0 => UsbFsEnet::Level0,
            1 => UsbFsEnet::Level1,
            2 => UsbFsEnet::Level2,
            3 => UsbFsEnet::Level3,
            _ => unreachable!(),
        }
    }
    #[doc = "level 0"]
    #[inline(always)]
    pub fn is_level0(&self) -> bool {
        *self == UsbFsEnet::Level0
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn is_level1(&self) -> bool {
        *self == UsbFsEnet::Level1
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn is_level2(&self) -> bool {
        *self == UsbFsEnet::Level2
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_level3(&self) -> bool {
        *self == UsbFsEnet::Level3
    }
}
#[doc = "Field `USB_FS_ENET` writer - USB-FS bus master priority level"]
pub type UsbFsEnetW<'a, REG> = crate::FieldWriter<'a, REG, 2, UsbFsEnet, crate::Safe>;
impl<'a, REG> UsbFsEnetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "level 0"]
    #[inline(always)]
    pub fn level0(self) -> &'a mut crate::W<REG> {
        self.variant(UsbFsEnet::Level0)
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn level1(self) -> &'a mut crate::W<REG> {
        self.variant(UsbFsEnet::Level1)
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn level2(self) -> &'a mut crate::W<REG> {
        self.variant(UsbFsEnet::Level2)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn level3(self) -> &'a mut crate::W<REG> {
        self.variant(UsbFsEnet::Level3)
    }
}
impl R {
    #[doc = "Bits 0:1 - CPU0 C-AHB bus master priority level"]
    #[inline(always)]
    pub fn cpu0_cbus(&self) -> Cpu0CbusR {
        Cpu0CbusR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - CPU0 S-AHB bus master priority level"]
    #[inline(always)]
    pub fn cpu0_sbus(&self) -> Cpu0SbusR {
        Cpu0SbusR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 8:9 - DMA0 controller bus master priority level"]
    #[inline(always)]
    pub fn dma0(&self) -> Dma0R {
        Dma0R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 24:25 - USB-FS bus master priority level"]
    #[inline(always)]
    pub fn usb_fs_enet(&self) -> UsbFsEnetR {
        UsbFsEnetR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - CPU0 C-AHB bus master priority level"]
    #[inline(always)]
    pub fn cpu0_cbus(&mut self) -> Cpu0CbusW<'_, AhbmatprioSpec> {
        Cpu0CbusW::new(self, 0)
    }
    #[doc = "Bits 2:3 - CPU0 S-AHB bus master priority level"]
    #[inline(always)]
    pub fn cpu0_sbus(&mut self) -> Cpu0SbusW<'_, AhbmatprioSpec> {
        Cpu0SbusW::new(self, 2)
    }
    #[doc = "Bits 8:9 - DMA0 controller bus master priority level"]
    #[inline(always)]
    pub fn dma0(&mut self) -> Dma0W<'_, AhbmatprioSpec> {
        Dma0W::new(self, 8)
    }
    #[doc = "Bits 24:25 - USB-FS bus master priority level"]
    #[inline(always)]
    pub fn usb_fs_enet(&mut self) -> UsbFsEnetW<'_, AhbmatprioSpec> {
        UsbFsEnetW::new(self, 24)
    }
}
#[doc = "AHB Matrix Priority Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbmatprio::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbmatprio::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AhbmatprioSpec;
impl crate::RegisterSpec for AhbmatprioSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbmatprio::R`](R) reader structure"]
impl crate::Readable for AhbmatprioSpec {}
#[doc = "`write(|w| ..)` method takes [`ahbmatprio::W`](W) writer structure"]
impl crate::Writable for AhbmatprioSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHBMATPRIO to value 0"]
impl crate::Resettable for AhbmatprioSpec {}
