#![feature(stdsimd, asm)]
#![no_std]
#![no_main]

mod mcg;
mod osc;
mod port;
mod sim;
mod watchdog;

use core::{arch::arm::__nop, panic::PanicInfo, ptr};

use cortex_m_rt::entry;

use crate::mcg::Mcg;
use crate::osc::Osc;
use crate::port::{GpioOutputPin, Port, PortName};
use crate::sim::Sim;

#[entry]
fn main() -> ! {
    unsafe {
        watchdog::disable();
    }

    let mut osc = Osc::take().unwrap();
    osc.enable(10);

    let mut sim = Sim::take().unwrap();
    unsafe {
        sim.enable_portc_clock_gate();
        sim.set_clock_dividers(1, 1, 1, 2);
    }

    let mcg = Mcg::take().unwrap();

    // current problem: everything hangs here
    let mcg = mcg.into_fbe();

    loop {}
}

#[inline(never)]
fn delay(cycles: usize) {
    unsafe {
        for _ in 0..cycles {
            __nop();
        }
    }
}

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

    for _ in 0..5 {
        on(5);
        delay(500_000);
        off(5);
        delay(200_000);
    }

    loop {}
}
