#![feature(lang_items, asm)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;

extern crate bit_field;
extern crate volatile;

mod mcg;
mod osc;
mod port;
mod sim;
mod watchdog;

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
        for i in 0..cycles {
            asm!("nop" : : : "memory");
        }
    }
}

fn yep(gpio: &mut port::Gpio) {
    gpio.high();
    delay(1_200_000);
    gpio.low();
}

fn nope(gpio: &mut port::Gpio) {
    gpio.high();
    delay(172_000);
    gpio.low();

    delay(640_000);

    gpio.high();
    delay(172_000);
    gpio.low();
}

#[no_mangle]
pub extern "C" fn main() {
    let (wdog, sim, port, osc, mcg) = unsafe {
        (
            watchdog::Watchdog::new(),
            sim::Sim::new(),
            port::Port::new(port::PortName::C),
            osc::Osc::new(),
            mcg::Mcg::new(),
        )
    };

    wdog.disable();

    // osc.enable(10);

    sim.enable_clock(sim::Clock::PortC);

    // sim.set_dividers(1, 2, 3);

    // mcg.fei_to_pee_120mhz();

    let pin = unsafe { port.pin(5) };
    let mut gpio = pin.make_gpio();
    gpio.output();
    gpio.high();

    unsafe {
        let pin2 = port.pin(4);
        let mut gpio2 = pin2.make_gpio();
        gpio2.output();

        loop {
            gpio.high();
            gpio2.low();
            delay(500_000);
            gpio.low();
            gpio2.high();
            delay(500_000);
        }
    }

    // {
    //     use bit_field::BitField;

    //     if mcg.c1.read().get_bits(3..6) == 0b100 {
    //         yep(&mut gpio);
    //     } else {
    //         nope(&mut gpio);
    //     }
    // }

    loop {}
}
