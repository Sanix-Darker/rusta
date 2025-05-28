/*======================================================================
  Enhanced SMP / per-core helpers for Raspberry-Pi (A53/A72)
  ======================================================================*/

use crate::{board::LOCAL_BASE, volatile::Volatile};
use core::arch::asm;

#[repr(C)]
#[allow(non_snake_case)]
struct LocalRegs {
    /* 0x000 */ MAILBOX0:    [Volatile<u32>; 4], // per-core mailboxes
    /* 0x010 */ MAILBOX_SET: Volatile<u32>,
    /* 0x014 */ MAILBOX_CLR: Volatile<u32>,
    /* 0x018 */ _r0:         [Volatile<u32>; 5],
    /* 0x02C */ CORE_TIMER:  Volatile<u32>,      // local timer reload reg
    /* 0x030 */ LOCAL_IRQ:   Volatile<u32>,      // pending local IRQ flags
    /* 0x034 */ _r1:         [Volatile<u32>; 3],
    /* 0x040 */ TIMER_CFG:   Volatile<u32>,      // timer enable & div
    /* 0x044 */ TIMER_CNT:   Volatile<u32>,      // 1 MHz free-run counter
    /* 0x048 */ TIMER_CMP:   [Volatile<u32>; 4], // compare registers
}

pub struct Cpu;

impl Cpu {
    #[inline(always)]
    fn regs() -> &'static mut LocalRegs {
        unsafe { &mut *(LOCAL_BASE as *mut LocalRegs) }
    }

    // Core identity helpers
    /// Return 0-3 for running core (reads MPIDR_EL1).
    #[inline(always)]
    pub fn core_id() -> usize {
        let mpidr: usize;
        unsafe { asm!("mrs {0}, mpidr_el1", out(reg) mpidr) };
        mpidr & 0b11
    }

    #[inline(always)]
    pub fn current_core() -> u32 {
        Self::core_id() as u32        // legacy alias
    }

    // Core bring-up / inter-core messaging
    /// Launch secondary `core` (1-3) at `entry`.
    ///
    /// # Safety
    /// *The entry function must set up its own stack
    /// and must never return.*
    pub fn launch_core(core: usize, entry: extern "C" fn()) {
        let r = Self::regs();
        r.MAILBOX0[core].write(entry as u32); // pass PC
        r.MAILBOX_SET.write(1 << core);       // set flag
        Self::sev();                          // wake core
    }

    /// Send software interrupt (IPI) to `core`.
    pub fn send_ipi(core: usize) {
        Self::regs().MAILBOX_SET.write(1 << core);
    }

    /// Clear pending IPI on *this* core.
    pub fn clear_ipi() {
        let id = Self::core_id();
        Self::regs().MAILBOX_CLR.write(1 << id);
    }

    // Local timer (1 MHz) – simple compare/IRQ source
    /// Arm per-core timer interrupt `interval_us` µs from now.
    pub fn timer_enable(interval_us: u32) {
        let r = Self::regs();
        r.CORE_TIMER.write(interval_us);          // reload value
    }

    /// Read free-running 1 MHz counter (wraps at 2³²-1 µs ≈ 71 min).
    #[inline(always)]
    pub fn timer_now() -> u32 {
        Self::regs().TIMER_CNT.read()
    }

    // Barrier & power helpers
    #[inline(always)] pub fn dsb() { unsafe { asm!("dsb sy") } }
    #[inline(always)] pub fn isb() { unsafe { asm!("isb sy") } }
    #[inline(always)] pub fn sev() { unsafe { asm!("sev") } }
    #[inline(always)] pub fn wfe() { unsafe { asm!("wfe") } }
    #[inline(always)] pub fn wfi() { unsafe { asm!("wfi") } }

    /// Park the CPU in low-power wait-for-interrupt.
    #[inline(never)]
    pub fn park() -> ! { loop { Self::wfi(); } }
}
