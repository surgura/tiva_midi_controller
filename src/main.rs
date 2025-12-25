#![no_std]
#![no_main]
#![allow(static_mut_refs)] // String descriptors are initialized once during USB init

use cortex_m_rt::entry;
use tm4c123x::{GPIO_PORTD, GPIO_PORTF, SYSCTL};
use core::ptr;
use core::ffi::c_void;

// HardFault handler - blinks red LED on crash
#[exception]
unsafe fn HardFault(_ef: &cortex_m_rt::ExceptionFrame) -> ! {
    let sysctl = unsafe { &*SYSCTL::ptr() };
    let portf = unsafe { &*GPIO_PORTF::ptr() };
    
    sysctl.rcgcgpio.modify(|r, w| unsafe { w.bits(r.bits() | (1 << 5)) });
    while sysctl.prgpio.read().bits() & (1 << 5) == 0 {}
    
    portf.dir.modify(|r, w| unsafe { w.bits(r.bits() | 0x02) });
    portf.den.modify(|r, w| unsafe { w.bits(r.bits() | 0x02) });
    
    loop {
        portf.data.modify(|r, w| unsafe { w.bits(r.bits() ^ 0x02) });
        for _ in 0..10_000 {
            cortex_m::asm::nop();
        }
    }
}

// Panic handler - blinks red LED on panic
#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    let sysctl = unsafe { &*SYSCTL::ptr() };
    let portf = unsafe { &*GPIO_PORTF::ptr() };
    
    sysctl.rcgcgpio.modify(|r, w| unsafe { w.bits(r.bits() | (1 << 5)) });
    while sysctl.prgpio.read().bits() & (1 << 5) == 0 {}
    
    portf.dir.modify(|r, w| unsafe { w.bits(r.bits() | 0x02) });
    portf.den.modify(|r, w| unsafe { w.bits(r.bits() | 0x02) });
    
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

    // Configure PF2 (blue LED) as output
    portf.dir.modify(|r, w| unsafe { w.bits(r.bits() | 0x04) });
    portf.den.modify(|r, w| unsafe { w.bits(r.bits() | 0x04) });
    
    // Initialize LED to OFF
    portf.data.modify(|r, w| unsafe { w.bits(r.bits() & !0x04) });
    
    unsafe {
        usb_device::FPULazyStackingEnable();
        usb_device::SysCtlClockSet(
            usb_device::sysctl_clock::SYSCTL_SYSDIV_4 |
            usb_device::sysctl_clock::SYSCTL_USE_PLL |
            usb_device::sysctl_clock::SYSCTL_OSC_MAIN |
            usb_device::sysctl_clock::SYSCTL_XTAL_16MHZ
        );
        let sys_clock = usb_device::SysCtlClockGet();
        usb_device::SysTickPeriodSet(sys_clock / 100);
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
    
    let string_descriptors_ptr = usb_descriptors::get_string_descriptors();
    
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
            ppui8StringDescriptors: string_descriptors_ptr,
            ui32NumStringDescriptors: 6,
            sPrivateData: core::mem::zeroed(),
        });
        
        if let Some(device) = CDC_DEVICE.as_mut() {
            device.pvControlCBData = device as *mut _ as *mut c_void;
            device.pvRxCBData = device as *mut _ as *mut c_void;
            device.pvTxCBData = device as *mut _ as *mut c_void;
        }
        
        usb_device::USBIntRegister(
            usb_device::usb_base::USB0_BASE,
            usb_device::USB0DeviceIntHandler,
        );
        
        let cdc_device_ptr = CDC_DEVICE.as_mut().unwrap();
        let instance = usb_device::USBDCDCInit(0, cdc_device_ptr);
        
        if instance.is_null() {
            loop {
                portf.data.modify(|r, w| w.bits(r.bits() ^ 0x02));
                for _ in 0..50_000 {
                    cortex_m::asm::nop();
                }
            }
        }
        
        cortex_m::interrupt::enable();
    }
    
    for _ in 0..1_000_000 {
        cortex_m::asm::nop();
    }

    loop {
        portf.data.modify(|r, w| unsafe { w.bits(r.bits() | 0x04) });
        for _ in 0..5_000_000 {
            cortex_m::asm::nop();
        }
        portf.data.modify(|r, w| unsafe { w.bits(r.bits() & !0x04) });
        for _ in 0..5_000_000 {
            cortex_m::asm::nop();
        }
    }
}

#[exception]
#[allow(non_snake_case)]
fn SysTick() {
    // Acknowledge SysTick interrupt
}
