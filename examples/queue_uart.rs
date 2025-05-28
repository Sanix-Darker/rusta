#![no_std]
#![no_main]
extern crate panic_halt;

use heapless::spsc::Queue;
use rusta::{delay, uart::UART};

static mut Q: Queue<u8, 64> = Queue::new();

#[no_mangle]
fn _start() -> ! {
    UART::init(115_200);

    unsafe {
        let (mut p, mut c) = Q.split();

        // Producer core
        if rusta::cpu::current_core() == 0 {
            loop {
                p.enqueue(b'A').ok();
                delay::ms(1);
            }
        }
        // Consumer core
        loop {
            if let Some(b) = c.dequeue() {
                UART::putc(b);
            }
        }
    }
}
