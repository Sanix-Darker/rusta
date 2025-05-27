#![no_std]
#![no_main]
extern crate panic_halt;
use rusta::{gpio::{Mode, GPIO, Edge}, println};

static mut COUNTER: u32 = 0;

#[no_mangle]
fn _start() -> ! {
    const BUTTON_PIN: usize = 17;
    GPIO::set_mode(BUTTON_PIN, Mode::Input);
    GPIO::set_edge(BUTTON_PIN, Edge::Falling);
    GPIO::set_interrupt(BUTTON_PIN, true);

    println!("Press the button...");
    loop {}
}

#[no_mangle]
extern "C" fn handle_gpio() {
    unsafe { COUNTER += 1 };
    println!("Pressed {} times", unsafe { COUNTER });
    GPIO::clear_interrupt(17);
}
