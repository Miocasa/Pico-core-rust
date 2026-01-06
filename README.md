# 🦀 RP Pico OLED Rust

> A starter template for Raspberry Pi Pico (RP2040) projects in Rust, designed to simplify development with an Arduino-like structure. This example demonstrates sharing a single I²C bus with multiple devices—specifically, two SSD1306 OLED displays.

This project provides a foundation for embedded Rust development on the Raspberry Pi Pico, including hardware abstraction, an application loop similar to Arduino's `setup()` and `loop()`, and convenient features like double-tap reset to bootloader.

## Hardware Setup

### Required Components
- Raspberry Pi Pico (RP2040 microcontroller)
- Two SSD1306 OLED displays (typically 128x64 or 128x32 resolution, I²C interface)

### Wiring Example
Connect both OLED displays to the same I²C bus (different addresses required, e.g., 0x3C and 0x3D):


### Raspberry Pi Pico Pinout Reference

Typical I²C pins used in this project (configurable in `src/display.rs`, in future configuration will move to `src/hardware.rs`):
- GP2  → SDA (I²C0)
- GP3  → SCL (I²C0)
- 3.3V → VCC
- GND  → GND

Adjust addresses and pins as needed for your displays.

## Prerequisites

- Rust toolchain: `rustup target add thumbv6m-none-eabi`
- `elf2uf2-rs`: `cargo install elf2uf2-rs`
- Optional (for advanced flashing/debugging): `probe-rs` or `probe-run`

## Building and Flashing

1. Clone the repository.
2. Build the project:
```
cargo build --release
```
3. To generate a UF2 file and flash (hold BOOTSEL button while connecting USB, or use double-tap reset):
```
cargo run --release   # If using probe-run
```
   Or manually:
```
elf2uf2-rs target/thumbv6m-none-eabi/release/rp-pico-oled-rust
```
   Copy the resulting `.uf2` file to the mounted RPI-RP2 drive.

## Project Structure

```txt
repo/
├── build.rs                # Build script for Rust configuration
├── Cargo.toml              # Dependencies and project metadata
├── Embed.toml              # Embedded-specific configuration
├── memory.x                # Linker script for memory layout (flash/RAM placement)
├── README.md               # This file
└── src/
├── app.rs              # Application logic (Arduino-like setup/loop)
├── display.rs          # Configuration and drivers for two SSD1306 displays (and i2c pins)
├── double_tap_reboot.rs # Double-tap reset to bootloader (Pico SDK-inspired)
├── hardware.rs         # Hardware initialization (I²C, UART, SPI, etc.)
└── main.rs             # Entry point (no_std, panic handler, etc.)
```

## Usage

- The main application loop is defined in `src/app.rs`. Modify this file to implement your logic, similar to Arduino's `loop()`.
- Displays are initialized in `src/display.rs` using the `ssd1306` crate and `embedded-graphics` for drawing.
- Hardware peripherals (e.g., I²C bus) are configured in `src/hardware.rs`.
- Double-tap the reset button (if enabled) to enter bootloader mode for easy flashing.

Extend this template by adding new peripherals in `hardware.rs` or additional logic in `app.rs`.

## Features

- Shared I²C bus with multiple SSD1306 devices
- Arduino-style application structure for familiarity
- Double-tap reset to bootloader
- No-std compatible with `rp2040-hal`

## License

[MIT License](LICENSE) (or specify your preferred license).

## Contributing

Contributions are welcome. Please open issues or pull requests for improvements.
