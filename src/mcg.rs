//! The Multipurpose Clock Generator (MCG) controls clock generation for the
//! device.
//!
//! State transitions are from section 27.6.3.1.
//!
//! See the K66 Reference, Chapter 27 for details.

#![allow(unused)]

use core::sync::atomic::{AtomicBool, Ordering};

use bitfield::{Bit, BitRange};
use volatile::{ReadOnly, ReadWrite, WriteOnly};

static MCG_TAKEN: AtomicBool = AtomicBool::new(false);

const MCG_BASE_ADDRESS: usize = 0x4006_4000;

pub struct Mcg<State> {
    data: *mut McgData,
    state: State,
}

impl Mcg<StateFei> {
    pub fn take() -> Option<Mcg<StateFei>> {
        let taken = MCG_TAKEN.swap(true, Ordering::Relaxed);
        if taken {
            return None;
        }

        let data = MCG_BASE_ADDRESS as *mut McgData;
        Some(Mcg {
            data,
            state: StateFei,
        })
    }
}

impl Mcg<StateFei> {
    pub fn into_fbe(self) -> Mcg<StateFbe> {
        use crate::port::{Port, PortName};

        let port_c = Port::take(PortName::C).unwrap();
        let mut pin_c5 = port_c.take_pin(5).unwrap().into_gpio().into_output();
        let mut pin_c6 = port_c.take_pin(6).unwrap().into_gpio().into_output();
        let mut pin_c4 = port_c.take_pin(4).unwrap().into_gpio().into_output();

        unsafe {
            (*self.data).c2.update(|c2| {
                // select "very high" frequency range
                c2.set_bit_range(4, 5, 0b10);

                // set high gain operation
                c2.set_bit(3, true);

                // set external reference to oscillator
                c2.set_bit(2, true);
            });

            (*self.data).c1.update(|c1| {
                // select external reference clock as source for MCGOUTCLK
                c1.set_bit_range(6, 7, McgOutClkSource::ExternalRef as u8);

                // set FLL reference divider to divide-by-512
                //
                // "8 MHz / 512 = 31.25 kHz which is in the 31.25 kHz to 39.0625
                // kHz range required by the FLL"
                c1.set_bit_range(3, 5, 0b100);

                // select external reference clock; enable external oscillator
                c1.set_bit(2, false);
            });

            pin_c5.high();

            // wait for crystal configured in c2 to be initialized
            while !(*self.data).s.read().bit(1) {}

            pin_c4.high();

            // wait for FLL reference clock to be the external clock
            while (*self.data).s.read().bit(4) {}

            pin_c6.high();

            // wait for external reference clock to be feeding MCGOUTCLK
            loop {
                // inference from the BitRange trait is super wonky
                let value: u8 = (*self.data).s.read().bit_range(2, 3);

                if value == 0b10 {
                    break;
                }
            }

            pin_c5.low();
        }

        Mcg {
            data: self.data,
            state: StateFbe,
        }
    }
}

impl Mcg<StateFbe> {
    pub fn into_pbe(self) -> Mcg<StatePbe> {
        unimplemented!();
    }
}

impl Mcg<StatePbe> {
    pub fn into_pee(self) -> Mcg<StatePee> {
        unimplemented!();
    }
}

/// FLL-engaged internal
#[derive(Debug, Clone, Copy)]
pub struct StateFei;

/// FLL-engaged external
#[derive(Debug, Clone, Copy)]
pub struct StateFee;

/// FLL-bypassed internal
#[derive(Debug, Clone, Copy)]
pub struct StateFbi;

/// FLL-bypassed external
#[derive(Debug, Clone, Copy)]
pub struct StateFbe;

/// PLL-engaged external
#[derive(Debug, Clone, Copy)]
pub struct StatePbe;

/// PLL-bypassed external
#[derive(Debug, Clone, Copy)]
pub struct StatePee;

/// From section 27.4.1, MCG_C1[CLKS]
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
enum McgOutClkSource {
    FllOrPllcs = 0b00,
    InternalRef = 0b01,
    ExternalRef = 0b10,
}

#[repr(packed)]
struct McgData {
    /// Control 1
    c1: ReadWrite<u8>,

    /// Control 2
    c2: ReadWrite<u8>,

    /// Control 3
    c3: ReadWrite<u8>,

    /// Control 4
    c4: ReadWrite<u8>,

    /// Control 5
    c5: ReadWrite<u8>,

    /// Control 6
    c6: ReadWrite<u8>,

    /// Status
    s: ReadOnly<u8>,

    _pad0: u8,

    /// Status and Control
    sc: ReadWrite<u8>,

    _pad1: u8,

    /// Auto Trim Compare Value High
    atcvh: ReadWrite<u8>,

    /// Auto Trim Compare Value Low
    atcvl: ReadWrite<u8>,

    /// Control 7
    c7: ReadWrite<u8>,

    /// Control 8
    c8: ReadWrite<u8>,

    /// Control 9
    c9: ReadWrite<u8>,

    _pad2: u8,

    /// Control 11
    c11: ReadWrite<u8>,

    /// Control 12
    c12: ReadWrite<u8>,

    /// Status 2
    s2: ReadWrite<u8>,

    /// Test 3
    t3: ReadWrite<u8>,
}
