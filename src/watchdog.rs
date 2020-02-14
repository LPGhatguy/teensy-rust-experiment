//! The watchdog is in charge of making sure the running program doesn't get
//! itself into trouble. The intent is that every once in awhile, you ping the
//! watchdog to let it know you're still okay.
//!
//! No one should be using this code in a safety-critical system, so the only
//! valid operation is to turn off the watchdog because it's annoying.
//!
//! See the K66 Reference, Chapter 26 for actually sound advice.

use core::{arch::arm::__nop, ptr};

const WDOG_STCTRLH: *mut u16 = 0x4005_2000 as *mut u16;
const WDOG_UNLOCK: *mut u16 = 0x4005_200E as *mut u16;

pub unsafe fn disable() {
    ptr::write_volatile(WDOG_UNLOCK, 0xC520);
    ptr::write_volatile(WDOG_UNLOCK, 0xD928);

    __nop();
    __nop();

    let value = ptr::read_volatile(WDOG_STCTRLH);
    let mask = 1;
    ptr::write_volatile(WDOG_STCTRLH, value & !mask);
}
