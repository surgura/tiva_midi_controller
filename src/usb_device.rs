//! USB Device Module
//! 
//! This module provides USB device functionality using FFI bindings to TivaWare.
//! The microcontroller will enumerate as a USB device when connected to a host.
//!
//! The C files remain in their original TivaWare location and are compiled
//! as part of the Rust build process via build.rs.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use core::ffi::c_void;

// ============================================================================
// Type Definitions
// ============================================================================

/// USB mode enumeration
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum tUSBMode {
    None = 0,
    Device = 1,
    Host = 2,
    OTG = 3,
    ForceDevice = 4,
    ForceHost = 5,
}

/// USB request structure (matches tUSBRequest from usblib.h)
#[repr(C, packed)]
pub struct tUSBRequest {
    pub bmRequestType: u8,
    pub bRequest: u8,
    pub wValue: u16,
    pub wIndex: u16,
    pub wLength: u16,
}

/// Device descriptor header (matches tDescriptorHeader from usblib.h)
#[repr(C, packed)]
pub struct tDescriptorHeader {
    pub bLength: u8,
    pub bDescriptorType: u8,
}

/// Configuration header (matches tConfigHeader from usbdevice.h)
#[repr(C)]
pub struct tConfigHeader {
    pub ui16TotalSize: u16,
    pub ui16NumInterfaces: u8,
    pub ui8ConfigIndex: u8,
}

/// Custom handlers structure (matches tCustomHandlers from usbdevice.h)
#[repr(C)]
pub struct tCustomHandlers {
    pub pfnGetDescriptor: Option<unsafe extern "C" fn(*const c_void, *mut tUSBRequest, *const u8, *mut u32) -> u32>,
    pub pfnRequestHandler: Option<unsafe extern "C" fn(*const c_void, *mut tUSBRequest)>,
    pub pfnConfigChange: Option<unsafe extern "C" fn(*const c_void, u32)>,
    pub pfnDataReceived: Option<unsafe extern "C" fn(*const c_void, u32, u32)>,
    pub pfnDataSent: Option<unsafe extern "C" fn(*const c_void, u32, u32)>,
    pub pfnResetHandler: Option<unsafe extern "C" fn(*const c_void)>,
    pub pfnSuspendHandler: Option<unsafe extern "C" fn(*const c_void)>,
    pub pfnResumeHandler: Option<unsafe extern "C" fn(*const c_void)>,
    pub pfnDisconnectHandler: Option<unsafe extern "C" fn(*const c_void)>,
    pub pfnConfigDescGet: Option<unsafe extern "C" fn(*const c_void, *const tConfigHeader, u32, u32, *mut u32) -> *const tDescriptorHeader>,
}

/// Device info structure (matches tDeviceInfo from usbdevice.h)
#[repr(C)]
pub struct tDeviceInfo {
    pub psCallbacks: *const tCustomHandlers,
    pub pui8DeviceDescriptor: *const u8,
    pub ppsConfigDescriptors: *const *const tConfigHeader,
    pub ppui8StringDescriptors: *const *const u8,
    pub ui32NumStringDescriptors: u32,
}

/// Composite entry structure (matches tCompositeEntry from usbdevice.h)
#[repr(C)]
pub struct tCompositeEntry {
    pub psDevInfo: *const tDeviceInfo,
    pub pvInstance: *mut c_void,
    pub ui32DeviceWorkspace: u32,
}

// ============================================================================
// USB Device Control Driver (USBDCD) Functions
// ============================================================================

extern "C" {
    /// Initialize the USB device controller driver
    /// 
    /// # Arguments
    /// * `ui32Index` - USB controller index (typically 0)
    /// * `psDevice` - Pointer to device information structure
    /// * `pvDCDCBData` - Callback data pointer
    pub fn USBDCDInit(
        ui32Index: u32,
        psDevice: *const tDeviceInfo,
        pvDCDCBData: *mut c_void,
    );

    /// Terminate the USB device controller driver
    pub fn USBDCDTerm(ui32Index: u32);

    /// Stall endpoint 0
    pub fn USBDCDStallEP0(ui32Index: u32);

    /// Request data on endpoint 0
    pub fn USBDCDRequestDataEP0(
        ui32Index: u32,
        pui8Data: *mut u8,
        ui32Size: u32,
    );

    /// Send data on endpoint 0
    pub fn USBDCDSendDataEP0(
        ui32Index: u32,
        pui8Data: *const u8,
        ui32Size: u32,
    );

    /// Set default configuration
    pub fn USBDCDSetDefaultConfiguration(
        ui32Index: u32,
        ui32DefaultConfig: u32,
    );

    /// Get configuration descriptor size
    pub fn USBDCDConfigDescGetSize(psConfig: *const tConfigHeader) -> u32;

    /// Get configuration descriptor count
    pub fn USBDCDConfigDescGetNum(
        psConfig: *const tConfigHeader,
        ui32Type: u32,
    ) -> u32;

    /// Request remote wakeup
    pub fn USBDCDRemoteWakeupRequest(ui32Index: u32) -> bool;

    /// Set USB feature
    pub fn USBDCDFeatureSet(
        ui32Index: u32,
        ui32Feature: u32,
        pvFeature: *mut c_void,
    ) -> bool;

    /// Get USB feature
    pub fn USBDCDFeatureGet(
        ui32Index: u32,
        ui32Feature: u32,
        pvFeature: *mut c_void,
    ) -> bool;

    /// USB device interrupt handler for controller 0
    pub fn USB0DeviceIntHandler();
}

