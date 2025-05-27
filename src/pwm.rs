use crate::{board::PWM_BASE, volatile::Volatile};

#[repr(C)]
struct Reg {
    ctl: Volatile<u32>,
    sta: Volatile<u32>,
    dmac: Volatile<u32>,
    _0: Volatile<u32>,
    rng1: Volatile<u32>,
    dat1: Volatile<u32>,
}
pub struct PWM;
impl PWM {
    fn r() -> &'static mut Reg {
        unsafe { &mut *(PWM_BASE as *mut Reg) }
    }
    pub fn init(freq: u32, duty: f32) {
        let r = Self::r();
        r.ctl.write(0);
        let rng = 19_200_000 / freq;
        r.rng1.write(rng);
        r.dat1.write((rng as f32 * duty) as u32);
        r.ctl.write(0x81);
    }
    pub fn set_duty(d: f32) {
        let r = Self::r();
        let rng = r.rng1.read();
        r.dat1.write((rng as f32 * d) as u32);
    }
}
