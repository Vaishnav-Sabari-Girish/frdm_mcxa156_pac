#[doc = "Register `OPAMP0_TRIG` reader"]
pub type R = crate::R<Opamp0TrigSpec>;
#[doc = "Register `OPAMP0_TRIG` writer"]
pub type W = crate::W<Opamp0TrigSpec>;
#[doc = "DAC0 trigger input\n\nValue on reset: 63"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Inp {
    #[doc = "2: AOI0_OUT0 input is selected"]
    Val2 = 2,
    #[doc = "3: AOI0_OUT1 input is selected"]
    Val3 = 3,
    #[doc = "4: AOI0_OUT2 input is selected"]
    Val4 = 4,
    #[doc = "5: AOI0_OUT3 input is selected"]
    Val5 = 5,
    #[doc = "6: CMP0_OUT input is selected"]
    Val6 = 6,
    #[doc = "7: CMP1_OUT input is selected"]
    Val7 = 7,
    #[doc = "9: CTimer0_MAT0 input is selected"]
    Val9 = 9,
    #[doc = "10: CTimer0_MAT1 input is selected"]
    Val10 = 10,
    #[doc = "11: CTimer1_MAT0 input is selected"]
    Val11 = 11,
    #[doc = "12: CTimer1_MAT1 input is selected"]
    Val12 = 12,
    #[doc = "13: CTimer2_MAT0 input is selected"]
    Val13 = 13,
    #[doc = "14: CTimer2_MAT1 input is selected"]
    Val14 = 14,
    #[doc = "15: LPTMR0 input is selected"]
    Val15 = 15,
    #[doc = "18: PWM0_SM0_MUX_TRIG0 input is selected"]
    Val18 = 18,
    #[doc = "19: PWM0_SM0_MUX_TRIG1 input is selected"]
    Val19 = 19,
    #[doc = "20: PWM0_SM1_MUX_TRIG0 input is selected"]
    Val20 = 20,
    #[doc = "21: PWM0_SM1_MUX_TRIG1 input is selected"]
    Val21 = 21,
    #[doc = "22: PWM0_SM2_MUX_TRIG0 input is selected"]
    Val22 = 22,
    #[doc = "23: PWM0_SM2_MUX_TRIG1 input is selected"]
    Val23 = 23,
    #[doc = "26: GPIO0 Pin Event Trig 0 input is selected"]
    Val26 = 26,
    #[doc = "27: GPIO1 Pin Event Trig 0 input is selected"]
    Val27 = 27,
    #[doc = "28: GPIO2 Pin Event Trig 0 input is selected"]
    Val28 = 28,
    #[doc = "29: GPIO3 Pin Event Trig 0 input is selected"]
    Val29 = 29,
    #[doc = "30: GPIO4 Pin Event Trig 0 input is selected"]
    Val30 = 30,
    #[doc = "31: WUU input is selected"]
    Val31 = 31,
    #[doc = "33: AOI1_OUT0 input is selected"]
    Val33 = 33,
    #[doc = "34: AOI1_OUT1 input is selected"]
    Val34 = 34,
    #[doc = "35: AOI1_OUT2 input is selected"]
    Val35 = 35,
    #[doc = "36: AOI1_OUT3 input is selected"]
    Val36 = 36,
    #[doc = "37: ADC0_tcomp\\[0\\] input is selected"]
    Val37 = 37,
    #[doc = "38: ADC0_tcomp\\[1\\] input is selected"]
    Val38 = 38,
    #[doc = "39: ADC1_tcomp\\[0\\] input is selected"]
    Val39 = 39,
    #[doc = "40: ADC1_tcomp\\[1\\] input is selected"]
    Val40 = 40,
    #[doc = "41: CTimer3_MAT0 input is selected"]
    Val41 = 41,
    #[doc = "42: CTimer3_MAT1 input is selected"]
    Val42 = 42,
    #[doc = "43: CTimer4_MAT0 input is selected"]
    Val43 = 43,
    #[doc = "44: CTimer4_MAT1 input is selected"]
    Val44 = 44,
    #[doc = "45: FlexIO CH0 input is selected"]
    Val45 = 45,
    #[doc = "46: FlexIO CH1 input is selected"]
    Val46 = 46,
    #[doc = "47: FlexIO CH2 input is selected"]
    Val47 = 47,
    #[doc = "48: FlexIO CH3 input is selected"]
    Val48 = 48,
    #[doc = "50: PWM1_SM0_MUX_TRIG0 input is selected"]
    Val50 = 50,
    #[doc = "51: PWM1_SM0_MUX_TRIG1 input is selected"]
    Val51 = 51,
    #[doc = "52: PWM1_SM1_MUX_TRIG0 input is selected"]
    Val52 = 52,
    #[doc = "53: PWM1_SM1_MUX_TRIG1 input is selected"]
    Val53 = 53,
    #[doc = "54: PWM1_SM2_MUX_TRIG0 input is selected"]
    Val54 = 54,
    #[doc = "55: PWM1_SM2_MUX_TRIG1 input is selected"]
    Val55 = 55,
}
impl From<Inp> for u8 {
    #[inline(always)]
    fn from(variant: Inp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Inp {
    type Ux = u8;
}
impl crate::IsEnum for Inp {}
#[doc = "Field `INP` reader - DAC0 trigger input"]
pub type InpR = crate::FieldReader<Inp>;
impl InpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Inp> {
        match self.bits {
            2 => Some(Inp::Val2),
            3 => Some(Inp::Val3),
            4 => Some(Inp::Val4),
            5 => Some(Inp::Val5),
            6 => Some(Inp::Val6),
            7 => Some(Inp::Val7),
            9 => Some(Inp::Val9),
            10 => Some(Inp::Val10),
            11 => Some(Inp::Val11),
            12 => Some(Inp::Val12),
            13 => Some(Inp::Val13),
            14 => Some(Inp::Val14),
            15 => Some(Inp::Val15),
            18 => Some(Inp::Val18),
            19 => Some(Inp::Val19),
            20 => Some(Inp::Val20),
            21 => Some(Inp::Val21),
            22 => Some(Inp::Val22),
            23 => Some(Inp::Val23),
            26 => Some(Inp::Val26),
            27 => Some(Inp::Val27),
            28 => Some(Inp::Val28),
            29 => Some(Inp::Val29),
            30 => Some(Inp::Val30),
            31 => Some(Inp::Val31),
            33 => Some(Inp::Val33),
            34 => Some(Inp::Val34),
            35 => Some(Inp::Val35),
            36 => Some(Inp::Val36),
            37 => Some(Inp::Val37),
            38 => Some(Inp::Val38),
            39 => Some(Inp::Val39),
            40 => Some(Inp::Val40),
            41 => Some(Inp::Val41),
            42 => Some(Inp::Val42),
            43 => Some(Inp::Val43),
            44 => Some(Inp::Val44),
            45 => Some(Inp::Val45),
            46 => Some(Inp::Val46),
            47 => Some(Inp::Val47),
            48 => Some(Inp::Val48),
            50 => Some(Inp::Val50),
            51 => Some(Inp::Val51),
            52 => Some(Inp::Val52),
            53 => Some(Inp::Val53),
            54 => Some(Inp::Val54),
            55 => Some(Inp::Val55),
            _ => None,
        }
    }
    #[doc = "AOI0_OUT0 input is selected"]
    #[inline(always)]
    pub fn is_val2(&self) -> bool {
        *self == Inp::Val2
    }
    #[doc = "AOI0_OUT1 input is selected"]
    #[inline(always)]
    pub fn is_val3(&self) -> bool {
        *self == Inp::Val3
    }
    #[doc = "AOI0_OUT2 input is selected"]
    #[inline(always)]
    pub fn is_val4(&self) -> bool {
        *self == Inp::Val4
    }
    #[doc = "AOI0_OUT3 input is selected"]
    #[inline(always)]
    pub fn is_val5(&self) -> bool {
        *self == Inp::Val5
    }
    #[doc = "CMP0_OUT input is selected"]
    #[inline(always)]
    pub fn is_val6(&self) -> bool {
        *self == Inp::Val6
    }
    #[doc = "CMP1_OUT input is selected"]
    #[inline(always)]
    pub fn is_val7(&self) -> bool {
        *self == Inp::Val7
    }
    #[doc = "CTimer0_MAT0 input is selected"]
    #[inline(always)]
    pub fn is_val9(&self) -> bool {
        *self == Inp::Val9
    }
    #[doc = "CTimer0_MAT1 input is selected"]
    #[inline(always)]
    pub fn is_val10(&self) -> bool {
        *self == Inp::Val10
    }
    #[doc = "CTimer1_MAT0 input is selected"]
    #[inline(always)]
    pub fn is_val11(&self) -> bool {
        *self == Inp::Val11
    }
    #[doc = "CTimer1_MAT1 input is selected"]
    #[inline(always)]
    pub fn is_val12(&self) -> bool {
        *self == Inp::Val12
    }
    #[doc = "CTimer2_MAT0 input is selected"]
    #[inline(always)]
    pub fn is_val13(&self) -> bool {
        *self == Inp::Val13
    }
    #[doc = "CTimer2_MAT1 input is selected"]
    #[inline(always)]
    pub fn is_val14(&self) -> bool {
        *self == Inp::Val14
    }
    #[doc = "LPTMR0 input is selected"]
    #[inline(always)]
    pub fn is_val15(&self) -> bool {
        *self == Inp::Val15
    }
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected"]
    #[inline(always)]
    pub fn is_val18(&self) -> bool {
        *self == Inp::Val18
    }
    #[doc = "PWM0_SM0_MUX_TRIG1 input is selected"]
    #[inline(always)]
    pub fn is_val19(&self) -> bool {
        *self == Inp::Val19
    }
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected"]
    #[inline(always)]
    pub fn is_val20(&self) -> bool {
        *self == Inp::Val20
    }
    #[doc = "PWM0_SM1_MUX_TRIG1 input is selected"]
    #[inline(always)]
    pub fn is_val21(&self) -> bool {
        *self == Inp::Val21
    }
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected"]
    #[inline(always)]
    pub fn is_val22(&self) -> bool {
        *self == Inp::Val22
    }
    #[doc = "PWM0_SM2_MUX_TRIG1 input is selected"]
    #[inline(always)]
    pub fn is_val23(&self) -> bool {
        *self == Inp::Val23
    }
    #[doc = "GPIO0 Pin Event Trig 0 input is selected"]
    #[inline(always)]
    pub fn is_val26(&self) -> bool {
        *self == Inp::Val26
    }
    #[doc = "GPIO1 Pin Event Trig 0 input is selected"]
    #[inline(always)]
    pub fn is_val27(&self) -> bool {
        *self == Inp::Val27
    }
    #[doc = "GPIO2 Pin Event Trig 0 input is selected"]
    #[inline(always)]
    pub fn is_val28(&self) -> bool {
        *self == Inp::Val28
    }
    #[doc = "GPIO3 Pin Event Trig 0 input is selected"]
    #[inline(always)]
    pub fn is_val29(&self) -> bool {
        *self == Inp::Val29
    }
    #[doc = "GPIO4 Pin Event Trig 0 input is selected"]
    #[inline(always)]
    pub fn is_val30(&self) -> bool {
        *self == Inp::Val30
    }
    #[doc = "WUU input is selected"]
    #[inline(always)]
    pub fn is_val31(&self) -> bool {
        *self == Inp::Val31
    }
    #[doc = "AOI1_OUT0 input is selected"]
    #[inline(always)]
    pub fn is_val33(&self) -> bool {
        *self == Inp::Val33
    }
    #[doc = "AOI1_OUT1 input is selected"]
    #[inline(always)]
    pub fn is_val34(&self) -> bool {
        *self == Inp::Val34
    }
    #[doc = "AOI1_OUT2 input is selected"]
    #[inline(always)]
    pub fn is_val35(&self) -> bool {
        *self == Inp::Val35
    }
    #[doc = "AOI1_OUT3 input is selected"]
    #[inline(always)]
    pub fn is_val36(&self) -> bool {
        *self == Inp::Val36
    }
    #[doc = "ADC0_tcomp\\[0\\] input is selected"]
    #[inline(always)]
    pub fn is_val37(&self) -> bool {
        *self == Inp::Val37
    }
    #[doc = "ADC0_tcomp\\[1\\] input is selected"]
    #[inline(always)]
    pub fn is_val38(&self) -> bool {
        *self == Inp::Val38
    }
    #[doc = "ADC1_tcomp\\[0\\] input is selected"]
    #[inline(always)]
    pub fn is_val39(&self) -> bool {
        *self == Inp::Val39
    }
    #[doc = "ADC1_tcomp\\[1\\] input is selected"]
    #[inline(always)]
    pub fn is_val40(&self) -> bool {
        *self == Inp::Val40
    }
    #[doc = "CTimer3_MAT0 input is selected"]
    #[inline(always)]
    pub fn is_val41(&self) -> bool {
        *self == Inp::Val41
    }
    #[doc = "CTimer3_MAT1 input is selected"]
    #[inline(always)]
    pub fn is_val42(&self) -> bool {
        *self == Inp::Val42
    }
    #[doc = "CTimer4_MAT0 input is selected"]
    #[inline(always)]
    pub fn is_val43(&self) -> bool {
        *self == Inp::Val43
    }
    #[doc = "CTimer4_MAT1 input is selected"]
    #[inline(always)]
    pub fn is_val44(&self) -> bool {
        *self == Inp::Val44
    }
    #[doc = "FlexIO CH0 input is selected"]
    #[inline(always)]
    pub fn is_val45(&self) -> bool {
        *self == Inp::Val45
    }
    #[doc = "FlexIO CH1 input is selected"]
    #[inline(always)]
    pub fn is_val46(&self) -> bool {
        *self == Inp::Val46
    }
    #[doc = "FlexIO CH2 input is selected"]
    #[inline(always)]
    pub fn is_val47(&self) -> bool {
        *self == Inp::Val47
    }
    #[doc = "FlexIO CH3 input is selected"]
    #[inline(always)]
    pub fn is_val48(&self) -> bool {
        *self == Inp::Val48
    }
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected"]
    #[inline(always)]
    pub fn is_val50(&self) -> bool {
        *self == Inp::Val50
    }
    #[doc = "PWM1_SM0_MUX_TRIG1 input is selected"]
    #[inline(always)]
    pub fn is_val51(&self) -> bool {
        *self == Inp::Val51
    }
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected"]
    #[inline(always)]
    pub fn is_val52(&self) -> bool {
        *self == Inp::Val52
    }
    #[doc = "PWM1_SM1_MUX_TRIG1 input is selected"]
    #[inline(always)]
    pub fn is_val53(&self) -> bool {
        *self == Inp::Val53
    }
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected"]
    #[inline(always)]
    pub fn is_val54(&self) -> bool {
        *self == Inp::Val54
    }
    #[doc = "PWM1_SM2_MUX_TRIG1 input is selected"]
    #[inline(always)]
    pub fn is_val55(&self) -> bool {
        *self == Inp::Val55
    }
}
#[doc = "Field `INP` writer - DAC0 trigger input"]
pub type InpW<'a, REG> = crate::FieldWriter<'a, REG, 6, Inp>;
impl<'a, REG> InpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "AOI0_OUT0 input is selected"]
    #[inline(always)]
    pub fn val2(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val2)
    }
    #[doc = "AOI0_OUT1 input is selected"]
    #[inline(always)]
    pub fn val3(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val3)
    }
    #[doc = "AOI0_OUT2 input is selected"]
    #[inline(always)]
    pub fn val4(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val4)
    }
    #[doc = "AOI0_OUT3 input is selected"]
    #[inline(always)]
    pub fn val5(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val5)
    }
    #[doc = "CMP0_OUT input is selected"]
    #[inline(always)]
    pub fn val6(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val6)
    }
    #[doc = "CMP1_OUT input is selected"]
    #[inline(always)]
    pub fn val7(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val7)
    }
    #[doc = "CTimer0_MAT0 input is selected"]
    #[inline(always)]
    pub fn val9(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val9)
    }
    #[doc = "CTimer0_MAT1 input is selected"]
    #[inline(always)]
    pub fn val10(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val10)
    }
    #[doc = "CTimer1_MAT0 input is selected"]
    #[inline(always)]
    pub fn val11(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val11)
    }
    #[doc = "CTimer1_MAT1 input is selected"]
    #[inline(always)]
    pub fn val12(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val12)
    }
    #[doc = "CTimer2_MAT0 input is selected"]
    #[inline(always)]
    pub fn val13(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val13)
    }
    #[doc = "CTimer2_MAT1 input is selected"]
    #[inline(always)]
    pub fn val14(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val14)
    }
    #[doc = "LPTMR0 input is selected"]
    #[inline(always)]
    pub fn val15(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val15)
    }
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected"]
    #[inline(always)]
    pub fn val18(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val18)
    }
    #[doc = "PWM0_SM0_MUX_TRIG1 input is selected"]
    #[inline(always)]
    pub fn val19(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val19)
    }
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected"]
    #[inline(always)]
    pub fn val20(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val20)
    }
    #[doc = "PWM0_SM1_MUX_TRIG1 input is selected"]
    #[inline(always)]
    pub fn val21(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val21)
    }
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected"]
    #[inline(always)]
    pub fn val22(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val22)
    }
    #[doc = "PWM0_SM2_MUX_TRIG1 input is selected"]
    #[inline(always)]
    pub fn val23(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val23)
    }
    #[doc = "GPIO0 Pin Event Trig 0 input is selected"]
    #[inline(always)]
    pub fn val26(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val26)
    }
    #[doc = "GPIO1 Pin Event Trig 0 input is selected"]
    #[inline(always)]
    pub fn val27(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val27)
    }
    #[doc = "GPIO2 Pin Event Trig 0 input is selected"]
    #[inline(always)]
    pub fn val28(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val28)
    }
    #[doc = "GPIO3 Pin Event Trig 0 input is selected"]
    #[inline(always)]
    pub fn val29(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val29)
    }
    #[doc = "GPIO4 Pin Event Trig 0 input is selected"]
    #[inline(always)]
    pub fn val30(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val30)
    }
    #[doc = "WUU input is selected"]
    #[inline(always)]
    pub fn val31(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val31)
    }
    #[doc = "AOI1_OUT0 input is selected"]
    #[inline(always)]
    pub fn val33(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val33)
    }
    #[doc = "AOI1_OUT1 input is selected"]
    #[inline(always)]
    pub fn val34(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val34)
    }
    #[doc = "AOI1_OUT2 input is selected"]
    #[inline(always)]
    pub fn val35(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val35)
    }
    #[doc = "AOI1_OUT3 input is selected"]
    #[inline(always)]
    pub fn val36(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val36)
    }
    #[doc = "ADC0_tcomp\\[0\\] input is selected"]
    #[inline(always)]
    pub fn val37(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val37)
    }
    #[doc = "ADC0_tcomp\\[1\\] input is selected"]
    #[inline(always)]
    pub fn val38(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val38)
    }
    #[doc = "ADC1_tcomp\\[0\\] input is selected"]
    #[inline(always)]
    pub fn val39(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val39)
    }
    #[doc = "ADC1_tcomp\\[1\\] input is selected"]
    #[inline(always)]
    pub fn val40(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val40)
    }
    #[doc = "CTimer3_MAT0 input is selected"]
    #[inline(always)]
    pub fn val41(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val41)
    }
    #[doc = "CTimer3_MAT1 input is selected"]
    #[inline(always)]
    pub fn val42(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val42)
    }
    #[doc = "CTimer4_MAT0 input is selected"]
    #[inline(always)]
    pub fn val43(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val43)
    }
    #[doc = "CTimer4_MAT1 input is selected"]
    #[inline(always)]
    pub fn val44(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val44)
    }
    #[doc = "FlexIO CH0 input is selected"]
    #[inline(always)]
    pub fn val45(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val45)
    }
    #[doc = "FlexIO CH1 input is selected"]
    #[inline(always)]
    pub fn val46(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val46)
    }
    #[doc = "FlexIO CH2 input is selected"]
    #[inline(always)]
    pub fn val47(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val47)
    }
    #[doc = "FlexIO CH3 input is selected"]
    #[inline(always)]
    pub fn val48(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val48)
    }
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected"]
    #[inline(always)]
    pub fn val50(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val50)
    }
    #[doc = "PWM1_SM0_MUX_TRIG1 input is selected"]
    #[inline(always)]
    pub fn val51(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val51)
    }
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected"]
    #[inline(always)]
    pub fn val52(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val52)
    }
    #[doc = "PWM1_SM1_MUX_TRIG1 input is selected"]
    #[inline(always)]
    pub fn val53(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val53)
    }
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected"]
    #[inline(always)]
    pub fn val54(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val54)
    }
    #[doc = "PWM1_SM2_MUX_TRIG1 input is selected"]
    #[inline(always)]
    pub fn val55(self) -> &'a mut crate::W<REG> {
        self.variant(Inp::Val55)
    }
}
impl R {
    #[doc = "Bits 0:5 - DAC0 trigger input"]
    #[inline(always)]
    pub fn inp(&self) -> InpR {
        InpR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - DAC0 trigger input"]
    #[inline(always)]
    pub fn inp(&mut self) -> InpW<'_, Opamp0TrigSpec> {
        InpW::new(self, 0)
    }
}
#[doc = "OPAMP0 Trigger Input Connections\n\nYou can [`read`](crate::Reg::read) this register and get [`opamp0_trig::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp0_trig::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Opamp0TrigSpec;
impl crate::RegisterSpec for Opamp0TrigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`opamp0_trig::R`](R) reader structure"]
impl crate::Readable for Opamp0TrigSpec {}
#[doc = "`write(|w| ..)` method takes [`opamp0_trig::W`](W) writer structure"]
impl crate::Writable for Opamp0TrigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OPAMP0_TRIG to value 0x3f"]
impl crate::Resettable for Opamp0TrigSpec {
    const RESET_VALUE: u32 = 0x3f;
}
