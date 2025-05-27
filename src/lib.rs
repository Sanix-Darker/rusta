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
pub mod adc;
pub mod uart;
pub mod volatile;
pub mod pin;        // Arduino-style Digital/Analog helpers
pub mod watchdog;   // PM-block watchdog API
pub mod arduino;    // digitalWrite / digitalRead macro façade
