#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(panic_info_message)]

use embassy_executor::Spawner;
use embassy_rp::{gpio, uart, i2c};
use embassy_time::{Duration, Timer};
use gpio::{Level, Output};
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};
use core::panic::PanicInfo;
use heapless::String;

#[panic_handler]
fn panic(info :&PanicInfo) -> !{
    // conjure a new copy of the peripherals - we've already panic'd so no-one should mind
    let p: embassy_rp::Peripherals = unsafe {core::mem::transmute(())};
    // set the LED high. hopefully this is a good enough sign that things are not good
    let mut led = Output::new(p.PIN_25, Level::High);
    let config = uart::Config::default();
    let mut uart = uart::Uart::new_blocking(p.UART0, p.PIN_0, p.PIN_1, config);
    use core::fmt::Write;
    let mut text: String<1024> = String::new();
    if let Some(message) = info.message() {
        let _ = writeln!(&mut text, "{}\r\n", message);
        let _ = uart.blocking_write(text.as_bytes());
        let _ = uart.blocking_flush();
    }
    loop{
        cortex_m::asm::delay(10000000);
        led.set_low();
        cortex_m::asm::delay(10000000);
        led.set_high();
    }
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let config = uart::Config::default();
    let mut uart = uart::Uart::new_blocking(p.UART0, p.PIN_0, p.PIN_1, config);
    let mut led = Output::new(p.PIN_25, Level::Low);

    uart.blocking_write("Program start!\r\n".as_bytes())
        .unwrap();

    // panic!("at the disco");

    let scl = p.PIN_9;
    let sda = p.PIN_8;
    let mut config = i2c::Config::default();
    config.frequency = 400_000;
    let i2c = i2c::I2c::new_blocking(p.I2C0, scl, sda, config);

    // let interface = I2CDisplayInterface::new(i2c);
    // let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0).into_terminal_mode();
    //display.init().unwrap();
    // let _ = display.print_char('a');
    loop {
        led.set_high();
        uart.blocking_write("LED on\r\n".as_bytes()).unwrap();
        Timer::after(Duration::from_secs(1)).await;

        led.set_low();
        uart.blocking_write("LED off\r\n".as_bytes()).unwrap();
        Timer::after(Duration::from_secs(1)).await;
    }
}
