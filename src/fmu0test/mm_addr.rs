#[doc = "Register `MM_ADDR` reader"]
pub type R = crate::R<MmAddrSpec>;
#[doc = "Register `MM_ADDR` writer"]
pub type W = crate::W<MmAddrSpec>;
#[doc = "Field `MM_ADDR` reader - Memory Map Address"]
pub type MmAddrR = crate::FieldReader<u32>;
#[doc = "Field `MM_ADDR` writer - Memory Map Address"]
pub type MmAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Memory Map Address"]
    #[inline(always)]
    pub fn mm_addr(&self) -> MmAddrR {
        MmAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Memory Map Address"]
    #[inline(always)]
    pub fn mm_addr(&mut self) -> MmAddrW<'_, MmAddrSpec> {
        MmAddrW::new(self, 0)
    }
}
#[doc = "Memory Map Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mm_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mm_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmAddrSpec;
impl crate::RegisterSpec for MmAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mm_addr::R`](R) reader structure"]
impl crate::Readable for MmAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`mm_addr::W`](W) writer structure"]
impl crate::Writable for MmAddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MM_ADDR to value 0"]
impl crate::Resettable for MmAddrSpec {}