// ============================================================================
// USB CDC Serial Functions
// ============================================================================

/// USB callback function type (matches tUSBCallback from usblib.h)
pub type tUSBCallback = Option<unsafe extern "C" fn(*mut c_void, u32, u32, *mut c_void) -> u32>;

/// CDC state enumeration (matches tCDCState from usbdcdc.h)
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum tCDCState {
    Idle = 0,
    WaitingOnSendData = 1,
    WaitingOnReceiveData = 2,
}

/// Line coding structure (matches tLineCoding from usbdcdc.h)
#[repr(C)]
pub struct tLineCoding {
    pub ui32Rate: u32,
    pub ui8CharFormat: u8,
    pub ui8ParityType: u8,
    pub ui8DataBits: u8,
}

/// CDC serial instance (matches tCDCSerInstance from usbdcdc.h)
/// NOTE: This structure is large and contains internal state managed by TivaWare.
/// The C code initializes all fields, so we only need to ensure correct layout.
#[repr(C)]
pub struct tCDCSerInstance {
    pub ui32USBBase: u32,                    // Offset 0
    pub sDevInfo: tDeviceInfo,               // Offset 4 - CRITICAL: This is where callbacks are stored!
    pub iCDCRxState: tCDCState,              // Offset varies
    pub iCDCTxState: tCDCState,
    pub iCDCRequestState: tCDCState,
    pub iCDCInterruptState: tCDCState,
    pub ui8PendingRequest: u8,
    pub ui16BreakDuration: u16,
    pub ui16ControlLineState: u16,
    pub ui16SerialState: u16,
    pub ui16DeferredOpFlags: u16,
    pub ui16LastTxSize: u16,
    pub sLineCoding: tLineCoding,
    pub bRxBlocked: bool,
    pub bControlBlocked: bool,
    pub bConnected: bool,
    pub ui8ControlEndpoint: u8,
    pub ui8BulkINEndpoint: u8,
    pub ui8BulkOUTEndpoint: u8,
    pub ui8InterfaceControl: u8,
    pub ui8InterfaceData: u8,
}

/// USB CDC device structure (matches tUSBDCDCDevice from usbdcdc.h)
#[repr(C)]
pub struct tUSBDCDCDevice {
    pub ui16VID: u16,
    pub ui16PID: u16,
    pub ui16MaxPowermA: u16,
    pub ui8PwrAttributes: u8,
    pub pfnControlCallback: tUSBCallback,
    pub pvControlCBData: *mut c_void,
    pub pfnRxCallback: tUSBCallback,
    pub pvRxCBData: *mut c_void,
    pub pfnTxCallback: tUSBCallback,
    pub pvTxCBData: *mut c_void,
    pub ppui8StringDescriptors: *const *const u8,
    pub ui32NumStringDescriptors: u32,
    pub sPrivateData: tCDCSerInstance,
}

