#![no_std]
extern crate panic_halt as _;

// Board selection & re‑export handled inside src/board/mod.rs
pub mod board;

pub mod delay;
pub mod gpio;
pub mod i2c;
pub mod mmio;
pub mod pwm;
pub mod serial;
pub mod spi;
pub mod uart;
pub mod volatile;
