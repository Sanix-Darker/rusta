#![no_std]
#![no_main]
extern crate panic_halt;

use rusta::{arduino::*, delay};

const LED: usize = 21; // Pi-4 ACT LED ⇒ GPIO21

#[no_mangle]
fn _start() -> ! {
    pin_mode!(LED, OUTPUT);

    loop {
        digital_write!(LED, HIGH);
        delay::ms(500);

        digital_write!(LED, LOW);
        delay::ms(500);
    }
}
