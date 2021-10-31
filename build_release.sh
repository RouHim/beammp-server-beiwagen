#!/bin/sh
#
# This script cross compiles the application for the following platforms in release mode:
#   * x86_64-unknown-linux-gnu | 64-bit Linux (kernel 2.6.32+, glibc 2.11+)
#   * aarch64-unknown-linux-gnu |	ARM64 Linux (kernel 4.2, glibc 2.17+)
#
# Requirements to be installed:
#   * For ARM64: aarch64-linux-gnu-gcc
#   * For ARMv7: arm-linux-gnueabihf-gcc
#
# Set the installed linkers in the cargo config (~/.cargo/config):
#     [target.armv7-unknown-linux-gnueabihf]
#     linker = "arm-linux-gnueabihf-gcc"
#
#     [target.aarch64-unknown-linux-gnu]
#     linker = "aarch64-linux-gnu-gcc"
#
######################
rustup target add x86_64-unknown-linux-gnu \
                  aarch64-unknown-linux-gnu
#                  armv7-unknown-linux-gnueabihf

# build x86_64 linux release
cargo build --release --target=x86_64-unknown-linux-gnu
mv target/x86_64-unknown-linux-gnu/release/beammp-server-beiwagen target/beiwagen-x86_64

# build arm64 linux release
cargo build --release --target=aarch64-unknown-linux-gnu
mv target/aarch64-unknown-linux-gnu/release/beammp-server-beiwagen target/beiwagen-arm64

# Not compatible with rust-ssl
# build armv7 linux release
#cargo build --release --target=armv7-unknown-linux-gnueabihf
#mv target/armv7-unknown-linux-gnueabihf/release/beammp-server-beiwagen target/beiwagen-armv7