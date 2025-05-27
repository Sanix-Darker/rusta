#[inline(always)]
pub fn cycles(n: u32) {
    unsafe {
        core::arch::asm!(
            "1:",
            "subs {cnt:w}, {cnt:w}, #1", // decrement and set flags
            "bne 1b",                    // loop while cnt != 0
            cnt = inout(reg) n => _,     // read-only for Rust, read-write for asm
            options(nostack)
        );
    }
}
