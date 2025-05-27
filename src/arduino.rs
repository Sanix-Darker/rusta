//! src/arduino.rs  — *Compatibility layer*
//
//  This file tries to mimic as much of the classic Arduino
//  core API as practical on bare-metal Raspberry-Pi/RUSTA.
//
//  All helpers are `const-fn`, `inline(always)` or macros so
//  the optimiser can elide any overhead.  Uses only the
//  existing zero-allocation drivers already in the project.
//
//  ────────────────────────────────────────────────────────────────
//  DIGITAL I/O     pin_mode!(21, OUTPUT); digital_write!(21, HIGH);
//  ANALOG / PWM    let v = analog_read!(0);  analog_write!(21, 128);
//  TIMING          delay(1000);  let t = millis();
//  COMM            serial_println!("value = {}", 42);
//  MISC            map!(x, 0,1023 , 0,180);
//  ────────────────────────────────────────────────────────────────
//
//  *No additional crates required.*  This compiles `#![no_std]`
//
// ----------------------------------------------------------------

#![allow(dead_code)]
#![allow(unused_macros)]

use core::sync::atomic::{AtomicBool, Ordering};

use crate::{
    delay,
    gpio::{Edge, GPIO, Mode},
    pwm::PWM,
};

pub const INPUT: Mode = Mode::Input;
pub const OUTPUT: Mode = Mode::Output;
pub const INPUT_PULLUP: Mode = Mode::Input;

pub const HIGH: bool = true;
pub const LOW: bool = false;

pub const LSBFIRST: bool = false;
pub const MSBFIRST: bool = true;

/// Milliseconds since boot (wraps ~71min).
#[inline(always)]
pub fn millis() -> u64 {
    delay::micros() / 1_000
}

/// Micro-seconds since boot (wraps ~71min).
#[inline(always)]
pub fn micros() -> u64 {
    delay::micros()
}

/// Blocking delay in *milliseconds*.
#[inline(always)]
pub fn delay(ms: u32) {
    delay::ms(ms)
}

/// Blocking delay in *micro-seconds*.
#[inline(always)]
pub fn delay_microseconds(us: u32) {
    delay::us(us)
}

/// Globally enable interrupts.
#[inline(always)]
pub fn interrupts() {
    unsafe { core::arch::asm!("cpsie i") };
}
/// Globally disable interrupts.
#[inline(always)]
pub fn no_interrupts() {
    unsafe { core::arch::asm!("cpsid i") };
}

#[macro_export]
macro_rules! pin_mode {
    ($pin:expr, OUTPUT) => { $crate::gpio::GPIO::set_mode($pin, $crate::gpio::Mode::Output) };
    ($pin:expr, INPUT) => { $crate::gpio::GPIO::set_mode($pin, $crate::gpio::Mode::Input) };
    ($pin:expr, INPUT_PULLUP) => {{
        use $crate::gpio::{GPIO, Mode, Pull};
        GPIO::set_mode($pin, Mode::Input);
        GPIO::set_pull($pin, Pull::Up);
    }};
}

#[macro_export]
macro_rules! digital_write {
    ($pin:expr, HIGH) => { $crate::gpio::GPIO::write($pin, true) };
    ($pin:expr, LOW)  => { $crate::gpio::GPIO::write($pin, false) };
}

#[macro_export]
macro_rules! digital_read {
    ($pin:expr) => { $crate::gpio::GPIO::read($pin) };
}

/// 12-bit right-aligned ADC read.
/// `analog_read!(0)` → `u16`.
#[macro_export]
macro_rules! analog_read {
    ($ch:expr) => { $crate::pin::AnalogPin::<$ch>::read() };
}

/// “analogWrite” – duty 0-255 like Arduino.
/// For >8-bit fidelity call `pwm::PWM::set_duty` directly.
#[macro_export]
macro_rules! analog_write {
    ($pin:expr, $val:expr) => {{
        let duty: f32 = ($val as f32) / 255.0;
        $crate::pwm::PWM::set_duty(duty);
    }};
}

static TONE_ACTIVE: AtomicBool = AtomicBool::new(false);

/// Start a continuous square-wave on the *PWM* channel.
#[inline]
pub fn tone(_pin: usize, freq_hz: u32) {
    if !TONE_ACTIVE.swap(true, Ordering::SeqCst) {
        PWM::init(freq_hz, 0.5); // 50 % duty
    } else {
        PWM::set_duty(0.5);
    }
}

/// Stop the tone.
#[inline]
pub fn no_tone(_pin: usize) {
    if TONE_ACTIVE.swap(false, Ordering::SeqCst) {
        PWM::set_duty(0.0);
    }
}

/// Read a byte using bit-bang SPI (max ~10 MHz reliable on Pi-4 @1.5 GHz).
pub fn shift_in<const ORDER: bool>(data_pin: usize, clock_pin: usize) -> u8 {
    let mut value = 0u8;
    for i in 0..8 {
        GPIO::write(clock_pin, HIGH);
        delay::cycles(1);
        if ORDER == LSBFIRST {
            value |= (GPIO::read(data_pin) as u8) << i;
        } else {
            value |= (GPIO::read(data_pin) as u8) << (7 - i);
        }
        GPIO::write(clock_pin, LOW);
        delay::cycles(1);
    }
    value
}

