# TM4C123GXL LaunchPad - Rust Firmware

Bare-metal Rust firmware for the TI TM4C123GXL (Tiva C Series) LaunchPad.

## Features

- Blinks the onboard RGB LED (red LED on PF1)
- Minimal bare-metal implementation using `tm4c123x` PAC
- No RTOS, no std library

## Prerequisites

### Rust Toolchain

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# The thumbv7em-none-eabihf target will be installed automatically
# via rust-toolchain.toml when you build
```

### ARM Toolchain (for debugging/size info)

```bash
# Ubuntu/Debian
sudo apt install gcc-arm-none-eabi gdb-multiarch

# Or use arm-none-eabi-gdb if available
```

### OpenOCD

#### Linux
```bash
sudo apt install openocd
```

#### Windows (for WSL users)
1. Download **xPack OpenOCD (Windows)**  
   https://github.com/xpack-dev-tools/openocd-xpack/releases
2. Extract, e.g. `C:\xpack-openocd-0.12.0-7\`
3. Install **WinUSB** on **In-Circuit Debug Interface (Interface 2)** using Zadig  
   (do not change Interface 3)

## Building

```bash
# Debug build
make

# Release build (optimized)
make release

# Check code without building
make check
```

## Flashing

```bash
# Flash release build (recommended)
make flash

# Flash debug build
make flash-debug

# Flash via Windows OpenOCD (for WSL users)
make flash-win
```

## Debugging

```bash
# Terminal 1: Start OpenOCD server
make debug

# Terminal 2: Connect GDB
make gdb
```

## Project Structure

```
tiva_controller/
├── .cargo/
│   └── config.toml      # Cargo configuration (target, linker flags)
├── src/
│   └── main.rs          # Main application code
├── build.rs             # Build script (copies memory.x)
├── Cargo.toml           # Rust package manifest
├── memory.x             # Memory layout for TM4C123GH6PM
├── openocd.cfg          # OpenOCD configuration for TI-ICDI
├── openocd.gdb          # GDB initialization script
├── rust-toolchain.toml  # Rust toolchain specification
├── Makefile             # Build/flash/debug commands
└── README.md
```

## Memory Map

- **Flash**: 256KB @ 0x00000000
- **SRAM**: 32KB @ 0x20000000

## Hardware

- **MCU**: TM4C123GH6PM (ARM Cortex-M4F, 80MHz)
- **Board**: EK-TM4C123GXL LaunchPad
- **LEDs**: RGB LED on PF1 (Red), PF2 (Blue), PF3 (Green)
- **Debug**: TI ICDI (JTAG only, SWD not supported)

## License

MIT or Apache-2.0, at your option.
