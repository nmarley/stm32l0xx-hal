#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use stm32l0xx_hal::{pac, prelude::*, rcc::Config};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Moie, welt! STM32 L053R8T6 pilled?");
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    // Configure the clock.
    let mut rcc = dp.RCC.freeze(Config::hsi16());

    // Get the delay provider.
    let mut delay = cp.SYST.delay(rcc.clocks);

    // Acquire the GPIOA peripheral. This also enables the clock for GPIOA in
    // the RCC register.
    let gpioa = dp.GPIOA.split(&mut rcc);

    // Configure PA5 as output.
    let mut led = gpioa.pa5.into_push_pull_output();

    loop {
        led.set_high().unwrap();
        delay.delay_ms(1000_u16);

        led.set_low().unwrap();
        delay.delay_ms(1000_u16);
    }
}
