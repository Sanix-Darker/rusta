// DHT11/DHT22 Temp / Humidity sensors
#![no_std]
#![no_main]
extern crate panic_halt;
use rusta::{
    delay,
    gpio::{Mode, Pull, GPIO},
    println,
};

#[no_mangle]
fn _start() -> ! {
    const DHT_PIN: usize = 4;
    GPIO::set_mode(DHT_PIN, Mode::Input);
    GPIO::set_pull(DHT_PIN, Pull::Up);

    loop {
        match read_dht(DHT_PIN) {
            Ok((temp, humi)) => println!("Temp: {}°C, Humi: {}%", temp, humi),
            Err(e) => println!("Error: {:?}", e),
        }
        delay::cycles(2_000_000); // 2s delay between reads
    }
}

fn read_dht(pin: usize) -> Result<(i8, u8), &'static str> {
    // Send start signal
    GPIO::set_mode(pin, Mode::Output);
    GPIO::write(pin, false);
    delay::cycles(18_000); // 18ms
    GPIO::write(pin, true);
    delay::cycles(30); // 30µs

    // Switch to input and wait for response
    GPIO::set_mode(pin, Mode::Input);
    wait_pin(pin, false)?; // Wait for low
    wait_pin(pin, true)?; // Wait for high
    wait_pin(pin, false)?; // Ready for data

    // Read 40 bits
    let mut data = [0u8; 5];
    for byte in &mut data {
        for _ in 0..8 {
            *byte <<= 1;
            wait_pin(pin, true)?;
            let dur = measure_pin(pin, false)?;
            if dur > 40 {
                // 40µs threshold
                *byte |= 1;
            }
        }
    }

    // Verify checksum
    if data[4]
        == data[0]
            .wrapping_add(data[1])
            .wrapping_add(data[2])
            .wrapping_add(data[3])
    {
        Ok((data[2] as i8, data[0]))
    } else {
        Err("Checksum error")
    }
}

// Helper functions for precise timing
fn wait_pin(pin: usize, state: bool) -> Result<(), &'static str> {
    for _ in 0..100 {
        // Timeout ~100µs
        if GPIO::read(pin) == state {
            return Ok(());
        }
        delay::cycles(1);
    }
    Err("Timeout")
}

fn measure_pin(pin: usize, state: bool) -> Result<u32, &'static str> {
    let mut count = 0;
    while GPIO::read(pin) == state {
        count += 1;
        delay::cycles(1);
        if count > 100 {
            return Err("Timeout");
        }
    }
    Ok(count)
}
