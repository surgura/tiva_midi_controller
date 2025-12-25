# GDB Debugging Commands

## Basic Commands

- `break <function>` - Set breakpoint at function
- `break <file>:<line>` - Set breakpoint at line number
- `continue` or `c` - Continue execution
- `step` or `s` - Step into function
- `next` or `n` - Step over function
- `finish` - Step out of current function
- `backtrace` or `bt` - Show call stack
- `info registers` - Show CPU registers
- `print <variable>` - Print variable value
- `x/10i $pc` - Disassemble 10 instructions around program counter
- `list` - Show source code around current line

## Debugging the USBDCDCInit Crash

Since the crash happens in `USBDCDCInit`, try:

```gdb
# Set breakpoint at the function that crashes
(gdb) break USBDCDCInit
(gdb) continue

# When it breaks, step through to find where it crashes
(gdb) step
(gdb) step
# ... keep stepping until crash

# Or set breakpoint at the composite init function
(gdb) break USBDCDCCompositeInit
(gdb) continue
(gdb) step

# Check the call stack when it crashes
(gdb) backtrace

# Check registers to see if there's a hard fault
(gdb) info registers
(gdb) print/x $pc
(gdb) print/x $sp
(gdb) print/x $lr
```

## Useful Breakpoints

```gdb
# Break at main entry
(gdb) break main
(gdb) continue

# Break at USB initialization
(gdb) break USBDCDCInit
(gdb) break USBDCDCCompositeInit
(gdb) break USBDCDInit
(gdb) break USBDCDDeviceInfoInit

# Break at panic handler (if it's called)
(gdb) break rust_begin_unwind
```

## Examining Memory

```gdb
# Check if stack overflow
(gdb) print/x $sp
(gdb) x/20x $sp

# Check global variables
(gdb) print g_pui8CDCSerDeviceDescriptor
(gdb) print g_ppCDCSerConfigDescriptors
(gdb) print g_sCDCHandlers
```

## When It Crashes

```gdb
# Show where we are
(gdb) backtrace
(gdb) info registers
(gdb) x/10i $pc

# Check if it's a hard fault
(gdb) print/x $pc
# If PC is 0xFFFFFFF9 or similar, it's a hard fault handler
```

