use core;
use volatile::Volatile;
use bit_field::BitField;

pub const PORT_B: usize = 0x4004A000;
pub const PORT_B_BITBAND: usize = 0x43FE0800;

pub const PORT_C: usize = 0x4004B000;
pub const PORT_C_BITBAND: usize = 0x43FE1000;

pub struct Tx(u8);
pub struct Rx(u8);

pub enum PortName {
	B,
	C,
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
			PortName::B => PORT_B as *mut Port,
			PortName::C => PORT_C as *mut Port,
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
	pdor: [Volatile<u32>; 32], // data output
	psor: [Volatile<u32>; 32], // set output
	pcor: [Volatile<u32>; 32], // clear output
	ptor: [Volatile<u32>; 32], // toggle output
	pdir: [Volatile<u32>; 32], // data input
	pddr: [Volatile<u32>; 32], // data direction
}

pub struct Gpio {
	gpio: *mut GpioBitband,
	pin: usize
}

impl Port {
	pub fn name(&self) -> PortName {
		let addr = (self as *const Port) as usize;
		match addr {
			PORT_B => PortName::B,
			PORT_C => PortName::C,
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

	pub fn make_rx(self) -> Rx {
		unsafe {
			let port = &mut *self.port;

			match (port.name(), self.pin) {
				(PortName::B, 16) => {
					port.set_pin_mode(self.pin, 3);
					Rx(0)
				},
				_ => panic!("Invalid serial RX pin!"),
			}
		}
	}

	pub fn make_tx(self) -> Tx {
		unsafe {
			let port = &mut *self.port;

			match (port.name(), self.pin) {
				(PortName::B, 17) => {
					port.set_pin_mode(self.pin, 3);
					Tx(0)
				},
				_ => panic!("Invalid serial RX pin!"),
			}
		}
	}
}

impl Gpio {
	pub unsafe fn new(port: PortName, pin: usize) -> Gpio {
		let gpio = match port {
			PortName::C => PORT_C_BITBAND as *mut GpioBitband,
			PortName::B => PORT_B_BITBAND as *mut GpioBitband,
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
			(*self.gpio).pdor[self.pin].update(|pdor| {
				*pdor = 1;
			});
		}
	}

	pub fn low(&mut self) {
		unsafe {
			(*self.gpio).pdor[self.pin].update(|pdor| {
				*pdor = 0;
			});
		}
	}

	pub fn toggle(&mut self) {
		unsafe {
			(*self.gpio).ptor[self.pin].update(|ptor| {
				*ptor = 1;
			});
		}
	}
}