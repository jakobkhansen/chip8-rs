# chip8-rs

A simple Chip-8 emulator written in Rust using SDL2.

## Usage

Run any `.ch8` ROM with

`cargo run <rom>`

Release binary can be built as usual with

`cargo build --release`

## Implementation

This emulator implements the instructions as outlined in [this blog
post](https://tobiasvl.github.io/blog/write-a-chip-8-emulator/#fx33-binary-coded-decimal-conversion)
which seems to be one of the most popular resources for implementing Chip-8. Since Chip-8
has quite a few different implementations (Super-chip, Chip-48, etc), there are some
ambiguous hardware instructions in there. This emulator will successfully emulate most
ROMs, but since some ROMs and test suites utilize these ambiguous instructions, YMMV.
