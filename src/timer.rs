use crate::{board::TIMER_BASE, volatile::Volatile};

#[repr(C)]
struct Reg {
    cs: Volatile<u32>,
    clo: Volatile<u32>,
    chi: Volatile<u32>,
    c0: Volatile<u32>,
    c1: Volatile<u32>,
    c2: Volatile<u32>,
    c3: Volatile<u32>,
}

static mut HANDLER: Option<extern "C" fn()> = None;

pub struct Timer;
impl Timer {
    fn r() -> &'static mut Reg {
        unsafe { &mut *(TIMER_BASE as *mut Reg) }
    }

    pub fn init(freq: u32) {
        let r = Self::r();
        let interval = 1_000_000 / freq; // Microseconds
        r.c1.write(r.clo.read() + interval);
    }

    pub unsafe fn set_handler(handler: extern "C" fn()) {
        HANDLER = Some(handler);
    }
}

#[no_mangle]
pub extern "C" fn timer_irq_handler() {
    let r = Timer::r();
    r.cs.write(1 << 1); // Clear interrupt

    if let Some(handler) = unsafe { HANDLER } {
        handler();
    }

    // Schedule next interrupt
    r.c1.write(r.clo.read() + 1_000_000);
}
