#![no_std]
#![no_main]
extern crate panic_halt;
use rusta::{adc::ADC, delay, println, uart::UART};

#[no_mangle]
fn _start() -> ! {
    UART::init(115_200);
    ADC::init();

    loop {
        let value = ADC::read(0); // Read from channel 0
        println!("ADC value: {}", value);
        delay::cycles(1_000_000);
    }
}
