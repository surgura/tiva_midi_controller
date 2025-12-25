# Testing USB Device Detection

## After Flashing

1. **Disconnect the debugger/programmer** (if using ICDI/OpenOCD)
2. **Connect the USB cable** from the board to your computer
3. **Check if the device is detected**

## Windows Testing

### Method 1: Device Manager
1. Open **Device Manager** (Win+X → Device Manager)
2. Look for:
   - **"Ports (COM & LPT)"** → Should show "Virtual COM Port" or similar
   - **"Universal Serial Bus devices"** → Should show a USB device
   - **"Other devices"** → If driver is missing, it may appear here with a yellow warning

### Method 2: PowerShell/Command Prompt
```powershell
# List all USB devices
Get-PnpDevice | Where-Object {$_.Class -eq "USB" -or $_.Class -eq "Ports"} | Format-Table FriendlyName, Status, InstanceId

# Or use devcon (if available)
devcon findall USB
```

### Method 3: USB Device Viewer
1. Download **USBDeview** from NirSoft (free utility)
2. Run it to see all USB devices
3. Look for:
   - **VID: 0x1CBE** (Texas Instruments)
   - **PID: 0x0002** (Serial)
   - **Manufacturer: "Texas Instruments"**
   - **Product: "Virtual COM Port"**

### Expected Results
- **If working**: Device appears as "Virtual COM Port" or "USB Serial Device" in COM ports
- **If driver missing**: Device appears in "Other devices" with yellow warning, or as "Unknown USB Device"
- **If not detected**: Nothing appears, or device appears briefly then disappears

## Linux Testing

### Method 1: lsusb
```bash
# List all USB devices
lsusb

# Look for:
# Bus XXX Device XXX: ID 1cbe:0002 Texas Instruments Virtual COM Port
```

### Method 2: dmesg (kernel messages)
```bash
# Watch kernel messages in real-time
sudo dmesg -w

# Or check recent messages
dmesg | tail -20

# Look for messages like:
# usb 1-1: new full-speed USB device number X using ...
# cdc_acm 1-1:1.0: ttyACM0: USB ACM device
```

### Method 3: Check /dev/ttyACM* or /dev/ttyUSB*
```bash
# List serial devices
ls -l /dev/ttyACM* /dev/ttyUSB* 2>/dev/null

# Or check what was created
ls -l /dev/tty* | grep -E "ACM|USB"

# Should see something like:
# crw-rw---- 1 root dialout 166, 0 Dec 25 18:00 /dev/ttyACM0
```

### Method 4: udevadm (device information)
```bash
# Monitor USB device events
sudo udevadm monitor --property --udev

# Or check device info
udevadm info /dev/ttyACM0 2>/dev/null | grep -E "ID_VENDOR|ID_MODEL|ID_SERIAL"
```

### Expected Results
- **If working**: 
  - `lsusb` shows: `ID 1cbe:0002 Texas Instruments Virtual COM Port`
  - `/dev/ttyACM0` (or similar) is created
  - `dmesg` shows successful enumeration
- **If not detected**: Nothing appears in `lsusb` or `dmesg` shows errors

## Troubleshooting

### Device Not Detected

1. **Check power**
   - Ensure board is powered (USB or external power)
   - LED should be blinking (if your code blinks LED)

2. **Check USB cable**
   - Try a different USB cable (data cable, not charge-only)
   - Try a different USB port

3. **Check connections**
   - Verify PD4/PD5 are connected to USB connector
   - Check for shorts or bad connections

4. **Check code**
   - Verify the code actually flashed (LED blinking confirms code is running)
   - Check if USB initialization succeeded (no error LED pattern)

5. **Windows Driver**
   - If device appears in "Other devices", install driver from:
     - `TivaWare_C_Series-2.2.0.295/windows_drivers/`
   - Or use Windows Update to find driver automatically

### Device Detected But Not Working

1. **Check device properties**
   - Right-click device in Device Manager → Properties
   - Check "Device status" for errors

2. **Test serial communication**
   ```bash
   # Linux: Use minicom or screen
   sudo minicom -D /dev/ttyACM0 -b 115200
   # Or
   sudo screen /dev/ttyACM0 115200
   ```

3. **Check permissions (Linux)**
   ```bash
   # Add user to dialout group
   sudo usermod -a -G dialout $USER
   # Log out and back in
   ```

## Quick Test Script (Linux)

```bash
#!/bin/bash
# test_usb.sh - Quick USB device test

echo "Checking for USB device..."
if lsusb | grep -q "1cbe:0002"; then
    echo "✅ USB device detected!"
    lsusb | grep "1cbe:0002"
    
    if [ -e /dev/ttyACM0 ]; then
        echo "✅ Serial port created: /dev/ttyACM0"
        ls -l /dev/ttyACM0
    else
        echo "⚠️  Device detected but no serial port created"
    fi
else
    echo "❌ USB device NOT detected"
    echo "Recent USB events:"
    dmesg | tail -10 | grep -i usb
fi
```

Run with: `chmod +x test_usb.sh && ./test_usb.sh`

