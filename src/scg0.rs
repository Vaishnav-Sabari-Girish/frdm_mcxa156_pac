#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    verid: Verid,
    param: Param,
    trim_lock: TrimLock,
    _reserved3: [u8; 0x04],
    csr: Csr,
    rccr: Rccr,
    _reserved5: [u8; 0xe8],
    sosccsr: Sosccsr,
    _reserved6: [u8; 0x04],
    sosccfg: Sosccfg,
    _reserved7: [u8; 0xf4],
    sirccsr: Sirccsr,
    _reserved8: [u8; 0x08],
    sirctcfg: Sirctcfg,
    sirctrim: Sirctrim,
    _reserved10: [u8; 0x04],
    sircstat: Sircstat,
    _reserved11: [u8; 0xe4],
    firccsr: Firccsr,
    _reserved12: [u8; 0x04],
    firccfg: Firccfg,
    firctcfg: Firctcfg,
    firctrim: Firctrim,
    _reserved15: [u8; 0x04],
    fircstat: Fircstat,
    fircatc1: Fircatc1,
    fircatc2: Fircatc2,
    fircatc3: Fircatc3,
    _reserved19: [u8; 0xd8],
    rosccsr: Rosccsr,
}
impl RegisterBlock {
    #[doc = "0x00 - Version ID Register"]
    #[inline(always)]
    pub const fn verid(&self) -> &Verid {
        &self.verid
    }
    #[doc = "0x04 - Parameter Register"]
    #[inline(always)]
    pub const fn param(&self) -> &Param {
        &self.param
    }
    #[doc = "0x08 - Trim Lock register"]
    #[inline(always)]
    pub const fn trim_lock(&self) -> &TrimLock {
        &self.trim_lock
    }
    #[doc = "0x10 - Clock Status Register"]
    #[inline(always)]
    pub const fn csr(&self) -> &Csr {
        &self.csr
    }
    #[doc = "0x14 - Run Clock Control Register"]
    #[inline(always)]
    pub const fn rccr(&self) -> &Rccr {
        &self.rccr
    }
    #[doc = "0x100 - SOSC Control Status Register"]
    #[inline(always)]
    pub const fn sosccsr(&self) -> &Sosccsr {
        &self.sosccsr
    }
    #[doc = "0x108 - SOSC Configuration Register"]
    #[inline(always)]
    pub const fn sosccfg(&self) -> &Sosccfg {
        &self.sosccfg
    }
    #[doc = "0x200 - SIRC Control Status Register"]
    #[inline(always)]
    pub const fn sirccsr(&self) -> &Sirccsr {
        &self.sirccsr
    }
    #[doc = "0x20c - SIRC Trim Configuration Register"]
    #[inline(always)]
    pub const fn sirctcfg(&self) -> &Sirctcfg {
        &self.sirctcfg
    }
    #[doc = "0x210 - SIRC Trim Register"]
    #[inline(always)]
    pub const fn sirctrim(&self) -> &Sirctrim {
        &self.sirctrim
    }
    #[doc = "0x218 - SIRC Auto-trimming Status Register"]
    #[inline(always)]
    pub const fn sircstat(&self) -> &Sircstat {
        &self.sircstat
    }
    #[doc = "0x300 - FIRC Control Status Register"]
    #[inline(always)]
    pub const fn firccsr(&self) -> &Firccsr {
        &self.firccsr
    }
    #[doc = "0x308 - FIRC Configuration Register"]
    #[inline(always)]
    pub const fn firccfg(&self) -> &Firccfg {
        &self.firccfg
    }
    #[doc = "0x30c - FIRC Trim Configuration Register"]
    #[inline(always)]
    pub const fn firctcfg(&self) -> &Firctcfg {
        &self.firctcfg
    }
    #[doc = "0x310 - FIRC Trim Register"]
    #[inline(always)]
    pub const fn firctrim(&self) -> &Firctrim {
        &self.firctrim
    }
    #[doc = "0x318 - FIRC Auto-trimming Status Register"]
    #[inline(always)]
    pub const fn fircstat(&self) -> &Fircstat {
        &self.fircstat
    }
    #[doc = "0x31c - FIRC Auto-trimming Counter 1"]
    #[inline(always)]
    pub const fn fircatc1(&self) -> &Fircatc1 {
        &self.fircatc1
    }
    #[doc = "0x320 - FIRC Auto-trimming Counter 2"]
    #[inline(always)]
    pub const fn fircatc2(&self) -> &Fircatc2 {
        &self.fircatc2
    }
    #[doc = "0x324 - FIRC Auto-trimming Counter 2"]
    #[inline(always)]
    pub const fn fircatc3(&self) -> &Fircatc3 {
        &self.fircatc3
    }
    #[doc = "0x400 - ROSC Control Status Register"]
    #[inline(always)]
    pub const fn rosccsr(&self) -> &Rosccsr {
        &self.rosccsr
    }
}
#[doc = "VERID (r) register accessor: Version ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`verid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@verid`] module"]
#[doc(alias = "VERID")]
pub type Verid = crate::Reg<verid::VeridSpec>;
#[doc = "Version ID Register"]
pub mod verid;
#[doc = "PARAM (r) register accessor: Parameter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`param::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@param`] module"]
#[doc(alias = "PARAM")]
pub type Param = crate::Reg<param::ParamSpec>;
#[doc = "Parameter Register"]
pub mod param;
#[doc = "TRIM_LOCK (rw) register accessor: Trim Lock register\n\nYou can [`read`](crate::Reg::read) this register and get [`trim_lock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trim_lock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trim_lock`] module"]
#[doc(alias = "TRIM_LOCK")]
pub type TrimLock = crate::Reg<trim_lock::TrimLockSpec>;
#[doc = "Trim Lock register"]
pub mod trim_lock;
#[doc = "CSR (r) register accessor: Clock Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`csr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr`] module"]
#[doc(alias = "CSR")]
pub type Csr = crate::Reg<csr::CsrSpec>;
#[doc = "Clock Status Register"]
pub mod csr;
#[doc = "RCCR (rw) register accessor: Run Clock Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rccr`] module"]
#[doc(alias = "RCCR")]
pub type Rccr = crate::Reg<rccr::RccrSpec>;
#[doc = "Run Clock Control Register"]
pub mod rccr;
#[doc = "SOSCCSR (rw) register accessor: SOSC Control Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sosccsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sosccsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sosccsr`] module"]
#[doc(alias = "SOSCCSR")]
pub type Sosccsr = crate::Reg<sosccsr::SosccsrSpec>;
#[doc = "SOSC Control Status Register"]
pub mod sosccsr;
#[doc = "SOSCCFG (rw) register accessor: SOSC Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sosccfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sosccfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sosccfg`] module"]
#[doc(alias = "SOSCCFG")]
pub type Sosccfg = crate::Reg<sosccfg::SosccfgSpec>;
#[doc = "SOSC Configuration Register"]
pub mod sosccfg;
#[doc = "SIRCCSR (rw) register accessor: SIRC Control Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sirccsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sirccsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sirccsr`] module"]
#[doc(alias = "SIRCCSR")]
pub type Sirccsr = crate::Reg<sirccsr::SirccsrSpec>;
#[doc = "SIRC Control Status Register"]
pub mod sirccsr;
#[doc = "SIRCTCFG (rw) register accessor: SIRC Trim Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sirctcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sirctcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sirctcfg`] module"]
#[doc(alias = "SIRCTCFG")]
pub type Sirctcfg = crate::Reg<sirctcfg::SirctcfgSpec>;
#[doc = "SIRC Trim Configuration Register"]
pub mod sirctcfg;
#[doc = "SIRCTRIM (rw) register accessor: SIRC Trim Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sirctrim::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sirctrim::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sirctrim`] module"]
#[doc(alias = "SIRCTRIM")]
pub type Sirctrim = crate::Reg<sirctrim::SirctrimSpec>;
#[doc = "SIRC Trim Register"]
pub mod sirctrim;
#[doc = "SIRCSTAT (rw) register accessor: SIRC Auto-trimming Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sircstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sircstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sircstat`] module"]
#[doc(alias = "SIRCSTAT")]
pub type Sircstat = crate::Reg<sircstat::SircstatSpec>;
#[doc = "SIRC Auto-trimming Status Register"]
pub mod sircstat;
#[doc = "FIRCCSR (rw) register accessor: FIRC Control Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`firccsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`firccsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@firccsr`] module"]
#[doc(alias = "FIRCCSR")]
pub type Firccsr = crate::Reg<firccsr::FirccsrSpec>;
#[doc = "FIRC Control Status Register"]
pub mod firccsr;
#[doc = "FIRCCFG (rw) register accessor: FIRC Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`firccfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`firccfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@firccfg`] module"]
#[doc(alias = "FIRCCFG")]
pub type Firccfg = crate::Reg<firccfg::FirccfgSpec>;
#[doc = "FIRC Configuration Register"]
pub mod firccfg;
#[doc = "FIRCTCFG (rw) register accessor: FIRC Trim Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`firctcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`firctcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@firctcfg`] module"]
#[doc(alias = "FIRCTCFG")]
pub type Firctcfg = crate::Reg<firctcfg::FirctcfgSpec>;
#[doc = "FIRC Trim Configuration Register"]
pub mod firctcfg;
#[doc = "FIRCTRIM (rw) register accessor: FIRC Trim Register\n\nYou can [`read`](crate::Reg::read) this register and get [`firctrim::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`firctrim::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@firctrim`] module"]
#[doc(alias = "FIRCTRIM")]
pub type Firctrim = crate::Reg<firctrim::FirctrimSpec>;
#[doc = "FIRC Trim Register"]
pub mod firctrim;
#[doc = "FIRCSTAT (rw) register accessor: FIRC Auto-trimming Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fircstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fircstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fircstat`] module"]
#[doc(alias = "FIRCSTAT")]
pub type Fircstat = crate::Reg<fircstat::FircstatSpec>;
#[doc = "FIRC Auto-trimming Status Register"]
pub mod fircstat;
#[doc = "FIRCATC1 (rw) register accessor: FIRC Auto-trimming Counter 1\n\nYou can [`read`](crate::Reg::read) this register and get [`fircatc1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fircatc1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fircatc1`] module"]
#[doc(alias = "FIRCATC1")]
pub type Fircatc1 = crate::Reg<fircatc1::Fircatc1Spec>;
#[doc = "FIRC Auto-trimming Counter 1"]
pub mod fircatc1;
#[doc = "FIRCATC2 (rw) register accessor: FIRC Auto-trimming Counter 2\n\nYou can [`read`](crate::Reg::read) this register and get [`fircatc2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fircatc2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fircatc2`] module"]
#[doc(alias = "FIRCATC2")]
pub type Fircatc2 = crate::Reg<fircatc2::Fircatc2Spec>;
#[doc = "FIRC Auto-trimming Counter 2"]
pub mod fircatc2;
#[doc = "FIRCATC3 (rw) register accessor: FIRC Auto-trimming Counter 2\n\nYou can [`read`](crate::Reg::read) this register and get [`fircatc3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fircatc3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fircatc3`] module"]
#[doc(alias = "FIRCATC3")]
pub type Fircatc3 = crate::Reg<fircatc3::Fircatc3Spec>;
#[doc = "FIRC Auto-trimming Counter 2"]
pub mod fircatc3;
#[doc = "ROSCCSR (rw) register accessor: ROSC Control Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rosccsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rosccsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rosccsr`] module"]
#[doc(alias = "ROSCCSR")]
pub type Rosccsr = crate::Reg<rosccsr::RosccsrSpec>;
#[doc = "ROSC Control Status Register"]
pub mod rosccsr;
