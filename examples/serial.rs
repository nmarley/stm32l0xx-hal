// #![deny(warnings)]
#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate panic_halt;

use core::fmt::Write;
use cortex_m_rt::entry;
use stm32l0xx_hal::{pac, prelude::*, pwr::PWR, rcc::Config, serial};

use nb::block;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    // Configure the clock.
    let mut rcc = dp.RCC.freeze(Config::hsi16());
    let pwr = PWR::new(dp.PWR, &mut rcc);
    let lse = rcc.enable_lse(&pwr);

    // Acquire the GPIOA peripheral. This also enables the clock for GPIOA in
    // the RCC register.
    let gpioa = dp.GPIOA.split(&mut rcc);
    let gpioc = dp.GPIOC.split(&mut rcc);

    // Choose TX / RX pins
    let tx_pin = gpioa.pa2;
    let rx_pin = gpioa.pa3;

    let lptx_pin = gpioc.pc10;
    let lprx_pin = gpioc.pc11;

    // Configure the serial peripheral.
    let serial = dp
        .USART2
        .usart(tx_pin, rx_pin, serial::Config::default(), &mut rcc)
        .unwrap();
    let (mut tx, mut rx) = serial.split();

    // Configure the OTHER serial peripheral.
    let mut lp_serial = dp
        .LPUART1
        .usart(lptx_pin, lprx_pin, serial::Config::default(), &mut rcc)
        .unwrap();
    lp_serial.use_lse(&mut rcc, &lse);
    let (mut lptx, mut lprx) = lp_serial.split();

    // core::fmt::Write is implemented for tx.
    // writeln!(tx, "Hello, world!").unwrap();
    //
    // writeln!(lptx, "Hi from LPUART!!").unwrap();
    // // let received = block!(rx.read()).unwrap();
    // // block!(tx.write(received)).ok();
    //
    // loop {
    //     // Echo what is received on the serial link.
    //     let received = block!(rx.read()).unwrap();
    //     block!(tx.write(received)).ok();
    // }

    const HIGH_BYTE: u8 = 0xFF;
    loop {
        block!(lptx.write(HIGH_BYTE)).unwrap();
        // // For demonstration, we're just going to send a simple message
        // let message: &[u8] = b"Hello, USART!\r\n";
        // for byte in message {
        //     block!(tx.write(*byte)).unwrap();
        // }
        // // TODO: add logic to read from rx and echo it back or handle it as needed.
    }
}
