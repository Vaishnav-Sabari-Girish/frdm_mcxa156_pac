#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;

#[entry]
fn main() -> ! {
    let p = frdm_mcxa156_pac::Peripherals::take().unwrap();

    // 1. Enable Clocks for Port 1/GPIO1 (Button) and Port 3/GPIO3 (LED)
    // NOTE: Depending on the MCXA156 register map, PORT1/GPIO1 might be in glb_cc0 instead of
    // glb_cc1 
    p.mrcc0.mrcc_glb_cc1().modify(|_, w| {
        w.port1().enabled();
        w.gpio1().enabled();
        w
    });

    p.mrcc0.mrcc_glb_cc1().modify(|_, w| {
        w.port3().enabled();
        w.gpio3().enabled();
        w
    });

    // DSB
    cortex_m::asm::dsb();

    // Release RESET for button and led
    p.mrcc0.mrcc_glb_rst1().modify(|_, w| {
        w.port1().enabled();
        w.gpio1().enabled();
        w
    });

    p.mrcc0.mrcc_glb_rst1().modify(|_, w| {
        w.port3().enabled();
        w.gpio3().enabled();
        w
    });

    // Configure Pin Muxing
    p.port3.pcr12().modify(|_, w| {
        w.mux().mux00();
        w
    });         // Red LED

    p.port1.pcr7().modify(|_, w| {
        w.mux().mux00();   // Button SW2    (button0)
        // Optional: Enable internal pull-up
        w.pe().set_bit();   // Pullup Enable = 1
        w.ps().set_bit();   // Pull select = 1 (Up)
        w.ibe().set_bit();  // Digital Input buffer Enabled
        w
    });

    // Configure data direction 
    p.gpio3.pddr().modify(|_, w| {
        w.pdd12().pdd1();       // Output (1)
        w
    });

    p.gpio1.pddr().modify(|_, w| {
        w.pdd7().pdd0();       // Input (0)
        w
    });

    // Initial state: turn LED OFF
    p.gpio3.psor().write(|w| w.ptso12().ptso1());

    let mut was_pressed = false;


    loop {
        // Read Port 1 Data Input Register (PDIR)
        // Masking the 7th bit. If it equals 0, the active-low button is pressed
        let is_pressed = (p.gpio1.pdir().read().bits() & (1 << 7)) == 0;

        // Detect falling edge (Pressed)
        if is_pressed && !was_pressed {
            // Toggle the RED LED
            p.gpio3.ptor().write(|w| w.ptto12().ptto1());
        }

        was_pressed = is_pressed;

        cortex_m::asm::delay(96_000);
    }
}
