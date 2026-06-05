#![no_std]
#![no_main]

use core::sync::atomic::compiler_fence;
use core::sync::atomic::Ordering;
use cortex_m_rt::entry;
use panic_halt as _;

#[entry]
fn main() -> ! {
    let p = frdm_mcxa156_pac::Peripherals::take().unwrap();

    // 1. Enable Clocks
    p.mrcc0.mrcc_glb_cc1().modify(|_, w| {
        w.port3().enabled();
        w.gpio3().enabled();
        w
    });
    cortex_m::asm::dsb();

    // 2. Release Reset
    p.mrcc0.mrcc_glb_rst1().modify(|_, w| {
        w.port3().enabled();
        w.gpio3().enabled();
        w
    });

    // 3. Pin Mux (GPIO)
    p.port3.pcr0() .modify(|_, w| { w.mux().mux00(); w });  // Blue
    p.port3.pcr12().modify(|_, w| { w.mux().mux00(); w });  // Red
    p.port3.pcr13().modify(|_, w| { w.mux().mux00(); w });  // Green

    // 4. Direction (output)
    p.gpio3.pddr().modify(|_, w| {
        w.pdd0().pdd1();
        w.pdd12().pdd1();
        w.pdd13().pdd1();
        w
    });

    // 5. Start with all LEDs OFF (active-low: 1 = OFF)
    p.gpio3.psor().write(|w| w.ptso0().ptso1());
    compiler_fence(Ordering::SeqCst);
    p.gpio3.psor().write(|w| w.ptso12().ptso1());
    compiler_fence(Ordering::SeqCst);
    p.gpio3.psor().write(|w| w.ptso13().ptso1());
    compiler_fence(Ordering::SeqCst);

    let d = 96_000_000 / 2;

    loop {
        // Red ON → delay → Red OFF
        p.gpio3.pcor().write(|w| w.ptco12().ptco1());
        compiler_fence(Ordering::SeqCst);
        cortex_m::asm::delay(d);
        p.gpio3.psor().write(|w| w.ptso12().ptso1());
        compiler_fence(Ordering::SeqCst);

        // Green ON → delay → Green OFF
        p.gpio3.pcor().write(|w| w.ptco13().ptco1());
        compiler_fence(Ordering::SeqCst);
        cortex_m::asm::delay(d);
        p.gpio3.psor().write(|w| w.ptso13().ptso1());
        compiler_fence(Ordering::SeqCst);

        // Blue ON → delay → Blue OFF
        p.gpio3.pcor().write(|w| w.ptco0().ptco1());
        compiler_fence(Ordering::SeqCst);
        cortex_m::asm::delay(d);
        p.gpio3.psor().write(|w| w.ptso0().ptso1());
        compiler_fence(Ordering::SeqCst);
    }
}
