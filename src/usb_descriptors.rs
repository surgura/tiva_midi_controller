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

// Wrapper type to make raw pointers Sync-safe for embedded use
// Safe because: embedded is single-threaded, pointers are const, only read
struct StringDescriptorPtr(*const u8);
unsafe impl Sync for StringDescriptorPtr {}

// String descriptor table - static const array like in C example
static STRING_DESCRIPTORS: [StringDescriptorPtr; 6] = [
    StringDescriptorPtr(LANG_DESCRIPTOR.as_ptr()),
    StringDescriptorPtr(MANUFACTURER_STRING.as_ptr()),
    StringDescriptorPtr(PRODUCT_STRING.as_ptr()),
    StringDescriptorPtr(SERIAL_STRING.as_ptr()),
    StringDescriptorPtr(CONTROL_INTERFACE_STRING.as_ptr()),
    StringDescriptorPtr(CONFIG_STRING.as_ptr()),
];

// Function to get the raw pointer array for C FFI
pub fn get_string_descriptors() -> [*const u8; 6] {
    [
        STRING_DESCRIPTORS[0].0,
        STRING_DESCRIPTORS[1].0,
        STRING_DESCRIPTORS[2].0,
        STRING_DESCRIPTORS[3].0,
        STRING_DESCRIPTORS[4].0,
        STRING_DESCRIPTORS[5].0,
    ]
}

// USB callback functions
// These are minimal implementations that just return success

/// Control handler callback
pub unsafe extern "C" fn control_handler(
    _pv_cb_data: *mut c_void,
    _ui32_event: u32,
    _ui32_msg_value: u32,
    _pv_msg_data: *mut c_void,
) -> u32 {
    0 // Success
}

/// Receive handler callback
pub unsafe extern "C" fn rx_handler(
    _pv_cb_data: *mut c_void,
    _ui32_event: u32,
    _ui32_msg_value: u32,
    _pv_msg_data: *mut c_void,
) -> u32 {
    0 // Success
}

/// Transmit handler callback
pub unsafe extern "C" fn tx_handler(
    _pv_cb_data: *mut c_void,
    _ui32_event: u32,
    _ui32_msg_value: u32,
    _pv_msg_data: *mut c_void,
) -> u32 {
    0 // Success
}

