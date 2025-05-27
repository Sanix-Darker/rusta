#![no_std]
#![no_main]
extern crate panic_halt;
use rusta::{audio, dma, println};

const SAMPLE_RATE: u32 = 44100;
const BUFFER_SIZE: usize = 4096;

static mut AUDIO_BUFFER: [u16; BUFFER_SIZE] = [0; BUFFER_SIZE];

#[no_mangle]
fn _start() -> ! {
    println!("Initializing audio...");

    // Initialize audio system
    audio::init(SAMPLE_RATE);

    // Generate sine wave
    unsafe {
        for i in 0..BUFFER_SIZE {
            let t = i as f32 / SAMPLE_RATE as f32;
            let freq = 440.0; // A4 note
            let sample = (t * freq * 2.0 * core::f32::consts::PI).sin();
            AUDIO_BUFFER[i] = ((sample * 0.5 + 0.5) * 65535.0) as u16;
        }
    }

    // Start playback
    unsafe {
        audio::play(&AUDIO_BUFFER);
    }

    println!("Playing sound...");
    loop {}
}
