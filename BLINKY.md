# Blinky Example — FRDM-MCXA156

Working blinky example for NXP FRDM-MCXA156 using the `frdm_mcxa156_pac` (svd2rust-generated PAC).

---

## Hardware

| Item | Detail |
|------|--------|
| Board | NXP FRDM-MCXA156 |
| MCU | MCXA156 (Cortex-M33, 96 MHz FRO) |
| Green LED | GPIO3 pin 13 (active low) |
| Red LED | GPIO3 pin 12 (active low) |
| Blue LED | GPIO3 pin 0 (active low) |
| Debug probe | Onboard MCU-Link with J-Link firmware |

---

## File: `examples/blinky.rs`

```rust
#![no_std]
#![no_main]

use core::ptr::write_volatile;
use cortex_m_rt::entry;
use panic_halt as _;

// MRCC0 base (0x4009_1000)
const MRCC0: *mut u32 = 0x4009_1000 as *mut u32;
const GLB_CC1_SET: usize = 0x54 / 4;  // offset 0x54
const GLB_RST1_SET: usize = 0x14 / 4; // offset 0x14

// PORT3 base (0x400B_F000)
const PORT3: *mut u32 = 0x400B_F000 as *mut u32;
const PCR13: usize = 0xB4 / 4; // offset 0xB4

// GPIO3 base (0x4010_5000)
const GPIO3: *mut u32 = 0x4010_5000 as *mut u32;
const PDDR: usize = 0x54 / 4; // offset 0x54
const PSOR: usize = 0x44 / 4; // offset 0x44
const PCOR: usize = 0x48 / 4; // offset 0x48

#[entry]
fn main() -> ! {
    // Step 1: Enable clocks — GLB_CC1_SET bit 10 (PORT3) + bit 23 (GPIO3)
    unsafe { write_volatile(MRCC0.add(GLB_CC1_SET), (1 << 10) | (1 << 23)); }

    // DSB is CRITICAL: ensures the clock-enable write completes before
    // the reset-release write. Without this barrier, the AHB bus may
    // reorder the stores. If GLB_RST1_SET arrives before GLB_CC1_SET,
    // the hardware silently ignores the reset release (clock must be
    // enabled first).
    cortex_m::asm::dsb();

    // Step 2: Release from reset — GLB_RST1_SET bit 10 + bit 23
    unsafe { write_volatile(MRCC0.add(GLB_RST1_SET), (1 << 10) | (1 << 23)); }

    // Step 3: Pin mux — PORT3 PCR13 = ALT0 (GPIO, already reset default)
    unsafe { write_volatile(PORT3.add(PCR13), 0); }

    // Step 4: Data direction — GPIO3 PDDR bit 13 = output
    unsafe { write_volatile(GPIO3.add(PDDR), 1 << 13); }

    loop {
        // LED ON: clear pin 13 (active low → pin pulled low → LED conducts)
        unsafe { write_volatile(GPIO3.add(PCOR), 1 << 13); }
        cortex_m::asm::delay(96_000_000 / 2); // ~500 ms at 96 MHz

        // LED OFF: set pin 13
        unsafe { write_volatile(GPIO3.add(PSOR), 1 << 13); }
        cortex_m::asm::delay(96_000_000 / 2);
    }
}
```

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

Shell script invoked by `cargo run`.  Generates a temporary J‑Link command
script and flashes the ELF binary.  Also bypasses the ROM bootloader by
setting PC=0x800 directly.

```bash
#!/bin/bash
# Flash a binary to FRDM-MCXA156 via J-Link
BIN="$1"
SCRIPT=$(mktemp)
cat > "$SCRIPT" <<FLASH_EOF
device MCXA156
si SWD
speed 1000
connect
r
h
loadfile $BIN
SetPC 0x800
g
qc
FLASH_EOF
JLinkExe -NoGui 1 -CommanderScript "$SCRIPT"
rm -f "$SCRIPT"
```

Make it executable: `chmod +x flash.sh`

---

## File: `Cargo.toml` (additions)

```toml
[dev-dependencies]
cortex-m-rt = "0.7.5"
panic-halt = "0.2.0"
```

---

## Build, Flash & Run

```bash
# One command — builds, flashes, and runs
cargo run --example blinky --release
```

The runner (`flash.sh`) uses JLinkExe because `probe-rs` has intermittent
DAP FAULT errors with the on-board MCU‑Link J‑Link firmware.  JLinkExe is
reliable and also bypasses the ROM bootloader (Issue 1) automatically.

If you need the ROM bootloader bypass as a standalone script, see `flash.jlink`:

```
device MCXA156
si SWD
speed 1000
connect
r
h
SetPC 0x800
g
qc
```

---

## Register Map

### MRCC0 (base 0x4009_1000)

| Register | Offset | Description | Bit 10 | Bit 23 |
|----------|--------|-------------|--------|--------|
| GLB_CC1 | 0x50 | AHB Clock Control 1 | PORT3 clock | GPIO3 clock |
| GLB_CC1_SET | 0x54 | Set bits in GLB_CC1 | Enable PORT3 clock | Enable GPIO3 clock |
| GLB_RST1 | 0x10 | Peripheral Reset Control 1 | PORT3 reset | GPIO3 reset |
| GLB_RST1_SET | 0x14 | Set bits in GLB_RST1 | Release PORT3 reset | Release GPIO3 reset |

### PORT3 (base 0x400B_F000)

| Register | Offset | Description |
|----------|--------|-------------|
| PCR13 | 0xB4 | Pin Control 13 — MUX field (bits 8-11): 0=ALT0/GPIO |

