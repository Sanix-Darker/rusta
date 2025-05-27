#![no_std]
#![no_main]
extern crate panic_halt;
use rusta::{
    gpio::{Mode, GPIO},
    uart::UART,
};
#[no_mangle]
fn _start() -> ! {
    const BTN: usize = 17;
    GPIO::set_mode(BTN, Mode::Input);
    UART::init(115_200);
    loop {
        UART::putc(if GPIO::read(BTN) { b'1' } else { b'0' });
    }
}
