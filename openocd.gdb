# GDB script for OpenOCD debugging
# Usage: arm-none-eabi-gdb -x openocd.gdb <binary>

# Connect to OpenOCD
target extended-remote :3333

# Reset and halt the target
monitor reset halt

# Load the program
load

# Set a breakpoint at main
break main

# Continue to the breakpoint
continue

