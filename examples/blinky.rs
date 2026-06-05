#![no_std]
#![no_main]

use core::ptr::write_volatile;
use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use panic_halt as _;

// MRCC0 base (0x4009_1000)
const MRCC0: *mut u32 = 0x4009_1000 as *mut u32;
const GLB_CC1_SET: usize = 0x54 / 4;
const GLB_RST1_SET: usize = 0x14 / 4;

// PORT3 / GPIO3
const PORT3: *mut u32 = 0x400B_F000 as *mut u32;
const PCR13: usize = 0xB4 / 4;
const GPIO3: *mut u32 = 0x4010_5000 as *mut u32;
const PDDR: usize = 0x54 / 4;
const PSOR: usize = 0x44 / 4;
const PCOR: usize = 0x48 / 4;

#[entry]
fn main() -> ! {
    // Set up the cycle-counting delay timer
    let cp = cortex_m::Peripherals::take().unwrap();
    let mut delay = cortex_m::delay::Delay::new(cp.SYST, 96_000_000);

    // --- GPIO init (unchanged) ---
    unsafe { write_volatile(MRCC0.add(GLB_CC1_SET), (1 << 10) | (1 << 23)); }
    cortex_m::asm::dsb();
    unsafe { write_volatile(MRCC0.add(GLB_RST1_SET), (1 << 10) | (1 << 23)); }
    unsafe { write_volatile(PORT3.add(PCR13), 0); }
    unsafe { write_volatile(GPIO3.add(PDDR), 1 << 13); }

    loop {
        unsafe { write_volatile(GPIO3.add(PCOR), 1 << 13); } // LED ON
        delay.delay_ms(500_u32);

        unsafe { write_volatile(GPIO3.add(PSOR), 1 << 13); } // LED OFF
        delay.delay_ms(500_u32);
    }
}
