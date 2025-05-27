#![no_std]
#![no_main]
extern crate panic_halt;
use rusta::{cpu, delay, gpio::{Mode, GPIO}};

#[no_mangle]
fn _start() -> ! {
    // Core 0: Blink LED 21
    if cpu::current_core() == 0 {
        const LED1: usize = 21;
        GPIO::set_mode(LED1, Mode::Output);
        loop {
            GPIO::write(LED1, true);
            delay::cycles(5_000_000);
            GPIO::write(LED1, false);
            delay::cycles(5_000_000);
        }
    }
    // Core 1: Blink LED 20
    else {
        const LED2: usize = 20;
        GPIO::set_mode(LED2, Mode::Output);
        loop {
            GPIO::write(LED2, true);
            delay::cycles(3_000_000);
            GPIO::write(LED2, false);
            delay::cycles(3_000_000);
        }
    }
}