extern "C" {
    /// Initialize USB CDC device
    /// 
    /// # Returns
    /// Pointer to CDC device instance or NULL on error
    pub fn USBDCDCInit(
        ui32Index: u32,
        psCDCDevice: *mut tUSBDCDCDevice,
    ) -> *mut c_void;

    /// Initialize USB CDC device for composite device
    pub fn USBDCDCCompositeInit(
        ui32Index: u32,
        psCDCDevice: *mut tUSBDCDCDevice,
        psCompEntry: *mut tCompositeEntry,
    ) -> *mut c_void;

    /// Terminate USB CDC device
    pub fn USBDCDCTerm(pvCDCDevice: *mut c_void);

    /// Set control callback data
    pub fn USBDCDCSetControlCBData(
        pvCDCDevice: *mut c_void,
        pvCBData: *mut c_void,
    ) -> *mut c_void;

    /// Set receive callback data
    pub fn USBDCDCSetRxCBData(
        pvCDCDevice: *mut c_void,
        pvCBData: *mut c_void,
    ) -> *mut c_void;

    /// Set transmit callback data
    pub fn USBDCDCSetTxCBData(
        pvCDCDevice: *mut c_void,
        pvCBData: *mut c_void,
    ) -> *mut c_void;

    /// Write packet to CDC device
    pub fn USBDCDCPacketWrite(
        pvCDCDevice: *mut c_void,
        pi8Data: *const u8,
        ui32Length: u32,
        bLast: bool,
    ) -> u32;

    /// Read packet from CDC device
    pub fn USBDCDCPacketRead(
        pvCDCDevice: *mut c_void,
        pi8Data: *mut u8,
        ui32Length: u32,
        bLast: bool,
    ) -> u32;

    /// Get available transmit packet space
    pub fn USBDCDCTxPacketAvailable(pvCDCDevice: *mut c_void) -> u32;

    /// Get available receive packets
    pub fn USBDCDCRxPacketAvailable(pvCDCDevice: *mut c_void) -> u32;

    /// Notify serial state change
    pub fn USBDCDCSerialStateChange(
        pvCDCDevice: *mut c_void,
        ui16State: u16,
    );

    /// Request remote wakeup
    pub fn USBDCDCRemoteWakeupRequest(pvCDCDevice: *mut c_void) -> bool;
}

// ============================================================================
// USB Library Functions
// ============================================================================

extern "C" {
    /// Set USB stack mode
    pub fn USBStackModeSet(
        ui32Index: u32,
        eMode: tUSBMode,
        pfnCallback: Option<unsafe extern "C" fn(*mut c_void, u32, u32)>,
    );
}

// ============================================================================
// Constants
// ============================================================================

/// USB descriptor types
pub mod usb_descriptor_types {
    pub const USB_DTYPE_DEVICE: u8 = 1;
    pub const USB_DTYPE_CONFIGURATION: u8 = 2;
    pub const USB_DTYPE_STRING: u8 = 3;
    pub const USB_DTYPE_INTERFACE: u8 = 4;
    pub const USB_DTYPE_ENDPOINT: u8 = 5;
}

/// USB request types
pub mod usb_request_types {
    pub const USB_RTYPE_DIR_IN: u8 = 0x80;
    pub const USB_RTYPE_DIR_OUT: u8 = 0x00;
    pub const USB_RTYPE_STANDARD: u8 = 0x00;
    pub const USB_RTYPE_CLASS: u8 = 0x20;
    pub const USB_RTYPE_VENDOR: u8 = 0x40;
    pub const USB_RTYPE_DEVICE: u8 = 0x00;
    pub const USB_RTYPE_INTERFACE: u8 = 0x01;
    pub const USB_RTYPE_ENDPOINT: u8 = 0x02;
}

/// USB standard requests
pub mod usb_requests {
    pub const USBREQ_GET_STATUS: u8 = 0x00;
    pub const USBREQ_CLEAR_FEATURE: u8 = 0x01;
    pub const USBREQ_SET_FEATURE: u8 = 0x03;
    pub const USBREQ_SET_ADDRESS: u8 = 0x05;
    pub const USBREQ_GET_DESCRIPTOR: u8 = 0x06;
    pub const USBREQ_SET_DESCRIPTOR: u8 = 0x07;
    pub const USBREQ_GET_CONFIG: u8 = 0x08;
    pub const USBREQ_SET_CONFIG: u8 = 0x09;
    pub const USBREQ_GET_INTERFACE: u8 = 0x0a;
    pub const USBREQ_SET_INTERFACE: u8 = 0x0b;
}

/// USB feature selectors
pub mod usb_features {
    pub const USB_FEATURE_EP_HALT: u16 = 0x0000;
    pub const USB_FEATURE_REMOTE_WAKE: u16 = 0x0001;
}

/// USB library features
pub mod usblib_features {
    pub const USBLIB_FEATURE_POWER: u32 = 0;
    pub const USBLIB_FEATURE_ULPI: u32 = 1;
    pub const USBLIB_FEATURE_ULPI_HS: u32 = 2;
    pub const USBLIB_FEATURE_CPUCLK: u32 = 3;
    pub const USBLIB_FEATURE_USBPLL: u32 = 4;
}

