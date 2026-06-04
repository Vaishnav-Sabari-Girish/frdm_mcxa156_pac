#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    syst_csr: SystCsr,
    syst_rvr: SystRvr,
    syst_cvr: SystCvr,
    syst_calib: SystCalib,
}
impl RegisterBlock {
    #[doc = "0x00 - SysTick Control and Status Register"]
    #[inline(always)]
    pub const fn syst_csr(&self) -> &SystCsr {
        &self.syst_csr
    }
    #[doc = "0x04 - SysTick Reload Value Register"]
    #[inline(always)]
    pub const fn syst_rvr(&self) -> &SystRvr {
        &self.syst_rvr
    }
    #[doc = "0x08 - SysTick Current Value Register"]
    #[inline(always)]
    pub const fn syst_cvr(&self) -> &SystCvr {
        &self.syst_cvr
    }
    #[doc = "0x0c - SysTick Calibration Value Register"]
    #[inline(always)]
    pub const fn syst_calib(&self) -> &SystCalib {
        &self.syst_calib
    }
}
#[doc = "SYST_CSR (rw) register accessor: SysTick Control and Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`syst_csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syst_csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syst_csr`] module"]
#[doc(alias = "SYST_CSR")]
pub type SystCsr = crate::Reg<syst_csr::SystCsrSpec>;
#[doc = "SysTick Control and Status Register"]
pub mod syst_csr;
#[doc = "SYST_RVR (rw) register accessor: SysTick Reload Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`syst_rvr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syst_rvr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syst_rvr`] module"]
#[doc(alias = "SYST_RVR")]
pub type SystRvr = crate::Reg<syst_rvr::SystRvrSpec>;
#[doc = "SysTick Reload Value Register"]
pub mod syst_rvr;
#[doc = "SYST_CVR (rw) register accessor: SysTick Current Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`syst_cvr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syst_cvr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syst_cvr`] module"]
#[doc(alias = "SYST_CVR")]
pub type SystCvr = crate::Reg<syst_cvr::SystCvrSpec>;
#[doc = "SysTick Current Value Register"]
pub mod syst_cvr;
#[doc = "SYST_CALIB (r) register accessor: SysTick Calibration Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`syst_calib::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syst_calib`] module"]
#[doc(alias = "SYST_CALIB")]
pub type SystCalib = crate::Reg<syst_calib::SystCalibSpec>;
#[doc = "SysTick Calibration Value Register"]
pub mod syst_calib;
