use core;
use volatile::Volatile;
use bit_field::BitField;

pub enum Clock {
	PortC,
}

// TODO: convert all these fields to Volatile
#[repr(C,packed)]
pub struct Sim {
	sopt1: u32,
	sopt1_cfg: u32,
	_pad0: [u32; 1023],
	sopt2: u32,
	_pad1: u32,
	sopt4: u32,
	sopt5: u32,
	_pad2: u32,
	sopt7: u32,
	_pad3: [u32; 2],
	sdid: u32,
	_pad4: [u32; 3],
	scgc4: u32,
	scgc5: Volatile<u32>,
	scgc6: u32,
	scgc7: u32,
	clkdiv1: Volatile<u32>,
	clkviv2: u32,
	fcfg1: u32,
	fcfg2: u32,
	uidh: u32,
	uidmh: u32,
	uidml: u32,
	uidl: u32
}

impl Sim {
	pub unsafe fn new() -> &'static mut Sim {
		&mut *(0x40047000 as *mut Sim)
	}

	pub fn enable_clock(&mut self, clock: Clock) {
		match clock {
			Clock::PortC => {
				self.scgc5.update(|scgc5| {
					*scgc5 |= 0x00000800;
				})
			}
		}
	}

	pub fn set_dividers(&mut self, core: u32, bus: u32, flash: u32) {
		let mut clkdiv: u32 = 0;
		clkdiv.set_bits(28..32, core-1);
		clkdiv.set_bits(24..28, bus-1);
		clkdiv.set_bits(16..20, flash-1);
		self.clkdiv1.write(clkdiv);
	}
}