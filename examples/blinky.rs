#![no_std]
#![no_main]

use cortex_m_rt::entry;
use frdm_mcxa156_pac::Peripherals;
use panic_halt as _;

#[entry]
fn main() -> ! {
    let p = unsafe { Peripherals::steal() };

    //--Clock gating: Enable PORT3 and GPIO3----
    // MRCC_GLB_CC1: PORT3=bit10, GPIO3=bit23  (Both disabled by default)
    p.mrcc0.mrcc_glb_cc1().modify(|_, w| {
        w.port3().enabled();
        w.gpio3().enabled();
        w
    });

    // -- Pin Mux: PORT3 pin 13 = ALT0 (GPIO) ---
    // ALT0 is the reset default, so this is a no-op, but explicit is safer
    p.port3.pcr13().modify(|_, w| w.mux().mux00());

    // --- Data direction: GPIO3 PIN 13 = OUTPUT ----
    p.gpio3.pddr().modify(|_, w| {
        w.pdd13().pdd1();
        w
    });

    loop {
        // Clear the bit 
        p.gpio3.pcor().write(|w| w.ptco13().ptco1());

        cortex_m::asm::delay(96_000_000);

        // Set the bit
        p.gpio3.psor().write(|w| w.ptso13().ptso1());

        cortex_m::asm::delay(96_000_000);
    }
}
