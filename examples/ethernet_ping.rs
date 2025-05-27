#![no_std]
#![no_main]
extern crate panic_halt;
use rusta::{delay, ethernet, net::ip::Ipv4Addr, println};

#[no_mangle]
fn _start() -> ! {
    println!("Initializing Ethernet...");

    // Initialize with MAC address
    ethernet::init([0xDE, 0xAD, 0xBE, 0xEF, 0xFE, 0xED]);

    // Configure IP (static)
    ethernet::configure(
        Ipv4Addr::new(192, 168, 1, 100),
        Ipv4Addr::new(255, 255, 255, 0),
        Ipv4Addr::new(192, 168, 1, 1),
    );

    loop {
        match ethernet::ping(Ipv4Addr::new(8, 8, 8, 8)) {
            Ok(time) => println!("Ping response in {}ms", time),
            Err(e) => println!("Ping failed: {:?}", e),
        }
        delay::cycles(10_000_000);
    }
}
