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

    let mut data = pins.pd0.into_output_high();
    let _debug_clock = pins.pd1.into_output();
    let mut status = pins.pc7.into_output();
    let mut delay: Delay<MHz16> = Delay::new();

    let payload = b"Hello World";

    loop {
        interrupt::free(|cs| {
            data.set_low();
            for &byte in payload {
                delay.delay_us(20u8);
                SoftSerialPin::write_byte(&mut data, cs, 5, byte, true);
            }
        });
        data.set_high();
        delay.delay_ms(1u8);
        status.set_high();
    }
}
