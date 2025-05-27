#![no_std]
#![no_main]
extern crate panic_halt;
use rusta::{delay, spi::SPI};
#[no_mangle]
fn _start() -> ! {
    SPI::init(64);
    for &b in &[0xAE, 0xA1, 0xAF] {
        SPI::xfer(b);
    }
    loop {
        delay::cycles(1_000_000);
    }
}
