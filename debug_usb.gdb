# GDB script to debug USB interrupt handling
# Usage: gdb target/thumbv7em-none-eabihf/debug/tiva_controller -x debug_usb.gdb

# Disable pagination
set pagination off

# Connect to OpenOCD
target remote :3333

# Load the program
monitor reset halt
load
monitor reset

# Set breakpoints
break USB0DeviceIntHandler
break USBDeviceIntHandlerInternal
break USBDeviceEnumHandler
break USBDReadAndDispatchRequest
break control_handler

# Define a command to check interrupt status
define check_status
    printf "=== Interrupt Status Check ===\n"
    printf "Control Status: 0x%x\n", $ui32Status
    printf "Endpoint Status Register (0x40050004): 0x%x\n", *(volatile uint32_t*)0x40050004
    printf "Endpoint 0 Control/Status (0x40050010): 0x%x\n", *(volatile uint32_t*)0x40050010
    printf "USB0_IS Register (0x4005000A): 0x%x\n", *(volatile uint32_t*)0x4005000A
end

# Auto-print status when USB0DeviceIntHandler hits
commands 1
    printf "\n=== USB0DeviceIntHandler called ===\n"
    step
    printf "ui32Status = 0x%x\n", $ui32Status
    printf "Endpoint Status (0x40050004): 0x%x\n", *(volatile uint32_t*)0x40050004
    printf "EP0 Status (0x40050010): 0x%x\n", *(volatile uint32_t*)0x40050010
    continue
end

# Auto-print when USBDeviceIntHandlerInternal hits
commands 2
    printf "\n=== USBDeviceIntHandlerInternal called ===\n"
    printf "Control Status = 0x%x\n", $ui32Status
    # Check endpoint interrupts
    set $ep_status = *(volatile uint32_t*)0x40050004
    printf "Endpoint Interrupt Status (0x40050004) = 0x%x\n", $ep_status
    if $ep_status & 1
        printf "*** EP0 INTERRUPT DETECTED! ***\n"
    end
    continue
end

# Auto-print when USBDeviceEnumHandler hits
commands 3
    printf "\n=== USBDeviceEnumHandler called ===\n"
    printf "EP0 State: %d\n", pDevInstance->iEP0State
    printf "EP0 Status: 0x%x\n", ui32EPStatus
    if ui32EPStatus & 0x40
        printf "*** EP0 OUT PKTRDY is SET! ***\n"
    end
    continue
end

# Auto-print when USBDReadAndDispatchRequest hits
commands 4
    printf "\n=== USBDReadAndDispatchRequest called ===\n"
    printf "Request Type: 0x%x\n", psRequest->bmRequestType
    printf "Request: 0x%x\n", psRequest->bRequest
    printf "Value: 0x%x\n", psRequest->wValue
    printf "Index: 0x%x\n", psRequest->wIndex
    continue
end

# Auto-print when control_handler hits
commands 5
    printf "\n=== control_handler called ===\n"
    printf "Event: %d (0x%x)\n", ui32_event, ui32_event
    printf "Msg Value: 0x%x\n", ui32_msg_value
    continue
end

# Start execution
printf "\n========================================\n"
printf "USB Debugging Started\n"
printf "========================================\n"
printf "Breakpoints set on:\n"
printf "  1. USB0DeviceIntHandler\n"
printf "  2. USBDeviceIntHandlerInternal\n"
printf "  3. USBDeviceEnumHandler\n"
printf "  4. USBDReadAndDispatchRequest\n"
printf "  5. control_handler\n"
printf "\nINSTRUCTIONS:\n"
printf "  1. UNPLUG the USB device NOW (if connected)\n"
printf "  2. Wait 2 seconds\n"
printf "  3. PLUG the USB device back in\n"
printf "  4. Watch for breakpoint output below\n"
printf "\nBreakpoints will automatically print status and continue.\n"
printf "Press Ctrl+C in GDB to stop.\n"
printf "========================================\n\n"
set pagination off
# Start running - breakpoints will auto-continue
continue
# Keep running - breakpoints will hit and print automatically
while 1
    continue
end

