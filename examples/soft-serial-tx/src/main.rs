#![no_std]
#![no_main]

use atmega_hal::{clock::MHz16, delay::Delay, pins, Peripherals};
use embedded_hal::blocking::delay::DelayMs;
use polybius::arch::avr::soft_serial::{Baud100k, Serial};

extern crate avr_std_stub;

#[avr_device::entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let pins = pins!(dp);

    let _debug_clock = pins.pd1.into_output();
    let mut status = pins.pc7.into_output();
    let trigger = pins.pd4.into_pull_up_input();
    let mut delay: Delay<MHz16> = Delay::new();
    let mut serial: Serial<_, MHz16, Baud100k> = Serial::new(pins.pd0);

    let payload = b"Hello World";
    let mut recv_buffer = [0u8; 16];

    loop {
        if trigger.is_low() {
            if let Ok(bytes_read) = serial.send_transaction(payload, &mut recv_buffer) {
                if &recv_buffer[..bytes_read] == payload {
                    status.set_high();
                }
            }
            while trigger.is_low() {}
        }
        delay.delay_ms(1u8);
    }
}
