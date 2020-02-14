use core::ptr;

const SIM_SCGC5: *mut u32 = 0x4004_8038 as *mut u32;

pub unsafe fn enable_port5_clock_gate() {
    let mut value = ptr::read_volatile(SIM_SCGC5);
    value |= 1 << 11;
    ptr::write_volatile(SIM_SCGC5, value);
}