### GPIO3 (base 0x4010_5000)

| Register | Offset | Bit 13 field | Description |
|----------|--------|-------------|-------------|
| PDOR | 0x40 | PDO13 | Port Data Output |
| PSOR | 0x44 | PTSO13 | Port Set Output (1 → PDOR bit = 1) |
| PCOR | 0x48 | PTCO13 | Port Clear Output (1 → PDOR bit = 0) |
| PTOR | 0x4C | PTTO13 | Port Toggle Output |
| PDIR | 0x50 | PDI13 | Port Data Input (read-only) |
| PDDR | 0x54 | PDD13 | Port Data Direction (1 = output) |

---

## Known Issues & Workarounds

### 1. ROM bootloader does not jump to user code

The MCXA156 ROM enters ISP mode instead of booting the application at 0x00000000.
The vector table (SP=0x20020000, Reset=0x0801) is valid but the ROM requires a
boot configuration header (BCA / FCF) in flash that is not yet included.

**Workaround:** `flash.sh` (the `cargo run` runner) sets PC=0x800 directly via
J‑Link after flashing, bypassing the ROM bootloader entirely.

**Proper fix:** Add a Boot Configuration Area to the binary.  Investigate the
MCXA156 BCA format used by the NXP MCUXpresso SDK.

### 2. GLB_RST1 write ignored by CPU — FIXED: AHB write ordering

**Root cause:** The AHB bus can reorder `write_volatile` operations.  If the
`GLB_RST1_SET` store arrives at the MRCC peripheral before the `GLB_CC1_SET`
store, the hardware silently ignores the reset release (clock must be enabled
before reset can be released).

**Fix:** Insert a `cortex_m::asm::dsb()` (Data Synchronization Barrier) between
the clock-enable and reset-release writes.  This guarantees write ordering.

Without DSB a precise bus fault occurs at PORT3 PCR13 (`BFAR=0x400BF0B4`,
`BFSR.PRECISERR=1`, `HFSR.FORCED=1`).

### 3. svd2rust v0.37 API patterns

Generated PAC (v0.37.1) uses these closure patterns:

- **`modify(|r, w| { ... })`** — 2 arguments (reader, writer), must **return `w`**
- **`write(|w| { ... })`** — 1 argument (writer only), returns nothing
- **`_SET` / `_CLR` registers** — use `write(|w| w.data(…))`; the exact `data()`
  API differs between svd2rust versions.

The generated code produces 72 warnings about `unsafe_op_in_unsafe_fn` (Rust 2024
edition compatibility).  These are cosmetic and can be suppressed with
`#![allow(unsafe_op_in_unsafe_fn)]` in `src/lib.rs`.

### 4. probe-rs DAP FAULT with J-Link firmware

`probe-rs` intermittently fails with DAP FAULT / `sticky_err` after the J‑Link
probe has been used in a debug session.  Power‑cycling the board clears the
error, but the `flash.sh` runner (JLinkExe `loadfile`) is the reliable everyday
alternative.

---

## Debugging Summary

### ROM bootloader state (post-reset, CPU halted)

```
PC      = 0x20002040  (ROM ISP handler in RAM)
LR      = 0x030033C7  (ROM return address)
MSP     = 0x20002800
IPSR    = 0           (thread mode)
PRIMASK = 1           (interrupts masked)

GLB_CC1  = 0x00000B80  (PORT0/1/2/4, GPIO0/1/2 clocks enabled)
GLB_RST1 = 0x00000000  (ALL peripherals held in reset)

PORT3 PCR13 — inaccessible (bus fault)
GPIO3 PDOR  — inaccessible (bus fault)
```

### After successful blinky execution (code running)

```
GLB_CC1  = 0x00800F80  (PORT3 + GPIO3 clocks enabled)
GLB_RST1 = 0x00800400  (PORT3 + GPIO3 released from reset)
GPIO3 PDOR = toggling between 0x00000000 and 0x00002000
```

---

## Rationale for raw register writes

The blinky uses raw `write_volatile` instead of the PAC's type-safe API because
the `_SET` register `data()` API in svd2rust v0.37 was not fully resolved during
development.  Raw writes are also clearer for demonstrating register-level
operations in a PAC example.

The PAC's type-safe `modify()` API works correctly for the base registers and
produces identical machine code—verified for `GLB_CC1`.

---

## Lessons Learned

1. **Release peripherals from reset after enabling clocks.**  The ROM leaves many
   peripherals gated and held in reset.  After enabling `GLB_CCn` (clock), you
   must also write `GLB_RSTn` (reset release) before accessing the peripheral.
   Add a **DSB barrier** between the two writes — the hardware silently ignores
   the reset-release if it arrives before the clock enable.

2. **The ROM bootloader is not a simple jump-to-vector-table.**  It configures
   clocks, security, and locks registers.  Understanding the ROM's post-boot
   state is critical for bare-metal development.

3. **DAP (debug probe) writes and CPU (AHB) writes are NOT equivalent**
   when hardware has ordering requirements.  A DAP write always appears to
   "work" because the debugger inserts implicit barriers.

4. **Check fault status registers early.**  CFSR, HFSR, and BFAR pinpointed the
   exact bus fault address (`BFAR=0x400BF0B4` = PORT3 PCR13), saving hours of
   guesswork.

5. **The minimal test technique works.**  A firmware that writes a magic value
   to RAM and spins forever is the fastest way to verify that the CPU executes
   user code at all.

6. **Start with raw register writes when bring-up stalls.**  They eliminate
   abstraction-layer doubt and produce identical machine code to the PAC API.
