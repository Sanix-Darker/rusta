#[cfg(all(not(feature = "pi4"), not(feature = "pi5")))]
const SYS_TIMER_BASE: usize = crate::board::PERIPHERAL_BASE + 0x0000_3000;
#[cfg(feature = "pi4")]
const SYS_TIMER_BASE: usize = crate::board::PERIPHERAL_BASE + 0x0000_3000;
#[cfg(feature = "pi5")]
const SYS_TIMER_BASE: usize = crate::board::PERIPHERAL_BASE + 0x0000_3000;

#[inline(always)]
pub fn micros() -> u64 {
    unsafe { core::ptr::read_volatile((SYS_TIMER_BASE + 0x04) as *const u32) as u64 }
    // lower 32-bit counter
}

/// Delay *n* micro-seconds (max ~71 min before wrap).
pub fn us(n: u32) {
    let start = micros();
    while micros().wrapping_sub(start) < n as u64 {}
}

/// Delay *n* milli-seconds.
pub fn ms(n: u32) {
    for _ in 0..n {
        us(1_000);
    }
}

/// Fallback cycle-burner (kept for ultra-short <1 µs spins)
pub fn cycles(mut n: u32) {
    while n > 0 {
        unsafe { core::arch::asm!("nop") };
        n -= 1;
    }
}
