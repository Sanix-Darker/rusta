#![no_std]
#![no_main]
extern crate panic_halt;
use rusta::uart::UART;
#[no_mangle]
fn _start() -> ! {
    UART::init(115_200);
    loop {
        let c = UART::getc();
        UART::putc(c);
    }
}
