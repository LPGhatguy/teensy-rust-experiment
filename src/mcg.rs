use volatile::Volatile;

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

pub struct Fei {
	mcg: &'static mut Mcg,
}

pub struct Fbe {
	mcg: &'static mut Mcg,
}

pub struct Pbe {
	mcg: &'static mut Mcg,
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