#![no_std]
#![no_main]
extern crate panic_halt;
use rusta::{println, watchdog::Watchdog, delay};

#[no_mangle]
fn _start() -> ! {
    println!("Enabling watchdog for 4 s");
    Watchdog::enable(4_000);

    // Simulate hang after 10 s – board will auto-reset
    delay::ms(10_000);

    loop {} // never reached
}
