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
