use crate::{board::I2C1_BASE, volatile::Volatile};
#[repr(C)]
struct Reg {
    c: Volatile<u32>,
    s: Volatile<u32>,
    dlen: Volatile<u32>,
    a: Volatile<u32>,
    fifo: Volatile<u32>,
    div: Volatile<u32>,
}
pub struct I2C;
impl I2C {
    fn r() -> &'static mut Reg {
        unsafe { &mut *(I2C1_BASE as *mut Reg) }
    }
    pub fn init(div: u32) {
        Self::r().div.write(div);
    }
    pub fn write(addr: u8, data: &[u8]) {
        let r = Self::r();
        r.a.write(addr as u32);
        r.dlen.write(data.len() as u32);
        r.s.write(0);
        r.c.write(0x80);
        r.c.write(0x8080);
        for &b in data {
            while r.s.read() & 0x10 == 0 {}
            r.fifo.write(b as u32);
        }
        while r.s.read() & 0x02 == 0 {}
    }
}
