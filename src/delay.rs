#[inline(never)]
pub fn cycles(mut n: u32) {
    while n > 0 {
        unsafe { core::arch::asm!("nop") };
        n -= 1;
    }
}

// TODO: fix out by using the asm
//#[inline(always)]
//pub fn cycles(n: u32) {
//    // move the immutable parameter into a mutable local
//    let mut cnt = n;
//
//    unsafe {
//        core::arch::asm!(
//            "1:",
//            "subs {cnt}, {cnt}, #1",   // cnt--, set flags
//            "bne 1b",                 // branch back while cnt != 0
//            cnt = inout(reg) cnt,     // read-write GPR (Xn); no width modifier needed
//            options(nostack)
//        );
//    }
//}
