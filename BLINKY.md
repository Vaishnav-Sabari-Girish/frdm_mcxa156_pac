# FRDM-MCXA156 — LED Blinky Examples

LED blinky examples for the NXP FRDM-MCXA156 board using the `frdm_mcxa156_pac`
(svd2rust-generated PAC).  Three single-LED examples plus one combined sequential
blink that cycles through all three colours.

---

## Hardware

| Item | Detail |
|------|--------|
| Board | NXP FRDM-MCXA156 |
| MCU | MCXA156 (Cortex-M33) |
| Green LED | GPIO3 pin 13 (active low) |
| Red LED | GPIO3 pin 12 (active low) |
| Blue LED | GPIO3 pin 0 (active low) |
| Debug probe | Onboard MCU-Link with J-Link firmware |

Pin assignments confirmed from Zephyr device tree:
```
red_led:   <&gpio3 0xc 0x1>  → GPIO3 pin 12, active low
green_led: <&gpio3 0xd 0x1>  → GPIO3 pin 13, active low
blue_led:  <&gpio3 0x0 0x1>  → GPIO3 pin 0,  active low
```

---

## File: `examples/green.rs`

```rust
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
```

---

## File: `examples/red.rs`

```rust
// Red LED — GPIO3 pin 12 (active low)
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
    p.port3.pcr12().modify(|_, w| { w.mux().mux00(); w });
    p.gpio3.pddr().modify(|_, w| { w.pdd12().pdd1(); w });

    loop {
        p.gpio3.ptor().write(|w| w.ptto12().ptto1());
        cortex_m::asm::delay(96_000_000 / 2);
    }
}
```

---

## File: `examples/blue.rs`

```rust
// Blue LED — GPIO3 pin 0 (active low)
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
    p.port3.pcr0().modify(|_, w| { w.mux().mux00(); w });
    p.gpio3.pddr().modify(|_, w| { w.pdd0().pdd1(); w });

    loop {
        p.gpio3.ptor().write(|w| w.ptto0().ptto1());
        cortex_m::asm::delay(96_000_000 / 2);
    }
}
```

---

## File: `Cargo.toml` (critical additions)

```toml
[dependencies]
cortex-m = { version = "0.7.7", features = ["critical-section-single-core"] }
critical-section = "1.2.0"
vcell = "0.1.3"

[dev-dependencies]
cortex-m-rt = "0.7.5"
panic-halt = "0.2.0"
```

`cortex-m` needs `critical-section-single-core` so that `Peripherals::take()`
(a safe wrapper around the one-shot peripheral singleton) can disable interrupts.

---

## File: `.cargo/config.toml`

```toml
[target.thumbv8m.main-none-eabihf]
runner = "./flash.sh"
rustflags = [
    "-C", "link-arg=-Tlink.x",
]

[build]
target = "thumbv8m.main-none-eabihf"
```

---

## File: `flash.sh`

JLinkExe cannot parse the Rust ELF format (`loadfile` fails silently).  The
script converts ELF → raw binary with `arm-none-eabi-objcopy`, then flashes
with `loadbin`.  It also bypasses the ROM bootloader by setting `PC = 0x800`.

```bash
#!/bin/bash
ELF="$1"
BIN=$(mktemp /tmp/flash_XXXXXX.bin)
arm-none-eabi-objcopy -O binary "$ELF" "$BIN"
SCRIPT=$(mktemp)
cat > "$SCRIPT" <<EOF
device MCXA156
si SWD
speed 1000
connect
r
h
loadbin $BIN 0x0
SetPC 0x800
g
qc
EOF
JLinkExe -NoGui 1 -CommanderScript "$SCRIPT"
rm -f "$BIN" "$SCRIPT"
```

Make it executable: `chmod +x flash.sh`

---

## File: `memory.x`

```ld
MEMORY
{
    FLASH : ORIGIN = 0x00000000, LENGTH = 1024K
    RAM   : ORIGIN = 0x20000000, LENGTH = 128K
}
```

---

## Build, Flash & Run

```bash
cargo run --example green --release
cargo run --example red   --release
cargo run --example blue  --release
```

---

## File: `examples/blinky.rs` — Combined Sequential Blink

