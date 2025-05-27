#![no_std]
#![no_main]
extern crate panic_halt;
use rusta::{delay, gpio::{Mode, GPIO}, println};

#[no_mangle]
fn _start() -> ! {
    const TRIG_PIN: usize = 17;
    const ECHO_PIN: usize = 27;

    GPIO::set_mode(TRIG_PIN, Mode::Output);
    GPIO::set_mode(ECHO_PIN, Mode::Input);

    loop {
        let distance_cm = measure_distance(TRIG_PIN, ECHO_PIN);
        println!("Distance: {} cm", distance_cm);
        delay::cycles(1_000_000); // 1s delay
    }
}

fn measure_distance(trig: usize, echo: usize) -> u32 {
    // Send 10µs pulse
    GPIO::write(trig, true);
    delay::cycles(10); // 10µs
    GPIO::write(trig, false);

    // Wait for echo start
    while !GPIO::read(echo) {}
    let start = get_micros();

    // Wait for echo end
    while GPIO::read(echo) {}
    let duration = get_micros() - start;

    // Calculate distance (sound travels 343m/s or 0.0343cm/µs)
    (duration * 343) / 20_000 // Simplified cm calculation
}

#[inline(always)]
fn get_micros() -> u32 {
    unsafe { core::arch::asm!("mrs {}, cntpct_el0", out(reg) x) };
    x / 50 // Assuming 50MHz timer
}
