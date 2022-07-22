#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use atmega_hal::{entry, Peripherals};
use avr_device::{asm::sleep, interrupt};
use avr_std_stub as _;
use polybius::arch::avr::atmega32u4::nonblocking_i2c::NonblockingI2c;

#[entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let pins = atmega_hal::pins!(dp);

    let ctx = TwiContext {
        i2c: NonblockingI2c::new(dp.TWI, pins.pd1, pins.pd0),
    };
    unsafe { TWI_CTX = Some(ctx) };

    unsafe { interrupt::enable() };
    loop {
        sleep();
    }
}

struct TwiContext {
    i2c: NonblockingI2c,
}

static mut TWI_CTX: Option<TwiContext> = None;

#[interrupt(atmega32u4)]
fn TWI() {
    let ctx = unsafe { TWI_CTX.as_mut().unwrap() };
    ctx.i2c.poll()
}
