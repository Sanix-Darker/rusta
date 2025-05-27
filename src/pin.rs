//! High-level Digital / Analog pin wrappers (Arduino-style).

use crate::gpio::{GPIO, Mode};

/// A compile-time digital pin.
pub struct DigitalPin<const P: usize>;

impl<const P: usize> DigitalPin<P> {
    /// Configure pin direction.
    #[inline(always)]
    pub fn set_mode(mode: Mode) {
        GPIO::set_mode(P, mode);
    }

    /// Drive logic-high.
    #[inline(always)]
    pub fn high() {
        GPIO::write(P, true);
    }

    /// Drive logic-low.
    #[inline(always)]
    pub fn low() {
        GPIO::write(P, false);
    }

    /// Toggle output.
    #[inline(always)]
    pub fn toggle() {
        if GPIO::read(P) {
            Self::low()
        } else {
            Self::high()
        }
    }

    /// Read current logic level.
    #[inline(always)]
    pub fn read() -> bool {
        GPIO::read(P)
    }
}

/// A compile-time ADC channel.
pub struct AnalogPin<const CH: u8>;
impl<const CH: u8> AnalogPin<CH> {
    /// Perform single conversion and return 12-bit value.
    #[inline(always)]
    pub fn read() -> u16 {
        crate::adc::ADC::read(CH) as u16
    }
}
