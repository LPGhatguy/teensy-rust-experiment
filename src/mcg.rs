use volatile::Volatile;
use bit_field::BitField;

// see section 27.4
#[repr(C, packed)]
pub struct Mcg {
	c1: Volatile<u8>,
	c2: Volatile<u8>,
	c3: Volatile<u8>,
	c4: Volatile<u8>,
	c5: Volatile<u8>,
	c6: Volatile<u8>,
	s: Volatile<u8>, // status
	_pad0: u8,
	sc: Volatile<u8>, // status and control
	_pad1: u8,
	atcvh: Volatile<u8>, // auto trim compare value high
	atcvl: Volatile<u8>, //  auto trim compare value low
	c7: Volatile<u8>,
	c8: Volatile<u8>,
	c9: Volatile<u8>,
	_pad2: u8,
	c11: Volatile<u8>,
	c12: Volatile<u8>,
	s2: Volatile<u8>, // status 2
	t3: Volatile<u8>, // test 3
}

impl Mcg {
	pub unsafe fn new() -> &'static mut Mcg {
		&mut *(0x40064000 as *mut Mcg)
	}
}

pub enum OscRange {
	Low = 0,
	High = 1,
	VeryHigh = 2,
}

enum OscSource {
	LockedLoop = 0,
	Internal = 1,
	External = 2,
}

pub struct Fei {
	pub mcg: &'static mut Mcg,
}

impl Fei {
	pub fn enable_external(&mut self, range: OscRange) {
		self.mcg.c2.update(|c2| {
			c2.set_bits(4..6, range as u8);
			c2.set_bit(2, true);
		});

		// Wait for the crystal oscillator to become enabled.
		while !self.mcg.s.read().get_bit(1) {}
	}

	pub fn to_fbe(self, divide: u32) -> Fbe {
		let osc = self.mcg.c2.read().get_bits(4..6);
		let frdiv = if osc == OscRange::Low as u8 {
			match divide {
				1 => 0,
				2 => 1,
				4 => 2,
				8 => 3,
				16 => 4,
				32 => 5,
				64 => 6,
				128 => 7,
				_ => panic!("Invalid external clock divider: {}", divide),
			}
		} else {
			match divide {
				32 => 0,
				64 => 1,
				128 => 2,
				256 => 3,
				512 => 4,
				1024 => 5,
				1280 => 6,
				1536 => 7,
				_ => panic!("Invalid external clock divider: {}", divide),
			}
		};

		self.mcg.c1.update(|c1| {
			// use external reference as system clock source
			c1.set_bits(6..8, OscSource::External as u8);

			// set clock divider
			c1.set_bits(3..6, frdiv);

			// switch to external reference
			c1.set_bit(2, false);
		});

		// wait for FLL to point to crystal
		while self.mcg.s.read().get_bit(4) {}

		// wait for clock source to be crystal
		while self.mcg.s.read().get_bits(2..4) != OscSource::External as u8 {}

		Fbe {
			mcg: self.mcg,
		}
	}
}

pub struct Fbe {
	mcg: &'static mut Mcg,
}

pub struct Pbe {
	mcg: &'static mut Mcg,
}