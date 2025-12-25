#![no_std]
#![no_main]
#![allow(static_mut_refs)] // String descriptors are initialized once during USB init

use cortex_m_rt::entry;
use tm4c123x::{GPIO_PORTD, GPIO_PORTF, SYSCTL};
use core::ptr;
use core::ffi::c_void;

// HardFault handler to debug crashes
#[exception]
unsafe fn HardFault(ef: &cortex_m_rt::ExceptionFrame) -> ! {
    let sysctl = unsafe { &*SYSCTL::ptr() };
    let portf = unsafe { &*GPIO_PORTF::ptr() };
    
    // Enable GPIO Port F
    sysctl.rcgcgpio.modify(|r, w| unsafe { w.bits(r.bits() | (1 << 5)) });
    while sysctl.prgpio.read().bits() & (1 << 5) == 0 {}
    
    // Configure PF1 as output
    portf.dir.modify(|r, w| unsafe { w.bits(r.bits() | 0x02) });
    portf.den.modify(|r, w| unsafe { w.bits(r.bits() | 0x02) });
    
    // Blink LED rapidly to indicate HardFault
    // The fault address is in ef.fault_address, PC is in ef.pc
    loop {
        portf.data.modify(|r, w| unsafe { w.bits(r.bits() ^ 0x02) });
        for _ in 0..10_000 {
            cortex_m::asm::nop();
        }
    }
}

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

use cortex_m_rt::exception;

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
    
    // Initialize LED to OFF
    portf.data.modify(|r, w| unsafe { w.bits(r.bits() & !0x02) });
    
    // Enable FPU lazy stacking (required for USB interrupt handlers)
    // This must be done early, before any USB initialization
    unsafe {
        usb_device::FPULazyStackingEnable();
    }
    
    // Set the clocking to run from the PLL at 50MHz (like C example)
    // This is critical for USB to work properly
    unsafe {
        usb_device::SysCtlClockSet(
            usb_device::sysctl_clock::SYSCTL_SYSDIV_4 |
            usb_device::sysctl_clock::SYSCTL_USE_PLL |
            usb_device::sysctl_clock::SYSCTL_OSC_MAIN |
            usb_device::sysctl_clock::SYSCTL_XTAL_16MHZ
        );
    }
    
    // Enable the system tick (like C example)
    // USB library may use this for timing
    unsafe {
        let sys_clock = usb_device::SysCtlClockGet();
        usb_device::SysTickPeriodSet(sys_clock / 100); // 100 ticks per second
        usb_device::SysTickIntEnable();
        usb_device::SysTickEnable();
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
    
    // Get pointer to static string descriptor array
    // This is safe because the array is static and lives for the entire program lifetime
    let string_descriptors_ptr = usb_descriptors::get_string_descriptors();
    
    // Create CDC device structure as static (like C example uses global static)
    // This ensures it lives for the lifetime of the program
    static mut CDC_DEVICE: Option<usb_device::tUSBDCDCDevice> = None;
    unsafe {
        // Initialize the structure first
        CDC_DEVICE = Some(usb_device::tUSBDCDCDevice {
            ui16VID: usb_device::usb_ids::USB_VID_TI_1CBE,
            ui16PID: usb_device::usb_ids::USB_PID_SERIAL,
            ui16MaxPowermA: 0,
            ui8PwrAttributes: usb_device::usb_conf::USB_CONF_ATTR_SELF_PWR,
            pfnControlCallback: Some(usb_descriptors::control_handler),
            pvControlCBData: ptr::null_mut(), // Will set to &CDC_DEVICE after initialization
            pfnRxCallback: Some(usb_descriptors::rx_handler),
            pvRxCBData: ptr::null_mut(),
            pfnTxCallback: Some(usb_descriptors::tx_handler),
            pvTxCBData: ptr::null_mut(),
            ppui8StringDescriptors: string_descriptors_ptr,
            ui32NumStringDescriptors: 6,
            // sPrivateData will be fully initialized by USBDCDCInit
            // We just need to zero-initialize it to ensure proper layout
            sPrivateData: unsafe { core::mem::zeroed() },
        });
        
        // Set callback data to point to the CDC device structure (like C example)
        if let Some(device) = CDC_DEVICE.as_mut() {
            device.pvControlCBData = device as *mut _ as *mut c_void;
            device.pvRxCBData = device as *mut _ as *mut c_void;
            device.pvTxCBData = device as *mut _ as *mut c_void;
        }
    }
    
    // CRITICAL: First call to external C code - USBDCDCInit
    // Note: The C example doesn't call USBIntRegister explicitly - it relies on
    // the interrupt vector table in the startup file. We call it here to ensure
    // the handler is registered in the RAM vector table before USBDCDInit enables interrupts.
    unsafe {
        usb_device::USBIntRegister(
            usb_device::usb_base::USB0_BASE,
            usb_device::USB0DeviceIntHandler,
        );
    }
    
    // CRITICAL: First call to external C code - USBDCDCInit
    // This calls USBDCDCCompositeInit which then calls USBDCDInit
    // USBDCDInit does hardware initialization (reset, enable clock, PLL, etc.)
    // and calls USBDevConnect internally
    unsafe {
        // Call USBDCDCInit directly like the C example does
        // The C example doesn't disable interrupts or do any special setup
        // USBDCDCInit handles all hardware initialization internally
        // Use static CDC device structure like C example
        let cdc_device_ptr = CDC_DEVICE.as_mut().unwrap();
        let instance = usb_device::USBDCDCInit(0, cdc_device_ptr);
        
        if instance.is_null() {
            // Initialization failed - rapid blink forever
            loop {
                portf.data.modify(|r, w| w.bits(r.bits() ^ 0x02));
                for _ in 0..50_000 {
                    cortex_m::asm::nop();
                }
            }
        }
        
        // Enable global interrupts (like C example does after USBDCDCInit)
        // USBDCDInit already calls USBDevConnect and enables USB interrupt at NVIC
        // NOTE: Interrupts must be enabled BEFORE entering main loop
        // so USB interrupts can be processed
        cortex_m::interrupt::enable();
    }
    
    // Small delay to allow USB to stabilize
    for _ in 0..1_000_000 {
        cortex_m::asm::nop();
    }

    // Main loop - slow blink = USB initialized, waiting for host
    loop {
        // Blink LED slowly to show code is running and waiting for USB connection
        // Turn ON
        portf.data.modify(|r, w| unsafe { w.bits(r.bits() | 0x02) });
        
        // Delay (~1 second at 50MHz)
        for _ in 0..5_000_000 {
            cortex_m::asm::nop();
        }
        
        // Turn OFF
        portf.data.modify(|r, w| unsafe { w.bits(r.bits() & !0x02) });
        
        // Delay (~1 second at 50MHz)
        for _ in 0..5_000_000 {
            cortex_m::asm::nop();
        }
    }
}

// SysTick interrupt handler (required when SysTick interrupts are enabled)
// The USB library may use SysTick for timing
#[exception]
#[allow(non_snake_case)]
fn SysTick() {
    // Minimal handler - just acknowledge the interrupt
    // The C example increments a counter here, but we don't need it
}

// Note: USB interrupt handler is registered via USBIntRegister() call
// The TivaWare library handles the interrupt vector setup internally
// USB0DeviceIntHandler is called automatically when USB interrupts occur
