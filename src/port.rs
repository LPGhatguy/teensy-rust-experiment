//! Port Control and Interrupt (PORT) module. The K66 manual lists it like PORT
//! is an acronym for those words, but I don't think it is.
//!
//! This module controls all the device's ports, and they all have a bunch of
//! pins.
//!
//! See the K66 Reference, Chapter 12 for details.

use core::{
    ptr,
    sync::atomic::{AtomicBool, Ordering},
};

static PORTS_TAKEN: [AtomicBool; 5] = [
    AtomicBool::new(false),
    AtomicBool::new(false),
    AtomicBool::new(false),
    AtomicBool::new(false),
    AtomicBool::new(false),
];

const CONTROL_BASE_ADDRESSES: [u32; 5] = [
    0x4004_9000,
    0x4004_A000,
    0x4004_B000,
    0x4004_C000,
    0x4004_D000,
];

const GPIO_BASE_ADDRESSES: [u32; 5] = [
    0x400F_F000,
    0x400F_F040,
    0x400F_F080,
    0x400F_F0C0,
    0x400F_F100,
];

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum PortName {
    A,
    B,
    C,
    D,
    E,
}

pub struct Port {
    name: PortName,
    control: *mut PortControlData,
    gpio: *mut PortGpioData,
    pins_taken: [AtomicBool; 32],
}

impl Port {
    pub fn take(name: PortName) -> Option<Self> {
        let index = name as u8 as usize;
        let port_taken = PORTS_TAKEN[index].swap(true, Ordering::Relaxed);

        if port_taken {
            return None;
        }

        let control = CONTROL_BASE_ADDRESSES[index] as *mut PortControlData;
        let gpio = GPIO_BASE_ADDRESSES[index] as *mut PortGpioData;
        let pins_taken = Default::default();

        Some(Port {
            name,
            control,
            gpio,
            pins_taken,
        })
    }

    pub fn take_pin(&self, num: u8) -> Option<Pin<'_>> {
        let index = num as usize;
        let pin_taken = self.pins_taken[index].swap(true, Ordering::Relaxed);

        if pin_taken {
            return None;
        }

        Some(Pin { port: self, num })
    }
}

impl Drop for Port {
    fn drop(&mut self) {
        let index = self.name as u8 as usize;
        // assert!(PORTS_TAKEN[index].swap(false, Ordering::Relaxed));
    }
}

pub struct Pin<'a> {
    port: &'a Port,
    num: u8,
}

impl<'a> Pin<'a> {
    pub fn into_gpio(self) -> GpioInputPin<'a> {
        unsafe {
            let index = self.num as usize;
            let pcr = &mut (*self.port.control).pcr[index] as *mut u32;

            let value = ptr::read_volatile(pcr);
            ptr::write_volatile(pcr, value | (1 << 8));
        }

        GpioInputPin { pin: self }
    }
}

impl<'a> Drop for Pin<'a> {
    fn drop(&mut self) {
        let index = self.num as usize;
        // assert!(self.port.pins_taken[index].swap(false, Ordering::Relaxed));
    }
}

pub struct GpioInputPin<'a> {
    pin: Pin<'a>,
}

impl<'a> GpioInputPin<'a> {
    pub fn into_output(self) -> GpioOutputPin<'a> {
        unsafe {
            let pddr = &mut (*self.pin.port.gpio).pddr as *mut u32;

            let value = ptr::read_volatile(pddr);
            let mask = 1 << self.pin.num;
            ptr::write_volatile(pddr, value | mask);
        }

        GpioOutputPin { pin: self.pin }
    }

    pub fn read(&self) -> bool {
        unsafe {
            let pdir = &mut (*self.pin.port.gpio).pdir as *mut u32;

            let value = ptr::read_volatile(pdir);
            (value & (1 << self.pin.num)) != 0
        }
    }
}

pub struct GpioOutputPin<'a> {
    pin: Pin<'a>,
}

impl<'a> GpioOutputPin<'a> {
    pub fn low(&mut self) {
        unsafe {
            let pdor = &mut (*self.pin.port.gpio).pdor as *mut u32;

            let value = ptr::read_volatile(pdor);
            let mask = 1 << self.pin.num;
            ptr::write_volatile(pdor, value & !mask);
        }
    }

    pub fn high(&mut self) {
        unsafe {
            let pdor = &mut (*self.pin.port.gpio).pdor as *mut u32;

            let value = ptr::read_volatile(pdor);
            let mask = 1 << self.pin.num;
            ptr::write_volatile(pdor, value | mask);
        }
    }
}

#[repr(C, packed)]
struct PortControlData {
    /// Pin control registers
    pcr: [u32; 32],

    /// Global pin control low register
    gpclr: u32,

    /// Global pin control high register
    gpchr: u32,

    /// Interrupt status flag register
    isfr: u32,

    /// Digital flag enable register
    dfer: u32,

    /// Digital filter clock register
    dfcr: u32,

    /// Digital filter width register
    dfwr: u32,
}

#[repr(C, packed)]
struct PortGpioData {
    /// Port data output register
    pdor: u32,

    /// Port set output register
    psor: u32,

    /// Port clear output register
    pcor: u32,

    /// Port toggle output register
    ptor: u32,

    /// Port data input register
    pdir: u32,

    /// Port data direction register
    pddr: u32,
}
