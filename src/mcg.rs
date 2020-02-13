use bit_field::BitField;
use volatile::Volatile;

// see section 27.4
#[repr(C, packed)]
pub struct Mcg {
    pub c1: Volatile<u8>,
    pub c2: Volatile<u8>,
    pub c3: Volatile<u8>,
    pub c4: Volatile<u8>,
    pub c5: Volatile<u8>,
    pub c6: Volatile<u8>,
    pub s: Volatile<u8>, // status
    pub _pad0: u8,
    pub sc: Volatile<u8>, // status and control
    pub _pad1: u8,
    pub atcvh: Volatile<u8>, // auto trim compare value high
    pub atcvl: Volatile<u8>, //  auto trim compare value low
    pub c7: Volatile<u8>,
    pub c8: Volatile<u8>,
    pub c9: Volatile<u8>,
    pub _pad2: u8,
    pub c11: Volatile<u8>,
    pub c12: Volatile<u8>,
    pub s2: Volatile<u8>, // status 2
    pub t3: Volatile<u8>, // test 3
}

impl Mcg {
    pub unsafe fn new() -> &'static mut Mcg {
        &mut *(0x40064000 as *mut Mcg)
    }

    // section 27.6.3.1
    pub fn fei_to_pee_120mhz(&mut self) {
        // transition from FEI to FBE:
        {
            self.c2.update(|c2| {
                // set RANGE to 'high frequency'
                c2.set_bits(4..6, 0b10);

                // set crystal to high gain operation
                c2.set_bit(3, true);

                // external reference select, oscillator crystal
                c2.set_bit(2, true);
            });

            self.c1.update(|c1| {
                // select external reference clock as system clock source
                c1.set_bits(6..8, 0b10);

                // set divider to 512
                // 8 MHz / 512 = 31.25 kHz
                // required to be between 31.25 kHZ and 39.0625 kHZ by FLL
                c1.set_bits(3..6, 0b100);

                // select external referenc clock, enable external oscillator
                c1.set_bit(2, false);
            });

            // wait for crystal initialization
            while !self.s.read().get_bit(1) {}

            // wait until current reference clock source is switched to external reference
            while self.s.read().get_bit(4) {}

            // wait until external reference clock starts feeding MCGOUTCLK
            while self.s.read().get_bits(2..4) != 0b10 {}

            // generate correct PLL reference frequency
            self.c5.update(|c5| {
                // divide by 2, PLL reference frequency of 16MHz/2 = 8MHz
                c5.set_bits(0..3, 0b001);
            });
        }

        // transition from FBE to PBE mode
        {
            self.c6.update(|c6| {
                // select the PLL
                c6.set_bit(6, true);

                // multiply by 30, because 8MHz * 30 = 240MHz
                // MCGPLL0CLK2X, frequency of VCO
                // divded by 2 to achieve MCGPLL0CLK, later used as MCGOUTCLK
                c6.set_bits(0..5, 0b1110);
            });

            // wait until current source for PLLS clock is the PLL
            while !self.s.read().get_bit(5) {}

            // wait until PLL acquires lock
            while !self.s.read().get_bit(6) {}
        }

        // transition from PBE to PEE
        {
            self.c1.update(|c1| {
                // select PLL as system clock source
                // this hangs and I don't know why
                // see cores/teensy3/mk20dx128.c, line 1054 for C equivalent
                c1.set_bits(6..8, 0b00);
            });

            // wait until PLL output is selected to feed MCGOUTCLK
            while self.s.read().get_bits(2..4) != 0b11 {}
        }

        // MCGOUTCLK should now be 120 MHz!
    }
}
