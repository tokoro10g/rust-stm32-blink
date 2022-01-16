#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;
use stm32f4xx_hal as hal;

use crate::hal::{pac, prelude::*, delay::Delay};

#[entry]
fn main() -> ! {
    if let (Some(dp), Some(cp)) = (
        pac::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        let gpioa = dp.GPIOA.split();
        let mut led = gpioa.pa5.into_push_pull_output();

        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(84.mhz()).freeze();
        let mut delay = Delay::new(cp.SYST, &clocks);

        loop {
            led.set_high();
            delay.delay_ms(500_u32);
            led.set_low();
            delay.delay_ms(500_u32);
        }
    }
    loop {}
}