/// USB CDC serial state flags
pub mod usb_cdc_serial_state {
    pub const USB_CDC_SERIAL_STATE_DCD: u16 = 0x0001;
    pub const USB_CDC_SERIAL_STATE_DSR: u16 = 0x0002;
    pub const USB_CDC_SERIAL_STATE_BREAK: u16 = 0x0004;
    pub const USB_CDC_SERIAL_STATE_RING: u16 = 0x0008;
    pub const USB_CDC_SERIAL_STATE_FRAMING: u16 = 0x0010;
    pub const USB_CDC_SERIAL_STATE_PARITY: u16 = 0x0020;
    pub const USB_CDC_SERIAL_STATE_OVERRUN: u16 = 0x0040;
    pub const USB_CDC_SERIAL_STATE_TXCARRIER: u16 = 0x0100;
    pub const USB_CDC_SERIAL_STATE_RXCARRIER: u16 = 0x0200;
}

// ============================================================================
// Safe Wrapper Functions
// ============================================================================

/// Initialize USB device controller
/// 
/// # Safety
/// This function is unsafe because it calls C code that may have side effects.
/// The device info structure must be valid and remain valid for the lifetime
/// of the USB device.
pub unsafe fn usb_device_init(
    index: u32,
    device_info: *const tDeviceInfo,
    callback_data: *mut c_void,
) {
    USBDCDInit(index, device_info, callback_data);
}

/// USB device interrupt handler
/// 
/// This should be called from the USB interrupt handler.
/// 
/// # Safety
/// This function is unsafe because it handles hardware interrupts.
pub unsafe fn usb_device_interrupt_handler() {
    USB0DeviceIntHandler();
}

/// Set USB stack to device mode
/// 
/// # Safety
/// This function is unsafe because it modifies global USB state.
pub unsafe fn usb_set_device_mode(index: u32) {
    USBStackModeSet(index, tUSBMode::ForceDevice, None);
}

// ============================================================================
// GPIO and System Control Functions
// ============================================================================

extern "C" {
    /// Enable a peripheral
    pub fn SysCtlPeripheralEnable(ui32Peripheral: u32);
    
    /// Reset a peripheral
    pub fn SysCtlPeripheralReset(ui32Peripheral: u32);
    
    /// Enable USB PLL
    pub fn SysCtlUSBPLLEnable();
    
    /// Set system clock configuration
    pub fn SysCtlClockSet(ui32Config: u32);
    
    /// Get system clock frequency
    pub fn SysCtlClockGet() -> u32;
    
    /// Configure GPIO pins for USB analog mode
    pub fn GPIOPinTypeUSBAnalog(ui32Port: u32, ui8Pins: u8);
    
    /// Register USB interrupt handler
    pub fn USBIntRegister(ui32Base: u32, pfnHandler: unsafe extern "C" fn());
    
    /// Connect USB device to bus (soft connect)
    pub fn USBDevConnect(ui32Base: u32);
    
    /// Disconnect USB device from bus
    pub fn USBDevDisconnect(ui32Base: u32);
    
    /// Set USB to device mode
    pub fn USBDevMode(ui32Base: u32);
    
    /// Enable USB clock
    pub fn USBClockEnable(ui32Base: u32, ui32Div: u32, ui32Flags: u32);
    
    /// Enable FPU lazy stacking for interrupt handlers
    pub fn FPULazyStackingEnable();
    
    /// Set SysTick period
    pub fn SysTickPeriodSet(ui32Period: u32);
    
    /// Enable SysTick interrupt
    pub fn SysTickIntEnable();
    
    /// Enable SysTick counter
    pub fn SysTickEnable();
}

// Constants for GPIO and System Control
pub mod sysctl_periph {
    pub const SYSCTL_PERIPH_GPIOD: u32 = 0x00000008;
    pub const SYSCTL_PERIPH_USB0: u32 = 0x01000010;
}

// System clock configuration constants (from sysctl.h)
pub mod sysctl_clock {
    pub const SYSCTL_SYSDIV_4: u32 = 0x01C00000;  // Processor clock is osc/pll /4
    pub const SYSCTL_USE_PLL: u32 = 0x00000000;   // System clock is the PLL clock
    pub const SYSCTL_OSC_MAIN: u32 = 0x00000000;  // Osc source is main osc
    pub const SYSCTL_XTAL_16MHZ: u32 = 0x00000540; // External crystal is 16 MHz
}

pub mod gpio {
    pub const GPIO_PORTD_BASE: u32 = 0x40007000;
    pub const GPIO_PIN_4: u8 = 0x10;
    pub const GPIO_PIN_5: u8 = 0x20;
}

pub mod usb_ids {
    pub const USB_VID_TI_1CBE: u16 = 0x1cbe;
    pub const USB_PID_SERIAL: u16 = 0x0002;
}

pub mod usb_conf {
    pub const USB_CONF_ATTR_SELF_PWR: u8 = 0xC0;
}

pub mod usb_base {
    pub const USB0_BASE: u32 = 0x40050000;
}
