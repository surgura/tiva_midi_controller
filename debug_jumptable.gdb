# Simplified GDB script - only breaks on important events
# Usage: make gdb-jt

set pagination off
target remote :3333
monitor reset halt
load
monitor reset

# Only break on non-standard requests (class-specific, vendor, etc.)
# These are the ones that might cause issues
break USBDReadAndDispatchRequest
commands
    set $req = (tUSBRequest*)g_pui8DataBufferIn
    # Only stop if it's NOT a standard request (type != 0x00 in bits 6:5)
    if (($req->bmRequestType & 0x60) != 0)
        printf "\n=== Non-Standard Request: %d, Type: 0x%02x ===\n", $req->bRequest, $req->bmRequestType
        printf "Value: 0x%04x, Index: 0x%04x\n", $req->wValue, $req->wIndex
        
        # Check the callback handler that will be called
        set $dev_info_addr = &g_ppsDevInfo[0]
        set $dev_info = *((uint32_t*)$dev_info_addr)
        printf "g_ppsDevInfo[0] address: 0x%x\n", $dev_info_addr
        printf "g_ppsDevInfo[0] value: 0x%x\n", $dev_info
        if $dev_info != 0 && $dev_info != 0xffffffff
            # psCallbacks is first field in tDeviceInfo (offset 0)
            set $callbacks = *((uint32_t*)$dev_info)
            printf "psCallbacks: 0x%x\n", $callbacks
            if $callbacks != 0 && $callbacks != 0xffffffff
                # pfnRequestHandler is second field in tCustomHandlers (offset 4, after pfnGetDescriptor)
                set $req_handler = *((uint32_t*)($callbacks + 4))
                set $cb_data = g_psDCDInst[0].pvCBData
                printf "pfnRequestHandler: 0x%x\n", $req_handler
                printf "pvCBData: 0x%x\n", $cb_data
                if $req_handler == 0
                    printf "ERROR: pfnRequestHandler is NULL!\n"
                end
            else
                printf "ERROR: psCallbacks is invalid (0x%x)!\n", $callbacks
            end
        else
            printf "ERROR: g_ppsDevInfo[0] is invalid (0x%x)!\n", $dev_info
        end
        stop
    end
    continue
end

# Break right before calling pfnRequestHandler (line 1131)
break TivaWare_C_Series-2.2.0.295/usblib/device/usbdenum.c:1131
commands
    printf "\n=== About to call pfnRequestHandler ===\n"
    printf "SP: 0x%x\n", $sp
    # Get handler from callbacks structure
    set $dev_info_addr = &g_ppsDevInfo[0]
    set $dev_info = *((uint32_t*)$dev_info_addr)
    if $dev_info != 0 && $dev_info != 0xffffffff
        set $callbacks = *((uint32_t*)$dev_info)
        if $callbacks != 0 && $callbacks != 0xffffffff
            set $req_handler = *((uint32_t*)($callbacks + 4))
            printf "Handler function: 0x%x\n", $req_handler
            set $cb_data = g_psDCDInst[0].pvCBData
            printf "Callback data: 0x%x\n", $cb_data
            if ($req_handler & 1) == 0 && $req_handler != 0
                printf "ERROR: Handler address is even (invalid for Thumb)!\n"
                stop
            end
        else
            printf "ERROR: psCallbacks is invalid (0x%x)!\n", $callbacks
            stop
        end
    else
        printf "ERROR: g_ppsDevInfo[0] is invalid (0x%x)!\n", $dev_info
        stop
    end
    continue
end

# Break on HardFault
break HardFault
commands
    printf "\n=== HARD FAULT ===\n"
    printf "PC: 0x%x, LR: 0x%x, SP: 0x%x\n", $pc, $lr, $sp
    printf "CFSR: 0x%08x, HFSR: 0x%08x\n", *(uint32_t*)0xE000ED28, *(uint32_t*)0xE000ED2C
    
    # Get the last USB request
    set $req_addr = g_pui8DataBufferIn
    if $req_addr != 0
        set $req = (tUSBRequest*)$req_addr
        printf "\nLast USB Request:\n"
        printf "  Request: %d (0x%02x), Type: 0x%02x\n", $req->bRequest, $req->bRequest, $req->bmRequestType
    end
    stop
end

printf "\nUSB Debugging Started (breaks only on non-standard requests and HardFaults)\n"
printf "Unplug and replug USB device to trigger enumeration\n\n"
continue
