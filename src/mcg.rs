//! The Multipurpose Clock Generator (MCG) controls clock generation for the
//! device.
//!
//! See the K66 Reference, Chapter 27 for details.

use core::{
    ptr,
    sync::atomic::{AtomicBool, Ordering},
};

static MCG_TAKEN: AtomicBool = AtomicBool::new(false);

const MCG_BASE_ADDRESS: usize = 0x4006_4000;

/// See section 27.1.1 for details here
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
enum ClockSource {
    ExternalCrystal = 0b00,
    ExternalRtc = 0b01,
    Internal = 0b10,
}

pub struct Mcg {
    data: *mut McgData,
}

impl Mcg {
    pub fn take() -> Option<Self> {
        let taken = MCG_TAKEN.swap(true, Ordering::Relaxed);
        if taken {
            return None;
        }

        let data = MCG_BASE_ADDRESS as *mut McgData;
        Some(Self { data })
    }
}

impl Drop for Mcg {
    fn drop(&mut self) {
        assert!(MCG_TAKEN.swap(false, Ordering::Relaxed));
    }
}

#[repr(packed)]
struct McgData {
    /// Control 1
    c1: u8,

    /// Control 2
    c2: u8,

    /// Control 3
    c3: u8,

    /// Control 4
    c4: u8,

    /// Control 5
    c5: u8,

    /// Control 6
    c6: u8,

    /// Status
    s: u8,

    _pad0: u8,

    /// Status and Control
    sc: u8,

    _pad1: u8,

    /// Auto Trim Compare Value High
    atcvh: u8,

    /// Auto Trim Compare Value Low
    atcvl: u8,

    /// Control 7
    c7: u8,

    /// Control 8
    c8: u8,

    /// Control 9
    c9: u8,

    _pad2: u8,

    /// Control 11
    c11: u8,

    /// Control 12
    c12: u8,

    /// Status 2
    s2: u8,

    /// Test 3
    t3: u8,
}
