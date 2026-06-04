# `mcxa156-pac` — Peripheral Access Crate for NXP MCXA156

[![crates.io](https://img.shields.io/crates/v/mcxa156-pac.svg)](https://crates.io/crates/mcxa156-pac)
[![docs.rs](https://docs.rs/mcxa156-pac/badge.svg)](https://docs.rs/mcxa156-pac)

Low-level register mappings for the **NXP MCXA156** Arm® Cortex®-M33 microcontroller, generated from NXP's official CMSIS-SVD file.

This crate provides **type-safe, zero-cost** access to every peripheral, register, and field on the MCXA156. It is the foundation layer for a future Hardware Abstraction Layer (HAL).

## Supported Board

- [NXP FRDM-MCXA156](https://www.nxp.com/FRDM-MCXA156) development board

## MCU Specifications

| Feature | Value |
|---------|-------|
| Core | Arm Cortex-M33 @ 96 MHz |
| Flash | 1 MB (dual-bank) |
| RAM | 128 KB (8 KB with ECC) |
| Connectivity | CAN-FD, I3C, LPSPI, LPI2C, LPLUART, USB FS, FlexIO |

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
frdm-mcxa156-pac = "0.1.0"
```

For portable atomic support (recommended on Cortex-M33):

```toml
[dependencies]
frdm-mcxa156-pac = { version = "0.1.0", features = ["portable-atomic"] }
```

### Example

```rust
#![no_std]
#![no_main]

use frdm_mcxa156_pac::Peripherals;
use cortex_m_rt::entry;
use panic_halt as _;

#[entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();

    // Access a peripheral
    let gpio = &p.GPIO0;

    loop {
        // Your application logic here
    }
}
```

## Prerequisites

- Rust 1.96+ with `thumbv8m.main-none-eabihf` target
  ```bash
  rustup target add thumbv8m.main-none-eabihf
  ```
- `cortex-m-rt` for startup and interrupt vector table
- `panic-halt` (or `panic-semihosting`) for panic behavior

## Regenerating from SVD

This crate is generated from NXP's official SVD file using [`svd2rust`](https://docs.rs/svd2rust/). To regenerate:

```bash
# Install tools
cargo install svd2rust
cargo install form

# Download SVD
curl -L -o svd/MCXA156.xml \
  https://raw.githubusercontent.com/nxp-mcuxpresso/mcux-soc-svd/refs/heads/release/25.06.00/MCXA156/MCXA156.xml

# Generate PAC
svd2rust -i svd/MCXA156.xml \
  --target cortex-m \
  --atomics \
  --atomics_feature "portable-atomic"

# Format into per-module files
form -i lib.rs -o src/ && rm lib.rs

# Verify
cargo build
```

## SVD Source

The SVD file is sourced from NXP's official repository:
[NXP MCUXpresso SDK SoC SVDs](https://github.com/nxp-mcuxpresso/mcux-soc-svd)

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.

## Status

⚠️ **Early development** — in progress. Not yet published to crates.io.

---

**Disclaimer:** This is an individual effort, not an official NXP project. The PAC is generated from NXP's publicly available SVD file.
