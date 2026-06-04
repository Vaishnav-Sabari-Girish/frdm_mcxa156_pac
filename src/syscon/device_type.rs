#[doc = "Register `DEVICE_TYPE` reader"]
pub type R = crate::R<DeviceTypeSpec>;
#[doc = "Field `DEVICE_TYPE` reader - Indicates DEVICE TYPE."]
pub type DeviceTypeR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Indicates DEVICE TYPE."]
    #[inline(always)]
    pub fn device_type(&self) -> DeviceTypeR {
        DeviceTypeR::new(self.bits)
    }
}
#[doc = "Device Type\n\nYou can [`read`](crate::Reg::read) this register and get [`device_type::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DeviceTypeSpec;
impl crate::RegisterSpec for DeviceTypeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`device_type::R`](R) reader structure"]
impl crate::Readable for DeviceTypeSpec {}
#[doc = "`reset()` method sets DEVICE_TYPE to value 0x2000"]
impl crate::Resettable for DeviceTypeSpec {
    const RESET_VALUE: u32 = 0x2000;
}
