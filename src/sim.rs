//! The System Integration Module (SIM) does a bunch of stuff like manage system
//! clocks, memory, and serial communication.
//!
//! See the K66 Reference, Chapter 13 for details.

#![allow(unused)]

use core::ptr;
use core::sync::atomic::{AtomicBool, Ordering};

use bitfield::{Bit, BitRange};
use volatile::{ReadOnly, ReadWrite};

static SIM_TAKEN: AtomicBool = AtomicBool::new(false);

const SIM_BASE_ADDRESS_0: usize = 0x4004_7000;
const SIM_BASE_ADDRESS_1: usize = 0x4004_8004;

pub struct Sim {
    data0: *mut SimData0,
    data1: *mut SimData1,
}

impl Sim {
    pub fn take() -> Option<Self> {
        let taken = SIM_TAKEN.swap(true, Ordering::Relaxed);
        if taken {
            return None;
        }

        let data0 = SIM_BASE_ADDRESS_0 as *mut SimData0;
        let data1 = SIM_BASE_ADDRESS_1 as *mut SimData1;

        Some(Sim { data0, data1 })
    }

    pub unsafe fn set_clock_dividers(&mut self, system: u8, bus: u8, flash: u8) {
        (*self.data1).clkdiv1.update(|value| {
            value.set_bit_range(28, 31, system - 1);
            value.set_bit_range(24, 27, bus - 1);
            value.set_bit_range(16, 19, flash - 1);
        });
    }

    pub unsafe fn enable_portc_clock_gate(&mut self) {
        (*self.data1).scgc5.update(|value| {
            value.set_bit(11, true);
        });
    }
}

#[repr(packed)]
struct SimData0 {
    /// System Options Register 1
    sopt1: u32,

    /// SOPT1 Configuration Register
    sopt1cfg: u32,

    /// USB PHY Control Register
    usbphyctl: u32,
}

#[repr(packed)]
struct SimData1 {
    /// System Options Register 2
    sopt2: ReadWrite<u32>,

    _pad0: u32,

    /// System Options Register 4
    sopt4: ReadWrite<u32>,

    /// System Options Register 5
    sopt5: ReadWrite<u32>,

    _pad1: u32,

    /// System Options Register 7
    sopt7: ReadWrite<u32>,

    /// System Options Register 8
    sopt8: ReadWrite<u32>,

    /// System Options Register 9
    sopt9: ReadWrite<u32>,

    /// System Device Identification Register
    sdid: ReadOnly<u32>,

    /// System Clock Gating Control Register 1
    scgc1: ReadWrite<u32>,

    /// System Clock Gating Control Register 2
    scgc2: ReadWrite<u32>,

    /// System Clock Gating Control Register 3
    scgc3: ReadWrite<u32>,

    /// System Clock Gating Control Register 4
    scgc4: ReadWrite<u32>,

    /// System Clock Gating Control Register 5
    scgc5: ReadWrite<u32>,

    /// System Clock Gating Control Register 6
    scgc6: ReadWrite<u32>,

    /// System Clock Gating Control Register 7
    scgc7: ReadWrite<u32>,

    /// System Clock Divider Register 1
    clkdiv1: ReadWrite<u32>,

    /// System Clock Divider Register 2
    clkdiv2: ReadWrite<u32>,

    /// Flash Configuration Register 1
    fcfg1: ReadOnly<u32>,

    /// Flash Configuration Register 2
    fcfg2: ReadOnly<u32>,

    /// Unique Identification Register High
    uidh: ReadOnly<u32>,

    /// Unique Identification Register Mid-High
    uidmh: ReadOnly<u32>,

    /// Unique Identification Register Mid Low
    uidml: ReadOnly<u32>,

    /// Unique Identification Register Low
    uidl: ReadOnly<u32>,

    /// System Clock Divider Register 3
    clkdiv3: ReadWrite<u32>,

    /// System Clock Divider Register 4
    clkdiv4: ReadWrite<u32>,
}