/// Write a byte using bit-bang SPI.
pub fn shift_out<const ORDER: bool>(data_pin: usize, clock_pin: usize, value: u8) {
    for i in 0..8 {
        let bit_val = if ORDER == LSBFIRST {
            (value >> i) & 1
        } else {
            (value >> (7 - i)) & 1
        };
        GPIO::write(data_pin, bit_val != 0);
        GPIO::write(clock_pin, HIGH);
        delay::cycles(1);
        GPIO::write(clock_pin, LOW);
        delay::cycles(1);
    }
}

// pulseIn

/// Measure pulse length (µs).  Returns 0 on timeout.
pub fn pulse_in(pin: usize, level: bool, timeout_us: u32) -> u32 {
    let start_time = micros();

    // Wait for any previous pulse to end
    while GPIO::read(pin) == level {
        if micros() - start_time > timeout_us as u64 {
            return 0;
        }
    }

    // Wait for the pulse to start
    while GPIO::read(pin) != level {
        if micros() - start_time > timeout_us as u64 {
            return 0;
        }
    }
    let pulse_start = micros();

    // Measure how long the pulse goes high/low
    while GPIO::read(pin) == level {
        if micros() - pulse_start > timeout_us as u64 {
            return 0;
        }
    }
    (micros() - pulse_start) as u32
}

/* attachInterrupt / detachInterrupt */

type Isr = extern "C" fn();

static mut ISR_TABLE: [Option<Isr>; 54] = [None; 54];

/// Attach an ISR to a GPIO pin (RISING/FALLING/CHANGE).
pub fn attach_interrupt(pin: usize, mode: Edge, handler: Isr) {
    unsafe {
        ISR_TABLE[pin] = Some(handler);
        GPIO::set_edge(pin, mode);
        GPIO::enable_interrupt(pin);
    }
}

/// Detach the ISR and disable interrupt.
pub fn detach_interrupt(pin: usize) {
    unsafe {
        ISR_TABLE[pin] = None;
        GPIO::disable_interrupt(pin);
    }
}

/// *Called by low-level IRQ handler in your vector table.*
#[no_mangle]
pub extern "C" fn handle_gpio_isr(pin: usize) {
    unsafe {
        if let Some(isr) = ISR_TABLE[pin] {
            isr();
        }
        GPIO::clear_interrupt(pin);
    }
}

#[macro_export]
macro_rules! constrain {
    ($x:expr, $a:expr, $b:expr) => {
        if $x < $a { $a } else if $x > $b { $b } else { $x }
    };
}

#[macro_export]
macro_rules! map {
    ($x:expr, $in_min:expr, $in_max:expr, $out_min:expr, $out_max:expr) => {
        ($x as i32 - $in_min as i32) * ($out_max as i32 - $out_min as i32)
            / ($in_max as i32 - $in_min as i32)
            + $out_min as i32
    };
}

#[macro_export]
macro_rules! min {
    ($a:expr, $b:expr) => {
        if $a < $b { $a } else { $b }
    };
}

#[macro_export]
macro_rules! max {
    ($a:expr, $b:expr) => {
        if $a > $b { $a } else { $b }
    };
}

static mut LCG_SEED: u32 = 1;

/// Seed the pseudo-random generator.
#[inline(always)]
pub fn random_seed(seed: u32) {
    unsafe { LCG_SEED = seed }
}

/// Return a random 31-bit integer.
#[inline(always)]
pub fn random_u32() -> u32 {
    unsafe {
        // LCG parameters from Numerical Recipes
        LCG_SEED = LCG_SEED.wrapping_mul(1664525).wrapping_add(1013904223);
        LCG_SEED & 0x7FFF_FFFF
    }
}

/// Random in range [0, max)
#[inline(always)]
pub fn random(max: u32) -> u32 {
    random_u32() % max
}

/// Random in range [min, max)
#[inline(always)]
pub fn random_range(min: u32, max: u32) -> u32 {
    min + random(max - min)
}

#[macro_export]
macro_rules! bit_set {
    ($x:expr, $n:expr) => { $x |= 1 << $n };
}

#[macro_export]
macro_rules! bit_clear {
    ($x:expr, $n:expr) => { $x &= !(1 << $n) };
}

#[macro_export]
macro_rules! bit_read {
    ($x:expr, $n:expr) => { ($x >> $n) & 1 };
}

#[macro_export]
macro_rules! bit_write {
    ($x:expr, $n:expr, $b:expr) => {
        if $b != 0 { bit_set!($x, $n) } else { bit_clear!($x, $n) }
    };
}

/// Formatted print on primary UART (115 200 Bd default).
#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        let _ = core::fmt::write(&mut $crate::uart::UART::writer(),
                                 format_args!($($arg)*));
    }};
}

/// Like `serial_print!` but with LF.
#[macro_export]
macro_rules! serial_println {
    () => { $crate::serial_print!("\r\n"); };
    ($fmt:expr) => { $crate::serial_print!(concat!($fmt, "\r\n")); };
    ($fmt:expr, $($arg:tt)*) => { $crate::serial_print!(concat!($fmt, "\r\n"), $($arg)*); };
}

/// Enter low-power *Wait-For-Interrupt* state until next IRQ.
#[inline(always)]
pub fn sleep() {
    unsafe { core::arch::asm!("wfi") };
}

/* --------------------------------------------------------------------- */
/* Example usage                                                         */
/* --------------------------------------------------------------------- */
/*
#![no_std]
#![no_main]
extern crate panic_halt;

use rusta::{arduino::*, delay};

#[no_mangle]
fn _start() -> ! {
    pin_mode!(21, OUTPUT);

    loop {
        digital_write!(21, HIGH);
        delay(500);
        digital_write!(21, LOW);
        delay(500);
    }
}
*/
