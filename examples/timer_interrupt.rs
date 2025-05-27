#![no_std]
#![no_main]
extern crate panic_halt;
use rusta::{println, timer, uart::UART};

static mut COUNTER: u32 = 0;

#[no_mangle]
fn _start() -> ! {
    UART::init(115_200);
    timer::init(1_000_000); // 1MHz timer

    // Set timer interrupt handler
    unsafe {
        timer::set_handler(timer_handler);
    }

    loop {}
}

extern "C" fn timer_handler() {
    unsafe {
        COUNTER += 1;
        println!("Timer tick: {}", COUNTER);
    }
}
