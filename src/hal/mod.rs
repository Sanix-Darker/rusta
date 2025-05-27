use core::marker::PhantomData;
use crate::{
    gpio::{self, Mode},
    uart::UART,
    cpu,
    timer
};

pub struct Peripherals {
    pub gpio: GPIO,
    pub uart0: UART,
}

impl Peripherals {
    pub fn take() -> Option<Self> {
        Some(Self {
            gpio: GPIO::new(),
            uart0: UART::new(),
        })
    }
}

pub struct GPIO {
    _private: (),
}

impl GPIO {
    pub fn new() -> Self {
        Self { _private: () }
    }

    pub fn pin(&self, pin: u8) -> Pin<Unknown> {
        Pin::new(pin)
    }
}

pub struct Unknown;
pub struct Input;
pub struct Output;
pub struct Alternate;

pub struct Pin<MODE> {
    pin: u8,
    _mode: PhantomData<MODE>,
}

impl<MODE> Pin<MODE> {
    fn new(pin: u8) -> Self {
        Self { pin, _mode: PhantomData }
    }

    pub fn into_input(self) -> Pin<Input> {
        gpio::set_mode(self.pin as usize, Mode::Input);
        Pin::new(self.pin)
    }

    pub fn into_output(self) -> Pin<Output> {
        gpio::set_mode(self.pin as usize, Mode::Output);
        Pin::new(self.pin)
    }
}

impl Pin<Output> {
    pub fn set_high(&self) {
        gpio::write(self.pin as usize, true);
    }

    pub fn set_low(&self) {
        gpio::write(self.pin as usize, false);
    }

    pub fn toggle(&self) {
        gpio::toggle(self.pin as usize);
    }
}
