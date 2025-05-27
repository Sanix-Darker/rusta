// examples/brushless_control.rs
#![no_std]
#![no_main]
extern crate panic_halt;
use rusta::{pwm::PWM, qei::QuadratureEncoder, println};

#[no_mangle]
fn _start() -> ! {
    // 16kHz PWM for silent operation
    PWM::init(16_000, 0.0);
    let enc = QuadratureEncoder::new(2, 3); // Pins for A/B phases

    // PID controller
    let mut pid = Pid::new(1.0, 0.1, 0.01);
    pid.setpoint = 1000; // Target RPM

    loop {
        let rpm = enc.rpm();
        let duty = pid.update(rpm);
        PWM::set_duty(duty.clamp(0.05, 0.95)); // Safe limits

        println!("RPM: {:.1}, Duty: {:.1}%", rpm, duty*100.0);
        delay::cycles(1_000_000 / 20); // 20Hz update
    }
}
