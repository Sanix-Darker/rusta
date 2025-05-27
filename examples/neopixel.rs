#![no_std]
#![no_main]
extern crate panic_halt;
use rusta::{
    delay,
    gpio::{Mode, GPIO},
};

#[no_mangle]
fn _start() -> ! {
    const DATA_PIN: usize = 18;
    GPIO::set_mode(DATA_PIN, Mode::Output);

    // Rainbow cycle demo
    let mut hue = 0;
    loop {
        let (r, g, b) = hsv_to_rgb(hue, 255, 255);
        send_neopixel_color(r, g, b);
        hue = (hue + 1) % 360;
        delay::cycles(100_000);
    }
}

fn send_neopixel_color(r: u8, g: u8, b: u8) {
    let data = [g, r, b]; // GRB format

    // Critical timing section - disable interrupts if using them
    unsafe {
        core::arch::asm!("cpsid i"); // Disable interrupts
    }

    for byte in data {
        for i in (0..8).rev() {
            let bit = (byte >> i) & 1;

            // Timing critical - adjust based on your CPU speed
            if bit == 1 {
                // Send '1' (T1H = 0.8µs, T1L = 0.45µs)
                GPIO::write(DATA_PIN, true);
                delay::cycles(40); // ~0.8µs at 50MHz
                GPIO::write(DATA_PIN, false);
                delay::cycles(20); // ~0.45µs
            } else {
                // Send '0' (T0H = 0.4µs, T0L = 0.85µs)
                GPIO::write(DATA_PIN, true);
                delay::cycles(20); // ~0.4µs
                GPIO::write(DATA_PIN, false);
                delay::cycles(40); // ~0.85µs
            }
        }
    }

    // Re-enable interrupts
    unsafe {
        core::arch::asm!("cpsie i");
    }

    // Reset signal (>50µs)
    delay::cycles(2500);
}

fn hsv_to_rgb(h: u16, s: u8, v: u8) -> (u8, u8, u8) {
    let c = (v as u16 * s as u16) / 255;
    let x = (c as u16 * (60 - h % 60)) / 60;
    let m = v as u16 - c;

    let (r, g, b) = match h / 60 {
        0 => (c, x, 0),
        1 => (x, c, 0),
        2 => (0, c, x),
        3 => (0, x, c),
        4 => (x, 0, c),
        _ => (c, 0, x),
    };

    (
        (r + m).min(255) as u8,
        (g + m).min(255) as u8,
        (b + m).min(255) as u8,
    )
}
