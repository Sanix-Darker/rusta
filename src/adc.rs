// analog to digital converter
use crate::{board::ADC_BASE, delay, volatile::Volatile};

#[repr(C)]
struct Reg {
    cs: Volatile<u32>,      // Control and status
    result: Volatile<u32>,  // Conversion result
    fcs: Volatile<u32>,     // FIFO control and status
    fcv: Volatile<u32>,     // FIFO count value
    fdf: Volatile<u32>,     // FIFO data (read-only)
    fst: Volatile<u32>,     // FIFO status
    _reserved0: [Volatile<u32>; 2],
    rng: Volatile<u32>,     // Voltage range
    div: Volatile<u32>,     // Clock divider
}

pub struct ADC;
impl ADC {
    fn r() -> &'static mut Reg {
        unsafe { &mut *(ADC_BASE as *mut Reg) }
    }

    pub fn init() {
        let r = Self::r();
        // Reset the ADC
        r.cs.write(1 << 4); // Set reset bit
        delay::cycles(10);
        r.cs.write(0);      // Clear reset bit

        // Configure for standard operation
        r.rng.write(1);     // 0-3.3V range
        r.div.write(2000);  // 500kHz clock (25MHz / 50)
        r.fcs.write(0);     // Disable FIFO
        r.cs.write(1);      // Enable ADC
    }

    pub fn read(channel: u8) -> u32 {
        let r = Self::r();

        // Validate channel (0-7 typically)
        let channel = channel & 0x7;

        // Start conversion
        r.cs.write(1 << 8 | 1 << (channel + 1));

        // Wait for conversion to complete
        while r.cs.read() & (1 << 16) == 0 {}

        // Read result (12-bit value)
        r.result.read() & 0xFFF
    }
}
