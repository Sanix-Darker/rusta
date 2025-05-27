#![no_std]
#![no_main]
extern crate panic_halt;
use rusta::{delay, i2c::I2C, println};

#[no_mangle]
fn _start() -> ! {
    const PN532_ADDR: u8 = 0x24;
    I2C::init(400_000); // Fast mode required

    // Wake up NFC reader
    I2C::write(PN532_ADDR, &[0x00, 0xFF, 0x01, 0xFE]);

    loop {
        if let Some(uid) = scan_card(PN532_ADDR) {
            println!("Card UID: {:X?}", uid);
        }
        delay::ms(100);
    }
}

fn scan_card(addr: u8) -> Option<[u8; 7]> {
    // Send ISO14443A init
    I2C::write(addr, &[0x00, 0x4A, 0x01, 0x00]);

    // Read response
    let mut buf = [0; 32];
    I2C::read(addr, &mut buf);

    if buf[0] == 0x01 {
        // Valid card
        Some(buf[1..8].try_into().unwrap())
    } else {
        None
    }
}
