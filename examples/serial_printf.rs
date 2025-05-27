#![no_std]
#![no_main]
extern crate panic_halt;
use rusta::{delay, println, uart::UART};
#[no_mangle]
fn _start() -> ! {
    UART::init(115_200);
    let mut n = 0;
    loop {
        println!("counter = {}", n);
        n += 1;
        delay::cycles(2_000_000);
    }
}
