# Rust Experiment on Teensy 3.6
Repository containing a bunch of random tinkering on the Teensy 3.6. I'm trying to learn electrical engineering and embedded system development by starting from scratch with a pure Rust application. Not very smart, right?

Things I want to do:

- [x] One blinking LED
- [ ] Two blinking LEDs (wow!)
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

## License
This project is available under the MIT license. Details are available in [LICENSE.txt](LICENSE.txt) or <https://opensource.org/licenses/MIT>.