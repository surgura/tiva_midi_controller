//! Minimal Tiva C Series TM4C123GXL LaunchPad Example
//! Blinks the onboard RGB LED (Red on PF1)

#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use tm4c123x::{SYSCTL, GPIO_PORTF};

/// LED pins on Port F
const LED_RED: u8 = 1 << 1;   // PF1
const LED_BLUE: u8 = 1 << 2;  // PF2
const LED_GREEN: u8 = 1 << 3; // PF3

/// Simple delay function
#[inline(never)]
fn delay(count: u32) {
    for _ in 0..count {
        cortex_m::asm::nop();
    }
}

#[entry]
fn main() -> ! {
    // Take ownership of peripherals
    let sysctl = unsafe { &*SYSCTL::ptr() };
    let gpio_portf = unsafe { &*GPIO_PORTF::ptr() };

    // Enable clock for GPIO Port F (bit 5)
    sysctl.rcgcgpio.modify(|r, w| unsafe { w.bits(r.bits() | (1 << 5)) });

    // Wait for peripheral to be ready
    while sysctl.prgpio.read().bits() & (1 << 5) == 0 {}

    // Configure PF1, PF2, PF3 as outputs (RGB LED)
    gpio_portf.dir.modify(|r, w| unsafe {
        w.bits(r.bits() | (LED_RED | LED_BLUE | LED_GREEN) as u32)
    });

    // Enable digital function for the LED pins
    gpio_portf.den.modify(|r, w| unsafe {
        w.bits(r.bits() | (LED_RED | LED_BLUE | LED_GREEN) as u32)
    });

    // Main loop - blink red LED
    loop {
        // Toggle red LED
        gpio_portf.data.modify(|r, w| unsafe {
            w.bits(r.bits() ^ LED_RED as u32)
        });
        delay(500_000);
    }
}

