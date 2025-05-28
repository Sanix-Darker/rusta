// Button input with interrupt
#![no_std]
#![no_main]
use rusta::{hal::Peripherals, interrupt, println};

#[no_mangle]
fn _start() -> ! {
    let p = Peripherals::take().unwrap();
    let button = p.gpio.pin(17).into_input();

    interrupt::register(17, button_pressed);
    interrupt::enable(17);

    loop {}
}

fn button_pressed() {
    println!("Button pressed!");
}
