use crate::{board::SPI0_BASE, volatile::Volatile};
#[repr(C)]
struct Reg {
    cs: Volatile<u32>,
    fifo: Volatile<u32>,
    clk: Volatile<u32>,
    dlen: Volatile<u32>,
    ltoh: Volatile<u32>,
    dc: Volatile<u32>,
}
pub struct SPI;
impl SPI {
    fn r() -> &'static mut Reg {
        unsafe { &mut *(SPI0_BASE as *mut Reg) }
    }
    pub fn init(div: u32) {
        let r = Self::r();
        r.cs.write(0);
        r.clk.write(div);
        r.cs.write(0x30);
        r.cs.write(0x80);
    }
    pub fn xfer(b: u8) -> u8 {
        let r = Self::r();
        while r.cs.read() & 0x400 == 0 {}
        r.fifo.write(b as u32);
        while r.cs.read() & 0x200 == 0 {}
        r.fifo.read() as u8
    }
}
