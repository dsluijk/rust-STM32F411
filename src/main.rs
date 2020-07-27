#![no_std]
#![no_main]

#[cfg(debug_assertions)]
use panic_halt as _;

#[cfg(not(debug_assertions))]
use panic_abort as _;

use cortex_m_rt::entry;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!(BlockIfFull);

    let mut i = 0;
    loop {
        rprintln!("Hello from RTT! {}", i);
        i += 1;

        if i == 42 {
            panic!("Something bad happend!");
        }
    }
}
