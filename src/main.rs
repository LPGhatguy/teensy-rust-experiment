#![feature(lang_items, stdsimd)]
#![no_std]
#![no_main]

mod sim;
mod watchdog;

use core::{arch::arm::__nop, panic::PanicInfo, ptr};

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
            __nop();
        }
    }
}

#[no_mangle]
pub extern "C" fn main() {
    let portc_pcr4 = 0x4004_B010 as *mut u32;
    let portc_pcr5 = 0x4004_B014 as *mut u32;
    let portc_pcr6 = 0x4004_B018 as *mut u32;

    let gpioc_pdor = 0x400F_F080 as *mut u32;
    let gpioc_psor = 0x400F_F084 as *mut u32;
    let gpioc_pddr = 0x400F_F094 as *mut u32;
    let gpioc_pdir = 0x400F_F090 as *mut u32;

    unsafe {
        watchdog::disable();
        sim::enable_port5_clock_gate();
    }

    // mark port c, pin 4 as GPIO
    // labeled as pin 10 on my board diagram
    unsafe {
        let mut value = ptr::read_volatile(portc_pcr4);
        value |= 1 << 8;
        ptr::write_volatile(portc_pcr4, value);
    }

    // mark port c, pin 5 as GPIO
    // labeled as pin 13 on my board diagram
    unsafe {
        let mut value = ptr::read_volatile(portc_pcr5);
        value |= 1 << 8;
        ptr::write_volatile(portc_pcr5, value);
    }

    // mark port c, pin 6 as GPIO
    // labeled as pin 11 on my board diagram
    unsafe {
        let mut value = ptr::read_volatile(portc_pcr6);
        value |= 1 << 8;
        ptr::write_volatile(portc_pcr6, value);
    }

    let output = |pin: u8| unsafe {
        let value = ptr::read_volatile(gpioc_pddr);
        let mask = 1 << pin;
        ptr::write_volatile(gpioc_pddr, value | mask);
    };

    let get = |pin: u8| -> bool {
        unsafe {
            let value = ptr::read_volatile(gpioc_pdir);
            (value & (1 << pin)) != 0
        }
    };

    let on = |pin: u8| unsafe {
        let value = ptr::read_volatile(gpioc_pdor);
        let mask = 1 << pin;
        ptr::write_volatile(gpioc_pdor, value | mask);
    };

    let off = |pin: u8| unsafe {
        let value = ptr::read_volatile(gpioc_pdor);
        let mask = 1 << pin;
        ptr::write_volatile(gpioc_pdor, value & !mask);
    };

    output(5);

    on(5);
    delay(500_000);
    off(5);
    delay(500_000);

    let mut last_value = false;

    loop {
        let next_value = get(4);

        if next_value != last_value {
            if next_value {
                on(5);
                delay(500_000);
                off(5);
            } else {
                on(5);
                delay(300_000);
                off(5);
                delay(300_000);
                on(5);
                delay(300_000);
                off(5);
            }
        }

        last_value = next_value;
        delay(50_000);
    }
}
