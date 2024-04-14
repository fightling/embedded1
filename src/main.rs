#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{gpio::IO, peripherals::Peripherals, prelude::*};
// use esp_println::println;
// use embedded_hal::digital::v2::OutputPin;
use max7219::*;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    // let system = peripherals.SYSTEM.split();

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    let sck_pin = io.pins.gpio6.into_push_pull_output();
    let mosi_pin = io.pins.gpio7.into_push_pull_output();
    let cs_pin = io.pins.gpio10.into_push_pull_output();

    let mut display = MAX7219::from_pins(1, mosi_pin, cs_pin, sck_pin).unwrap();

    // make sure to wake the display up
    display.power_on().unwrap();
    // write given octet of ASCII characters with dots specified by 3rd param bits
    display.write_str(0, b"pls help", 0b00100000).unwrap();
    // set display intensity lower
    display.set_intensity(0, 0x1).unwrap();

    loop {}

    // let clocks = ClockControl::max(system.clock_control).freeze();
    // let mut delay = Delay::new(&clocks);
    //
    // println!("Hello world!");
    // loop {
    //     println!("Loop...");
    //     delay.delay_ms(500u32);
    // }
}
