#[doc = "Register `R_TESTCODE` reader"]
pub type R = crate::R<RTestcodeSpec>;
#[doc = "Register `R_TESTCODE` writer"]
pub type W = crate::W<RTestcodeSpec>;
#[doc = "Field `TESTCODE` reader - Used to store test code information before running TMR-RST/TMRSET BIST command"]
pub type TestcodeR = crate::FieldReader;
#[doc = "Field `TESTCODE` writer - Used to store test code information before running TMR-RST/TMRSET BIST command"]
pub type TestcodeW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Used to store test code information before running TMR-RST/TMRSET BIST command"]
    #[inline(always)]
    pub fn testcode(&self) -> TestcodeR {
        TestcodeR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Used to store test code information before running TMR-RST/TMRSET BIST command"]
    #[inline(always)]
    pub fn testcode(&mut self) -> TestcodeW<'_, RTestcodeSpec> {
        TestcodeW::new(self, 0)
    }
}
#[doc = "BIST Test Code Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r_testcode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r_testcode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTestcodeSpec;
impl crate::RegisterSpec for RTestcodeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r_testcode::R`](R) reader structure"]
impl crate::Readable for RTestcodeSpec {}
#[doc = "`write(|w| ..)` method takes [`r_testcode::W`](W) writer structure"]
impl crate::Writable for RTestcodeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R_TESTCODE to value 0"]
impl crate::Resettable for RTestcodeSpec {}