All three LEDs in one program.  Cycles Red → Green → Blue → Red → …
Uses `PSOR`/`PCOR` with `compiler_fence` to prevent the optimizer from
collapsing paired set/clear writes (see Problem 6 below).

```rust
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
```

**Why `compiler_fence`?**  Without it, LLVM collapses `PCOR(bit) ; PSOR(bit)`
across loop iterations into a single non-toggling store — the LED never turns
off.  `compiler_fence(SeqCst)` prevents this by marking each write as a
non-reorderable side effect.

**Why not PTOR?**  PTOR works for single-LED blink (see separate examples)
but cannot control the *absolute* state — only toggle.  For sequential blink
we need explicit ON/OFF so that at most one LED is lit at a time.

Run it:  `cargo run --example blinky --release`

---

## File: `examples/blinky_mix.rs` — Colour-Mix Blink

All three LEDs in one program.  Uses `PTOR` to toggle each LED twice per
cycle.  Because the LEDs are **not** reset to a known state, they accumulate
on top of each other, producing colour mixes: cyan → magenta → yellow → white.

```rust
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;

#[entry]
fn main() -> ! {
    let p = frdm_mcxa156_pac::Peripherals::take().unwrap();

    p.mrcc0.mrcc_glb_cc1().modify(|_, w| {
        w.port3().enabled();
        w.gpio3().enabled();
        w
    });
    cortex_m::asm::dsb();

    p.mrcc0.mrcc_glb_rst1().modify(|_, w| {
        w.port3().enabled();
        w.gpio3().enabled();
        w
    });

    p.port3.pcr0() .modify(|_, w| { w.mux().mux00(); w });  // Blue
    p.port3.pcr12().modify(|_, w| { w.mux().mux00(); w });  // Red
    p.port3.pcr13().modify(|_, w| { w.mux().mux00(); w });  // Green

    p.gpio3.pddr().modify(|_, w| {
        w.pdd0().pdd1();
        w.pdd12().pdd1();
        w.pdd13().pdd1();
        w
    });

    let delay_cycles = 96_000_000 / 2;

    loop {
        // Red toggle × 2 (on then off)
        p.gpio3.ptor().write(|w| w.ptto12().ptto1());
        cortex_m::asm::delay(delay_cycles);
        p.gpio3.ptor().write(|w| w.ptto12().ptto1());

        // Green toggle × 2
        p.gpio3.ptor().write(|w| w.ptto13().ptto1());
        cortex_m::asm::delay(delay_cycles);
        p.gpio3.ptor().write(|w| w.ptto13().ptto1());

        // Blue toggle × 2
        p.gpio3.ptor().write(|w| w.ptto0().ptto1());
        cortex_m::asm::delay(delay_cycles);
        p.gpio3.ptor().write(|w| w.ptto0().ptto1());
    }
}
```

**Why colour mixes?**  All LEDs start ON (active-low, output-low by default).
Each PTOR toggle inverts one LED while the other two stay lit:

| Step | R | G | B | Visible colour |
|------|---|---|---|----------------|
| Start | ON | ON | ON | White |
| R toggle | OFF | ON | ON | Cyan (light blue) |
| R toggle | ON | ON | ON | White |
| G toggle | ON | OFF | ON | Magenta (purple) |
| G toggle | ON | ON | ON | White |
| B toggle | ON | ON | OFF | Yellow |
| B toggle | ON | ON | ON | White |

Run it:  `cargo run --example blinky_mix --release`

---

## Register Map

### MRCC0 (base `0x4009_1000`)

| Register | Offset | Bit 10 | Bit 23 |
|----------|--------|--------|--------|
| `GLB_CC1` | `0x50` | PORT3 clock enable | GPIO3 clock enable |
| `GLB_CC1_SET` | `0x54` | Atomic set for above | |
| `GLB_RST1` | `0x10` | PORT3 reset release | GPIO3 reset release |
| `GLB_RST1_SET` | `0x14` | Atomic set for above | |

### PORT3 (base `0x400B_F000`)

| Register | Offset | Description |
|----------|--------|-------------|
| `PCR0` | `0x80` | Pin Control 0 — MUX field: `0`=ALT0 (GPIO) |
| `PCR12` | `0xB0` | Pin Control 12 |
| `PCR13` | `0xB4` | Pin Control 13 |

