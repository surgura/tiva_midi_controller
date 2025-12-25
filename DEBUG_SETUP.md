# Debugging Setup Guide

## Installing ARM GDB on openSUSE

ARM GDB is not available in openSUSE repositories. You need to install it manually:

### Option 1: ARM GNU Toolchain (Recommended)

1. **Download ARM GNU Toolchain:**
   - Visit: https://developer.arm.com/downloads/-/arm-gnu-toolchain-downloads
   - Select: **AArch32 bare-metal target (arm-none-eabi)**
   - Choose the Linux x86_64 version
   - Download the tarball

2. **Extract and install:**
   ```bash
   cd /opt  # or another location
   tar xf ~/Downloads/arm-gnu-toolchain-*.tar.xz
   sudo ln -s /opt/arm-gnu-toolchain-*/bin/arm-none-eabi-gdb /usr/local/bin/
   ```

3. **Verify installation:**
   ```bash
   arm-none-eabi-gdb --version
   ```

### Option 2: Use system GDB (may not work for embedded)

If you have the regular `gdb` installed, you can try using it, but it may not support ARM targets properly.

## Using the Debugger

1. **Start OpenOCD:**
   ```bash
   make debug
   ```

2. **In another terminal, connect GDB:**
   ```bash
   make gdb
   ```

3. **Or connect manually:**
   ```bash
   arm-none-eabi-gdb target/thumbv7em-none-eabihf/release/tiva_controller
   (gdb) target remote :3333
   (gdb) monitor reset halt
   (gdb) load
   (gdb) break USBDCDCInit
   (gdb) continue
   ```

## Useful GDB Commands

- `break <function>` - Set breakpoint at function
- `break <file>:<line>` - Set breakpoint at line
- `continue` or `c` - Continue execution
- `step` or `s` - Step into function
- `next` or `n` - Step over function
- `backtrace` or `bt` - Show call stack
- `info registers` - Show CPU registers
- `x/10i $pc` - Disassemble around program counter
- `print <variable>` - Print variable value
- `monitor reset halt` - Reset and halt target
- `monitor reset` - Reset target

## Stopping OpenOCD

```bash
make debug-stop
```

