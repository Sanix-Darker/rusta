#[inline(always)]
pub unsafe fn write(addr: usize, val: u32) {
    core::ptr::write_volatile(addr as *mut u32, val);
}
#[inline(always)]
pub unsafe fn read(addr: usize) -> u32 {
    core::ptr::read_volatile(addr as *const u32)
}
