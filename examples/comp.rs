//! Comparator example
#![no_main]
#![no_std]

// extern crate panic_semihosting;

use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use stm32l0xx_hal::{
    exti::{Exti, ExtiLine, GpioLine, TriggerEdge},
    pac::{
        self,
        syscfg::comp1_csr::{self, COMP1EN_A, COMP1INNSEL_A},
    },
    prelude::*,
    pwr::{self, PWR},
    rcc::{self, Enable, Reset},
    syscfg::SYSCFG,
};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("COMP1 example, entered main");
    let dp = pac::Peripherals::take().unwrap();
    let cp = pac::CorePeripherals::take().unwrap();

    let mut rcc = dp.RCC.freeze(rcc::Config::hsi16());
    let mut delay = cp.SYST.delay(rcc.clocks);

    let gpioa = dp.GPIOA.split(&mut rcc);
    let gpioc = dp.GPIOC.split(&mut rcc);

    let mut exti = Exti::new(dp.EXTI);
    let mut pwr = PWR::new(dp.PWR, &mut rcc);

    // User button and LED on NUCLEO-L053R8 board
    let button = gpioc.pc13.into_floating_input();
    let mut led = gpioa.pa5.into_push_pull_output();

    rprintln!(
        "comp1_is_enabled: {}",
        dp.SYSCFG.comp1_csr.read().comp1en().is_enabled()
    );

    pac::SYSCFG::enable(&mut rcc);
    pac::SYSCFG::reset(&mut rcc);

    // set up comparator inputs and behaviour
    dp.SYSCFG
        .comp1_csr
        .modify(|_, w| w.comp1en().variant(COMP1EN_A::Enabled));

    rprintln!(
        "comp1_is_enabled: {}",
        dp.SYSCFG.comp1_csr.read().comp1en().is_enabled()
    );

    // issue here
    let mut syscfg = SYSCFG::new(dp.SYSCFG, &mut rcc);

    let line = GpioLine::from_raw_line(button.pin_number()).unwrap();
    exti.listen_gpio(&mut syscfg, button.port(), line, TriggerEdge::Falling);

    loop {
        // exti.wait_for_irq(
        //     line,
        //     pwr.stop_mode(
        //         &mut scb,
        //         &mut rcc,
        //         pwr::StopModeConfig {
        //             ultra_low_power: true,
        //         },
        //     ),
        // );

        rprintln!(
            "comp1_is_enabled: {}",
            syscfg.syscfg.comp1_csr.read().comp1en().is_enabled()
        );
        led.set_high().unwrap();
        delay.delay_ms(1000_u32);
        led.set_low().unwrap();
    }
}
