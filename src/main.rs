#![feature(lang_items, asm)]
#![no_std]
#![no_main]

extern crate volatile;
extern crate bit_field;

mod port;
mod sim;
mod watchdog;

extern {
	fn _stack_top();
}

#[link_section = ".vectors"]
#[no_mangle]
pub static _VECTORS: [unsafe extern fn(); 2] = [
	_stack_top,
	main,
];

#[link_section = ".flashconfig"]
#[no_mangle]
pub static _FLASHCONFIG: [u8; 16] = [
	0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
	0xFF, 0xFF, 0xFF, 0xFF, 0xDE, 0xF9, 0xFF, 0xFF
];

#[lang = "panic_fmt"]
#[no_mangle]
pub extern "C" fn rust_begin_unwind(_msg: core::fmt::Arguments, _file: &'static str, _line: u32) -> ! {
	loop {};
}

#[no_mangle]
pub extern fn main() {
	let (wdog, sim, pin) = unsafe {(
		watchdog::Watchdog::new(),
		sim::Sim::new(),
		port::Port::new(port::PortName::C).pin(5)
	)};

	wdog.disable();
	sim.enable_clock(sim::Clock::PortC);

	let mut gpio = pin.make_gpio();

	gpio.output();
	gpio.high();

	loop {}
}