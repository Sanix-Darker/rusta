//! Simple power-management watchdog – reboot on software lock-up.
//! Works on BCM2835/6/7/2711 PM block.

#[cfg(all(not(feature = "pi4"), not(feature = "pi5")))]
const PM_BASE: usize = crate::board::PERIPHERAL_BASE + 0x0010_0000; // Pi-3
#[cfg(feature = "pi4")]
const PM_BASE: usize = crate::board::PERIPHERAL_BASE + 0x0010_0000; // Pi-4
#[cfg(feature = "pi5")]
const PM_BASE: usize = crate::board::PERIPHERAL_BASE + 0x0010_0000; // Pi-5

const PM_RSTC: usize = PM_BASE + 0x1C;
const PM_WDOG: usize = PM_BASE + 0x24;
const PM_PASSWD: u32 = 0x5A00_0000;

#[repr(u32)]
pub enum ResetCause {
    Full = 0x00000000,
    Halt = 0x00000020,
}

pub struct Watchdog;

impl Watchdog {
    /// Enable WDT with *timeout* in milliseconds (max ≈16 s).
    pub fn enable(timeout_ms: u32) {
        unsafe {
            core::ptr::write_volatile(PM_WDOG as *mut u32, PM_PASSWD | (timeout_ms & 0x0FFF));
            core::ptr::write_volatile(
                PM_RSTC as *mut u32,
                PM_PASSWD | 0x0000_0020, // enable + full-reset
            );
        }
    }

    /// Reload (“kick”) the watchdog to prevent reset.
    #[inline(always)]
    pub fn pet(timeout_ms: u32) {
        Self::enable(timeout_ms)
    }

    /// Disable watchdog completely.
    pub fn disable() {
        unsafe {
            core::ptr::write_volatile(PM_RSTC as *mut u32, PM_PASSWD);
            core::ptr::write_volatile(PM_WDOG as *mut u32, PM_PASSWD);
        }
    }
}
