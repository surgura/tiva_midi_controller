# GDB script to debug HardFault
# Usage: gdb target/thumbv7em-none-eabihf/debug/tiva_controller -x debug_hardfault.gdb

set pagination off

# Connect to OpenOCD
target remote :3333

# Load the program
monitor reset halt
load
monitor reset

# Set breakpoints at key initialization points
break main
break USBDCDCInit
break USBDCDInit
break HardFault

# When HardFault hits, print fault information
commands 4
    printf "\n=== HARD FAULT DETECTED ===\n"
    printf "Current PC: 0x%x\n", $pc
    printf "Current LR: 0x%x\n", $lr
    printf "Current SP: 0x%x\n", $sp
    printf "Current MSP: 0x%x\n", $msp
    printf "\nCall stack:\n"
    backtrace
    printf "\nRegisters:\n"
    info registers
    printf "\nStack contents (20 words):\n"
    x/20x $sp
    printf "\nDisassembly around current PC:\n"
    x/10i $pc
    printf "\nFault Status Registers:\n"
    set $cfsr = *((uint32_t*)0xE000ED28)
    set $hfsr = *((uint32_t*)0xE000ED2C)
    set $mmfar = *((uint32_t*)0xE000ED34)
    set $bfar = *((uint32_t*)0xE000ED38)
    printf "CFSR (0xE000ED28): 0x%x\n", $cfsr
    printf "HFSR (0xE000ED2C): 0x%x\n", $hfsr
    printf "MMFAR (0xE000ED34): 0x%x\n", $mmfar
    printf "BFAR (0xE000ED38): 0x%x\n", $bfar
    if ($cfsr & 0x80)
        printf "  IACCVIOL: Instruction access violation\n"
    end
    if ($cfsr & 0x100)
        printf "  DACCVIOL: Data access violation\n"
    end
    if ($cfsr & 0x200)
        printf "  MUNSTKERR: Memory management fault on unstacking\n"
    end
    if ($cfsr & 0x400)
        printf "  MSTKERR: Memory management fault on stacking\n"
    end
    if ($cfsr & 0x800)
        printf "  MMARVALID: MMFAR is valid\n"
    end
    printf "\nInspecting crash location:\n"
    set $saved_pc = *(uint32_t*)($sp+12)
    set $saved_lr = *(uint32_t*)($sp+16)
    printf "PC before fault: 0x%x\n", $saved_pc
    printf "LR before fault: 0x%x\n", $saved_lr
    printf "\nChecking USB request state:\n"
    set $req_addr = g_pui8DataBufferIn
    printf "Request buffer address: 0x%x\n", $req_addr
    if ($req_addr != 0)
        set $req = (tUSBRequest*)$req_addr
        printf "Request Type: 0x%x\n", $req->bmRequestType
        printf "Request: 0x%x (%d)\n", $req->bRequest, $req->bRequest
        printf "Value: 0x%x\n", $req->wValue
        printf "Index: 0x%x\n", $req->wIndex
        printf "Length: 0x%x\n", $req->wLength
        printf "\nFunction pointer being called:\n"
        set $req_num = $req->bRequest
        printf "Request number: %d\n", $req_num
        if ($req_num < 16)
            set $jump_table = g_psUSBDStdRequests
            printf "Jump table address: 0x%x\n", $jump_table
            set $func_ptr = *((uint32_t*)($jump_table + $req_num * 4))
            printf "g_psUSBDStdRequests[%d] = 0x%x\n", $req_num, $func_ptr
            if ($func_ptr != 0)
                if ($func_ptr & 1)
                    printf "Function pointer is ODD (OK for Thumb)\n"
                else
                    printf "Function pointer is EVEN (INVALID for Thumb!)\n"
                end
            end
        else
            printf "Request number %d is out of bounds!\n", $req_num
        end
    end
    printf "\nChecking device info:\n"
    set $dev_info = g_ppsDevInfo[0]
    printf "g_ppsDevInfo[0]: 0x%x\n", $dev_info
    if ($dev_info != 0)
        printf "psCallbacks: 0x%x\n", $dev_info->psCallbacks
        printf "ppui8StringDescriptors: 0x%x\n", $dev_info->ppui8StringDescriptors
    end
    stop
end

# When main hits, step through initialization
commands 1
    printf "\n=== main() called ===\n"
    continue
end

# When USBDCDCInit hits
commands 2
    printf "\n=== USBDCDCInit called ===\n"
    continue
end

# When USBDCDInit hits
commands 3
    printf "\n=== USBDCDInit called ===\n"
    continue
end

# Start execution
printf "\n========================================\n"
printf "HardFault Debugging Started\n"
printf "========================================\n"
printf "Breakpoints set on:\n"
printf "  1. main\n"
printf "  2. USBDCDCInit\n"
printf "  3. USBDCDInit\n"
printf "  4. HardFault\n"
printf "\nStepping through initialization...\n"
printf "========================================\n\n"
continue

