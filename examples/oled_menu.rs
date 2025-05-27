// examples/oled_menu.rs
#![no_std]
#![no_main]
extern crate panic_halt;
use rusta::{i2c::I2C, graphics::OLED, delay};

#[no_mangle]
fn _start() -> ! {
    let mut display = OLED::new(0x3C);
    display.init();

    loop {
        display.clear();
        display.text(5, 5, "RUSTA OS", Font::Large);
        display.rect(0, 20, 128, 30, Fill::Solid);
        display.text_centered(64, 35, "Temp: 23.5°C", Font::Medium);
        display.flush();
        delay::cycles(1_000_000);
    }
}
