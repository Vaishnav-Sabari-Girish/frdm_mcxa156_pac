#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;

#[entry]
fn main() -> ! {
    let p = frdm_mcxa156_pac::Peripherals::take().unwrap();

    // 1. Enable Clocks for Port 3 and GPIO 3
    p.mrcc0.mrcc_glb_cc1().modify(|_, w| { 
        w.port3().enabled(); 
        w.gpio3().enabled(); 
        w 
    });
    
    // Data Synchronization Barrier
    cortex_m::asm::dsb();
    
    // 2. Release Reset for Port 3 and GPIO 3
    p.mrcc0.mrcc_glb_rst1().modify(|_, w| { 
        w.port3().enabled(); 
        w.gpio3().enabled(); 
        w 
    });

    // 3. Configure Pin Muxing (Set pins as GPIO)
    p.port3.pcr0().modify(|_, w| { w.mux().mux00(); w });   // Blue
    p.port3.pcr12().modify(|_, w| { w.mux().mux00(); w });  // Red
    p.port3.pcr13().modify(|_, w| { w.mux().mux00(); w });  // Green

    // 4. Configure Pin Data Direction (Set pins as Outputs)
    p.gpio3.pddr().modify(|_, w| { 
        w.pdd0().pdd1()   // Blue Output
         .pdd12().pdd1()  // Red Output
         .pdd13().pdd1(); // Green Output
        w 
    });

    // Note: Because these are active low, they might all turn ON immediately 
    // when set as outputs. If your PAC supports `psor` (Port Set Output Register),
    // you could write 1 to all three pins here to ensure they start completely OFF.
    // Otherwise, the loop below will just toggle their current states.

    let delay_cycles = 96_000_000 / 2;

    // 5. Sequential Blink Loop
    loop {
        // --- RED LED (Pin 12) ---
        p.gpio3.ptor().write(|w| w.ptto12().ptto1()); // Toggle Red ON
        cortex_m::asm::delay(delay_cycles);
        p.gpio3.ptor().write(|w| w.ptto12().ptto1()); // Toggle Red OFF

        // --- GREEN LED (Pin 13) ---
        p.gpio3.ptor().write(|w| w.ptto13().ptto1()); // Toggle Green ON
        cortex_m::asm::delay(delay_cycles);
        p.gpio3.ptor().write(|w| w.ptto13().ptto1()); // Toggle Green OFF

        // --- BLUE LED (Pin 0) ---
        p.gpio3.ptor().write(|w| w.ptto0().ptto1());  // Toggle Blue ON
        cortex_m::asm::delay(delay_cycles);
        p.gpio3.ptor().write(|w| w.ptto0().ptto1());  // Toggle Blue OFF
    }
}
