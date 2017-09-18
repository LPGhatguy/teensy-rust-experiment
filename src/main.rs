#![feature(lang_items, asm)]
#![no_std]
#![no_main]

extern crate volatile;
extern crate bit_field;

mod port;
mod sim;
mod watchdog;
mod mcg;
mod osc;

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

fn delay(cycles: usize) {
	unsafe {
		for i in 0..cycles {
			asm!("nop" : : : "memory");
		}
	}
}

#[no_mangle]
pub extern fn main() {
	let (wdog, sim, pin, osc, mcg) = unsafe {(
		watchdog::Watchdog::new(),
		sim::Sim::new(),
		port::Port::new(port::PortName::C).pin(5),
		osc::Osc::new(),
		mcg::Mcg::new(),
	)};

	wdog.disable();

	// enable crsytal oscillator with 10pf of capacitance
	osc.enable(10);

	sim.enable_clock(sim::Clock::PortC);

	let mut fei = mcg::Fei {
		mcg
	};

	fei.enable_external(mcg::OscRange::VeryHigh);

	let fbe = fei.to_fbe(512);

	let mut gpio = pin.make_gpio();

	gpio.output();

	loop {
		gpio.toggle();
		delay(1720000);
	}
}