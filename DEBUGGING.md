# USB Device Not Detected - Debugging Guide

## First: Verify Code is Running

**Question: Is the LED blinking?**
- ✅ **YES** → Code is running, USB initialization may be failing
- ❌ **NO** → Code is not running, check flashing/debugger connection

## LED Patterns

- **Slow blink (~1 second)**: Code running, USB initialized, waiting for host
- **Rapid blink (~50ms)**: USB initialization FAILED
- **No blink**: Code not running

## Common Issues

### 1. Hardware Connection Issues

**Check:**
- USB cable is a **data cable** (not charge-only)
- USB connector on board is properly connected
- PD4 (USB D-) and PD5 (USB D+) are connected to USB connector
- Board has power (USB or external)

**Test:**
```bash
# On Linux, check if ANY USB device appears when you plug/unplug
watch -n 0.5 'lsusb | wc -l'
# Plug/unplug and see if count changes
```

### 2. USB Pins Not Configured

**Symptom:** LED blinks but no USB detection

**Check in code:**
- GPIO Port D enabled
- PD4/PD5 set to analog mode
- `GPIOPinTypeUSBAnalog()` called

### 3. USB Controller Not Enabled

**Symptom:** LED blinks but no USB detection

**What happens:**
- `USBDCDInit()` should enable USB peripheral clock
- `USBDCDInit()` should enable USB PLL
- If this fails, USB won't work

**Debug:**
- Check if initialization returns NULL (LED would blink rapidly)
- If NULL, USB initialization failed

### 4. USB Soft Connect Not Set

**Symptom:** USB controller initialized but device not visible

**What happens:**
- `USBDCDInit()` should call `USBDevConnect()` internally
- This sets the soft connect bit
- Without this, device won't appear on bus

### 5. Interrupts Not Enabled

**Symptom:** Device might appear briefly then disappear

**Check:**
- Global interrupts enabled after USB init
- USB interrupt handler registered
- USB interrupts enabled in controller

## Diagnostic Steps

### Step 1: Verify Code is Running
```bash
# Flash the code
make flash

# Check LED behavior
# - Slow blink = OK, waiting for USB
# - Rapid blink = USB init failed
# - No blink = Code not running
```

### Step 2: Check USB Hardware
```bash
# Monitor USB events
sudo dmesg -w

# Plug/unplug USB cable
# Should see USB events even if device not recognized
```

### Step 3: Check USB Controller Status
```bash
# List all USB devices
lsusb

# Check USB subsystem
lsmod | grep usb
```

### Step 4: Verify USB Pins
- Check schematic: PD4 = USB D-, PD5 = USB D+
- Verify connections with multimeter
- Check for shorts

## Next Steps

If LED is blinking slowly but USB not detected:

1. **Add explicit USBDevConnect call** (may help if internal call is failing)
2. **Add delays** after initialization
3. **Check USB register status** (requires debugger)
4. **Verify USB PLL is running** (requires debugger)

If LED is blinking rapidly:
- USB initialization is failing
- Check return value from USBDCDCInit
- Verify all required TivaWare files are compiled
- Check for linker errors

If LED is not blinking:
- Code is not running
- Check flashing process
- Verify debugger connection
- Check reset/power

