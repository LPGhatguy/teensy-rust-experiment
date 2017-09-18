use core;
use volatile::Volatile;
use bit_field::BitField;

pub enum PortName {
	C
}

#[repr(C, packed)]
pub struct Port {
	pcr: [Volatile<u32>; 32],
	gpclr: Volatile<u32>,
	gpchr: Volatile<u32>,
	reserved_0: [u8; 24],
	isfr: Volatile<u32>,
}

impl Port {
	pub unsafe fn new(name: PortName) -> &'static mut Port {
		&mut * match name {
			PortName::C => 0x4004B000 as *mut Port
		}
	}

	pub unsafe fn set_pin_mode(&mut self, p: usize, mut mode: u32) {
		self.pcr[p].update(|pcr| {
			*pcr &= 0xFFFFF8FF;
			mode &= 0x00000007;
			mode <<= 8;
			*pcr |= mode;
		});
	}
}

pub struct Pin {
	port: *mut Port,
	pin: usize
}

impl Port {
	pub unsafe fn pin(&mut self, p: usize) -> Pin {
		Pin { port: self, pin: p }
	}
}

#[repr(C, packed)]
struct GpioBitband {
	pdor: [Volatile<u32>; 32],
	psor: [Volatile<u32>; 32],
	pcor: [Volatile<u32>; 32],
	ptor: [Volatile<u32>; 32],
	pdir: [Volatile<u32>; 32],
	pddr: [Volatile<u32>; 32]
}

pub struct Gpio {
	gpio: *mut GpioBitband,
	pin: usize
}

impl Port {
	pub fn name(&self) -> PortName {
		let addr = (self as *const Port) as u32;
		match addr {
			0x4004B000 => PortName::C,
			_ => unreachable!(),
		}
	}
}

impl Pin {
	pub fn make_gpio(self) -> Gpio {
		unsafe {
			let port = &mut *self.port;
			port.set_pin_mode(self.pin, 1);
			Gpio::new(port.name(), self.pin)
		}
	}
}

impl Gpio {
	pub unsafe fn new(port: PortName, pin: usize) -> Gpio {
		let gpio = match port {
			PortName::C => 0x43FE1000 as *mut GpioBitband,
		};

		Gpio {
			gpio,
			pin,
		}
	}

	pub fn output(&mut self) {
		unsafe {
			(*self.gpio).pddr[self.pin].update(|pddr| {
				*pddr = 1;
			});
		}
	}

	pub fn high(&mut self) {
		unsafe {
			(*self.gpio).psor[self.pin].update(|psor| {
				*psor = 1;
			});
		}
	}

	pub fn low(&mut self) {
		unsafe {
			(*self.gpio).ptor[self.pin].update(|ptor| {
				*ptor = 1;
			});
		}
	}
}