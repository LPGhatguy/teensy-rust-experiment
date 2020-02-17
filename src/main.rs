#![feature(stdsimd, asm)]
#![no_std]
#![no_main]

mod port;
mod sim;
mod watchdog;

use core::{
    arch::arm::__nop,
    panic::PanicInfo,
    ptr,
    sync::atomic::{self, Ordering},
};

use cortex_m_rt::entry;

use crate::port::{GpioOutputPin, Port, PortName};

const C130_KINDA: usize = 10_000;

/// Junky tone generator function
fn tone_step(pin: &mut GpioOutputPin<'_>, tone: i32) {
    let exp = 1.059463f32;

    let mut multiplier = exp;
    for _ in 0..(tone as u32) {
        multiplier = multiplier * exp;
    }
    let duration = (C130_KINDA as f32 * multiplier) as usize;

    pin.high();
    delay(duration);
    pin.low();
    delay(duration);
}

#[entry]
fn main() -> ! {
    unsafe {
        watchdog::disable();
        sim::enable_portc_clock_gate();
    }

    // Assumes buttons are attached to C3 and C4, and a passive buzzer is
    // attached to C6.

    let port_c = Port::take(PortName::C).unwrap();
    let mut pin_c3 = port_c.take_pin(3).unwrap().into_gpio();
    let mut pin_c4 = port_c.take_pin(4).unwrap().into_gpio();
    let mut pin_c6 = port_c.take_pin(6).unwrap().into_gpio().into_output();

    pin_c6.low();

    loop {
        let tone = if pin_c3.read() {
            Some(0)
        } else if pin_c4.read() {
            Some(1)
        } else {
            None
        };

        if let Some(tone) = tone {
            tone_step(&mut pin_c6, tone);
        } else {
            delay(100_000);
        }
    }
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
