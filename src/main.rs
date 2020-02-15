#![feature(lang_items, stdsimd)]
#![no_std]
#![no_main]
#![no_builtins]

mod port;
mod sim;
mod watchdog;

use core::{arch::arm::__nop, panic::PanicInfo, ptr, slice};

use crate::port::{Port, PortName};

#[no_mangle]
pub extern "C" fn main() {
    unsafe {
        watchdog::disable();
        setup_sections();
        sim::enable_portc_clock_gate();
    }

    let port_c = Port::take(PortName::C).unwrap();

    let mut c5 = port_c.take_pin(5).unwrap().into_gpio().into_output();

    loop {
        c5.high();
        delay(100_000);
        c5.low();
        delay(500_000);
    }

    loop {}
}

fn delay(cycles: usize) {
    unsafe {
        for _ in 0..cycles {
            __nop();
        }
    }
}

unsafe fn setup_sections() {
    extern "C" {
        static mut _bss_start: u32;
        static mut _bss_end: u32;
        static mut _rodata_start: u32;
        static mut _rodata_end: u32;
        static mut _data_start: u32;
    }

    let mut bss_start = &mut _bss_start as *mut u32;
    let bss_end = &mut _bss_end as *mut u32;
    while bss_start < bss_end {
        ptr::write_volatile(bss_start, 0);
        bss_start = bss_start.offset(1);
    }

    let mut data_start = &mut _data_start as *mut u32;
    let mut rodata_start = &mut _rodata_start as *mut u32;
    let rodata_end = &mut _rodata_end as *mut u32;
    while rodata_start < rodata_end {
        ptr::write_volatile(data_start, ptr::read_volatile(rodata_start));
        rodata_start = rodata_start.offset(1);
        data_start = data_start.offset(1);
    }
}

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
    let portc_pcr5 = 0x4004_B014 as *mut u32;

    let gpioc_pdor = 0x400F_F080 as *mut u32;
    let gpioc_pddr = 0x400F_F094 as *mut u32;

    // mark port c, pin 5 as GPIO
    // labeled as pin 13 on my board diagram
    unsafe {
        let mut value = ptr::read_volatile(portc_pcr5);
        value |= 1 << 8;
        ptr::write_volatile(portc_pcr5, value);
    }

    let output = |pin: u8| unsafe {
        let value = ptr::read_volatile(gpioc_pddr);
        let mask = 1 << pin;
        ptr::write_volatile(gpioc_pddr, value | mask);
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

    for i in 0..4 {
        on(5);
        delay(500_000);
        off(5);
        delay(1_000_000);
    }

    loop {}
}
