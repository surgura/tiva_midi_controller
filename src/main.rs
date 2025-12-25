#![no_std]
#![no_main]
#![allow(static_mut_refs)] // String descriptors are initialized once during USB init

use cortex_m_rt::entry;
use tm4c123x::{GPIO_PORTD, GPIO_PORTF, SYSCTL};
use core::ptr;

// Custom panic handler that blinks LED rapidly (replaces panic_halt)
#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    let sysctl = unsafe { &*SYSCTL::ptr() };
    let portf = unsafe { &*GPIO_PORTF::ptr() };
    
    // Enable GPIO Port F
    sysctl.rcgcgpio.modify(|r, w| unsafe { w.bits(r.bits() | (1 << 5)) });
    while sysctl.prgpio.read().bits() & (1 << 5) == 0 {}
    
    // Configure PF1 as output
    portf.dir.modify(|r, w| unsafe { w.bits(r.bits() | 0x02) });
    portf.den.modify(|r, w| unsafe { w.bits(r.bits() | 0x02) });
    
    // Rapid blink to indicate panic
    loop {
        portf.data.modify(|r, w| unsafe { w.bits(r.bits() ^ 0x02) });
        for _ in 0..100_000 {
            cortex_m::asm::nop();
        }
    }
}

// Load modules AFTER panic handler is set up
mod usb_device;
mod usb_descriptors;

/// USB Device Example
/// Makes the TM4C123 enumerate as a USB CDC serial port when connected to a computer.
#[entry]
fn main() -> ! {
    let sysctl = unsafe { &*SYSCTL::ptr() };
    let portf = unsafe { &*GPIO_PORTF::ptr() };

    // Enable GPIO Port F
    sysctl.rcgcgpio.modify(|r, w| unsafe { w.bits(r.bits() | (1 << 5)) });
    while sysctl.prgpio.read().bits() & (1 << 5) == 0 {}

    // Configure PF1 (red LED) as output
    portf.dir.modify(|r, w| unsafe { w.bits(r.bits() | 0x02) });
    portf.den.modify(|r, w| unsafe { w.bits(r.bits() | 0x02) });
    
    // Enable FPU lazy stacking (required for USB interrupt handlers)
    // This must be done early, before any USB initialization
    unsafe {
        usb_device::FPULazyStackingEnable();
    }
    
    // Configure USB pins
    let portd = unsafe { &*GPIO_PORTD::ptr() };
    sysctl.rcgcgpio.modify(|r, w| unsafe { w.bits(r.bits() | (1 << 3)) });
    while sysctl.prgpio.read().bits() & (1 << 3) == 0 {}
    unsafe {
        portd.dir.modify(|r, w| w.bits(r.bits() & !0x30));
        portd.den.modify(|r, w| w.bits(r.bits() & !0x30));
        portd.amsel.modify(|r, w| w.bits(r.bits() | 0x30));
        usb_device::GPIOPinTypeUSBAnalog(
            usb_device::gpio::GPIO_PORTD_BASE,
            usb_device::gpio::GPIO_PIN_4 | usb_device::gpio::GPIO_PIN_5,
        );
    }

    // Set USB stack mode to device mode (like C example)
    unsafe {
        usb_device::usb_set_device_mode(0);
    }
    
    // Get string descriptors array - must store in variable to avoid dangling pointer
    let string_descriptors = usb_descriptors::get_string_descriptors();
    
    // Create CDC device structure as static (like C example uses global static)
    // This ensures it lives for the lifetime of the program
    static mut CDC_DEVICE: Option<usb_device::tUSBDCDCDevice> = None;
    unsafe {
        CDC_DEVICE = Some(usb_device::tUSBDCDCDevice {
            ui16VID: usb_device::usb_ids::USB_VID_TI_1CBE,
            ui16PID: usb_device::usb_ids::USB_PID_SERIAL,
            ui16MaxPowermA: 0,
            ui8PwrAttributes: usb_device::usb_conf::USB_CONF_ATTR_SELF_PWR,
            pfnControlCallback: Some(usb_descriptors::control_handler),
            pvControlCBData: ptr::null_mut(),
            pfnRxCallback: Some(usb_descriptors::rx_handler),
            pvRxCBData: ptr::null_mut(),
            pfnTxCallback: Some(usb_descriptors::tx_handler),
            pvTxCBData: ptr::null_mut(),
            ppui8StringDescriptors: string_descriptors.as_ptr() as *const *const u8,
            ui32NumStringDescriptors: 6,
            sPrivateData: usb_device::tCDCSerInstance {
                ui32Base: 0,
                ui32Flags: 0,
                ui16USBBase: 0,
                ui16USBEndpoint: 0,
                ui8USBInterface: 0,
                ui8InterfaceData: 0,
            },
        });
    }
    
    // CRITICAL: First call to external C code - USBDCDCInit
    // This calls USBDCDCCompositeInit which then calls USBDCDInit
    // USBDCDInit does hardware initialization (reset, enable clock, PLL, etc.)
    unsafe {
        // Call USBDCDCInit directly like the C example does
        // The C example doesn't disable interrupts or do any special setup
        // USBDCDCInit handles all hardware initialization internally
        // Use static CDC device structure like C example
        let cdc_device_ptr = unsafe { CDC_DEVICE.as_mut().unwrap() };
        let instance = usb_device::USBDCDCInit(0, cdc_device_ptr);
        
        if instance.is_null() {
            // Initialization failed - rapid blink forever
            loop {
                portf.data.modify(|r, w| unsafe { w.bits(r.bits() ^ 0x02) });
                for _ in 0..50_000 {
                    cortex_m::asm::nop();
                }
            }
        }
        
        // Register USB interrupt handler (must be done before connecting)
        usb_device::USBIntRegister(
            usb_device::usb_base::USB0_BASE,
            usb_device::USB0DeviceIntHandler,
        );
        
        // Connect USB device to bus
        usb_device::USBDevConnect(usb_device::usb_base::USB0_BASE);
        
        // Enable global interrupts (like C example does after USBDCDCInit)
        cortex_m::interrupt::enable();
    }

    // Main loop - slow blink = USB initialized, waiting for host
    loop {
        // Blink LED slowly to show code is running and waiting for USB connection
        portf.data.modify(|r, w| unsafe { w.bits(r.bits() ^ 0x02) });
        
        // Delay (~1 second)
        for _ in 0..5_000_000 {
            cortex_m::asm::nop();
        }
    }
}
