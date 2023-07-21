#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use embassy_executor::Spawner;
use embassy_rp::gpio;
use embassy_rp::uart;
use embassy_time::{Duration, Timer};
use gpio::{Level, Output};
use panic_probe as _;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let config = uart::Config::default();
    let mut uart = uart::Uart::new_blocking(p.UART0, p.PIN_0, p.PIN_1, config);
    let mut led = Output::new(p.PIN_25, Level::Low);
    uart.blocking_write("Program start!\r\n".as_bytes())
        .unwrap();

    loop {
        led.set_high();
        uart.blocking_write("LED on\r\n".as_bytes()).unwrap();
        Timer::after(Duration::from_secs(1)).await;

        led.set_low();
        uart.blocking_write("LED off\r\n".as_bytes()).unwrap();
        Timer::after(Duration::from_secs(1)).await;
    }
}
