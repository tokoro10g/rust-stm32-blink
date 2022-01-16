#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use hal::stm32;
use hal::rcc::Config;
use panic_halt as _;
use stm32g4xx_hal as hal;

use hal::prelude::*;

#[entry]
fn main() -> ! {
    if let (Some(dp), Some(cp)) = (
        stm32::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        let mut rcc = dp.RCC.freeze(Config::hsi());
        let mut delay = cp.SYST.delay(&rcc.clocks);

        let gpioa = dp.GPIOA.split(&mut rcc);
        let mut led = gpioa.pa5.into_push_pull_output();

        loop {
            led.toggle().unwrap();
            delay.delay_ms(500_u32);
        }
    }
    loop {}
}