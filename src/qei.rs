use crate::{board::GPIO_BASE, volatile::Volatile};

#[repr(C)]
struct QeiRegs {
    gpio: [Volatile<u32>; 54],
    gpfsel: [Volatile<u32>; 6],
    gpset: [Volatile<u32>; 2],
    gpclr: [Volatile<u32>; 2],
    gplev: [Volatile<u32>; 2],
    gpeds: [Volatile<u32>; 2],
    gpren: [Volatile<u32>; 2],
    gpfen: [Volatile<u32>; 2],
    gphen: [Volatile<u32>; 2],
    gplen: [Volatile<u32>; 2],
    gparen: [Volatile<u32>; 2],
    gpafen: [Volatile<u32>; 2],
    gppud: Volatile<u32>,
    gppudclk: [Volatile<u32>; 2],
    qei_control: Volatile<u32>,
    qei_status: Volatile<u32>,
    qei_count: Volatile<u32>,
    qei_max_count: Volatile<u32>,
    qei_load: Volatile<u32>,
    qei_debounce: Volatile<u32>,
}

pub struct QuadratureEncoder {
    regs: &'static mut QeiRegs,
    pins: (usize, usize),
}

impl QuadratureEncoder {
    pub fn new(pin_a: usize, pin_b: usize) -> Self {
        let regs = unsafe { &mut *(GPIO_BASE as *mut QeiRegs) };

        // Configure pins as inputs
        regs.gpfsel[pin_a / 10].update(|v| v & !(7 << ((pin_a % 10) * 3)));
        regs.gpfsel[pin_b / 10].update(|v| v & !(7 << ((pin_b % 10) * 3)));

        // Enable QEI hardware
        regs.qei_control.write(
            (1 << 31) | // Enable
            (1 << 8) |  // Filter enable
            (3 << 0)    // Count both edges
        );

        Self { regs, pins: (pin_a, pin_b) }
    }

    pub fn count(&self) -> i32 {
        self.regs.qei_count.read() as i32
    }

    pub fn reset(&mut self) {
        self.regs.qei_load.write(0);
    }

    pub fn set_max_count(&mut self, max: u32) {
        self.regs.qei_max_count.write(max);
    }
}
