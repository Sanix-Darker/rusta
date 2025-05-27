// dynamic memory access
use crate::{board::DMA_BASE, volatile::Volatile};

#[repr(C)]
struct DmaRegs {
    cs: Volatile<u32>,
    conblk_ad: Volatile<u32>,
    ti: Volatile<u32>,
    source_ad: Volatile<u32>,
    dest_ad: Volatile<u32>,
    txfr_len: Volatile<u32>,
    stride: Volatile<u32>,
    nextconbk: Volatile<u32>,
    debug: Volatile<u32>,
}

#[repr(C, align(32))]
struct DmaControlBlock {
    ti: u32,
    source_ad: u32,
    dest_ad: u32,
    txfr_len: u32,
    stride: u32,
    nextconbk: u32,
    _reserved: [u32; 2],
}

pub struct DMA;
impl DMA {
    fn regs() -> &'static mut DmaRegs {
        unsafe { &mut *(DMA_BASE as *mut DmaRegs) }
    }

    pub fn init() {
        // Reset DMA controller
        let regs = Self::regs();
        regs.cs.write(1 << 31);
        while regs.cs.read() & (1 << 31) != 0 {}
    }

    pub unsafe fn start_transfer(src: *const u8, dst: *mut u8, len: usize) {
        let cb = &mut *(0x1000_0000 as *mut DmaControlBlock); // Uncached memory

        cb.ti = (1 << 8) | (1 << 4); // SRC_INC, WAIT_RESP
        cb.source_ad = src as u32;
        cb.dest_ad = dst as u32;
        cb.txfr_len = len as u32;
        cb.nextconbk = 0;

        let regs = Self::regs();
        regs.conblk_ad.write(0x1000_0000);
        regs.cs.write(1 << 0); // Enable DMA
    }

    pub fn is_busy() -> bool {
        Self::regs().cs.read() & (1 << 0) != 0
    }
}
