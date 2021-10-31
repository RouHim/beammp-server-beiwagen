#!/bin/sh
#
# This shell script build the application for the following platforms in release mode:
#   * x86_64-unknown-linux-gnu | 64-bit Linux (kernel 2.6.32+, glibc 2.11+)
#   * aarch64-unknown-linux-gnu |	ARM64 Linux (kernel 4.2, glibc 2.17+)
#   * armv7-unknown-linux-gnueabihf | ARMv7 Linux, hardfloat (kernel 3.2, glibc 2.17)
#
######################
rustup target add x86_64-unknown-linux-gnu aarch64-unknown-linux-gnu armv7-unknown-linux-gnueabihf

# build x86_64 linux release
cargo build --release --target=x86_64-unknown-linux-gnu

# build arm64 linux release
# requirements: aarch64-linux-gnu-gcc,
cargo build --release --target=aarch64-unknown-linux-gnu

# build armv7 linux release
cargo build --release --target=armv7-unknown-linux-gnueabihf
