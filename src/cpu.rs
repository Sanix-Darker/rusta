use crate::{board::LOCAL_BASE, volatile::Volatile};

#[repr(C)]
struct CoreRegs {
    mailbox0: [Volatile<u32>; 4],
    mailbox_set: Volatile<u32>,
    mailbox_clr: Volatile<u32>,
    _reserved: [Volatile<u32>; 5],
    core_timer: Volatile<u32>,
    local_intr: Volatile<u32>,
}

pub struct CPU;
impl CPU {
    fn regs() -> &'static mut CoreRegs {
        unsafe { &mut *(LOCAL_BASE as *mut CoreRegs) }
    }

    pub fn current_core() -> u32 {
        let regs = Self::regs();
        regs.mailbox0[0].read()
    }

    pub fn start_core(core: u32, entry: extern "C" fn()) {
        let regs = Self::regs();

        // Write entry point to mailbox
        regs.mailbox0[core as usize].write(entry as u32);

        // Send event to wake up core
        regs.mailbox_set.write(1 << core);
    }

    pub fn enable_core_timer(interval_us: u32) {
        let regs = Self::regs();
        let freq = 1_000_000 / interval_us;
        regs.core_timer.write(freq);
    }
}
