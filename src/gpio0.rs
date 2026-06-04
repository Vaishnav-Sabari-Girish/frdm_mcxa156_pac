#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    verid: Verid,
    param: Param,
    _reserved2: [u8; 0x38],
    pdor: Pdor,
    psor: Psor,
    pcor: Pcor,
    ptor: Ptor,
    pdir: Pdir,
    pddr: Pddr,
    pidr: Pidr,
    _reserved9: [u8; 0x04],
    pdr: [Pdr; 32],
    icr0: Icr0,
    icr1: Icr1,
    icr2: Icr2,
    icr3: Icr3,
    icr4: Icr4,
    icr5: Icr5,
    icr6: Icr6,
    icr7: Icr7,
    icr8: Icr8,
    icr9: Icr9,
    icr10: Icr10,
    icr11: Icr11,
    icr12: Icr12,
    icr13: Icr13,
    icr14: Icr14,
    icr15: Icr15,
    icr16: Icr16,
    icr17: Icr17,
    icr18: Icr18,
    icr19: Icr19,
    icr20: Icr20,
    icr21: Icr21,
    icr22: Icr22,
    icr23: Icr23,
    icr24: Icr24,
    icr25: Icr25,
    icr26: Icr26,
    icr27: Icr27,
    icr28: Icr28,
    icr29: Icr29,
    icr30: Icr30,
    icr31: Icr31,
    giclr: Giclr,
    gichr: Gichr,
    _reserved44: [u8; 0x18],
    isfr0: Isfr0,
}
impl RegisterBlock {
    #[doc = "0x00 - Version ID"]
    #[inline(always)]
    pub const fn verid(&self) -> &Verid {
        &self.verid
    }
    #[doc = "0x04 - Parameter"]
    #[inline(always)]
    pub const fn param(&self) -> &Param {
        &self.param
    }
    #[doc = "0x40 - Port Data Output"]
    #[inline(always)]
    pub const fn pdor(&self) -> &Pdor {
        &self.pdor
    }
    #[doc = "0x44 - Port Set Output"]
    #[inline(always)]
    pub const fn psor(&self) -> &Psor {
        &self.psor
    }
    #[doc = "0x48 - Port Clear Output"]
    #[inline(always)]
    pub const fn pcor(&self) -> &Pcor {
        &self.pcor
    }
    #[doc = "0x4c - Port Toggle Output"]
    #[inline(always)]
    pub const fn ptor(&self) -> &Ptor {
        &self.ptor
    }
    #[doc = "0x50 - Port Data Input"]
    #[inline(always)]
    pub const fn pdir(&self) -> &Pdir {
        &self.pdir
    }
    #[doc = "0x54 - Port Data Direction"]
    #[inline(always)]
    pub const fn pddr(&self) -> &Pddr {
        &self.pddr
    }
    #[doc = "0x58 - Port Input Disable"]
    #[inline(always)]
    pub const fn pidr(&self) -> &Pidr {
        &self.pidr
    }
    #[doc = "0x60..0x80 - Pin Data"]
    #[inline(always)]
    pub const fn pdr(&self, n: usize) -> &Pdr {
        &self.pdr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x60..0x80 - Pin Data"]
    #[inline(always)]
    pub fn pdr_iter(&self) -> impl Iterator<Item = &Pdr> {
        self.pdr.iter()
    }
    #[doc = "0x80 - Interrupt Control 0"]
    #[inline(always)]
    pub const fn icr0(&self) -> &Icr0 {
        &self.icr0
    }
    #[doc = "0x84 - Interrupt Control 1"]
    #[inline(always)]
    pub const fn icr1(&self) -> &Icr1 {
        &self.icr1
    }
    #[doc = "0x88 - Interrupt Control 2"]
    #[inline(always)]
    pub const fn icr2(&self) -> &Icr2 {
        &self.icr2
    }
    #[doc = "0x8c - Interrupt Control 3"]
    #[inline(always)]
    pub const fn icr3(&self) -> &Icr3 {
        &self.icr3
    }
    #[doc = "0x90 - Interrupt Control 4"]
    #[inline(always)]
    pub const fn icr4(&self) -> &Icr4 {
        &self.icr4
    }
    #[doc = "0x94 - Interrupt Control 5"]
    #[inline(always)]
    pub const fn icr5(&self) -> &Icr5 {
        &self.icr5
    }
    #[doc = "0x98 - Interrupt Control 6"]
    #[inline(always)]
    pub const fn icr6(&self) -> &Icr6 {
        &self.icr6
    }
    #[doc = "0x9c - Interrupt Control 7"]
    #[inline(always)]
    pub const fn icr7(&self) -> &Icr7 {
        &self.icr7
    }
    #[doc = "0xa0 - Interrupt Control 8"]
    #[inline(always)]
    pub const fn icr8(&self) -> &Icr8 {
        &self.icr8
    }
    #[doc = "0xa4 - Interrupt Control 9"]
    #[inline(always)]
    pub const fn icr9(&self) -> &Icr9 {
        &self.icr9
    }
    #[doc = "0xa8 - Interrupt Control 10"]
    #[inline(always)]
    pub const fn icr10(&self) -> &Icr10 {
        &self.icr10
    }
    #[doc = "0xac - Interrupt Control 11"]
    #[inline(always)]
    pub const fn icr11(&self) -> &Icr11 {
        &self.icr11
    }
    #[doc = "0xb0 - Interrupt Control 12"]
    #[inline(always)]
    pub const fn icr12(&self) -> &Icr12 {
        &self.icr12
    }
    #[doc = "0xb4 - Interrupt Control 13"]
    #[inline(always)]
    pub const fn icr13(&self) -> &Icr13 {
        &self.icr13
    }
    #[doc = "0xb8 - Interrupt Control 14"]
    #[inline(always)]
    pub const fn icr14(&self) -> &Icr14 {
        &self.icr14
    }
    #[doc = "0xbc - Interrupt Control 15"]
    #[inline(always)]
    pub const fn icr15(&self) -> &Icr15 {
        &self.icr15
    }
    #[doc = "0xc0 - Interrupt Control 16"]
    #[inline(always)]
    pub const fn icr16(&self) -> &Icr16 {
        &self.icr16
    }
    #[doc = "0xc4 - Interrupt Control 17"]
    #[inline(always)]
    pub const fn icr17(&self) -> &Icr17 {
        &self.icr17
    }
    #[doc = "0xc8 - Interrupt Control 18"]
    #[inline(always)]
    pub const fn icr18(&self) -> &Icr18 {
        &self.icr18
    }
    #[doc = "0xcc - Interrupt Control 19"]
    #[inline(always)]
    pub const fn icr19(&self) -> &Icr19 {
        &self.icr19
    }
    #[doc = "0xd0 - Interrupt Control 20"]
    #[inline(always)]
    pub const fn icr20(&self) -> &Icr20 {
        &self.icr20
    }
    #[doc = "0xd4 - Interrupt Control 21"]
    #[inline(always)]
    pub const fn icr21(&self) -> &Icr21 {
        &self.icr21
    }
    #[doc = "0xd8 - Interrupt Control 22"]
    #[inline(always)]
    pub const fn icr22(&self) -> &Icr22 {
        &self.icr22
    }
    #[doc = "0xdc - Interrupt Control 23"]
    #[inline(always)]
    pub const fn icr23(&self) -> &Icr23 {
        &self.icr23
    }
    #[doc = "0xe0 - Interrupt Control 24"]
    #[inline(always)]
    pub const fn icr24(&self) -> &Icr24 {
        &self.icr24
    }
    #[doc = "0xe4 - Interrupt Control 25"]
    #[inline(always)]
    pub const fn icr25(&self) -> &Icr25 {
        &self.icr25
    }
    #[doc = "0xe8 - Interrupt Control 26"]
    #[inline(always)]
    pub const fn icr26(&self) -> &Icr26 {
        &self.icr26
    }
    #[doc = "0xec - Interrupt Control 27"]
    #[inline(always)]
    pub const fn icr27(&self) -> &Icr27 {
        &self.icr27
    }
    #[doc = "0xf0 - Interrupt Control 28"]
    #[inline(always)]
    pub const fn icr28(&self) -> &Icr28 {
        &self.icr28
    }
    #[doc = "0xf4 - Interrupt Control 29"]
    #[inline(always)]
    pub const fn icr29(&self) -> &Icr29 {
        &self.icr29
    }
    #[doc = "0xf8 - Interrupt Control 30"]
    #[inline(always)]
    pub const fn icr30(&self) -> &Icr30 {
        &self.icr30
    }
    #[doc = "0xfc - Interrupt Control 31"]
    #[inline(always)]
    pub const fn icr31(&self) -> &Icr31 {
        &self.icr31
    }
    #[doc = "0x100 - Global Interrupt Control Low"]
    #[inline(always)]
    pub const fn giclr(&self) -> &Giclr {
        &self.giclr
    }
    #[doc = "0x104 - Global Interrupt Control High"]
    #[inline(always)]
    pub const fn gichr(&self) -> &Gichr {
        &self.gichr
    }
    #[doc = "0x120 - Interrupt Status Flag"]
    #[inline(always)]
    pub const fn isfr0(&self) -> &Isfr0 {
        &self.isfr0
    }
}
#[doc = "VERID (r) register accessor: Version ID\n\nYou can [`read`](crate::Reg::read) this register and get [`verid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@verid`] module"]
#[doc(alias = "VERID")]
pub type Verid = crate::Reg<verid::VeridSpec>;
#[doc = "Version ID"]
pub mod verid;
#[doc = "PARAM (r) register accessor: Parameter\n\nYou can [`read`](crate::Reg::read) this register and get [`param::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@param`] module"]
#[doc(alias = "PARAM")]
pub type Param = crate::Reg<param::ParamSpec>;
#[doc = "Parameter"]
pub mod param;
#[doc = "PDOR (rw) register accessor: Port Data Output\n\nYou can [`read`](crate::Reg::read) this register and get [`pdor::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdor::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdor`] module"]
#[doc(alias = "PDOR")]
pub type Pdor = crate::Reg<pdor::PdorSpec>;
#[doc = "Port Data Output"]
pub mod pdor;
#[doc = "PSOR (rw) register accessor: Port Set Output\n\nYou can [`read`](crate::Reg::read) this register and get [`psor::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psor::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psor`] module"]
#[doc(alias = "PSOR")]
pub type Psor = crate::Reg<psor::PsorSpec>;
#[doc = "Port Set Output"]
pub mod psor;
#[doc = "PCOR (rw) register accessor: Port Clear Output\n\nYou can [`read`](crate::Reg::read) this register and get [`pcor::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcor::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcor`] module"]
#[doc(alias = "PCOR")]
pub type Pcor = crate::Reg<pcor::PcorSpec>;
#[doc = "Port Clear Output"]
pub mod pcor;
#[doc = "PTOR (rw) register accessor: Port Toggle Output\n\nYou can [`read`](crate::Reg::read) this register and get [`ptor::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptor::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptor`] module"]
#[doc(alias = "PTOR")]
pub type Ptor = crate::Reg<ptor::PtorSpec>;
#[doc = "Port Toggle Output"]
pub mod ptor;
#[doc = "PDIR (r) register accessor: Port Data Input\n\nYou can [`read`](crate::Reg::read) this register and get [`pdir::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdir`] module"]
#[doc(alias = "PDIR")]
pub type Pdir = crate::Reg<pdir::PdirSpec>;
#[doc = "Port Data Input"]
pub mod pdir;
#[doc = "PDDR (rw) register accessor: Port Data Direction\n\nYou can [`read`](crate::Reg::read) this register and get [`pddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pddr`] module"]
#[doc(alias = "PDDR")]
pub type Pddr = crate::Reg<pddr::PddrSpec>;
#[doc = "Port Data Direction"]
pub mod pddr;
#[doc = "PIDR (rw) register accessor: Port Input Disable\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pidr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr`] module"]
#[doc(alias = "PIDR")]
pub type Pidr = crate::Reg<pidr::PidrSpec>;
#[doc = "Port Input Disable"]
pub mod pidr;
#[doc = "PDR (rw) register accessor: Pin Data\n\nYou can [`read`](crate::Reg::read) this register and get [`pdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdr`] module"]
#[doc(alias = "PDR")]
pub type Pdr = crate::Reg<pdr::PdrSpec>;
#[doc = "Pin Data"]
pub mod pdr;
#[doc = "ICR0 (rw) register accessor: Interrupt Control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`icr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr0`] module"]
#[doc(alias = "ICR0")]
pub type Icr0 = crate::Reg<icr0::Icr0Spec>;
#[doc = "Interrupt Control 0"]
pub mod icr0;
#[doc = "ICR1 (rw) register accessor: Interrupt Control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`icr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr1`] module"]
#[doc(alias = "ICR1")]
pub type Icr1 = crate::Reg<icr1::Icr1Spec>;
#[doc = "Interrupt Control 1"]
pub mod icr1;
#[doc = "ICR2 (rw) register accessor: Interrupt Control 2\n\nYou can [`read`](crate::Reg::read) this register and get [`icr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr2`] module"]
#[doc(alias = "ICR2")]
pub type Icr2 = crate::Reg<icr2::Icr2Spec>;
#[doc = "Interrupt Control 2"]
pub mod icr2;
#[doc = "ICR3 (rw) register accessor: Interrupt Control 3\n\nYou can [`read`](crate::Reg::read) this register and get [`icr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr3`] module"]
#[doc(alias = "ICR3")]
pub type Icr3 = crate::Reg<icr3::Icr3Spec>;
#[doc = "Interrupt Control 3"]
pub mod icr3;
#[doc = "ICR4 (rw) register accessor: Interrupt Control 4\n\nYou can [`read`](crate::Reg::read) this register and get [`icr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr4`] module"]
#[doc(alias = "ICR4")]
pub type Icr4 = crate::Reg<icr4::Icr4Spec>;
#[doc = "Interrupt Control 4"]
pub mod icr4;
#[doc = "ICR5 (rw) register accessor: Interrupt Control 5\n\nYou can [`read`](crate::Reg::read) this register and get [`icr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr5`] module"]
#[doc(alias = "ICR5")]
pub type Icr5 = crate::Reg<icr5::Icr5Spec>;
#[doc = "Interrupt Control 5"]
pub mod icr5;
#[doc = "ICR6 (rw) register accessor: Interrupt Control 6\n\nYou can [`read`](crate::Reg::read) this register and get [`icr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr6`] module"]
#[doc(alias = "ICR6")]
pub type Icr6 = crate::Reg<icr6::Icr6Spec>;
#[doc = "Interrupt Control 6"]
pub mod icr6;
#[doc = "ICR7 (rw) register accessor: Interrupt Control 7\n\nYou can [`read`](crate::Reg::read) this register and get [`icr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr7`] module"]
#[doc(alias = "ICR7")]
pub type Icr7 = crate::Reg<icr7::Icr7Spec>;
#[doc = "Interrupt Control 7"]
pub mod icr7;
#[doc = "ICR8 (rw) register accessor: Interrupt Control 8\n\nYou can [`read`](crate::Reg::read) this register and get [`icr8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr8`] module"]
#[doc(alias = "ICR8")]
pub type Icr8 = crate::Reg<icr8::Icr8Spec>;
#[doc = "Interrupt Control 8"]
pub mod icr8;
#[doc = "ICR9 (rw) register accessor: Interrupt Control 9\n\nYou can [`read`](crate::Reg::read) this register and get [`icr9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr9`] module"]
#[doc(alias = "ICR9")]
pub type Icr9 = crate::Reg<icr9::Icr9Spec>;
#[doc = "Interrupt Control 9"]
pub mod icr9;
#[doc = "ICR10 (rw) register accessor: Interrupt Control 10\n\nYou can [`read`](crate::Reg::read) this register and get [`icr10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr10`] module"]
#[doc(alias = "ICR10")]
pub type Icr10 = crate::Reg<icr10::Icr10Spec>;
#[doc = "Interrupt Control 10"]
pub mod icr10;
#[doc = "ICR11 (rw) register accessor: Interrupt Control 11\n\nYou can [`read`](crate::Reg::read) this register and get [`icr11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr11`] module"]
#[doc(alias = "ICR11")]
pub type Icr11 = crate::Reg<icr11::Icr11Spec>;
#[doc = "Interrupt Control 11"]
pub mod icr11;
#[doc = "ICR12 (rw) register accessor: Interrupt Control 12\n\nYou can [`read`](crate::Reg::read) this register and get [`icr12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr12`] module"]
#[doc(alias = "ICR12")]
pub type Icr12 = crate::Reg<icr12::Icr12Spec>;
#[doc = "Interrupt Control 12"]
pub mod icr12;
#[doc = "ICR13 (rw) register accessor: Interrupt Control 13\n\nYou can [`read`](crate::Reg::read) this register and get [`icr13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr13`] module"]
#[doc(alias = "ICR13")]
pub type Icr13 = crate::Reg<icr13::Icr13Spec>;
#[doc = "Interrupt Control 13"]
pub mod icr13;
#[doc = "ICR14 (rw) register accessor: Interrupt Control 14\n\nYou can [`read`](crate::Reg::read) this register and get [`icr14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr14`] module"]
#[doc(alias = "ICR14")]
pub type Icr14 = crate::Reg<icr14::Icr14Spec>;
#[doc = "Interrupt Control 14"]
pub mod icr14;
#[doc = "ICR15 (rw) register accessor: Interrupt Control 15\n\nYou can [`read`](crate::Reg::read) this register and get [`icr15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr15`] module"]
#[doc(alias = "ICR15")]
pub type Icr15 = crate::Reg<icr15::Icr15Spec>;
#[doc = "Interrupt Control 15"]
pub mod icr15;
#[doc = "ICR16 (rw) register accessor: Interrupt Control 16\n\nYou can [`read`](crate::Reg::read) this register and get [`icr16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr16`] module"]
#[doc(alias = "ICR16")]
pub type Icr16 = crate::Reg<icr16::Icr16Spec>;
#[doc = "Interrupt Control 16"]
pub mod icr16;
#[doc = "ICR17 (rw) register accessor: Interrupt Control 17\n\nYou can [`read`](crate::Reg::read) this register and get [`icr17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr17`] module"]
#[doc(alias = "ICR17")]
pub type Icr17 = crate::Reg<icr17::Icr17Spec>;
#[doc = "Interrupt Control 17"]
pub mod icr17;
#[doc = "ICR18 (rw) register accessor: Interrupt Control 18\n\nYou can [`read`](crate::Reg::read) this register and get [`icr18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr18`] module"]
#[doc(alias = "ICR18")]
pub type Icr18 = crate::Reg<icr18::Icr18Spec>;
#[doc = "Interrupt Control 18"]
pub mod icr18;
#[doc = "ICR19 (rw) register accessor: Interrupt Control 19\n\nYou can [`read`](crate::Reg::read) this register and get [`icr19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr19`] module"]
#[doc(alias = "ICR19")]
pub type Icr19 = crate::Reg<icr19::Icr19Spec>;
#[doc = "Interrupt Control 19"]
pub mod icr19;
#[doc = "ICR20 (rw) register accessor: Interrupt Control 20\n\nYou can [`read`](crate::Reg::read) this register and get [`icr20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr20`] module"]
#[doc(alias = "ICR20")]
pub type Icr20 = crate::Reg<icr20::Icr20Spec>;
#[doc = "Interrupt Control 20"]
pub mod icr20;
#[doc = "ICR21 (rw) register accessor: Interrupt Control 21\n\nYou can [`read`](crate::Reg::read) this register and get [`icr21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr21`] module"]
#[doc(alias = "ICR21")]
pub type Icr21 = crate::Reg<icr21::Icr21Spec>;
#[doc = "Interrupt Control 21"]
pub mod icr21;
#[doc = "ICR22 (rw) register accessor: Interrupt Control 22\n\nYou can [`read`](crate::Reg::read) this register and get [`icr22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr22`] module"]
#[doc(alias = "ICR22")]
pub type Icr22 = crate::Reg<icr22::Icr22Spec>;
#[doc = "Interrupt Control 22"]
pub mod icr22;
#[doc = "ICR23 (rw) register accessor: Interrupt Control 23\n\nYou can [`read`](crate::Reg::read) this register and get [`icr23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr23`] module"]
#[doc(alias = "ICR23")]
pub type Icr23 = crate::Reg<icr23::Icr23Spec>;
#[doc = "Interrupt Control 23"]
pub mod icr23;
#[doc = "ICR24 (rw) register accessor: Interrupt Control 24\n\nYou can [`read`](crate::Reg::read) this register and get [`icr24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr24`] module"]
#[doc(alias = "ICR24")]
pub type Icr24 = crate::Reg<icr24::Icr24Spec>;
#[doc = "Interrupt Control 24"]
pub mod icr24;
#[doc = "ICR25 (rw) register accessor: Interrupt Control 25\n\nYou can [`read`](crate::Reg::read) this register and get [`icr25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr25`] module"]
#[doc(alias = "ICR25")]
pub type Icr25 = crate::Reg<icr25::Icr25Spec>;
#[doc = "Interrupt Control 25"]
pub mod icr25;
#[doc = "ICR26 (rw) register accessor: Interrupt Control 26\n\nYou can [`read`](crate::Reg::read) this register and get [`icr26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr26`] module"]
#[doc(alias = "ICR26")]
pub type Icr26 = crate::Reg<icr26::Icr26Spec>;
#[doc = "Interrupt Control 26"]
pub mod icr26;
#[doc = "ICR27 (rw) register accessor: Interrupt Control 27\n\nYou can [`read`](crate::Reg::read) this register and get [`icr27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr27`] module"]
#[doc(alias = "ICR27")]
pub type Icr27 = crate::Reg<icr27::Icr27Spec>;
#[doc = "Interrupt Control 27"]
pub mod icr27;
#[doc = "ICR28 (rw) register accessor: Interrupt Control 28\n\nYou can [`read`](crate::Reg::read) this register and get [`icr28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr28`] module"]
#[doc(alias = "ICR28")]
pub type Icr28 = crate::Reg<icr28::Icr28Spec>;
#[doc = "Interrupt Control 28"]
pub mod icr28;
#[doc = "ICR29 (rw) register accessor: Interrupt Control 29\n\nYou can [`read`](crate::Reg::read) this register and get [`icr29::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr29`] module"]
#[doc(alias = "ICR29")]
pub type Icr29 = crate::Reg<icr29::Icr29Spec>;
#[doc = "Interrupt Control 29"]
pub mod icr29;
#[doc = "ICR30 (rw) register accessor: Interrupt Control 30\n\nYou can [`read`](crate::Reg::read) this register and get [`icr30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr30`] module"]
#[doc(alias = "ICR30")]
pub type Icr30 = crate::Reg<icr30::Icr30Spec>;
#[doc = "Interrupt Control 30"]
pub mod icr30;
#[doc = "ICR31 (rw) register accessor: Interrupt Control 31\n\nYou can [`read`](crate::Reg::read) this register and get [`icr31::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr31`] module"]
#[doc(alias = "ICR31")]
pub type Icr31 = crate::Reg<icr31::Icr31Spec>;
#[doc = "Interrupt Control 31"]
pub mod icr31;
#[doc = "GICLR (rw) register accessor: Global Interrupt Control Low\n\nYou can [`read`](crate::Reg::read) this register and get [`giclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@giclr`] module"]
#[doc(alias = "GICLR")]
pub type Giclr = crate::Reg<giclr::GiclrSpec>;
#[doc = "Global Interrupt Control Low"]
pub mod giclr;
#[doc = "GICHR (rw) register accessor: Global Interrupt Control High\n\nYou can [`read`](crate::Reg::read) this register and get [`gichr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gichr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gichr`] module"]
#[doc(alias = "GICHR")]
pub type Gichr = crate::Reg<gichr::GichrSpec>;
#[doc = "Global Interrupt Control High"]
pub mod gichr;
#[doc = "ISFR0 (rw) register accessor: Interrupt Status Flag\n\nYou can [`read`](crate::Reg::read) this register and get [`isfr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isfr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isfr0`] module"]
#[doc(alias = "ISFR0")]
pub type Isfr0 = crate::Reg<isfr0::Isfr0Spec>;
#[doc = "Interrupt Status Flag"]
pub mod isfr0;
