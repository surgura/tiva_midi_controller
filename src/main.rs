#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use tm4c123x::{GPIO_PORTF, SYSCTL};

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

    loop {
        portf.data.modify(|r, w| unsafe { w.bits(r.bits() ^ 0x02) });
        for _ in 0..500_000 {
            cortex_m::asm::nop();
        }
    }
}