### GPIO3 (base `0x4010_5000`)

| Register | Offset | Description |
|----------|--------|-------------|
| `PDOR` | `0x40` | Port Data Output (read pin state) |
| `PSOR` | `0x44` | Port Set Output (write 1 → pin = 1) |
| `PCOR` | `0x48` | Port Clear Output (write 1 → pin = 0) |
| `PTOR` | `0x4C` | Port Toggle Output (write 1 → invert pin) |
| `PDDR` | `0x54` | Port Data Direction (1 = output) |

---

## Problems Encountered & Solutions

Each problem is listed in the order it appeared, with the root cause and fix.

### 1. LEDs don't blink — HardFault on PORT3 access

**Symptoms:**  CPU enters HardFault (IPSR=3), BFAR=`0x400BF0B4` (PORT3 PCR13),
BFSR.PRECISERR=1.

**Root cause:**  The ROM bootloader leaves PORT3 and GPIO3 clocks **disabled**
(`GLB_CC1` bit 10/23 = 0) and **held in reset** (`GLB_RST1` bit 10/23 = 0).
Accessing a peripheral while it is held in reset causes a precise bus fault.

**Fix:**  Enable clocks and release reset before touching the peripheral:

```rust
p.mrcc0.mrcc_glb_cc1().modify(|_, w| { w.port3().enabled(); w.gpio3().enabled(); w });
cortex_m::asm::dsb();  // ← critical (see problem 2)
p.mrcc0.mrcc_glb_rst1().modify(|_, w| { w.port3().enabled(); w.gpio3().enabled(); w });
```

### 2. `GLB_RST1` write silently ignored by CPU

**Symptoms:**  `GLB_CC1` (clock enable) works, but `GLB_RST1` (reset release)
stays at `0x00000000`.  Debug-probe (DAP) writes to the same register DO work.
Result: code still faults on PORT3 access (see problem 1).

**Root cause:**  The Cortex-M AHB bus can **reorder store instructions**.  The
`GLB_RST1_SET` store can arrive at the MRCC peripheral *before* `GLB_CC1_SET`.
The hardware requires the clock to be enabled before accepting the reset-release
and silently drops out-of-order writes.

Debug-probe writes appear to work because the DAP inserts implicit barriers
between each access.  CPU stores do not.

**Fix:**  Insert a `DSB` (Data Synchronization Barrier) between the two writes:

```rust
p.mrcc0.mrcc_glb_cc1().modify(…);
cortex_m::asm::dsb();   // guarantee clock-enable completes first
p.mrcc0.mrcc_glb_rst1().modify(…);
```

### 3. ROM bootloader does not jump to user code

**Symptoms:**  After reset, the CPU sits in ROM ISP handler at `PC = 0x20002040`
(LR = `0x030033C7`).  The vector table at `0x00000000` is valid
(SP=`0x20020000`, Reset=`0x00000801`) but the ROM ignores it.

**Root cause:**  The MCXA156 ROM requires a Boot Configuration Area (BCA / FCF)
in flash.  Without it, the ROM falls back to ISP mode.

**Fix:**  `flash.sh` sets `PC = 0x800` directly via the debug probe after
flashing, bypassing the ROM bootloader entirely.

### 4. `probe-rs` DAP FAULT with J-Link firmware

**Symptoms:**  `probe-rs download` intermittently fails with DAP FAULT /
`sticky_err` after a previous debug session.

**Root cause:**  The on-board MCU-Link with SEGGER J-Link firmware leaves the
debug port in a dirty state between sessions.  `probe-rs` cannot recover
without a power cycle.

**Fix:**  Use `JLinkExe` for flashing (see `flash.sh`).  It resets the debug
port on every connection and works reliably.

### 5. JLinkExe `loadfile` silently fails on Rust ELF

**Symptoms:**  `loadfile` prints "File is of unknown / unsupported format" and
does **not** program the flash.  The previous flash contents remain in place.
All three example programs appeared to blink the same LED (green) because only
the green binary actually made it into flash.

**Root cause:**  JLinkExe's ELF parser does not understand the ELF variant
produced by Rust + LLD.

