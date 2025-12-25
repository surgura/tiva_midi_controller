//! USB Descriptors
//! 
//! This module contains USB device descriptors for CDC serial port functionality.

use core::ffi::c_void;

// Language descriptor (English US)
pub const LANG_DESCRIPTOR: [u8; 4] = [
    4,                          // bLength
    3,                          // bDescriptorType (STRING)
    0x09, 0x04,                 // wLANGID (English US)
];

// Manufacturer string: "Texas Instruments"
pub const MANUFACTURER_STRING: [u8; 36] = [
    36,                         // bLength (17 chars * 2 + 2)
    3,                          // bDescriptorType (STRING)
    b'T', 0, b'e', 0, b'x', 0, b'a', 0, b's', 0, b' ', 0,
    b'I', 0, b'n', 0, b's', 0, b't', 0, b'r', 0, b'u', 0,
    b'm', 0, b'e', 0, b'n', 0, b't', 0, b's', 0,
];

// Product string: "Virtual COM Port"
pub const PRODUCT_STRING: [u8; 34] = [
    34,                         // bLength (16 chars * 2 + 2)
    3,                          // bDescriptorType (STRING)
    b'V', 0, b'i', 0, b'r', 0, b't', 0, b'u', 0, b'a', 0, b'l', 0, b' ', 0,
    b'C', 0, b'O', 0, b'M', 0, b' ', 0, b'P', 0, b'o', 0, b'r', 0, b't', 0,
];

// Serial number string: "12345678"
pub const SERIAL_STRING: [u8; 18] = [
    18,                         // bLength (8 chars * 2 + 2)
    3,                          // bDescriptorType (STRING)
    b'1', 0, b'2', 0, b'3', 0, b'4', 0, b'5', 0, b'6', 0, b'7', 0, b'8', 0,
];

// Control interface description string: "ACM Control Interface"
pub const CONTROL_INTERFACE_STRING: [u8; 44] = [
    44,                         // bLength (21 chars * 2 + 2)
    3,                          // bDescriptorType (STRING)
    b'A', 0, b'C', 0, b'M', 0, b' ', 0, b'C', 0, b'o', 0, b'n', 0, b't', 0,
    b'r', 0, b'o', 0, b'l', 0, b' ', 0, b'I', 0, b'n', 0, b't', 0, b'e', 0,
    b'r', 0, b'f', 0, b'a', 0, b'c', 0, b'e', 0,
];

// Configuration description string: "Self Powered Configuration"
pub const CONFIG_STRING: [u8; 54] = [
    54,                         // bLength (26 chars * 2 + 2)
    3,                          // bDescriptorType (STRING)
    b'S', 0, b'e', 0, b'l', 0, b'f', 0, b' ', 0, b'P', 0, b'o', 0, b'w', 0,
    b'e', 0, b'r', 0, b'e', 0, b'd', 0, b' ', 0, b'C', 0, b'o', 0, b'n', 0,
    b'f', 0, b'i', 0, b'g', 0, b'u', 0, b'r', 0, b'a', 0, b't', 0, b'i', 0,
    b'o', 0, b'n', 0,
];

// Wrapper to make raw pointers Sync-safe for embedded use
// Safe because: embedded is single-threaded, pointers are const, only read
struct StringDescriptorPtr(*const u8);
unsafe impl Sync for StringDescriptorPtr {}

// Static array of string descriptor pointers - must be static so it lives
// for the entire program lifetime (C code accesses it during interrupts)
// This matches the C example's g_ppui8StringDescriptors
// Using a wrapper type to make it Sync-safe
static STRING_DESCRIPTOR_PTRS: [StringDescriptorPtr; 6] = [
    StringDescriptorPtr(LANG_DESCRIPTOR.as_ptr()),
    StringDescriptorPtr(MANUFACTURER_STRING.as_ptr()),
    StringDescriptorPtr(PRODUCT_STRING.as_ptr()),
    StringDescriptorPtr(SERIAL_STRING.as_ptr()),
    StringDescriptorPtr(CONTROL_INTERFACE_STRING.as_ptr()),
    StringDescriptorPtr(CONFIG_STRING.as_ptr()),
];

// Static mut array of raw pointers - initialized once, then only read
// This is safe because:
// 1. Embedded is single-threaded
// 2. Initialization happens once before USB is used
// 3. After initialization, only read operations occur
static mut PTR_ARRAY: [*const u8; 6] = [core::ptr::null(); 6];
static mut INITIALIZED: bool = false;

// Function to get the static pointer array for C FFI
// This returns a pointer to the static array, which is safe because
// the array lives for the entire program lifetime
pub fn get_string_descriptors() -> *const *const u8 {
    unsafe {
        // Initialize once on first call
        if !INITIALIZED {
            PTR_ARRAY[0] = STRING_DESCRIPTOR_PTRS[0].0;
            PTR_ARRAY[1] = STRING_DESCRIPTOR_PTRS[1].0;
            PTR_ARRAY[2] = STRING_DESCRIPTOR_PTRS[2].0;
            PTR_ARRAY[3] = STRING_DESCRIPTOR_PTRS[3].0;
            PTR_ARRAY[4] = STRING_DESCRIPTOR_PTRS[4].0;
            PTR_ARRAY[5] = STRING_DESCRIPTOR_PTRS[5].0;
            INITIALIZED = true;
        }
        
        PTR_ARRAY.as_ptr()
    }
}

// USB callback functions
// These are minimal implementations that just return success

/// Control handler callback
#[no_mangle]
pub unsafe extern "C" fn control_handler(
    pv_cb_data: *mut c_void,
    ui32_event: u32,
    ui32_msg_value: u32,
    pv_msg_data: *mut c_void,
) -> u32 {
    // Event 7 = USB_EVENT_CONNECTED (device connected to host)
    // Event values from usblib.h:
    // 0 = USB_EVENT_POWER_FAULT
    // 1 = USB_EVENT_VBUS_ERR
    // 2 = USB_EVENT_POWER_ENABLE
    // 3 = USB_EVENT_POWER_DISABLE
    // 4 = USB_EVENT_OVRCURRENT_FAULT
    // 5 = USB_EVENT_OVRCURRENT_FAULT_CLEAR
    // 6 = USB_EVENT_SUSPEND
    // 7 = USB_EVENT_CONNECTED
    // 8 = USB_EVENT_DISCONNECTED
    // 9 = USB_EVENT_RESUME
    // 10 = USB_EVENT_RESET
    // 11 = USB_EVENT_SOF
    // 12 = USB_EVENT_SESSION_REQUEST
    // 13 = USB_EVENT_SESSION_VALID
    // 14 = USB_EVENT_ID_STATUS_CHANGE
    // 15 = USB_EVENT_VBUS_REQUEST
    // 16 = USB_EVENT_VBUS_RELEASE
    
    // For now, just return success for all events
    // The USB library handles descriptor requests internally
    0 // Success
}

/// Receive handler callback
#[no_mangle]
pub unsafe extern "C" fn rx_handler(
    _pv_cb_data: *mut c_void,
    _ui32_event: u32,
    _ui32_msg_value: u32,
    _pv_msg_data: *mut c_void,
) -> u32 {
    0 // Success
}

/// Transmit handler callback
#[no_mangle]
pub unsafe extern "C" fn tx_handler(
    _pv_cb_data: *mut c_void,
    _ui32_event: u32,
    _ui32_msg_value: u32,
    _pv_msg_data: *mut c_void,
) -> u32 {
    0 // Success
}

