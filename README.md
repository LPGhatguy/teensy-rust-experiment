# Rust Experiment on Teensy 3.6
Repository containing a bunch of random tinkering on the Teensy 3.6. I'm trying to learn electrical engineering and embedded system development by starting from scratch(-ish) with a pure Rust application. Not very smart, right?

Things I want to do:

- [x] One blinking LED
- [x] Two blinking LEDs (wow!)
- [ ] Audio synthesis to speakers
- [ ] An instrument like [Wintergatan's Modulin](https://www.youtube.com/watch?v=mFfe4ZRQOH8)
	- [Potentiometer for this project](https://www.sparkfun.com/products/8681)

## Requirements
- Teensy loader CLI
- arm-none-eabi target and binutils

Run `./build` to build and deploy to a connected device.

## Resources
- [Teensy 3.6 Data Sheet](https://www.pjrc.com/teensy/K66P144M180SF5RMV2.pdf) (19 MB)
- [Branan Riley's *Exploring Rust on Teensy*](https://branan.github.io/teensy/)
	- This is targeting the Teensy 3.2. I tried to port things where relevant.

## Open Questions
If you have answers to any of these questions, help is appreciated on any communication channel. Thanks.

### Pin numbers
The K66 data sheet uses letter + number pairs to talk about pins, but the printed Teensy 3.6 board diagram uses numbers for them.

For example:

- C5 (the internal LED) is marked as pin 13
- C4 is marked as pin 10
- C6 is marked as pin 11

Searching for this is confusing and was frustrating for trying to get my first few things connected correctly.

## License
This project is available under the MIT license. Details are available in [LICENSE.txt](LICENSE.txt) or <https://opensource.org/licenses/MIT>.