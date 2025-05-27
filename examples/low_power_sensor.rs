// examples/low_power_sensor.rs
#![no_std]
#![no_main]
extern crate panic_halt;
use rusta::{gpio::GPIO, power::SleepMode, rtc::RTC};

#[no_mangle]
fn _start() -> ! {
    let sensor_power = 12;
    GPIO::set_mode(sensor_power, Mode::Output);

    loop {
        // Take measurement
        GPIO::write(sensor_power, true);
        delay::ms(10); // Sensor warmup
        let temp = read_temp();
        GPIO::write(sensor_power, false);

        // Deep sleep until next hour
        RTC::set_alarm(1 * 60 * 60); // 1 hour
        SleepMode::enter(SleepMode::STANDBY);
    }
}
