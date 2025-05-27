#[inline(never)]
pub fn cycles(mut n: u32) {
    while n > 0 {
        unsafe { core::arch::asm!("nop") };
        n -= 1;
    }
}
