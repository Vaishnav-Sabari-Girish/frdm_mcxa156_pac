#![no_std]
#![no_main]

use panic_halt as _;
use cortex_m_rt::entry;


#[entry]
fn main() -> ! {
    let p = frdm_mcxa156_pac::Peripherals::take().unwrap();

    // 1. Enable Clocks
    // PORT0/GPIO0 (SW3) and PORT1/GPIO1 (SW2) are on CC1
    p.mrcc0.mrcc_glb_cc1().modify(|_, w| {
        w.port0().enabled();
        w.gpio0().enabled();
        w.port1().enabled();
        w.gpio1().enabled();
        w.port3().enabled();
        w.gpio3().enabled();
        w
    });

    // DSB
    cortex_m::asm::dsb();

    // 2. Release RESET 
    p.mrcc0.mrcc_glb_rst1().modify(|_, w| {
        w.port0().enabled();
        w.gpio0().enabled();
        w.port1().enabled();
        w.gpio1().enabled();
        w.port3().enabled();
        w.gpio3().enabled();
        w
    });

    // 3. Configure Pin Muxing & Hardware Registers 
    
    // LED (PORT3)
    p.port3.pcr12().modify(|_, w| { w.mux().mux00(); w}); // Red
    p.port3.pcr13().modify(|_, w| { w.mux().mux00(); w}); // Green
    p.port3.pcr0().modify(|_, w| { w.mux().mux00(); w});  // Blue

    // SW2 (PORT1, Pin 7)
    p.port1.pcr7().modify(|_, w| {
        w.mux().mux00();
        w.pe().set_bit();
        w.ps().set_bit();
        w.ibe().set_bit();
        w
    });

    // SW3 (PORT0, Pin 6)
    p.port0.pcr6().modify(|_, w| {
        w.mux().mux00();
        w.pe().set_bit();
        w.ps().set_bit();
        w.ibe().set_bit();
        w
    });

    // 4. Configure the Data Direction 
    // LEDs as Output (1)
    p.gpio3.pddr().modify(|_, w| {
        w.pdd12().pdd1();
        w.pdd13().pdd1();
        w.pdd0().pdd1();
        w
    });

    // Buttons as Input (0)
    p.gpio1.pddr().modify(|_, w| { w.pdd7().pdd0(); w});  // SW2
    p.gpio0.pddr().modify(|_, w| { w.pdd6().pdd0(); w});  // SW3

    // 5. Initial State 
    // All LEDs OFF
    p.gpio3.psor().write(|w| {
        w.ptso12().ptso1();
        w.ptso13().ptso1();
        w.ptso0().ptso1();
        w
    });



    loop {
        // Read Port Data Input Registers 
        // Mask the specific bits. If 0, the button is PRESSED 
        let sw2_pressed = (p.gpio1.pdir().read().bits() & (1 << 7)) == 0;
        let sw3_pressed = (p.gpio0.pdir().read().bits() & (1 << 6)) == 0;

        // Determine which LED should be ON based on priority 
        let mut red_on = false;
        let mut green_on = false;
        let mut blue_on = false;

        if sw2_pressed && sw3_pressed {
            blue_on = true;
        } else if sw2_pressed {
            red_on = true;
        } else if sw3_pressed {
            green_on = true;
        }

        if red_on {
            p.gpio3.pcor().write(|w| w.ptco12().ptco1());
        } else {
            p.gpio3.psor().write(|w| w.ptso12().ptso1());
        }

        if green_on {
            p.gpio3.pcor().write(|w| w.ptco13().ptco1());
        } else {
            p.gpio3.psor().write(|w| w.ptso13().ptso1());
        }

        if blue_on {
            p.gpio3.pcor().write(|w| w.ptco0().ptco1());
        } else {
            p.gpio3.psor().write(|w| w.ptso0().ptso1());
        }

        cortex_m::asm::delay(48_000);
    }
}
