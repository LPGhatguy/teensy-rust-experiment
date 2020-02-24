//! Oscillator (OSC)
//!
//! See chapter 28 for details.

#![allow(unused)]

use core::sync::atomic::{AtomicBool, Ordering};

use bitfield::{Bit, BitRange};
use volatile::ReadWrite;

static OSC_TAKEN: AtomicBool = AtomicBool::new(false);

const OSC_BASE_ADDRESS: usize = 0x4006_5000;

pub struct Osc {
    data: *mut OscData,
}

impl Osc {
    pub fn take() -> Option<Osc> {
        let taken = OSC_TAKEN.swap(true, Ordering::Relaxed);
        if taken {
            return None;
        }

        let data = OSC_BASE_ADDRESS as *mut OscData;
        Some(Osc { data })
    }

    pub fn enable(&mut self, capacitance: u8) {
        // The chip can only handle even capacitance values under 30.
        if capacitance % 2 == 1 || capacitance > 30 {
            panic!("Invalid crystal capacitance value: {}", capacitance)
        }

        let mut cr: u8 = 0;

        // The capacitance control bits are big-endian, and start at 2pf.
        cr.set_bit(3, capacitance.bit(1));
        cr.set_bit(2, capacitance.bit(2));
        cr.set_bit(1, capacitance.bit(3));
        cr.set_bit(0, capacitance.bit(4));

        // Enable crystal oscillator
        cr.set_bit(7, true);

        unsafe {
            (*self.data).cr.write(cr);
        }
    }
}

#[repr(packed)]
struct OscData {
    /// Control register
    cr: ReadWrite<u8>,

    _pad0: u8,

    /// Clock divider register
    div: ReadWrite<u8>,
}
