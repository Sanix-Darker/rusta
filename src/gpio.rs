use crate::{board::GPIO_BASE, delay, volatile::Volatile};

#[repr(u32)]
pub enum Mode {
    Input = 0,
    Output = 1,
    Alt5 = 2,
    Alt4 = 3,
    Alt0 = 4,
    Alt1 = 5,
    Alt2 = 6,
    Alt3 = 7,
}

#[repr(u32)]
pub enum Pull {
    Off = 0,
    Down = 1,
    Up = 2,
}

#[repr(u32)]
pub enum Edge {
    Rising = 0,
    Falling = 1,
    Both = 2,
}

#[repr(C)]
struct Regs {
    gpfsel: [Volatile<u32>; 6],
    _reserved0: Volatile<u32>,
    gpset: [Volatile<u32>; 2],
    _reserved1: Volatile<u32>,
    gpclr: [Volatile<u32>; 2],
    _reserved2: Volatile<u32>,
    gplev: [Volatile<u32>; 2],
    _reserved3: Volatile<u32>,
    gpeds: [Volatile<u32>; 2],
    _reserved4: Volatile<u32>,
    gpren: [Volatile<u32>; 2], // Rising edge detect enable
    _reserved5: Volatile<u32>,
    gpfen: [Volatile<u32>; 2], // Falling edge detect enable
    _reserved6: Volatile<u32>,
    gphen: [Volatile<u32>; 2], // High level detect enable
    _reserved7: Volatile<u32>,
    gplen: [Volatile<u32>; 2], // Low level detect enable
    _reserved8: Volatile<u32>,
    gparen: [Volatile<u32>; 2], // Async rising edge detect
    _reserved9: Volatile<u32>,
    gpafen: [Volatile<u32>; 2], // Async falling edge detect
    _reserved10: Volatile<u32>,
    gppud: Volatile<u32>,
    gppudclk: [Volatile<u32>; 2],
}

pub struct GPIO;
impl GPIO {
    fn r() -> &'static mut Regs {
        unsafe { &mut *(GPIO_BASE as *mut Regs) }
    }

    pub fn set_mode(p: usize, m: Mode) {
        let reg = p / 10;
        let s = (p % 10) * 3;
        let r = Self::r();
        let mut v = r.gpfsel[reg].read();
        v &= !(0b111 << s);
        v |= (m as u32) << s;
        r.gpfsel[reg].write(v);
    }

    pub fn write(p: usize, h: bool) {
        let reg = p / 32;
        let s = p % 32;
        if h {
            Self::r().gpset[reg].write(1 << s);
        } else {
            Self::r().gpclr[reg].write(1 << s);
        }
    }

    pub fn read(p: usize) -> bool {
        let reg = p / 32;
        let s = p % 32;
        (Self::r().gplev[reg].read() & (1 << s)) != 0
    }

    pub fn set_pull(p: usize, pull: Pull) {
        let r = Self::r();
        r.gppud.write(pull as u32);
        delay::cycles(150);
        r.gppudclk[p / 32].write(1 << (p % 32));
        delay::cycles(150);
        r.gppud.write(0);
        r.gppudclk[p / 32].write(0);
    }

    pub fn set_edge(p: usize, edge: Edge) {
        let bank = p / 32;
        let bit = p % 32;
        let r = Self::r();

        match edge {
            Edge::Rising => {
                r.gpren[bank].write(1 << bit);
                r.gpfen[bank].write(0);
            }
            Edge::Falling => {
                r.gpfen[bank].write(1 << bit);
                r.gpren[bank].write(0);
            }
            Edge::Both => {
                r.gpren[bank].write(1 << bit);
                r.gpfen[bank].write(1 << bit);
            }
        }
    }

    pub fn enable_interrupt(p: usize) {
        let bank = p / 32;
        let bit = p % 32;
        let r = Self::r();
        // Use gpfen for basic interrupt enable (falling edge)
        r.gpfen[bank].write(1 << bit);
    }

    pub fn disable_interrupt(p: usize) {
        let bank = p / 32;
        let _bit = p % 32;
        let r = Self::r();
        r.gpfen[bank].write(0);
    }

    pub fn clear_interrupt(p: usize) {
        let bank = p / 32;
        let bit = p % 32;
        Self::r().gpeds[bank].write(1 << bit);
    }

    pub fn has_interrupt(p: usize) -> bool {
        let bank = p / 32;
        let bit = p % 32;
        (Self::r().gpeds[bank].read() & (1 << bit)) != 0
    }
}

// Pack multiple pins into a single u32 for atomic ops
pub struct PinBank {
    pins: u32,
    mask: u32,
}

impl PinBank {
    pub const fn new(pins: &[usize]) -> Self {
        let mut mask = 0;
        // The 'for' loop is not allowed in const functions.
        // Replaced with a 'while' loop using manual indexing.
        let mut i = 0;
        while i < pins.len() {
            // Calculate the bitmask for the current pin.
            mask |= 1 << pins[i];
            i += 1;
        }
        Self { pins: 0, mask }
    }

    // Atomic write for all pins in bank
    pub fn write(&mut self, values: u32) {
        // Get a mutable reference to the GPIO registers.
        let r = GPIO::r();

        // Apply the pin bank's mask to the input values to ensure only
        // relevant bits are considered for setting/clearing.
        let bits = values & self.mask;

        // Write to the GPSET register to set the bits that are high in 'bits'.
        // The GPSET register only sets bits; it doesn't clear others.
        // This is safe for atomic operations as it only affects the desired pins.
        r.gpset[0].write(bits);

        // Write to the GPCLR register to clear the bits that are low in 'bits'
        // but are part of this pin bank's mask.
        // `!bits & self.mask` ensures that:
        // 1. Only bits that should be low (`!bits`) are targeted.
        // 2. Only pins managed by this `PinBank` (`self.mask`) are affected.
        // This ensures that pins that should remain high are not accidentally cleared.
        r.gpclr[0].write(!bits & self.mask);

        // Update the internal state of the PinBank to reflect the new pin values.
        self.pins = bits;
    }
}
