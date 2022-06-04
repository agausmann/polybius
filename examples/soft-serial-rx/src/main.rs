#![no_std]
#![no_main]

use atmega_hal::{clock::MHz16, delay::Delay, pins, Peripherals};
use avr_device::interrupt;
use embedded_hal::blocking::delay::{DelayMs, DelayUs};
use polybius::arch::avr::soft_serial::SoftSerialPin;

extern crate avr_std_stub;

#[avr_device::entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let pins = pins!(dp);

    let data = pins.pd0.into_pull_up_input();
    let _debug_clock = pins.pd1.into_output();
    let mut status = pins.pc7.into_output();
    let mut delay: Delay<MHz16> = Delay::new();

    interrupt::free(|cs| {
        loop {
            let result = SoftSerialPin::read_byte(&data, cs, 5);
            if result.is_ok() {
                status.set_high();
            }
        }
    })
}
