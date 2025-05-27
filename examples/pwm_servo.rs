#![no_std]
#![no_main]
extern crate panic_halt;
use rusta::{delay, pwm::PWM};
#[no_mangle]
fn _start() -> ! {
    PWM::init(50, 0.05);
    let mut d = 0.05;
    loop {
        PWM::set_duty(d);
        d += 0.01;
        if d > 0.10 {
            d = 0.05;
        }
        delay::cycles(2_000_000);
    }
}
