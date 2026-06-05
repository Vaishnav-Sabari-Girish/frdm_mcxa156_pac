// Green LED — GPIO3 pin 13 (active low)
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;

#[entry]
fn main() -> ! {
    let p = frdm_mcxa156_pac::Peripherals::take().unwrap();

    p.mrcc0.mrcc_glb_cc1().modify(|_, w| { w.port3().enabled(); w.gpio3().enabled(); w });
    cortex_m::asm::dsb();
    p.mrcc0.mrcc_glb_rst1().modify(|_, w| { w.port3().enabled(); w.gpio3().enabled(); w });
    p.port3.pcr13().modify(|_, w| { w.mux().mux00(); w });
    p.gpio3.pddr().modify(|_, w| { w.pdd13().pdd1(); w });

    loop {
        p.gpio3.ptor().write(|w| w.ptto13().ptto1());
        cortex_m::asm::delay(96_000_000 / 2);
    }
}
