// sd card access
#![no_std]
#![no_main]
extern crate panic_halt;
use rusta::{println, sdcard, delay};

#[no_mangle]
fn _start() -> ! {
    println!("Mounting SD card...");
    let mut card = sdcard::init().unwrap();

    // Read boot sector
    let mut buffer = [0u8; 512];
    card.read_sector(0, &mut buffer).unwrap();

    println!("First sector:");
    for i in 0..16 {
        println!("{:02x} ", buffer[i]);
    }

    loop {
        delay::cycles(10_000_000);
    }
}
