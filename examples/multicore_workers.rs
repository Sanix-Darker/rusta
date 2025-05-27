#![no_std]
#![no_main]
extern crate panic_halt;
use rusta::{cpu, println, delay};

static SHARED_COUNTER: spin::Mutex<u32> = spin::Mutex::new(0);

#[no_mangle]
fn _start() -> ! {
    if cpu::current_core() == 0 {
        // Core 0: Main coordinator
        println!("Starting worker cores...");

        // Start cores 1-3
        for core in 1..=3 {
            cpu::start_core(core, worker_thread);
        }

        // Monitor progress
        loop {
            let count = SHARED_COUNTER.lock();
            println!("Total work: {}", *count);
            delay::cycles(10_000_000);
        }
    } else {
        worker_thread()
    }
}

extern "C" fn worker_thread() -> ! {
    println!("Core {} working!", cpu::current_core());

    loop {
        let mut counter = SHARED_COUNTER.lock();
        *counter += 1;
        delay::cycles(1_000_000);
    }
}
