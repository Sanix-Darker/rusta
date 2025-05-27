use crate::{board::GPIO_BASE, volatile::Volatile};

#[repr(C)]
struct QeiRegs {
    gpio: [Volatile<u32>; 54],
    // ... other registers
}

pub struct QuadratureEncoder {
    pin_a: usize,
    pin_b: usize,
    last_state: u8,
    count: i32,
}

impl QuadratureEncoder {
    pub fn new(pin_a: usize, pin_b: usize) -> Self {
        unsafe {
            let regs = &mut *(GPIO_BASE as *mut QeiRegs);
            regs.gpio[pin_a].write(0); // Input mode
            regs.gpio[pin_b].write(0); // Input mode
        }

        Self {
            pin_a,
            pin_b,
            last_state: 0,
            count: 0,
        }
    }

    pub fn update(&mut self) {
        let state = (GPIO::read(self.pin_a) as u8 | ((GPIO::read(self.pin_b) as u8) << 1;
        let change = (self.last_state << 2) | state;

        match change {
            0b0001 | 0b0111 | 0b1110 | 0b1000 => self.count += 1,
            0b0010 | 0b1011 | 0b1101 | 0b0100 => self.count -= 1,
            _ => (),
        }

        self.last_state = state;
    }

    pub fn count(&self) -> i32 {
        self.count
    }

    pub fn rpm(&self, pulses_per_rev: u32, interval_ms: u32) -> f32 {
        (self.count as f32 * 60_000.0) / (pulses_per_rev as f32 * interval_ms as f32)
    }
}
