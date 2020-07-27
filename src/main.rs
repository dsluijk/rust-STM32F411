#![no_std]
#![no_main]

use panic_semihosting as _;

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

#[entry]
fn main() -> ! {
    hprintln!("Hello, Cortex 2").unwrap();

    loop {}
}