**Fix:**  Convert ELF → raw binary with `arm-none-eabi-objcopy -O binary`
before flashing with `loadbin`:

```bash
arm-none-eabi-objcopy -O binary "$ELF" "$BIN"
JLinkExe … loadbin "$BIN" 0x0 …
```

### 6. Compiler eliminates `PSOR` write in `PCOR → delay → PSOR` loop

**Symptoms:**  The LED appears stuck off (PDOR always = `0x00000000`).  The
disassembly shows the loop contains only `PCOR` + `delay` — the `PSOR` write
and second `delay` were removed.

**Root cause:**  The LLVM optimizer determines that `PCOR(bit) ; delay ; PSOR(bit)`
across two successive loop iterations is equivalent to `PCOR(bit) ; delay` and
collapses the loop.  It does not recognise that `PSOR` has a visible side effect
(the LED turning off).

**Fix:**  Use `PTOR` (Port Toggle) instead of separate `PCOR`/`PSOR`:

```rust
// WRONG — compiler eliminates PSOR:
loop {
    p.gpio3.pcor().write(|w| w.ptco13().ptco1());  // elimi-
    delay();                                        // -nated
    p.gpio3.psor().write(|w| w.ptso13().ptso1());  // ←
    delay();
}

// RIGHT — PTOR cannot be collapsed:
loop {
    p.gpio3.ptor().write(|w| w.ptto13().ptto1());   // toggle
    delay();
}
```

`PTOR` cannot be optimised away because each toggle depends on the *current*
pin state, which the compiler cannot statically determine.

---

## Register State (verified with JLinkExe)

### ROM bootloader state (post-reset, CPU halted)

```
PC       = 0x20002040   (ROM ISP handler in RAM)
LR       = 0x030033C7   (ROM return address)
IPSR     = 0            (thread mode)
PRIMASK  = 1            (interrupts masked)

GLB_CC1  = 0x00000B80   (PORT0/1/2/4, GPIO0/1/2 clocks enabled)
GLB_RST1 = 0x00000000   (all peripherals held in reset)

PORT3 PCR13 → inaccessible (bus fault)
GPIO3 PDOR  → inaccessible (bus fault)
```

### After successful execution

```
GLB_CC1   = 0x00800F80   (PORT3 + GPIO3 clocks enabled)
GLB_RST1  = 0x00800400   (PORT3 + GPIO3 released from reset)
GPIO3 PDOR = toggling 0x00000000 ↔ 0x00002000  (green)
                          0x00000000 ↔ 0x00001000  (red)
                          0x00000000 ↔ 0x00000001  (blue)
```

---

## Lessons Learned

1. **Release peripherals from reset AFTER enabling clocks, with a DSB barrier.**
   The MCXA156 ROM leaves many peripherals gated.  `GLB_CCn` (clock) must be
   written first, then a `DSB` forces completion, then `GLB_RSTn` (reset
   release).  Without the barrier the hardware silently ignores the
   reset-release.

2. **The ROM bootloader is not a simple jump-to-vector-table.**  It configures
   clocks, security, and may require a BCA/FCF header in flash.  Understanding
   the ROM's post-boot register state is critical.

3. **DAP writes ≠ CPU writes.**  Debug-probe accesses go through a different
   bus path with implicit barriers.  If a DAP write works but the same CPU
   write fails, suspect memory ordering, not access control.

4. **Verify the flash tool actually programmed the flash.**  JLinkExe
   `loadfile` printed a warning that was easy to miss ("File is of unknown …
   format") but did not abort the script.  Always check `mem32 0x00000000`
   after flashing to confirm the vector table was written.

5. **`PTOR` beats `PCOR`/`PSOR` for toggling.**  The compiler can collapse
   paired set/clear writes.  Port Toggle is immune to this optimisation
   because each write depends on the previous pin state.

6. **Check fault status registers early.**  `CFSR`, `HFSR`, and `BFAR`
   pinpointed the exact bus fault address (`BFAR=0x400BF0B4` = PORT3 PCR13),
   directly identifying the peripheral that was still in reset.

7. **The sysd2rust `modify(|_, w| { … ; w })` pattern must return `w`.**
   The `modify` closure takes a reader and writer and must return the writer.
   The `write(|w| { … })` closure takes only the writer and returns nothing.
