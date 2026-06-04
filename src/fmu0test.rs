#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    fstat: Fstat,
    fcnfg: Fcnfg,
    fctrl: Fctrl,
    ftest: Ftest,
    fccob0: Fccob0,
    fccob1: Fccob1,
    fccob2: Fccob2,
    fccob3: Fccob3,
    fccob4: Fccob4,
    fccob5: Fccob5,
    fccob6: Fccob6,
    fccob7: Fccob7,
    _reserved12: [u8; 0xd0],
    reset_status: ResetStatus,
    mctl: Mctl,
    bsel_gen: BselGen,
    pwr_opt: PwrOpt,
    cmd_check: CmdCheck,
    _reserved17: [u8; 0x0c],
    bsel: Bsel,
    msize: Msize,
    flash_rd_add: FlashRdAdd,
    _reserved20: [u8; 0x04],
    flash_stop_add: FlashStopAdd,
    flash_rd_ctrl: FlashRdCtrl,
    mm_addr: MmAddr,
    _reserved23: [u8; 0x04],
    mm_wdata: MmWdata,
    mm_ctl: MmCtl,
    uint_ctl: UintCtl,
    rd_data0: RdData0,
    rd_data1: RdData1,
    rd_data2: RdData2,
    rd_data3: RdData3,
    parity: Parity,
    rd_path_ctrl_status: RdPathCtrlStatus,
    smw_din0: SmwDin0,
    smw_din1: SmwDin1,
    smw_din2: SmwDin2,
    smw_din3: SmwDin3,
    smw_addr: SmwAddr,
    smw_cmd_wait: SmwCmdWait,
    smw_status: SmwStatus,
    soctrim0_0: Soctrim0_0,
    soctrim0_1: Soctrim0_1,
    soctrim0_2: Soctrim0_2,
    soctrim0_3: Soctrim0_3,
    soctrim1_0: Soctrim1_0,
    soctrim1_1: Soctrim1_1,
    soctrim1_2: Soctrim1_2,
    soctrim1_3: Soctrim1_3,
    soctrim2_0: Soctrim2_0,
    soctrim2_1: Soctrim2_1,
    soctrim2_2: Soctrim2_2,
    soctrim2_3: Soctrim2_3,
    soctrim3_0: Soctrim3_0,
    soctrim3_1: Soctrim3_1,
    soctrim3_2: Soctrim3_2,
    soctrim3_3: Soctrim3_3,
    soctrim4_0: Soctrim4_0,
    soctrim4_1: Soctrim4_1,
    soctrim4_2: Soctrim4_2,
    soctrim4_3: Soctrim4_3,
    soctrim5_0: Soctrim5_0,
    soctrim5_1: Soctrim5_1,
    soctrim5_2: Soctrim5_2,
    soctrim5_3: Soctrim5_3,
    soctrim6_0: Soctrim6_0,
    soctrim6_1: Soctrim6_1,
    soctrim6_2: Soctrim6_2,
    soctrim6_3: Soctrim6_3,
    soctrim7_0: Soctrim7_0,
    soctrim7_1: Soctrim7_1,
    soctrim7_2: Soctrim7_2,
    soctrim7_3: Soctrim7_3,
    _reserved71: [u8; 0x04],
    r_ip_config: RIpConfig,
    r_testcode: RTestcode,
    r_dft_ctrl: RDftCtrl,
    r_adr_ctrl: RAdrCtrl,
    r_data_ctrl0: RDataCtrl0,
    r_pin_ctrl: RPinCtrl,
    r_cnt_loop_ctrl: RCntLoopCtrl,
    r_timer_ctrl: RTimerCtrl,
    r_test_ctrl: RTestCtrl,
    r_abort_loop: RAbortLoop,
    r_adr_query: RAdrQuery,
    r_dout_query0: RDoutQuery0,
    _reserved83: [u8; 0x08],
    r_smw_query: RSmwQuery,
    r_smw_setting0: RSmwSetting0,
    r_smw_setting1: RSmwSetting1,
    r_smp_whv0: RSmpWhv0,
    r_smp_whv1: RSmpWhv1,
    r_sme_whv0: RSmeWhv0,
    r_sme_whv1: RSmeWhv1,
    r_smw_setting2: RSmwSetting2,
    r_d_misr0: RDMisr0,
    r_a_misr0: RAMisr0,
    r_c_misr0: RCMisr0,
    r_smw_setting3: RSmwSetting3,
    r_data_ctrl1: RDataCtrl1,
    r_data_ctrl2: RDataCtrl2,
    r_data_ctrl3: RDataCtrl3,
    _reserved98: [u8; 0x08],
    r_repair0_0: RRepair0_0,
    r_repair0_1: RRepair0_1,
    r_repair1_0: RRepair1_0,
    r_repair1_1: RRepair1_1,
    _reserved102: [u8; 0x84],
    r_data_ctrl0_ex: RDataCtrl0Ex,
    _reserved103: [u8; 0x08],
    r_timer_ctrl_ex: RTimerCtrlEx,
    _reserved104: [u8; 0x0c],
    r_dout_query1: RDoutQuery1,
    _reserved105: [u8; 0x28],
    r_d_misr1: RDMisr1,
    r_a_misr1: RAMisr1,
    r_c_misr1: RCMisr1,
    _reserved108: [u8; 0x04],
    r_data_ctrl1_ex: RDataCtrl1Ex,
    r_data_ctrl2_ex: RDataCtrl2Ex,
    r_data_ctrl3_ex: RDataCtrl3Ex,
    _reserved111: [u8; 0x88],
    smw_timer_option: SmwTimerOption,
    smw_setting_option0: SmwSettingOption0,
    smw_setting_option2: SmwSettingOption2,
    smw_setting_option3: SmwSettingOption3,
    smw_smp_whv_option0: SmwSmpWhvOption0,
    smw_sme_whv_option0: SmwSmeWhvOption0,
    smw_setting_option1: SmwSettingOption1,
    smw_smp_whv_option1: SmwSmpWhvOption1,
    smw_sme_whv_option1: SmwSmeWhvOption1,
    _reserved120: [u8; 0xdc],
    repair0_0: Repair0_0,
    repair0_1: Repair0_1,
    repair1_0: Repair1_0,
    repair1_1: Repair1_1,
    _reserved124: [u8; 0xf0],
    smw_hb_signals: SmwHbSignals,
    bist_dump_ctrl: BistDumpCtrl,
    _reserved126: [u8; 0x04],
    atx_pin_ctrl: AtxPinCtrl,
    failcnt: Failcnt,
    pgm_pulse_cnt0: PgmPulseCnt0,
    pgm_pulse_cnt1: PgmPulseCnt1,
    ers_pulse_cnt: ErsPulseCnt,
    max_pulse_cnt: MaxPulseCnt,
    port_ctrl: PortCtrl,
}
impl RegisterBlock {
    #[doc = "0x00 - Flash Status Register"]
    #[inline(always)]
    pub const fn fstat(&self) -> &Fstat {
        &self.fstat
    }
    #[doc = "0x04 - Flash Configuration Register"]
    #[inline(always)]
    pub const fn fcnfg(&self) -> &Fcnfg {
        &self.fcnfg
    }
    #[doc = "0x08 - Flash Control Register"]
    #[inline(always)]
    pub const fn fctrl(&self) -> &Fctrl {
        &self.fctrl
    }
    #[doc = "0x0c - Flash Test Register"]
    #[inline(always)]
    pub const fn ftest(&self) -> &Ftest {
        &self.ftest
    }
    #[doc = "0x10 - Flash Command Control 0 Register"]
    #[inline(always)]
    pub const fn fccob0(&self) -> &Fccob0 {
        &self.fccob0
    }
    #[doc = "0x14 - Flash Command Control 1 Register"]
    #[inline(always)]
    pub const fn fccob1(&self) -> &Fccob1 {
        &self.fccob1
    }
    #[doc = "0x18 - Flash Command Control 2 Register"]
    #[inline(always)]
    pub const fn fccob2(&self) -> &Fccob2 {
        &self.fccob2
    }
    #[doc = "0x1c - Flash Command Control 3 Register"]
    #[inline(always)]
    pub const fn fccob3(&self) -> &Fccob3 {
        &self.fccob3
    }
    #[doc = "0x20 - Flash Command Control 4 Register"]
    #[inline(always)]
    pub const fn fccob4(&self) -> &Fccob4 {
        &self.fccob4
    }
    #[doc = "0x24 - Flash Command Control 5 Register"]
    #[inline(always)]
    pub const fn fccob5(&self) -> &Fccob5 {
        &self.fccob5
    }
    #[doc = "0x28 - Flash Command Control 6 Register"]
    #[inline(always)]
    pub const fn fccob6(&self) -> &Fccob6 {
        &self.fccob6
    }
    #[doc = "0x2c - Flash Command Control 7 Register"]
    #[inline(always)]
    pub const fn fccob7(&self) -> &Fccob7 {
        &self.fccob7
    }
    #[doc = "0x100 - FMU Initialization Tracking Register"]
    #[inline(always)]
    pub const fn reset_status(&self) -> &ResetStatus {
        &self.reset_status
    }
    #[doc = "0x104 - FMU Control Register"]
    #[inline(always)]
    pub const fn mctl(&self) -> &Mctl {
        &self.mctl
    }
    #[doc = "0x108 - FMU Block Select Generation Register"]
    #[inline(always)]
    pub const fn bsel_gen(&self) -> &BselGen {
        &self.bsel_gen
    }
    #[doc = "0x10c - Power Mode Options Register"]
    #[inline(always)]
    pub const fn pwr_opt(&self) -> &PwrOpt {
        &self.pwr_opt
    }
    #[doc = "0x110 - FMU Command Check Register"]
    #[inline(always)]
    pub const fn cmd_check(&self) -> &CmdCheck {
        &self.cmd_check
    }
    #[doc = "0x120 - FMU Block Select Register"]
    #[inline(always)]
    pub const fn bsel(&self) -> &Bsel {
        &self.bsel
    }
    #[doc = "0x124 - FMU Memory Size Register"]
    #[inline(always)]
    pub const fn msize(&self) -> &Msize {
        &self.msize
    }
    #[doc = "0x128 - Flash Read Address Register"]
    #[inline(always)]
    pub const fn flash_rd_add(&self) -> &FlashRdAdd {
        &self.flash_rd_add
    }
    #[doc = "0x130 - Flash Stop Address Register"]
    #[inline(always)]
    pub const fn flash_stop_add(&self) -> &FlashStopAdd {
        &self.flash_stop_add
    }
    #[doc = "0x134 - Flash Read Control Register"]
    #[inline(always)]
    pub const fn flash_rd_ctrl(&self) -> &FlashRdCtrl {
        &self.flash_rd_ctrl
    }
    #[doc = "0x138 - Memory Map Address Register"]
    #[inline(always)]
    pub const fn mm_addr(&self) -> &MmAddr {
        &self.mm_addr
    }
    #[doc = "0x140 - Memory Map Write Data Register"]
    #[inline(always)]
    pub const fn mm_wdata(&self) -> &MmWdata {
        &self.mm_wdata
    }
    #[doc = "0x144 - Memory Map Control Register"]
    #[inline(always)]
    pub const fn mm_ctl(&self) -> &MmCtl {
        &self.mm_ctl
    }
    #[doc = "0x148 - User Interface Control Register"]
    #[inline(always)]
    pub const fn uint_ctl(&self) -> &UintCtl {
        &self.uint_ctl
    }
    #[doc = "0x14c - Read Data 0 Register"]
    #[inline(always)]
    pub const fn rd_data0(&self) -> &RdData0 {
        &self.rd_data0
    }
    #[doc = "0x150 - Read Data 1 Register"]
    #[inline(always)]
    pub const fn rd_data1(&self) -> &RdData1 {
        &self.rd_data1
    }
    #[doc = "0x154 - Read Data 2 Register"]
    #[inline(always)]
    pub const fn rd_data2(&self) -> &RdData2 {
        &self.rd_data2
    }
    #[doc = "0x158 - Read Data 3 Register"]
    #[inline(always)]
    pub const fn rd_data3(&self) -> &RdData3 {
        &self.rd_data3
    }
    #[doc = "0x15c - Parity Register"]
    #[inline(always)]
    pub const fn parity(&self) -> &Parity {
        &self.parity
    }
    #[doc = "0x160 - Read Path Control and Status Register"]
    #[inline(always)]
    pub const fn rd_path_ctrl_status(&self) -> &RdPathCtrlStatus {
        &self.rd_path_ctrl_status
    }
    #[doc = "0x164 - SMW DIN 0 Register"]
    #[inline(always)]
    pub const fn smw_din0(&self) -> &SmwDin0 {
        &self.smw_din0
    }
    #[doc = "0x168 - SMW DIN 1 Register"]
    #[inline(always)]
    pub const fn smw_din1(&self) -> &SmwDin1 {
        &self.smw_din1
    }
    #[doc = "0x16c - SMW DIN 2 Register"]
    #[inline(always)]
    pub const fn smw_din2(&self) -> &SmwDin2 {
        &self.smw_din2
    }
    #[doc = "0x170 - SMW DIN 3 Register"]
    #[inline(always)]
    pub const fn smw_din3(&self) -> &SmwDin3 {
        &self.smw_din3
    }
    #[doc = "0x174 - SMW Address Register"]
    #[inline(always)]
    pub const fn smw_addr(&self) -> &SmwAddr {
        &self.smw_addr
    }
    #[doc = "0x178 - SMW Command and Wait Register"]
    #[inline(always)]
    pub const fn smw_cmd_wait(&self) -> &SmwCmdWait {
        &self.smw_cmd_wait
    }
    #[doc = "0x17c - SMW Status Register"]
    #[inline(always)]
    pub const fn smw_status(&self) -> &SmwStatus {
        &self.smw_status
    }
    #[doc = "0x180 - SoC Trim Phrase 0 Word 0 Register"]
    #[inline(always)]
    pub const fn soctrim0_0(&self) -> &Soctrim0_0 {
        &self.soctrim0_0
    }
    #[doc = "0x184 - SoC Trim Phrase 0 Word 1 Register"]
    #[inline(always)]
    pub const fn soctrim0_1(&self) -> &Soctrim0_1 {
        &self.soctrim0_1
    }
    #[doc = "0x188 - SoC Trim Phrase 0 Word 2 Register"]
    #[inline(always)]
    pub const fn soctrim0_2(&self) -> &Soctrim0_2 {
        &self.soctrim0_2
    }
    #[doc = "0x18c - SoC Trim Phrase 0 Word 3 Register"]
    #[inline(always)]
    pub const fn soctrim0_3(&self) -> &Soctrim0_3 {
        &self.soctrim0_3
    }
    #[doc = "0x190 - SoC Trim Phrase 1 Word 0 Register"]
    #[inline(always)]
    pub const fn soctrim1_0(&self) -> &Soctrim1_0 {
        &self.soctrim1_0
    }
    #[doc = "0x194 - SoC Trim Phrase 1 Word 1 Register"]
    #[inline(always)]
    pub const fn soctrim1_1(&self) -> &Soctrim1_1 {
        &self.soctrim1_1
    }
    #[doc = "0x198 - SoC Trim Phrase 1 Word 2 Register"]
    #[inline(always)]
    pub const fn soctrim1_2(&self) -> &Soctrim1_2 {
        &self.soctrim1_2
    }
    #[doc = "0x19c - SoC Trim Phrase 1 Word 3 Register"]
    #[inline(always)]
    pub const fn soctrim1_3(&self) -> &Soctrim1_3 {
        &self.soctrim1_3
    }
    #[doc = "0x1a0 - SoC Trim Phrase 2 Word 0 Register"]
    #[inline(always)]
    pub const fn soctrim2_0(&self) -> &Soctrim2_0 {
        &self.soctrim2_0
    }
    #[doc = "0x1a4 - SoC Trim Phrase 2 Word 1 Register"]
    #[inline(always)]
    pub const fn soctrim2_1(&self) -> &Soctrim2_1 {
        &self.soctrim2_1
    }
    #[doc = "0x1a8 - SoC Trim Phrase 2 Word 2 Register"]
    #[inline(always)]
    pub const fn soctrim2_2(&self) -> &Soctrim2_2 {
        &self.soctrim2_2
    }
    #[doc = "0x1ac - SoC Trim Phrase 2 Word 3 Register"]
    #[inline(always)]
    pub const fn soctrim2_3(&self) -> &Soctrim2_3 {
        &self.soctrim2_3
    }
    #[doc = "0x1b0 - SoC Trim Phrase 3 Word 0 Register"]
    #[inline(always)]
    pub const fn soctrim3_0(&self) -> &Soctrim3_0 {
        &self.soctrim3_0
    }
    #[doc = "0x1b4 - SoC Trim Phrase 3 Word 1 Register"]
    #[inline(always)]
    pub const fn soctrim3_1(&self) -> &Soctrim3_1 {
        &self.soctrim3_1
    }
    #[doc = "0x1b8 - SoC Trim Phrase 3 Word 2 Register"]
    #[inline(always)]
    pub const fn soctrim3_2(&self) -> &Soctrim3_2 {
        &self.soctrim3_2
    }
    #[doc = "0x1bc - SoC Trim Phrase 3 Word 3 Register"]
    #[inline(always)]
    pub const fn soctrim3_3(&self) -> &Soctrim3_3 {
        &self.soctrim3_3
    }
    #[doc = "0x1c0 - SoC Trim Phrase 4 Word 0 Register"]
    #[inline(always)]
    pub const fn soctrim4_0(&self) -> &Soctrim4_0 {
        &self.soctrim4_0
    }
    #[doc = "0x1c4 - SoC Trim Phrase 4 Word 1 Register"]
    #[inline(always)]
    pub const fn soctrim4_1(&self) -> &Soctrim4_1 {
        &self.soctrim4_1
    }
    #[doc = "0x1c8 - SoC Trim Phrase 4 Word 2 Register"]
    #[inline(always)]
    pub const fn soctrim4_2(&self) -> &Soctrim4_2 {
        &self.soctrim4_2
    }
    #[doc = "0x1cc - SoC Trim Phrase 4 Word 3 Register"]
    #[inline(always)]
    pub const fn soctrim4_3(&self) -> &Soctrim4_3 {
        &self.soctrim4_3
    }
    #[doc = "0x1d0 - SoC Trim Phrase 5 Word 0 Register"]
    #[inline(always)]
    pub const fn soctrim5_0(&self) -> &Soctrim5_0 {
        &self.soctrim5_0
    }
    #[doc = "0x1d4 - SoC Trim Phrase 5 Word 1 Register"]
    #[inline(always)]
    pub const fn soctrim5_1(&self) -> &Soctrim5_1 {
        &self.soctrim5_1
    }
    #[doc = "0x1d8 - SoC Trim Phrase 5 Word 2 Register"]
    #[inline(always)]
    pub const fn soctrim5_2(&self) -> &Soctrim5_2 {
        &self.soctrim5_2
    }
    #[doc = "0x1dc - SoC Trim Phrase 5 Word 3 Register"]
    #[inline(always)]
    pub const fn soctrim5_3(&self) -> &Soctrim5_3 {
        &self.soctrim5_3
    }
    #[doc = "0x1e0 - SoC Trim Phrase 6 Word 0 Register"]
    #[inline(always)]
    pub const fn soctrim6_0(&self) -> &Soctrim6_0 {
        &self.soctrim6_0
    }
    #[doc = "0x1e4 - SoC Trim Phrase 6 Word 1 Register"]
    #[inline(always)]
    pub const fn soctrim6_1(&self) -> &Soctrim6_1 {
        &self.soctrim6_1
    }
    #[doc = "0x1e8 - SoC Trim Phrase 6 Word 2 Register"]
    #[inline(always)]
    pub const fn soctrim6_2(&self) -> &Soctrim6_2 {
        &self.soctrim6_2
    }
    #[doc = "0x1ec - SoC Trim Phrase 6 Word 3 Register"]
    #[inline(always)]
    pub const fn soctrim6_3(&self) -> &Soctrim6_3 {
        &self.soctrim6_3
    }
    #[doc = "0x1f0 - SoC Trim Phrase 7 Word 0 Register"]
    #[inline(always)]
    pub const fn soctrim7_0(&self) -> &Soctrim7_0 {
        &self.soctrim7_0
    }
    #[doc = "0x1f4 - SoC Trim Phrase 7 Word 1 Register"]
    #[inline(always)]
    pub const fn soctrim7_1(&self) -> &Soctrim7_1 {
        &self.soctrim7_1
    }
    #[doc = "0x1f8 - SoC Trim Phrase 7 Word 2 Register"]
    #[inline(always)]
    pub const fn soctrim7_2(&self) -> &Soctrim7_2 {
        &self.soctrim7_2
    }
    #[doc = "0x1fc - SoC Trim Phrase 7 Word 3 Register"]
    #[inline(always)]
    pub const fn soctrim7_3(&self) -> &Soctrim7_3 {
        &self.soctrim7_3
    }
    #[doc = "0x204 - BIST Configuration Register"]
    #[inline(always)]
    pub const fn r_ip_config(&self) -> &RIpConfig {
        &self.r_ip_config
    }
    #[doc = "0x208 - BIST Test Code Register"]
    #[inline(always)]
    pub const fn r_testcode(&self) -> &RTestcode {
        &self.r_testcode
    }
    #[doc = "0x20c - BIST DFT Control Register"]
    #[inline(always)]
    pub const fn r_dft_ctrl(&self) -> &RDftCtrl {
        &self.r_dft_ctrl
    }
    #[doc = "0x210 - BIST Address Control Register"]
    #[inline(always)]
    pub const fn r_adr_ctrl(&self) -> &RAdrCtrl {
        &self.r_adr_ctrl
    }
    #[doc = "0x214 - BIST Data Control 0 Register"]
    #[inline(always)]
    pub const fn r_data_ctrl0(&self) -> &RDataCtrl0 {
        &self.r_data_ctrl0
    }
    #[doc = "0x218 - BIST Pin Control Register"]
    #[inline(always)]
    pub const fn r_pin_ctrl(&self) -> &RPinCtrl {
        &self.r_pin_ctrl
    }
    #[doc = "0x21c - BIST Loop Count Control Register"]
    #[inline(always)]
    pub const fn r_cnt_loop_ctrl(&self) -> &RCntLoopCtrl {
        &self.r_cnt_loop_ctrl
    }
    #[doc = "0x220 - BIST Timer Control Register"]
    #[inline(always)]
    pub const fn r_timer_ctrl(&self) -> &RTimerCtrl {
        &self.r_timer_ctrl
    }
    #[doc = "0x224 - BIST Test Control Register"]
    #[inline(always)]
    pub const fn r_test_ctrl(&self) -> &RTestCtrl {
        &self.r_test_ctrl
    }
    #[doc = "0x228 - BIST Abort Loop Register"]
    #[inline(always)]
    pub const fn r_abort_loop(&self) -> &RAbortLoop {
        &self.r_abort_loop
    }
    #[doc = "0x22c - BIST Address Query Register"]
    #[inline(always)]
    pub const fn r_adr_query(&self) -> &RAdrQuery {
        &self.r_adr_query
    }
    #[doc = "0x230 - BIST DOUT Query 0 Register"]
    #[inline(always)]
    pub const fn r_dout_query0(&self) -> &RDoutQuery0 {
        &self.r_dout_query0
    }
    #[doc = "0x23c - BIST SMW Query Register"]
    #[inline(always)]
    pub const fn r_smw_query(&self) -> &RSmwQuery {
        &self.r_smw_query
    }
    #[doc = "0x240 - BIST SMW Setting 0 Register"]
    #[inline(always)]
    pub const fn r_smw_setting0(&self) -> &RSmwSetting0 {
        &self.r_smw_setting0
    }
    #[doc = "0x244 - BIST SMW Setting 1 Register"]
    #[inline(always)]
    pub const fn r_smw_setting1(&self) -> &RSmwSetting1 {
        &self.r_smw_setting1
    }
    #[doc = "0x248 - BIST SMP WHV Setting 0 Register"]
    #[inline(always)]
    pub const fn r_smp_whv0(&self) -> &RSmpWhv0 {
        &self.r_smp_whv0
    }
    #[doc = "0x24c - BIST SMP WHV Setting 1 Register"]
    #[inline(always)]
    pub const fn r_smp_whv1(&self) -> &RSmpWhv1 {
        &self.r_smp_whv1
    }
    #[doc = "0x250 - BIST SME WHV Setting 0 Register"]
    #[inline(always)]
    pub const fn r_sme_whv0(&self) -> &RSmeWhv0 {
        &self.r_sme_whv0
    }
    #[doc = "0x254 - BIST SME WHV Setting 1 Register"]
    #[inline(always)]
    pub const fn r_sme_whv1(&self) -> &RSmeWhv1 {
        &self.r_sme_whv1
    }
    #[doc = "0x258 - BIST SMW Setting 2 Register"]
    #[inline(always)]
    pub const fn r_smw_setting2(&self) -> &RSmwSetting2 {
        &self.r_smw_setting2
    }
    #[doc = "0x25c - BIST DIN MISR 0 Register"]
    #[inline(always)]
    pub const fn r_d_misr0(&self) -> &RDMisr0 {
        &self.r_d_misr0
    }
    #[doc = "0x260 - BIST Address MISR 0 Register"]
    #[inline(always)]
    pub const fn r_a_misr0(&self) -> &RAMisr0 {
        &self.r_a_misr0
    }
    #[doc = "0x264 - BIST Control MISR 0 Register"]
    #[inline(always)]
    pub const fn r_c_misr0(&self) -> &RCMisr0 {
        &self.r_c_misr0
    }
    #[doc = "0x268 - BIST SMW Setting 3 Register"]
    #[inline(always)]
    pub const fn r_smw_setting3(&self) -> &RSmwSetting3 {
        &self.r_smw_setting3
    }
    #[doc = "0x26c - BIST Data Control 1 Register"]
    #[inline(always)]
    pub const fn r_data_ctrl1(&self) -> &RDataCtrl1 {
        &self.r_data_ctrl1
    }
    #[doc = "0x270 - BIST Data Control 2 Register"]
    #[inline(always)]
    pub const fn r_data_ctrl2(&self) -> &RDataCtrl2 {
        &self.r_data_ctrl2
    }
    #[doc = "0x274 - BIST Data Control 3 Register"]
    #[inline(always)]
    pub const fn r_data_ctrl3(&self) -> &RDataCtrl3 {
        &self.r_data_ctrl3
    }
    #[doc = "0x280 - BIST Repair 0 for Block 0 Register"]
    #[inline(always)]
    pub const fn r_repair0_0(&self) -> &RRepair0_0 {
        &self.r_repair0_0
    }
    #[doc = "0x284 - BIST Repair 1 Block 0 Register"]
    #[inline(always)]
    pub const fn r_repair0_1(&self) -> &RRepair0_1 {
        &self.r_repair0_1
    }
    #[doc = "0x288 - BIST Repair 0 Block 1 Register"]
    #[inline(always)]
    pub const fn r_repair1_0(&self) -> &RRepair1_0 {
        &self.r_repair1_0
    }
    #[doc = "0x28c - BIST Repair 1 Block 1 Register"]
    #[inline(always)]
    pub const fn r_repair1_1(&self) -> &RRepair1_1 {
        &self.r_repair1_1
    }
    #[doc = "0x314 - BIST Data Control 0 Extension Register"]
    #[inline(always)]
    pub const fn r_data_ctrl0_ex(&self) -> &RDataCtrl0Ex {
        &self.r_data_ctrl0_ex
    }
    #[doc = "0x320 - BIST Timer Control Extension Register"]
    #[inline(always)]
    pub const fn r_timer_ctrl_ex(&self) -> &RTimerCtrlEx {
        &self.r_timer_ctrl_ex
    }
    #[doc = "0x330 - BIST DOUT Query 1 Register"]
    #[inline(always)]
    pub const fn r_dout_query1(&self) -> &RDoutQuery1 {
        &self.r_dout_query1
    }
    #[doc = "0x35c - BIST DIN MISR 1 Register"]
    #[inline(always)]
    pub const fn r_d_misr1(&self) -> &RDMisr1 {
        &self.r_d_misr1
    }
    #[doc = "0x360 - BIST Address MISR 1 Register"]
    #[inline(always)]
    pub const fn r_a_misr1(&self) -> &RAMisr1 {
        &self.r_a_misr1
    }
    #[doc = "0x364 - BIST Control MISR 1 Register"]
    #[inline(always)]
    pub const fn r_c_misr1(&self) -> &RCMisr1 {
        &self.r_c_misr1
    }
    #[doc = "0x36c - BIST Data Control 1 Extension Register"]
    #[inline(always)]
    pub const fn r_data_ctrl1_ex(&self) -> &RDataCtrl1Ex {
        &self.r_data_ctrl1_ex
    }
    #[doc = "0x370 - BIST Data Control 2 Extension Register"]
    #[inline(always)]
    pub const fn r_data_ctrl2_ex(&self) -> &RDataCtrl2Ex {
        &self.r_data_ctrl2_ex
    }
    #[doc = "0x374 - BIST Data Control 3 Extension Register"]
    #[inline(always)]
    pub const fn r_data_ctrl3_ex(&self) -> &RDataCtrl3Ex {
        &self.r_data_ctrl3_ex
    }
    #[doc = "0x400 - SMW Timer Option Register"]
    #[inline(always)]
    pub const fn smw_timer_option(&self) -> &SmwTimerOption {
        &self.smw_timer_option
    }
    #[doc = "0x404 - SMW Setting Option 0 Register"]
    #[inline(always)]
    pub const fn smw_setting_option0(&self) -> &SmwSettingOption0 {
        &self.smw_setting_option0
    }
    #[doc = "0x408 - SMW Setting Option 2 Register"]
    #[inline(always)]
    pub const fn smw_setting_option2(&self) -> &SmwSettingOption2 {
        &self.smw_setting_option2
    }
    #[doc = "0x40c - SMW Setting Option 3 Register"]
    #[inline(always)]
    pub const fn smw_setting_option3(&self) -> &SmwSettingOption3 {
        &self.smw_setting_option3
    }
    #[doc = "0x410 - SMW SMP WHV Option 0 Register"]
    #[inline(always)]
    pub const fn smw_smp_whv_option0(&self) -> &SmwSmpWhvOption0 {
        &self.smw_smp_whv_option0
    }
    #[doc = "0x414 - SMW SME WHV Option 0 Register"]
    #[inline(always)]
    pub const fn smw_sme_whv_option0(&self) -> &SmwSmeWhvOption0 {
        &self.smw_sme_whv_option0
    }
    #[doc = "0x418 - SMW Setting Option 1 Register"]
    #[inline(always)]
    pub const fn smw_setting_option1(&self) -> &SmwSettingOption1 {
        &self.smw_setting_option1
    }
    #[doc = "0x41c - SMW SMP WHV Option 1 Register"]
    #[inline(always)]
    pub const fn smw_smp_whv_option1(&self) -> &SmwSmpWhvOption1 {
        &self.smw_smp_whv_option1
    }
    #[doc = "0x420 - SMW SME WHV Option 1 Register"]
    #[inline(always)]
    pub const fn smw_sme_whv_option1(&self) -> &SmwSmeWhvOption1 {
        &self.smw_sme_whv_option1
    }
    #[doc = "0x500 - FMU Repair 0 Block 0 Register"]
    #[inline(always)]
    pub const fn repair0_0(&self) -> &Repair0_0 {
        &self.repair0_0
    }
    #[doc = "0x504 - FMU Repair 1 Block 0 Register"]
    #[inline(always)]
    pub const fn repair0_1(&self) -> &Repair0_1 {
        &self.repair0_1
    }
    #[doc = "0x508 - FMU Repair 0 Block 1 Register"]
    #[inline(always)]
    pub const fn repair1_0(&self) -> &Repair1_0 {
        &self.repair1_0
    }
    #[doc = "0x50c - FMU Repair 1 Block 1 Register"]
    #[inline(always)]
    pub const fn repair1_1(&self) -> &Repair1_1 {
        &self.repair1_1
    }
    #[doc = "0x600 - SMW HB Signals Register"]
    #[inline(always)]
    pub const fn smw_hb_signals(&self) -> &SmwHbSignals {
        &self.smw_hb_signals
    }
    #[doc = "0x604 - BIST Datadump Control Register"]
    #[inline(always)]
    pub const fn bist_dump_ctrl(&self) -> &BistDumpCtrl {
        &self.bist_dump_ctrl
    }
    #[doc = "0x60c - ATX Pin Control Register"]
    #[inline(always)]
    pub const fn atx_pin_ctrl(&self) -> &AtxPinCtrl {
        &self.atx_pin_ctrl
    }
    #[doc = "0x610 - Fail Count Register"]
    #[inline(always)]
    pub const fn failcnt(&self) -> &Failcnt {
        &self.failcnt
    }
    #[doc = "0x614 - Block 0 Program Pulse Count Register"]
    #[inline(always)]
    pub const fn pgm_pulse_cnt0(&self) -> &PgmPulseCnt0 {
        &self.pgm_pulse_cnt0
    }
    #[doc = "0x618 - Block 1 Program Pulse Count Register"]
    #[inline(always)]
    pub const fn pgm_pulse_cnt1(&self) -> &PgmPulseCnt1 {
        &self.pgm_pulse_cnt1
    }
    #[doc = "0x61c - Erase Pulse Count Register"]
    #[inline(always)]
    pub const fn ers_pulse_cnt(&self) -> &ErsPulseCnt {
        &self.ers_pulse_cnt
    }
    #[doc = "0x620 - Maximum Pulse Count Register"]
    #[inline(always)]
    pub const fn max_pulse_cnt(&self) -> &MaxPulseCnt {
        &self.max_pulse_cnt
    }
    #[doc = "0x624 - Port Control Register"]
    #[inline(always)]
    pub const fn port_ctrl(&self) -> &PortCtrl {
        &self.port_ctrl
    }
}
#[doc = "FSTAT (rw) register accessor: Flash Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fstat`] module"]
#[doc(alias = "FSTAT")]
pub type Fstat = crate::Reg<fstat::FstatSpec>;
#[doc = "Flash Status Register"]
pub mod fstat;
#[doc = "FCNFG (rw) register accessor: Flash Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fcnfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcnfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcnfg`] module"]
#[doc(alias = "FCNFG")]
pub type Fcnfg = crate::Reg<fcnfg::FcnfgSpec>;
#[doc = "Flash Configuration Register"]
pub mod fcnfg;
#[doc = "FCTRL (rw) register accessor: Flash Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fctrl`] module"]
#[doc(alias = "FCTRL")]
pub type Fctrl = crate::Reg<fctrl::FctrlSpec>;
#[doc = "Flash Control Register"]
pub mod fctrl;
#[doc = "FTEST (r) register accessor: Flash Test Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ftest::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ftest`] module"]
#[doc(alias = "FTEST")]
pub type Ftest = crate::Reg<ftest::FtestSpec>;
#[doc = "Flash Test Register"]
pub mod ftest;
#[doc = "FCCOB0 (rw) register accessor: Flash Command Control 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fccob0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fccob0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fccob0`] module"]
#[doc(alias = "FCCOB0")]
pub type Fccob0 = crate::Reg<fccob0::Fccob0Spec>;
#[doc = "Flash Command Control 0 Register"]
pub mod fccob0;
#[doc = "FCCOB1 (rw) register accessor: Flash Command Control 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fccob1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fccob1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fccob1`] module"]
#[doc(alias = "FCCOB1")]
pub type Fccob1 = crate::Reg<fccob1::Fccob1Spec>;
#[doc = "Flash Command Control 1 Register"]
pub mod fccob1;
#[doc = "FCCOB2 (rw) register accessor: Flash Command Control 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fccob2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fccob2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fccob2`] module"]
#[doc(alias = "FCCOB2")]
pub type Fccob2 = crate::Reg<fccob2::Fccob2Spec>;
#[doc = "Flash Command Control 2 Register"]
pub mod fccob2;
#[doc = "FCCOB3 (rw) register accessor: Flash Command Control 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fccob3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fccob3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fccob3`] module"]
#[doc(alias = "FCCOB3")]
pub type Fccob3 = crate::Reg<fccob3::Fccob3Spec>;
#[doc = "Flash Command Control 3 Register"]
pub mod fccob3;
#[doc = "FCCOB4 (rw) register accessor: Flash Command Control 4 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fccob4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fccob4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fccob4`] module"]
#[doc(alias = "FCCOB4")]
pub type Fccob4 = crate::Reg<fccob4::Fccob4Spec>;
#[doc = "Flash Command Control 4 Register"]
pub mod fccob4;
#[doc = "FCCOB5 (rw) register accessor: Flash Command Control 5 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fccob5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fccob5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fccob5`] module"]
#[doc(alias = "FCCOB5")]
pub type Fccob5 = crate::Reg<fccob5::Fccob5Spec>;
#[doc = "Flash Command Control 5 Register"]
pub mod fccob5;
#[doc = "FCCOB6 (rw) register accessor: Flash Command Control 6 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fccob6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fccob6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fccob6`] module"]
#[doc(alias = "FCCOB6")]
pub type Fccob6 = crate::Reg<fccob6::Fccob6Spec>;
#[doc = "Flash Command Control 6 Register"]
pub mod fccob6;
#[doc = "FCCOB7 (rw) register accessor: Flash Command Control 7 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fccob7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fccob7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fccob7`] module"]
#[doc(alias = "FCCOB7")]
pub type Fccob7 = crate::Reg<fccob7::Fccob7Spec>;
#[doc = "Flash Command Control 7 Register"]
pub mod fccob7;
#[doc = "RESET_STATUS (rw) register accessor: FMU Initialization Tracking Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reset_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reset_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reset_status`] module"]
#[doc(alias = "RESET_STATUS")]
pub type ResetStatus = crate::Reg<reset_status::ResetStatusSpec>;
#[doc = "FMU Initialization Tracking Register"]
pub mod reset_status;
#[doc = "MCTL (rw) register accessor: FMU Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mctl`] module"]
#[doc(alias = "MCTL")]
pub type Mctl = crate::Reg<mctl::MctlSpec>;
#[doc = "FMU Control Register"]
pub mod mctl;
#[doc = "BSEL_GEN (r) register accessor: FMU Block Select Generation Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bsel_gen::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bsel_gen`] module"]
#[doc(alias = "BSEL_GEN")]
pub type BselGen = crate::Reg<bsel_gen::BselGenSpec>;
#[doc = "FMU Block Select Generation Register"]
pub mod bsel_gen;
#[doc = "PWR_OPT (rw) register accessor: Power Mode Options Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwr_opt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_opt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_opt`] module"]
#[doc(alias = "PWR_OPT")]
pub type PwrOpt = crate::Reg<pwr_opt::PwrOptSpec>;
#[doc = "Power Mode Options Register"]
pub mod pwr_opt;
#[doc = "CMD_CHECK (r) register accessor: FMU Command Check Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd_check::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd_check`] module"]
#[doc(alias = "CMD_CHECK")]
pub type CmdCheck = crate::Reg<cmd_check::CmdCheckSpec>;
#[doc = "FMU Command Check Register"]
pub mod cmd_check;
#[doc = "BSEL (rw) register accessor: FMU Block Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bsel`] module"]
#[doc(alias = "BSEL")]
pub type Bsel = crate::Reg<bsel::BselSpec>;
#[doc = "FMU Block Select Register"]
pub mod bsel;
#[doc = "MSIZE (rw) register accessor: FMU Memory Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`msize::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`msize::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msize`] module"]
#[doc(alias = "MSIZE")]
pub type Msize = crate::Reg<msize::MsizeSpec>;
#[doc = "FMU Memory Size Register"]
pub mod msize;
#[doc = "FLASH_RD_ADD (rw) register accessor: Flash Read Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_rd_add::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_rd_add::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_rd_add`] module"]
#[doc(alias = "FLASH_RD_ADD")]
pub type FlashRdAdd = crate::Reg<flash_rd_add::FlashRdAddSpec>;
#[doc = "Flash Read Address Register"]
pub mod flash_rd_add;
#[doc = "FLASH_STOP_ADD (rw) register accessor: Flash Stop Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_stop_add::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_stop_add::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_stop_add`] module"]
#[doc(alias = "FLASH_STOP_ADD")]
pub type FlashStopAdd = crate::Reg<flash_stop_add::FlashStopAddSpec>;
#[doc = "Flash Stop Address Register"]
pub mod flash_stop_add;
#[doc = "FLASH_RD_CTRL (rw) register accessor: Flash Read Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_rd_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_rd_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_rd_ctrl`] module"]
#[doc(alias = "FLASH_RD_CTRL")]
pub type FlashRdCtrl = crate::Reg<flash_rd_ctrl::FlashRdCtrlSpec>;
#[doc = "Flash Read Control Register"]
pub mod flash_rd_ctrl;
#[doc = "MM_ADDR (rw) register accessor: Memory Map Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mm_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mm_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mm_addr`] module"]
#[doc(alias = "MM_ADDR")]
pub type MmAddr = crate::Reg<mm_addr::MmAddrSpec>;
#[doc = "Memory Map Address Register"]
pub mod mm_addr;
#[doc = "MM_WDATA (rw) register accessor: Memory Map Write Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mm_wdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mm_wdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mm_wdata`] module"]
#[doc(alias = "MM_WDATA")]
pub type MmWdata = crate::Reg<mm_wdata::MmWdataSpec>;
#[doc = "Memory Map Write Data Register"]
pub mod mm_wdata;
#[doc = "MM_CTL (rw) register accessor: Memory Map Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mm_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mm_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mm_ctl`] module"]
#[doc(alias = "MM_CTL")]
pub type MmCtl = crate::Reg<mm_ctl::MmCtlSpec>;
#[doc = "Memory Map Control Register"]
pub mod mm_ctl;
#[doc = "UINT_CTL (rw) register accessor: User Interface Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uint_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uint_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uint_ctl`] module"]
#[doc(alias = "UINT_CTL")]
pub type UintCtl = crate::Reg<uint_ctl::UintCtlSpec>;
#[doc = "User Interface Control Register"]
pub mod uint_ctl;
#[doc = "RD_DATA0 (rw) register accessor: Read Data 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_data0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rd_data0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_data0`] module"]
#[doc(alias = "RD_DATA0")]
pub type RdData0 = crate::Reg<rd_data0::RdData0Spec>;
#[doc = "Read Data 0 Register"]
pub mod rd_data0;
#[doc = "RD_DATA1 (rw) register accessor: Read Data 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_data1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rd_data1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_data1`] module"]
#[doc(alias = "RD_DATA1")]
pub type RdData1 = crate::Reg<rd_data1::RdData1Spec>;
#[doc = "Read Data 1 Register"]
pub mod rd_data1;
#[doc = "RD_DATA2 (rw) register accessor: Read Data 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_data2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rd_data2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_data2`] module"]
#[doc(alias = "RD_DATA2")]
pub type RdData2 = crate::Reg<rd_data2::RdData2Spec>;
#[doc = "Read Data 2 Register"]
pub mod rd_data2;
#[doc = "RD_DATA3 (rw) register accessor: Read Data 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_data3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rd_data3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_data3`] module"]
#[doc(alias = "RD_DATA3")]
pub type RdData3 = crate::Reg<rd_data3::RdData3Spec>;
#[doc = "Read Data 3 Register"]
pub mod rd_data3;
#[doc = "PARITY (rw) register accessor: Parity Register\n\nYou can [`read`](crate::Reg::read) this register and get [`parity::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`parity::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@parity`] module"]
#[doc(alias = "PARITY")]
pub type Parity = crate::Reg<parity::ParitySpec>;
#[doc = "Parity Register"]
pub mod parity;
#[doc = "RD_PATH_CTRL_STATUS (rw) register accessor: Read Path Control and Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_path_ctrl_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rd_path_ctrl_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_path_ctrl_status`] module"]
#[doc(alias = "RD_PATH_CTRL_STATUS")]
pub type RdPathCtrlStatus = crate::Reg<rd_path_ctrl_status::RdPathCtrlStatusSpec>;
#[doc = "Read Path Control and Status Register"]
pub mod rd_path_ctrl_status;
#[doc = "SMW_DIN0 (rw) register accessor: SMW DIN 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`smw_din0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smw_din0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smw_din0`] module"]
#[doc(alias = "SMW_DIN0")]
pub type SmwDin0 = crate::Reg<smw_din0::SmwDin0Spec>;
#[doc = "SMW DIN 0 Register"]
pub mod smw_din0;
#[doc = "SMW_DIN1 (rw) register accessor: SMW DIN 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`smw_din1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smw_din1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smw_din1`] module"]
#[doc(alias = "SMW_DIN1")]
pub type SmwDin1 = crate::Reg<smw_din1::SmwDin1Spec>;
#[doc = "SMW DIN 1 Register"]
pub mod smw_din1;
#[doc = "SMW_DIN2 (rw) register accessor: SMW DIN 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`smw_din2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smw_din2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smw_din2`] module"]
#[doc(alias = "SMW_DIN2")]
pub type SmwDin2 = crate::Reg<smw_din2::SmwDin2Spec>;
#[doc = "SMW DIN 2 Register"]
pub mod smw_din2;
#[doc = "SMW_DIN3 (rw) register accessor: SMW DIN 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`smw_din3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smw_din3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smw_din3`] module"]
#[doc(alias = "SMW_DIN3")]
pub type SmwDin3 = crate::Reg<smw_din3::SmwDin3Spec>;
#[doc = "SMW DIN 3 Register"]
pub mod smw_din3;
#[doc = "SMW_ADDR (rw) register accessor: SMW Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`smw_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smw_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smw_addr`] module"]
#[doc(alias = "SMW_ADDR")]
pub type SmwAddr = crate::Reg<smw_addr::SmwAddrSpec>;
#[doc = "SMW Address Register"]
pub mod smw_addr;
#[doc = "SMW_CMD_WAIT (rw) register accessor: SMW Command and Wait Register\n\nYou can [`read`](crate::Reg::read) this register and get [`smw_cmd_wait::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smw_cmd_wait::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smw_cmd_wait`] module"]
#[doc(alias = "SMW_CMD_WAIT")]
pub type SmwCmdWait = crate::Reg<smw_cmd_wait::SmwCmdWaitSpec>;
#[doc = "SMW Command and Wait Register"]
pub mod smw_cmd_wait;
#[doc = "SMW_STATUS (r) register accessor: SMW Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`smw_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smw_status`] module"]
#[doc(alias = "SMW_STATUS")]
pub type SmwStatus = crate::Reg<smw_status::SmwStatusSpec>;
#[doc = "SMW Status Register"]
pub mod smw_status;
#[doc = "SOCTRIM0_0 (rw) register accessor: SoC Trim Phrase 0 Word 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`soctrim0_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soctrim0_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soctrim0_0`] module"]
#[doc(alias = "SOCTRIM0_0")]
pub type Soctrim0_0 = crate::Reg<soctrim0_0::Soctrim0_0Spec>;
#[doc = "SoC Trim Phrase 0 Word 0 Register"]
pub mod soctrim0_0;
#[doc = "SOCTRIM0_1 (rw) register accessor: SoC Trim Phrase 0 Word 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`soctrim0_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soctrim0_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soctrim0_1`] module"]
#[doc(alias = "SOCTRIM0_1")]
pub type Soctrim0_1 = crate::Reg<soctrim0_1::Soctrim0_1Spec>;
#[doc = "SoC Trim Phrase 0 Word 1 Register"]
pub mod soctrim0_1;
#[doc = "SOCTRIM0_2 (rw) register accessor: SoC Trim Phrase 0 Word 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`soctrim0_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soctrim0_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soctrim0_2`] module"]
#[doc(alias = "SOCTRIM0_2")]
pub type Soctrim0_2 = crate::Reg<soctrim0_2::Soctrim0_2Spec>;
#[doc = "SoC Trim Phrase 0 Word 2 Register"]
pub mod soctrim0_2;
#[doc = "SOCTRIM0_3 (rw) register accessor: SoC Trim Phrase 0 Word 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`soctrim0_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soctrim0_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soctrim0_3`] module"]
#[doc(alias = "SOCTRIM0_3")]
pub type Soctrim0_3 = crate::Reg<soctrim0_3::Soctrim0_3Spec>;
#[doc = "SoC Trim Phrase 0 Word 3 Register"]
pub mod soctrim0_3;
#[doc = "SOCTRIM1_0 (rw) register accessor: SoC Trim Phrase 1 Word 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`soctrim1_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soctrim1_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soctrim1_0`] module"]
#[doc(alias = "SOCTRIM1_0")]
pub type Soctrim1_0 = crate::Reg<soctrim1_0::Soctrim1_0Spec>;
#[doc = "SoC Trim Phrase 1 Word 0 Register"]
pub mod soctrim1_0;
#[doc = "SOCTRIM1_1 (rw) register accessor: SoC Trim Phrase 1 Word 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`soctrim1_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soctrim1_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soctrim1_1`] module"]
#[doc(alias = "SOCTRIM1_1")]
pub type Soctrim1_1 = crate::Reg<soctrim1_1::Soctrim1_1Spec>;
#[doc = "SoC Trim Phrase 1 Word 1 Register"]
pub mod soctrim1_1;
#[doc = "SOCTRIM1_2 (rw) register accessor: SoC Trim Phrase 1 Word 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`soctrim1_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soctrim1_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soctrim1_2`] module"]
#[doc(alias = "SOCTRIM1_2")]
pub type Soctrim1_2 = crate::Reg<soctrim1_2::Soctrim1_2Spec>;
#[doc = "SoC Trim Phrase 1 Word 2 Register"]
pub mod soctrim1_2;
#[doc = "SOCTRIM1_3 (rw) register accessor: SoC Trim Phrase 1 Word 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`soctrim1_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soctrim1_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soctrim1_3`] module"]
#[doc(alias = "SOCTRIM1_3")]
pub type Soctrim1_3 = crate::Reg<soctrim1_3::Soctrim1_3Spec>;
#[doc = "SoC Trim Phrase 1 Word 3 Register"]
pub mod soctrim1_3;
#[doc = "SOCTRIM2_0 (rw) register accessor: SoC Trim Phrase 2 Word 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`soctrim2_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soctrim2_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soctrim2_0`] module"]
#[doc(alias = "SOCTRIM2_0")]
pub type Soctrim2_0 = crate::Reg<soctrim2_0::Soctrim2_0Spec>;
#[doc = "SoC Trim Phrase 2 Word 0 Register"]
pub mod soctrim2_0;
#[doc = "SOCTRIM2_1 (rw) register accessor: SoC Trim Phrase 2 Word 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`soctrim2_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soctrim2_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soctrim2_1`] module"]
#[doc(alias = "SOCTRIM2_1")]
pub type Soctrim2_1 = crate::Reg<soctrim2_1::Soctrim2_1Spec>;
#[doc = "SoC Trim Phrase 2 Word 1 Register"]
pub mod soctrim2_1;
#[doc = "SOCTRIM2_2 (rw) register accessor: SoC Trim Phrase 2 Word 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`soctrim2_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soctrim2_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soctrim2_2`] module"]
#[doc(alias = "SOCTRIM2_2")]
pub type Soctrim2_2 = crate::Reg<soctrim2_2::Soctrim2_2Spec>;
#[doc = "SoC Trim Phrase 2 Word 2 Register"]
pub mod soctrim2_2;
#[doc = "SOCTRIM2_3 (rw) register accessor: SoC Trim Phrase 2 Word 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`soctrim2_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soctrim2_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soctrim2_3`] module"]
#[doc(alias = "SOCTRIM2_3")]
pub type Soctrim2_3 = crate::Reg<soctrim2_3::Soctrim2_3Spec>;
#[doc = "SoC Trim Phrase 2 Word 3 Register"]
pub mod soctrim2_3;
#[doc = "SOCTRIM3_0 (rw) register accessor: SoC Trim Phrase 3 Word 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`soctrim3_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soctrim3_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soctrim3_0`] module"]
#[doc(alias = "SOCTRIM3_0")]
pub type Soctrim3_0 = crate::Reg<soctrim3_0::Soctrim3_0Spec>;
#[doc = "SoC Trim Phrase 3 Word 0 Register"]
pub mod soctrim3_0;
#[doc = "SOCTRIM3_1 (rw) register accessor: SoC Trim Phrase 3 Word 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`soctrim3_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soctrim3_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soctrim3_1`] module"]
#[doc(alias = "SOCTRIM3_1")]
pub type Soctrim3_1 = crate::Reg<soctrim3_1::Soctrim3_1Spec>;
#[doc = "SoC Trim Phrase 3 Word 1 Register"]
pub mod soctrim3_1;
#[doc = "SOCTRIM3_2 (rw) register accessor: SoC Trim Phrase 3 Word 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`soctrim3_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soctrim3_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soctrim3_2`] module"]
#[doc(alias = "SOCTRIM3_2")]
pub type Soctrim3_2 = crate::Reg<soctrim3_2::Soctrim3_2Spec>;
#[doc = "SoC Trim Phrase 3 Word 2 Register"]
pub mod soctrim3_2;
#[doc = "SOCTRIM3_3 (rw) register accessor: SoC Trim Phrase 3 Word 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`soctrim3_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soctrim3_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soctrim3_3`] module"]
#[doc(alias = "SOCTRIM3_3")]
pub type Soctrim3_3 = crate::Reg<soctrim3_3::Soctrim3_3Spec>;
#[doc = "SoC Trim Phrase 3 Word 3 Register"]
pub mod soctrim3_3;
#[doc = "SOCTRIM4_0 (rw) register accessor: SoC Trim Phrase 4 Word 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`soctrim4_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soctrim4_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soctrim4_0`] module"]
#[doc(alias = "SOCTRIM4_0")]
pub type Soctrim4_0 = crate::Reg<soctrim4_0::Soctrim4_0Spec>;
#[doc = "SoC Trim Phrase 4 Word 0 Register"]
pub mod soctrim4_0;
#[doc = "SOCTRIM4_1 (rw) register accessor: SoC Trim Phrase 4 Word 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`soctrim4_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soctrim4_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soctrim4_1`] module"]
#[doc(alias = "SOCTRIM4_1")]
pub type Soctrim4_1 = crate::Reg<soctrim4_1::Soctrim4_1Spec>;
#[doc = "SoC Trim Phrase 4 Word 1 Register"]
pub mod soctrim4_1;
#[doc = "SOCTRIM4_2 (rw) register accessor: SoC Trim Phrase 4 Word 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`soctrim4_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soctrim4_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soctrim4_2`] module"]
#[doc(alias = "SOCTRIM4_2")]
pub type Soctrim4_2 = crate::Reg<soctrim4_2::Soctrim4_2Spec>;
#[doc = "SoC Trim Phrase 4 Word 2 Register"]
pub mod soctrim4_2;
#[doc = "SOCTRIM4_3 (rw) register accessor: SoC Trim Phrase 4 Word 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`soctrim4_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soctrim4_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soctrim4_3`] module"]
#[doc(alias = "SOCTRIM4_3")]
pub type Soctrim4_3 = crate::Reg<soctrim4_3::Soctrim4_3Spec>;
#[doc = "SoC Trim Phrase 4 Word 3 Register"]
pub mod soctrim4_3;
#[doc = "SOCTRIM5_0 (rw) register accessor: SoC Trim Phrase 5 Word 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`soctrim5_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soctrim5_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soctrim5_0`] module"]
#[doc(alias = "SOCTRIM5_0")]
pub type Soctrim5_0 = crate::Reg<soctrim5_0::Soctrim5_0Spec>;
#[doc = "SoC Trim Phrase 5 Word 0 Register"]
pub mod soctrim5_0;
#[doc = "SOCTRIM5_1 (rw) register accessor: SoC Trim Phrase 5 Word 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`soctrim5_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soctrim5_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soctrim5_1`] module"]
#[doc(alias = "SOCTRIM5_1")]
pub type Soctrim5_1 = crate::Reg<soctrim5_1::Soctrim5_1Spec>;
#[doc = "SoC Trim Phrase 5 Word 1 Register"]
pub mod soctrim5_1;
#[doc = "SOCTRIM5_2 (rw) register accessor: SoC Trim Phrase 5 Word 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`soctrim5_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soctrim5_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soctrim5_2`] module"]
#[doc(alias = "SOCTRIM5_2")]
pub type Soctrim5_2 = crate::Reg<soctrim5_2::Soctrim5_2Spec>;
#[doc = "SoC Trim Phrase 5 Word 2 Register"]
pub mod soctrim5_2;
#[doc = "SOCTRIM5_3 (rw) register accessor: SoC Trim Phrase 5 Word 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`soctrim5_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soctrim5_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soctrim5_3`] module"]
#[doc(alias = "SOCTRIM5_3")]
pub type Soctrim5_3 = crate::Reg<soctrim5_3::Soctrim5_3Spec>;
#[doc = "SoC Trim Phrase 5 Word 3 Register"]
pub mod soctrim5_3;
#[doc = "SOCTRIM6_0 (rw) register accessor: SoC Trim Phrase 6 Word 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`soctrim6_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soctrim6_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soctrim6_0`] module"]
#[doc(alias = "SOCTRIM6_0")]
pub type Soctrim6_0 = crate::Reg<soctrim6_0::Soctrim6_0Spec>;
#[doc = "SoC Trim Phrase 6 Word 0 Register"]
pub mod soctrim6_0;
#[doc = "SOCTRIM6_1 (rw) register accessor: SoC Trim Phrase 6 Word 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`soctrim6_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soctrim6_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soctrim6_1`] module"]
#[doc(alias = "SOCTRIM6_1")]
pub type Soctrim6_1 = crate::Reg<soctrim6_1::Soctrim6_1Spec>;
#[doc = "SoC Trim Phrase 6 Word 1 Register"]
pub mod soctrim6_1;
#[doc = "SOCTRIM6_2 (rw) register accessor: SoC Trim Phrase 6 Word 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`soctrim6_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soctrim6_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soctrim6_2`] module"]
#[doc(alias = "SOCTRIM6_2")]
pub type Soctrim6_2 = crate::Reg<soctrim6_2::Soctrim6_2Spec>;
#[doc = "SoC Trim Phrase 6 Word 2 Register"]
pub mod soctrim6_2;
#[doc = "SOCTRIM6_3 (rw) register accessor: SoC Trim Phrase 6 Word 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`soctrim6_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soctrim6_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soctrim6_3`] module"]
#[doc(alias = "SOCTRIM6_3")]
pub type Soctrim6_3 = crate::Reg<soctrim6_3::Soctrim6_3Spec>;
#[doc = "SoC Trim Phrase 6 Word 3 Register"]
pub mod soctrim6_3;
#[doc = "SOCTRIM7_0 (rw) register accessor: SoC Trim Phrase 7 Word 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`soctrim7_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soctrim7_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soctrim7_0`] module"]
#[doc(alias = "SOCTRIM7_0")]
pub type Soctrim7_0 = crate::Reg<soctrim7_0::Soctrim7_0Spec>;
#[doc = "SoC Trim Phrase 7 Word 0 Register"]
pub mod soctrim7_0;
#[doc = "SOCTRIM7_1 (rw) register accessor: SoC Trim Phrase 7 Word 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`soctrim7_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soctrim7_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soctrim7_1`] module"]
#[doc(alias = "SOCTRIM7_1")]
pub type Soctrim7_1 = crate::Reg<soctrim7_1::Soctrim7_1Spec>;
#[doc = "SoC Trim Phrase 7 Word 1 Register"]
pub mod soctrim7_1;
#[doc = "SOCTRIM7_2 (rw) register accessor: SoC Trim Phrase 7 Word 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`soctrim7_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soctrim7_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soctrim7_2`] module"]
#[doc(alias = "SOCTRIM7_2")]
pub type Soctrim7_2 = crate::Reg<soctrim7_2::Soctrim7_2Spec>;
#[doc = "SoC Trim Phrase 7 Word 2 Register"]
pub mod soctrim7_2;
#[doc = "SOCTRIM7_3 (rw) register accessor: SoC Trim Phrase 7 Word 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`soctrim7_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soctrim7_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soctrim7_3`] module"]
#[doc(alias = "SOCTRIM7_3")]
pub type Soctrim7_3 = crate::Reg<soctrim7_3::Soctrim7_3Spec>;
#[doc = "SoC Trim Phrase 7 Word 3 Register"]
pub mod soctrim7_3;
#[doc = "R_IP_CONFIG (rw) register accessor: BIST Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r_ip_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r_ip_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r_ip_config`] module"]
#[doc(alias = "R_IP_CONFIG")]
pub type RIpConfig = crate::Reg<r_ip_config::RIpConfigSpec>;
#[doc = "BIST Configuration Register"]
pub mod r_ip_config;
#[doc = "R_TESTCODE (rw) register accessor: BIST Test Code Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r_testcode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r_testcode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r_testcode`] module"]
#[doc(alias = "R_TESTCODE")]
pub type RTestcode = crate::Reg<r_testcode::RTestcodeSpec>;
#[doc = "BIST Test Code Register"]
pub mod r_testcode;
#[doc = "R_DFT_CTRL (rw) register accessor: BIST DFT Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r_dft_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r_dft_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r_dft_ctrl`] module"]
#[doc(alias = "R_DFT_CTRL")]
pub type RDftCtrl = crate::Reg<r_dft_ctrl::RDftCtrlSpec>;
#[doc = "BIST DFT Control Register"]
pub mod r_dft_ctrl;
#[doc = "R_ADR_CTRL (rw) register accessor: BIST Address Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r_adr_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r_adr_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r_adr_ctrl`] module"]
#[doc(alias = "R_ADR_CTRL")]
pub type RAdrCtrl = crate::Reg<r_adr_ctrl::RAdrCtrlSpec>;
#[doc = "BIST Address Control Register"]
pub mod r_adr_ctrl;
#[doc = "R_DATA_CTRL0 (rw) register accessor: BIST Data Control 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r_data_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r_data_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r_data_ctrl0`] module"]
#[doc(alias = "R_DATA_CTRL0")]
pub type RDataCtrl0 = crate::Reg<r_data_ctrl0::RDataCtrl0Spec>;
#[doc = "BIST Data Control 0 Register"]
pub mod r_data_ctrl0;
#[doc = "R_PIN_CTRL (rw) register accessor: BIST Pin Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r_pin_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r_pin_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r_pin_ctrl`] module"]
#[doc(alias = "R_PIN_CTRL")]
pub type RPinCtrl = crate::Reg<r_pin_ctrl::RPinCtrlSpec>;
#[doc = "BIST Pin Control Register"]
pub mod r_pin_ctrl;
#[doc = "R_CNT_LOOP_CTRL (rw) register accessor: BIST Loop Count Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r_cnt_loop_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r_cnt_loop_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r_cnt_loop_ctrl`] module"]
#[doc(alias = "R_CNT_LOOP_CTRL")]
pub type RCntLoopCtrl = crate::Reg<r_cnt_loop_ctrl::RCntLoopCtrlSpec>;
#[doc = "BIST Loop Count Control Register"]
pub mod r_cnt_loop_ctrl;
#[doc = "R_TIMER_CTRL (rw) register accessor: BIST Timer Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r_timer_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r_timer_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r_timer_ctrl`] module"]
#[doc(alias = "R_TIMER_CTRL")]
pub type RTimerCtrl = crate::Reg<r_timer_ctrl::RTimerCtrlSpec>;
#[doc = "BIST Timer Control Register"]
pub mod r_timer_ctrl;
#[doc = "R_TEST_CTRL (rw) register accessor: BIST Test Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r_test_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r_test_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r_test_ctrl`] module"]
#[doc(alias = "R_TEST_CTRL")]
pub type RTestCtrl = crate::Reg<r_test_ctrl::RTestCtrlSpec>;
#[doc = "BIST Test Control Register"]
pub mod r_test_ctrl;
#[doc = "R_ABORT_LOOP (rw) register accessor: BIST Abort Loop Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r_abort_loop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r_abort_loop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r_abort_loop`] module"]
#[doc(alias = "R_ABORT_LOOP")]
pub type RAbortLoop = crate::Reg<r_abort_loop::RAbortLoopSpec>;
#[doc = "BIST Abort Loop Register"]
pub mod r_abort_loop;
#[doc = "R_ADR_QUERY (r) register accessor: BIST Address Query Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r_adr_query::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r_adr_query`] module"]
#[doc(alias = "R_ADR_QUERY")]
pub type RAdrQuery = crate::Reg<r_adr_query::RAdrQuerySpec>;
#[doc = "BIST Address Query Register"]
pub mod r_adr_query;
#[doc = "R_DOUT_QUERY0 (r) register accessor: BIST DOUT Query 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r_dout_query0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r_dout_query0`] module"]
#[doc(alias = "R_DOUT_QUERY0")]
pub type RDoutQuery0 = crate::Reg<r_dout_query0::RDoutQuery0Spec>;
#[doc = "BIST DOUT Query 0 Register"]
pub mod r_dout_query0;
#[doc = "R_SMW_QUERY (r) register accessor: BIST SMW Query Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r_smw_query::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r_smw_query`] module"]
#[doc(alias = "R_SMW_QUERY")]
pub type RSmwQuery = crate::Reg<r_smw_query::RSmwQuerySpec>;
#[doc = "BIST SMW Query Register"]
pub mod r_smw_query;
#[doc = "R_SMW_SETTING0 (rw) register accessor: BIST SMW Setting 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r_smw_setting0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r_smw_setting0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r_smw_setting0`] module"]
#[doc(alias = "R_SMW_SETTING0")]
pub type RSmwSetting0 = crate::Reg<r_smw_setting0::RSmwSetting0Spec>;
#[doc = "BIST SMW Setting 0 Register"]
pub mod r_smw_setting0;
#[doc = "R_SMW_SETTING1 (rw) register accessor: BIST SMW Setting 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r_smw_setting1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r_smw_setting1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r_smw_setting1`] module"]
#[doc(alias = "R_SMW_SETTING1")]
pub type RSmwSetting1 = crate::Reg<r_smw_setting1::RSmwSetting1Spec>;
#[doc = "BIST SMW Setting 1 Register"]
pub mod r_smw_setting1;
#[doc = "R_SMP_WHV0 (rw) register accessor: BIST SMP WHV Setting 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r_smp_whv0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r_smp_whv0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r_smp_whv0`] module"]
#[doc(alias = "R_SMP_WHV0")]
pub type RSmpWhv0 = crate::Reg<r_smp_whv0::RSmpWhv0Spec>;
#[doc = "BIST SMP WHV Setting 0 Register"]
pub mod r_smp_whv0;
#[doc = "R_SMP_WHV1 (rw) register accessor: BIST SMP WHV Setting 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r_smp_whv1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r_smp_whv1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r_smp_whv1`] module"]
#[doc(alias = "R_SMP_WHV1")]
pub type RSmpWhv1 = crate::Reg<r_smp_whv1::RSmpWhv1Spec>;
#[doc = "BIST SMP WHV Setting 1 Register"]
pub mod r_smp_whv1;
#[doc = "R_SME_WHV0 (rw) register accessor: BIST SME WHV Setting 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r_sme_whv0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r_sme_whv0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r_sme_whv0`] module"]
#[doc(alias = "R_SME_WHV0")]
pub type RSmeWhv0 = crate::Reg<r_sme_whv0::RSmeWhv0Spec>;
#[doc = "BIST SME WHV Setting 0 Register"]
pub mod r_sme_whv0;
#[doc = "R_SME_WHV1 (rw) register accessor: BIST SME WHV Setting 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r_sme_whv1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r_sme_whv1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r_sme_whv1`] module"]
#[doc(alias = "R_SME_WHV1")]
pub type RSmeWhv1 = crate::Reg<r_sme_whv1::RSmeWhv1Spec>;
#[doc = "BIST SME WHV Setting 1 Register"]
pub mod r_sme_whv1;
#[doc = "R_SMW_SETTING2 (rw) register accessor: BIST SMW Setting 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r_smw_setting2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r_smw_setting2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r_smw_setting2`] module"]
#[doc(alias = "R_SMW_SETTING2")]
pub type RSmwSetting2 = crate::Reg<r_smw_setting2::RSmwSetting2Spec>;
#[doc = "BIST SMW Setting 2 Register"]
pub mod r_smw_setting2;
#[doc = "R_D_MISR0 (r) register accessor: BIST DIN MISR 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r_d_misr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r_d_misr0`] module"]
#[doc(alias = "R_D_MISR0")]
pub type RDMisr0 = crate::Reg<r_d_misr0::RDMisr0Spec>;
#[doc = "BIST DIN MISR 0 Register"]
pub mod r_d_misr0;
#[doc = "R_A_MISR0 (r) register accessor: BIST Address MISR 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r_a_misr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r_a_misr0`] module"]
#[doc(alias = "R_A_MISR0")]
pub type RAMisr0 = crate::Reg<r_a_misr0::RAMisr0Spec>;
#[doc = "BIST Address MISR 0 Register"]
pub mod r_a_misr0;
#[doc = "R_C_MISR0 (r) register accessor: BIST Control MISR 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r_c_misr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r_c_misr0`] module"]
#[doc(alias = "R_C_MISR0")]
pub type RCMisr0 = crate::Reg<r_c_misr0::RCMisr0Spec>;
#[doc = "BIST Control MISR 0 Register"]
pub mod r_c_misr0;
#[doc = "R_SMW_SETTING3 (rw) register accessor: BIST SMW Setting 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r_smw_setting3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r_smw_setting3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r_smw_setting3`] module"]
#[doc(alias = "R_SMW_SETTING3")]
pub type RSmwSetting3 = crate::Reg<r_smw_setting3::RSmwSetting3Spec>;
#[doc = "BIST SMW Setting 3 Register"]
pub mod r_smw_setting3;
#[doc = "R_DATA_CTRL1 (rw) register accessor: BIST Data Control 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r_data_ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r_data_ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r_data_ctrl1`] module"]
#[doc(alias = "R_DATA_CTRL1")]
pub type RDataCtrl1 = crate::Reg<r_data_ctrl1::RDataCtrl1Spec>;
#[doc = "BIST Data Control 1 Register"]
pub mod r_data_ctrl1;
#[doc = "R_DATA_CTRL2 (rw) register accessor: BIST Data Control 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r_data_ctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r_data_ctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r_data_ctrl2`] module"]
#[doc(alias = "R_DATA_CTRL2")]
pub type RDataCtrl2 = crate::Reg<r_data_ctrl2::RDataCtrl2Spec>;
#[doc = "BIST Data Control 2 Register"]
pub mod r_data_ctrl2;
#[doc = "R_DATA_CTRL3 (rw) register accessor: BIST Data Control 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r_data_ctrl3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r_data_ctrl3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r_data_ctrl3`] module"]
#[doc(alias = "R_DATA_CTRL3")]
pub type RDataCtrl3 = crate::Reg<r_data_ctrl3::RDataCtrl3Spec>;
#[doc = "BIST Data Control 3 Register"]
pub mod r_data_ctrl3;
#[doc = "R_REPAIR0_0 (r) register accessor: BIST Repair 0 for Block 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r_repair0_0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r_repair0_0`] module"]
#[doc(alias = "R_REPAIR0_0")]
pub type RRepair0_0 = crate::Reg<r_repair0_0::RRepair0_0Spec>;
#[doc = "BIST Repair 0 for Block 0 Register"]
pub mod r_repair0_0;
#[doc = "R_REPAIR0_1 (r) register accessor: BIST Repair 1 Block 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r_repair0_1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r_repair0_1`] module"]
#[doc(alias = "R_REPAIR0_1")]
pub type RRepair0_1 = crate::Reg<r_repair0_1::RRepair0_1Spec>;
#[doc = "BIST Repair 1 Block 0 Register"]
pub mod r_repair0_1;
#[doc = "R_REPAIR1_0 (r) register accessor: BIST Repair 0 Block 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r_repair1_0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r_repair1_0`] module"]
#[doc(alias = "R_REPAIR1_0")]
pub type RRepair1_0 = crate::Reg<r_repair1_0::RRepair1_0Spec>;
#[doc = "BIST Repair 0 Block 1 Register"]
pub mod r_repair1_0;
#[doc = "R_REPAIR1_1 (r) register accessor: BIST Repair 1 Block 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r_repair1_1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r_repair1_1`] module"]
#[doc(alias = "R_REPAIR1_1")]
pub type RRepair1_1 = crate::Reg<r_repair1_1::RRepair1_1Spec>;
#[doc = "BIST Repair 1 Block 1 Register"]
pub mod r_repair1_1;
#[doc = "R_DATA_CTRL0_EX (rw) register accessor: BIST Data Control 0 Extension Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r_data_ctrl0_ex::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r_data_ctrl0_ex::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r_data_ctrl0_ex`] module"]
#[doc(alias = "R_DATA_CTRL0_EX")]
pub type RDataCtrl0Ex = crate::Reg<r_data_ctrl0_ex::RDataCtrl0ExSpec>;
#[doc = "BIST Data Control 0 Extension Register"]
pub mod r_data_ctrl0_ex;
#[doc = "R_TIMER_CTRL_EX (rw) register accessor: BIST Timer Control Extension Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r_timer_ctrl_ex::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r_timer_ctrl_ex::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r_timer_ctrl_ex`] module"]
#[doc(alias = "R_TIMER_CTRL_EX")]
pub type RTimerCtrlEx = crate::Reg<r_timer_ctrl_ex::RTimerCtrlExSpec>;
#[doc = "BIST Timer Control Extension Register"]
pub mod r_timer_ctrl_ex;
#[doc = "R_DOUT_QUERY1 (r) register accessor: BIST DOUT Query 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r_dout_query1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r_dout_query1`] module"]
#[doc(alias = "R_DOUT_QUERY1")]
pub type RDoutQuery1 = crate::Reg<r_dout_query1::RDoutQuery1Spec>;
#[doc = "BIST DOUT Query 1 Register"]
pub mod r_dout_query1;
#[doc = "R_D_MISR1 (r) register accessor: BIST DIN MISR 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r_d_misr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r_d_misr1`] module"]
#[doc(alias = "R_D_MISR1")]
pub type RDMisr1 = crate::Reg<r_d_misr1::RDMisr1Spec>;
#[doc = "BIST DIN MISR 1 Register"]
pub mod r_d_misr1;
#[doc = "R_A_MISR1 (r) register accessor: BIST Address MISR 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r_a_misr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r_a_misr1`] module"]
#[doc(alias = "R_A_MISR1")]
pub type RAMisr1 = crate::Reg<r_a_misr1::RAMisr1Spec>;
#[doc = "BIST Address MISR 1 Register"]
pub mod r_a_misr1;
#[doc = "R_C_MISR1 (r) register accessor: BIST Control MISR 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r_c_misr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r_c_misr1`] module"]
#[doc(alias = "R_C_MISR1")]
pub type RCMisr1 = crate::Reg<r_c_misr1::RCMisr1Spec>;
#[doc = "BIST Control MISR 1 Register"]
pub mod r_c_misr1;
#[doc = "R_DATA_CTRL1_EX (rw) register accessor: BIST Data Control 1 Extension Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r_data_ctrl1_ex::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r_data_ctrl1_ex::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r_data_ctrl1_ex`] module"]
#[doc(alias = "R_DATA_CTRL1_EX")]
pub type RDataCtrl1Ex = crate::Reg<r_data_ctrl1_ex::RDataCtrl1ExSpec>;
#[doc = "BIST Data Control 1 Extension Register"]
pub mod r_data_ctrl1_ex;
#[doc = "R_DATA_CTRL2_EX (rw) register accessor: BIST Data Control 2 Extension Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r_data_ctrl2_ex::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r_data_ctrl2_ex::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r_data_ctrl2_ex`] module"]
#[doc(alias = "R_DATA_CTRL2_EX")]
pub type RDataCtrl2Ex = crate::Reg<r_data_ctrl2_ex::RDataCtrl2ExSpec>;
#[doc = "BIST Data Control 2 Extension Register"]
pub mod r_data_ctrl2_ex;
#[doc = "R_DATA_CTRL3_EX (rw) register accessor: BIST Data Control 3 Extension Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r_data_ctrl3_ex::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r_data_ctrl3_ex::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r_data_ctrl3_ex`] module"]
#[doc(alias = "R_DATA_CTRL3_EX")]
pub type RDataCtrl3Ex = crate::Reg<r_data_ctrl3_ex::RDataCtrl3ExSpec>;
#[doc = "BIST Data Control 3 Extension Register"]
pub mod r_data_ctrl3_ex;
#[doc = "SMW_TIMER_OPTION (rw) register accessor: SMW Timer Option Register\n\nYou can [`read`](crate::Reg::read) this register and get [`smw_timer_option::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smw_timer_option::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smw_timer_option`] module"]
#[doc(alias = "SMW_TIMER_OPTION")]
pub type SmwTimerOption = crate::Reg<smw_timer_option::SmwTimerOptionSpec>;
#[doc = "SMW Timer Option Register"]
pub mod smw_timer_option;
#[doc = "SMW_SETTING_OPTION0 (rw) register accessor: SMW Setting Option 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`smw_setting_option0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smw_setting_option0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smw_setting_option0`] module"]
#[doc(alias = "SMW_SETTING_OPTION0")]
pub type SmwSettingOption0 = crate::Reg<smw_setting_option0::SmwSettingOption0Spec>;
#[doc = "SMW Setting Option 0 Register"]
pub mod smw_setting_option0;
#[doc = "SMW_SETTING_OPTION2 (rw) register accessor: SMW Setting Option 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`smw_setting_option2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smw_setting_option2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smw_setting_option2`] module"]
#[doc(alias = "SMW_SETTING_OPTION2")]
pub type SmwSettingOption2 = crate::Reg<smw_setting_option2::SmwSettingOption2Spec>;
#[doc = "SMW Setting Option 2 Register"]
pub mod smw_setting_option2;
#[doc = "SMW_SETTING_OPTION3 (rw) register accessor: SMW Setting Option 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`smw_setting_option3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smw_setting_option3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smw_setting_option3`] module"]
#[doc(alias = "SMW_SETTING_OPTION3")]
pub type SmwSettingOption3 = crate::Reg<smw_setting_option3::SmwSettingOption3Spec>;
#[doc = "SMW Setting Option 3 Register"]
pub mod smw_setting_option3;
#[doc = "SMW_SMP_WHV_OPTION0 (rw) register accessor: SMW SMP WHV Option 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`smw_smp_whv_option0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smw_smp_whv_option0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smw_smp_whv_option0`] module"]
#[doc(alias = "SMW_SMP_WHV_OPTION0")]
pub type SmwSmpWhvOption0 = crate::Reg<smw_smp_whv_option0::SmwSmpWhvOption0Spec>;
#[doc = "SMW SMP WHV Option 0 Register"]
pub mod smw_smp_whv_option0;
#[doc = "SMW_SME_WHV_OPTION0 (rw) register accessor: SMW SME WHV Option 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`smw_sme_whv_option0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smw_sme_whv_option0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smw_sme_whv_option0`] module"]
#[doc(alias = "SMW_SME_WHV_OPTION0")]
pub type SmwSmeWhvOption0 = crate::Reg<smw_sme_whv_option0::SmwSmeWhvOption0Spec>;
#[doc = "SMW SME WHV Option 0 Register"]
pub mod smw_sme_whv_option0;
#[doc = "SMW_SETTING_OPTION1 (rw) register accessor: SMW Setting Option 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`smw_setting_option1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smw_setting_option1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smw_setting_option1`] module"]
#[doc(alias = "SMW_SETTING_OPTION1")]
pub type SmwSettingOption1 = crate::Reg<smw_setting_option1::SmwSettingOption1Spec>;
#[doc = "SMW Setting Option 1 Register"]
pub mod smw_setting_option1;
#[doc = "SMW_SMP_WHV_OPTION1 (rw) register accessor: SMW SMP WHV Option 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`smw_smp_whv_option1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smw_smp_whv_option1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smw_smp_whv_option1`] module"]
#[doc(alias = "SMW_SMP_WHV_OPTION1")]
pub type SmwSmpWhvOption1 = crate::Reg<smw_smp_whv_option1::SmwSmpWhvOption1Spec>;
#[doc = "SMW SMP WHV Option 1 Register"]
pub mod smw_smp_whv_option1;
#[doc = "SMW_SME_WHV_OPTION1 (rw) register accessor: SMW SME WHV Option 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`smw_sme_whv_option1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smw_sme_whv_option1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smw_sme_whv_option1`] module"]
#[doc(alias = "SMW_SME_WHV_OPTION1")]
pub type SmwSmeWhvOption1 = crate::Reg<smw_sme_whv_option1::SmwSmeWhvOption1Spec>;
#[doc = "SMW SME WHV Option 1 Register"]
pub mod smw_sme_whv_option1;
#[doc = "REPAIR0_0 (rw) register accessor: FMU Repair 0 Block 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`repair0_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`repair0_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@repair0_0`] module"]
#[doc(alias = "REPAIR0_0")]
pub type Repair0_0 = crate::Reg<repair0_0::Repair0_0Spec>;
#[doc = "FMU Repair 0 Block 0 Register"]
pub mod repair0_0;
#[doc = "REPAIR0_1 (rw) register accessor: FMU Repair 1 Block 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`repair0_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`repair0_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@repair0_1`] module"]
#[doc(alias = "REPAIR0_1")]
pub type Repair0_1 = crate::Reg<repair0_1::Repair0_1Spec>;
#[doc = "FMU Repair 1 Block 0 Register"]
pub mod repair0_1;
#[doc = "REPAIR1_0 (rw) register accessor: FMU Repair 0 Block 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`repair1_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`repair1_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@repair1_0`] module"]
#[doc(alias = "REPAIR1_0")]
pub type Repair1_0 = crate::Reg<repair1_0::Repair1_0Spec>;
#[doc = "FMU Repair 0 Block 1 Register"]
pub mod repair1_0;
#[doc = "REPAIR1_1 (rw) register accessor: FMU Repair 1 Block 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`repair1_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`repair1_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@repair1_1`] module"]
#[doc(alias = "REPAIR1_1")]
pub type Repair1_1 = crate::Reg<repair1_1::Repair1_1Spec>;
#[doc = "FMU Repair 1 Block 1 Register"]
pub mod repair1_1;
#[doc = "SMW_HB_SIGNALS (rw) register accessor: SMW HB Signals Register\n\nYou can [`read`](crate::Reg::read) this register and get [`smw_hb_signals::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smw_hb_signals::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smw_hb_signals`] module"]
#[doc(alias = "SMW_HB_SIGNALS")]
pub type SmwHbSignals = crate::Reg<smw_hb_signals::SmwHbSignalsSpec>;
#[doc = "SMW HB Signals Register"]
pub mod smw_hb_signals;
#[doc = "BIST_DUMP_CTRL (rw) register accessor: BIST Datadump Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bist_dump_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bist_dump_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bist_dump_ctrl`] module"]
#[doc(alias = "BIST_DUMP_CTRL")]
pub type BistDumpCtrl = crate::Reg<bist_dump_ctrl::BistDumpCtrlSpec>;
#[doc = "BIST Datadump Control Register"]
pub mod bist_dump_ctrl;
#[doc = "ATX_PIN_CTRL (rw) register accessor: ATX Pin Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`atx_pin_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`atx_pin_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@atx_pin_ctrl`] module"]
#[doc(alias = "ATX_PIN_CTRL")]
pub type AtxPinCtrl = crate::Reg<atx_pin_ctrl::AtxPinCtrlSpec>;
#[doc = "ATX Pin Control Register"]
pub mod atx_pin_ctrl;
#[doc = "FAILCNT (rw) register accessor: Fail Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`failcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`failcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@failcnt`] module"]
#[doc(alias = "FAILCNT")]
pub type Failcnt = crate::Reg<failcnt::FailcntSpec>;
#[doc = "Fail Count Register"]
pub mod failcnt;
#[doc = "PGM_PULSE_CNT0 (rw) register accessor: Block 0 Program Pulse Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pgm_pulse_cnt0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pgm_pulse_cnt0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pgm_pulse_cnt0`] module"]
#[doc(alias = "PGM_PULSE_CNT0")]
pub type PgmPulseCnt0 = crate::Reg<pgm_pulse_cnt0::PgmPulseCnt0Spec>;
#[doc = "Block 0 Program Pulse Count Register"]
pub mod pgm_pulse_cnt0;
#[doc = "PGM_PULSE_CNT1 (rw) register accessor: Block 1 Program Pulse Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pgm_pulse_cnt1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pgm_pulse_cnt1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pgm_pulse_cnt1`] module"]
#[doc(alias = "PGM_PULSE_CNT1")]
pub type PgmPulseCnt1 = crate::Reg<pgm_pulse_cnt1::PgmPulseCnt1Spec>;
#[doc = "Block 1 Program Pulse Count Register"]
pub mod pgm_pulse_cnt1;
#[doc = "ERS_PULSE_CNT (rw) register accessor: Erase Pulse Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ers_pulse_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ers_pulse_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ers_pulse_cnt`] module"]
#[doc(alias = "ERS_PULSE_CNT")]
pub type ErsPulseCnt = crate::Reg<ers_pulse_cnt::ErsPulseCntSpec>;
#[doc = "Erase Pulse Count Register"]
pub mod ers_pulse_cnt;
#[doc = "MAX_PULSE_CNT (rw) register accessor: Maximum Pulse Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`max_pulse_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`max_pulse_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@max_pulse_cnt`] module"]
#[doc(alias = "MAX_PULSE_CNT")]
pub type MaxPulseCnt = crate::Reg<max_pulse_cnt::MaxPulseCntSpec>;
#[doc = "Maximum Pulse Count Register"]
pub mod max_pulse_cnt;
#[doc = "PORT_CTRL (rw) register accessor: Port Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`port_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`port_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@port_ctrl`] module"]
#[doc(alias = "PORT_CTRL")]
pub type PortCtrl = crate::Reg<port_ctrl::PortCtrlSpec>;
#[doc = "Port Control Register"]
pub mod port_ctrl;
