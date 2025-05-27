use crate::{board::GPIO_BASE, volatile::Volatile};

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

#[repr(C)]
struct Regs {
    gpfsel: [Volatile<u32>; 6],
    _0: Volatile<u32>,
    gpset: [Volatile<u32>; 2],
    _1: Volatile<u32>,
    gpclr: [Volatile<u32>; 2],
    _2: Volatile<u32>,
    gplev: [Volatile<u32>; 2],
    _3: Volatile<u32>,
    gpeds: [Volatile<u32>; 2],
    _4: Volatile<u32>,
    gpren: [Volatile<u32>; 2],
    _5: Volatile<u32>,
    gpfen: [Volatile<u32>; 2],
    _6: Volatile<u32>,
    gphen: [Volatile<u32>; 2],
    _7: Volatile<u32>,
    gplen: [Volatile<u32>; 2],
    _8: Volatile<u32>,
    gparen: [Volatile<u32>; 2],
    _9: Volatile<u32>,
    gpafen: [Volatile<u32>; 2],
    _10: Volatile<u32>,
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
        v &= !7 << s;
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
}
