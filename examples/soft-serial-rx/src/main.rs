#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use atmega_hal::{
    clock::MHz16,
    pins,
    port::{mode::Output, Pin, PC7, PD0},
    Peripherals,
};
use avr_device::interrupt;
use polybius::arch::avr::soft_serial::{Baud100k, Serial};

extern crate avr_std_stub;

#[avr_device::entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let pins = pins!(dp);

    let _debug_clock = pins.pd1.into_output();
    let status = pins.pc7.into_output();
    let serial: Serial<_, MHz16, Baud100k> = Serial::new(pins.pd0);

    unsafe { SERIAL_CONTEXT = Some(SerialContext { serial, status }) };

    // INT0 enable
    dp.EXINT.eimsk.write(|w| w.int().bits(1));

    unsafe { interrupt::enable() };

    loop {}
}

struct SerialContext {
    serial: Serial<PD0, MHz16, Baud100k>,
    status: Pin<Output, PC7>,
}

static mut SERIAL_CONTEXT: Option<SerialContext> = None;

#[interrupt(atmega32u4)]
fn INT0() {
    let ctx = unsafe { SERIAL_CONTEXT.as_mut().unwrap() };
    if ctx.serial.recv_transaction().is_ok() {
        ctx.status.set_high();
    }
}
