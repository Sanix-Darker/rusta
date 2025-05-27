#![no_std]
#![no_main]
extern crate panic_halt;
use rusta::{delay, i2c::I2C, uart::UART};
#[no_mangle]
fn _start() -> ! {
    I2C::init(2500);
    UART::init(115_200);
    loop {
        I2C::write(0x48, &[0x00]);
        UART::putc(b'.');
        delay::cycles(4_000_000);
    }
}
