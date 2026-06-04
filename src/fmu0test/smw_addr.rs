#[doc = "Register `SMW_ADDR` reader"]
pub type R = crate::R<SmwAddrSpec>;
#[doc = "Register `SMW_ADDR` writer"]
pub type W = crate::W<SmwAddrSpec>;
#[doc = "Field `SMW_ADDR` reader - SMW Address"]
pub type SmwAddrR = crate::FieldReader<u32>;
#[doc = "Field `SMW_ADDR` writer - SMW Address"]
pub type SmwAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SMW Address"]
    #[inline(always)]
    pub fn smw_addr(&self) -> SmwAddrR {
        SmwAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SMW Address"]
    #[inline(always)]
    pub fn smw_addr(&mut self) -> SmwAddrW<'_, SmwAddrSpec> {
        SmwAddrW::new(self, 0)
    }
}
#[doc = "SMW Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`smw_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smw_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmwAddrSpec;
impl crate::RegisterSpec for SmwAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smw_addr::R`](R) reader structure"]
impl crate::Readable for SmwAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`smw_addr::W`](W) writer structure"]
impl crate::Writable for SmwAddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SMW_ADDR to value 0"]
impl crate::Resettable for SmwAddrSpec {}
