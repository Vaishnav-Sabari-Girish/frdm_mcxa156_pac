#[doc = "Register `MSIZE` reader"]
pub type R = crate::R<MsizeSpec>;
#[doc = "Register `MSIZE` writer"]
pub type W = crate::W<MsizeSpec>;
#[doc = "Field `MAXADDR0` reader - Size of Flash Block 0"]
pub type Maxaddr0R = crate::FieldReader;
#[doc = "Field `MAXADDR0` writer - Size of Flash Block 0"]
pub type Maxaddr0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Size of Flash Block 0"]
    #[inline(always)]
    pub fn maxaddr0(&self) -> Maxaddr0R {
        Maxaddr0R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Size of Flash Block 0"]
    #[inline(always)]
    pub fn maxaddr0(&mut self) -> Maxaddr0W<'_, MsizeSpec> {
        Maxaddr0W::new(self, 0)
    }
}
#[doc = "FMU Memory Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`msize::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`msize::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MsizeSpec;
impl crate::RegisterSpec for MsizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msize::R`](R) reader structure"]
impl crate::Readable for MsizeSpec {}
#[doc = "`write(|w| ..)` method takes [`msize::W`](W) writer structure"]
impl crate::Writable for MsizeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MSIZE to value 0x20"]
impl crate::Resettable for MsizeSpec {
    const RESET_VALUE: u32 = 0x20;
}
