#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;

#[entry]
fn main() -> ! {
    let p = frdm_mcxa156_pac::Peripherals::take().unwrap();

    // ---- Enable clocks ----
    p.mrcc0.mrcc_glb_cc1().modify(|_, w| {
        w.port3().enabled();
        w.gpio3().enabled();
        w
    });

    // DSB: guarantee clock enable completes before reset release
    cortex_m::asm::dsb();

    // ---- Release from reset ----
    p.mrcc0.mrcc_glb_rst1().modify(|_, w| {
        w.port3().enabled();
        w.gpio3().enabled();
        w
    });

    // ---- Pin mux: PORT3 pin 13 = ALT0 (GPIO) ----
    p.port3.pcr13().modify(|_, w| {
        w.mux().mux00();
        w
    });

    // ---- Data direction: pin 13 = output ----
    p.gpio3.pddr().modify(|_, w| {
        w.pdd13().pdd1();
        w
    });

    loop {
        // LED ON (active low: clear → 0 → LED conducts)
        p.gpio3.pcor().write(|w| w.ptco13().ptco1());
        cortex_m::asm::delay(96_000_000 / 2);

        // LED OFF
        p.gpio3.psor().write(|w| w.ptso13().ptso1());
        cortex_m::asm::delay(96_000_000 / 2);
    }
}
