#![no_std]
#![no_main]

#[cfg(debug_assertions)]
use panic_halt as _;

#[cfg(not(debug_assertions))]
use panic_abort as _;

use cortex_m_rt::entry;
use rtt_target::{rprintln, rtt_init_print};
use stm32f4xx_hal as hal;

use hal::{prelude::*, stm32};

#[entry]
fn main() -> ! {
    rtt_init_print!(BlockIfFull);

    if let (Some(dp), Some(cp)) = (
        stm32::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        // Set up the LED. On the Nucleo-446RE it's connected to pin PA5.
        let gpioc = dp.GPIOC.split();
        let mut led = gpioc.pc13.into_push_pull_output();

        // Set up the system clock. We want to run at 48MHz for this one.
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(48.mhz()).freeze();

        // Create a delay abstraction based on SysTick
        let mut delay = hal::delay::Delay::new(cp.SYST, clocks);

        loop {
            // On for 1s, off for 1s.
            led.set_high().unwrap();
            delay.delay_ms(1000_u32);
            led.set_low().unwrap();
            delay.delay_ms(1000_u32);
        }
    }

    let mut i = 0;
    loop {
        rprintln!("Hello from RTT! {}", i);
        i += 1;

        if i == 42 {
            panic!("Something bad happend!");
        }
    }
}
