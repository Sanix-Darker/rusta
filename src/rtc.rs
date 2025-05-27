use crate::{board::RTC_BASE, volatile::Volatile};

#[repr(C)]
struct RtcRegs {
    dr: Volatile<u32>,   // Data register
    mr: Volatile<u32>,   // Match register
    lr: Volatile<u32>,   // Load register
    cr: Volatile<u32>,   // Control register
    imsc: Volatile<u32>, // Interrupt mask
    ris: Volatile<u32>,  // Raw interrupt status
    mis: Volatile<u32>,  // Masked interrupt status
    icr: Volatile<u32>,  // Interrupt clear
}

pub struct RTC;

impl RTC {
    pub fn init() {
        let regs = unsafe { &mut *(RTC_BASE as *mut RtcRegs) };
        regs.cr.write(1 << 0); // Enable RTC
    }

    pub fn set_alarm(seconds: u32) {
        let regs = unsafe { &mut *(RTC_BASE as *mut RtcRegs) };
        regs.mr.write(regs.dr.read() + seconds);
        regs.imsc.write(1 << 0); // Enable alarm interrupt
    }

    pub fn current_time() -> u32 {
        unsafe { (*RTC_BASE.cast::<RtcRegs>()).dr.read() }
    }
}
