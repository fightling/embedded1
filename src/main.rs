#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{clock::ClockControl, gpio::IO, peripherals::Peripherals, prelude::*, Delay};
use esp_println::println;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    let clocks = ClockControl::max(system.clock_control).freeze();
    let mut delay = Delay::new(&clocks);

    let mut led = io.pins.gpio7.into_push_pull_output();

    println!("Hello world!");
    loop {
        println!("Loop...");
        delay.delay_ms(500u32);
        led.set_high().unwrap();
        delay.delay_ms(500u32);
        led.set_low().unwrap();
    }
}
