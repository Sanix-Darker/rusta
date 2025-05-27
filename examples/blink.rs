#![no_std]
#![no_main]
extern crate panic_halt;
use rusta::{
    delay,
    gpio::{Mode, GPIO},
};
#[no_mangle]
fn _start() -> ! {
    const LED: usize = 21;
    GPIO::set_mode(LED, Mode::Output);
    loop {
        GPIO::write(LED, true);
        delay::cycles(50_000);

        GPIO::write(LED, false);
        delay::cycles(50_000);
    }
}
