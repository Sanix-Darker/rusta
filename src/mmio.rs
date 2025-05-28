#![allow(clippy::missing_safety_doc)]

//! Minimal *volatile* MMIO helpers.
//!
//! All routines are `unsafe` because **you** must guarantee that the
//! supplied address is a valid memory-mapped register on the running
//! SoC, naturally aligned, and that concurrent access rules are
//! respected.  Breaking any of those rules is immediate *undefined
//! behaviour* on ARM.

/// Write a 32-bit value to the given MMIO address.
///
/// # Safety
/// * `addr` **must** be a valid, correctly aligned peripheral register
///   mapped with Device/Strong-Order attributes (i.e. no caching).
/// * The caller must ensure that writes are allowed and cannot corrupt
///   critical hardware state.
///
/// Violating any of the above **will** lead to unpredictable behaviour.
#[inline(always)]
pub unsafe fn write(addr: usize, val: u32) {
    core::ptr::write_volatile(addr as *mut u32, val);
}

/// Read a 32-bit value from the given MMIO address.
///
/// # Safety
/// * `addr` **must** be a valid, correctly aligned peripheral register
///   mapped with Device/Strong-Order attributes.
/// * Concurrent access rules (e.g. RMW sequences) are the caller’s
///   responsibility.
///
/// Using an invalid address or aliasing with non-volatile access is UB.
#[inline(always)]
pub unsafe fn read(addr: usize) -> u32 {
    core::ptr::read_volatile(addr as *const u32)
}
