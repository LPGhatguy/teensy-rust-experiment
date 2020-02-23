#!/bin/sh
set -e

BIN=teensy-rs
OUTDIR=target/thumbv7em-none-eabihf/release
HEX=$OUTDIR/$BIN.HEX
ELF=$OUTDIR/$BIN

cargo build --release
rust-objcopy -O ihex $ELF $HEX

# I have to run the deployment script twice for some reason, the first one
# doesn't do anything.
teensy_loader_cli -w -mmcu=mk66fx1m0 $HEX -v
sleep 1
teensy_loader_cli -w -mmcu=mk66fx1m0 $HEX -v