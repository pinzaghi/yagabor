# YAGABOR - Yet Another GAme BOy emulator in Rust

A Game Boy (DMG) emulator writen in Rust for educational purposes. The current state of the implemented features:

| Module           | Status  |
| ---------------- |:-------:|
| CPU Instructions | ✅      |
| Timers           | ✅      |
| Keypad           | ⬜      |
| Sound            | ⬜      |
| Graphics         | ⬜      |
| Serial           | ⬜      |
| Interrupts       | ✅      |

# Interfaces

It is planned to support two interfaces, a desktop and a web version. Currently only the desktop GUI is implemented.

## Desktop

Simply run ```cargo run --release --bin desktop-gui [rom.gb]``` to start the desktop GUI.

# Tests

To run the tests, execute the following commands:

```git submodule update --init```

```cargo test```

This will run the following tests:

## Blargg's test roms

### cpu_instrs

| Module                   | Status  |
| ------------------------ |:-------:|
| 01-special.gb            | ✅      |
| 02-interrupts.gb         | ✅      |
| 03-op sp,hl.gb           | ✅      |
| 04-op r,imm.gb           | ✅      |
| 05-op rp.gb              | ✅      |
| 06-ld r,r.gb             | ✅      |
| 07-jr,jp,call,ret,rst.gb | ✅      |
| 08-misc instrs.gb        | ✅      |
| 09-op r,r.gb             | ✅      |
| 10-bit ops.gb            | ✅      |
| 11-op a,(hl).gb          | ✅      |
