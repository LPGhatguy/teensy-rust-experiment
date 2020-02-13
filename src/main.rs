#![feature(lang_items, asm)]
#![no_std]
#![no_main]

use core::{panic::PanicInfo, ptr};

extern "C" {
    fn _stack_top();
}

#[link_section = ".vectors"]
#[no_mangle]
pub static _VECTORS: [unsafe extern "C" fn(); 2] = [_stack_top, main];

#[link_section = ".flashconfig"]
#[no_mangle]
pub static _FLASHCONFIG: [u8; 16] = [
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xDE, 0xF9, 0xFF, 0xFF,
];

#[panic_handler]
fn teensy_panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn delay(cycles: usize) {
    unsafe {
        for _ in 0..cycles {
            asm!("nop" : : : "memory");
        }
    }
}

#[no_mangle]
pub extern "C" fn main() {
    let sim_scgc5 = 0x4004_8038 as *mut u32;

    let portc_pcr5 = 0x4004_B014 as *mut u32;

    let gpioc_psor = 0x400F_F084 as *mut u32;
    let gpioc_pddr = 0x400F_F094 as *mut u32;

    unsafe {
        let mut value = ptr::read_volatile(sim_scgc5);
        value |= 1 << 11;
        ptr::write_volatile(sim_scgc5, value);
    }

    unsafe {
        let mut value = ptr::read_volatile(portc_pcr5);
        value |= 1 << 8;
        ptr::write_volatile(portc_pcr5, value);
    }

    unsafe {
        ptr::write_volatile(gpioc_pddr, 1 << 5);
        ptr::write_volatile(gpioc_psor, 1 << 5);
    }

    loop {}
}
