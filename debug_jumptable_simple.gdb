# Simple script to verify jump table contents
# Usage: make gdb-jt (but rename this file to debug_jumptable.gdb first)

set pagination off
target remote :3333
monitor reset halt
load
monitor reset

# Break at jump table access
break USBDReadAndDispatchRequest
commands
    set $req = (tUSBRequest*)g_pui8DataBufferIn
    if (($req->bmRequestType & 0x60) == 0) && $req->bRequest < 13
        printf "\n=== Standard Request %d ===\n", $req->bRequest
        set $jt = g_psUSBDStdRequests
        printf "Jump table base: 0x%x\n", $jt
        
        # Read all 13 entries with correct 4-byte spacing
        printf "\nAll 13 entries (4-byte spacing):\n"
        set $i = 0
        while $i < 13
            set $addr = $jt + ($i * 4)
            set $val = *((uint32_t*)$addr)
            printf "  [%2d] @ 0x%04x = 0x%08x", $i, $addr, $val
            if $i == $req->bRequest
                printf " <-- REQUEST %d", $req->bRequest
            end
            if ($val & 1) == 0 && $val != 0
                printf " [INVALID!]"
            else
                if $val == 0
                    printf " [NULL]"
                else
                    printf " [OK]"
                end
            end
            printf "\n"
            set $i = $i + 1
        end
        
        # Check what the C code will read
        set $req_idx = $req->bRequest
        set $read_addr = $jt + ($req_idx * 4)
        set $read_val = *((uint32_t*)$read_addr)
        printf "\nC code will read: g_psUSBDStdRequests[%d] @ 0x%x = 0x%x\n", $req_idx, $read_addr, $read_val
    end
    continue
end

break HardFault
commands
    printf "\n=== HARD FAULT ===\n"
    printf "PC: 0x%x, LR: 0x%x\n", $pc, $lr
    stop
end

continue

