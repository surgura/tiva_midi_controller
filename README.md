# TM4C123GXL Rust Firmware

Bare-metal Rust for the TI TM4C123GXL LaunchPad.

## Windows Flashing Setup

1. Download **xPack OpenOCD**  
   https://github.com/xpack-dev-tools/openocd-xpack/releases
2. Extract to `C:\xpack-openocd\` and add `bin` to PATH
3. Install **WinUSB** on **In-Circuit Debug Interface (Interface 2)** using Zadig  
   (do not change Interface 3)

Note: ICDI is JTAG-only. SWD is not supported.
