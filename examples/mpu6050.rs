// MPU6050 Accelerometer/Gyro
#![no_std]
#![no_main]
extern crate panic_halt;
use rusta::{i2c::I2C, delay, println};

#[no_mangle]
fn _start() -> ! {
    const MPU_ADDR: u8 = 0x68;
    I2C::init(100_000); // 100kHz

    // Wake up MPU6050
    I2C::write(MPU_ADDR, &[0x6B, 0x00]);

    loop {
        let accel = read_accel(MPU_ADDR);
        println!("X: {}, Y: {}, Z: {}", accel.0, accel.1, accel.2);
        delay::cycles(500_000); // 50ms
    }
}

fn read_accel(addr: u8) -> (i16, i16, i16) {
    let mut buf = [0u8; 6];
    I2C::write(addr, &[0x3B]); // Start at ACCEL_XOUT_H
    I2C::read(addr, &mut buf);

    (
        ((buf[0] as i16) << 8 | buf[1] as i16) / 16384, // X
        ((buf[2] as i16) << 8 | buf[3] as i16) / 16384, // Y
        ((buf[4] as i16) << 8 | buf[5] as i16) / 16384, // Z
    )
}
