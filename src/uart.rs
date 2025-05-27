use crate::{board::UART0_BASE, volatile::Volatile};
use core::fmt::{self, Write};
#[repr(C)]
struct Reg {
    dr: Volatile<u32>,
    rsrecr: Volatile<u32>,
    _0: [Volatile<u32>; 4],
    fr: Volatile<u32>,
    _1: [Volatile<u32>; 2],
    ibrd: Volatile<u32>,
    fbrd: Volatile<u32>,
    lcrh: Volatile<u32>,
    cr: Volatile<u32>,
}
pub struct UART;
impl UART {
    fn r() -> &'static mut Reg {
        unsafe { &mut *(UART0_BASE as *mut Reg) }
    }
    pub fn init(baud: u32) {
        let r = Self::r();
        r.cr.write(0);
        let div = 48_000_000 / (16 * baud);
        r.ibrd.write(div);
        r.fbrd.write(0);
        r.lcrh.write(3 << 5);
        r.cr.write(0x301);
    }
    pub fn putc(c: u8) {
        while Self::r().fr.read() & 0x20 != 0 {}
        Self::r().dr.write(c as u32);
    }
    pub fn getc() -> u8 {
        while Self::r().fr.read() & 0x10 != 0 {}
        Self::r().dr.read() as u8
    }
    pub fn writer() -> UartWriter {
        UartWriter
    }
}
pub struct UartWriter;
impl Write for UartWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for b in s.bytes() {
            UART::putc(b);
        }
        Ok(())
    }
}
