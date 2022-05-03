use crate::diodes::ColToRow;
use crate::keymap::Keymap;
use crate::scanner::{Direct, ScanMatrix};
use crate::uplink::KeyEvent;
use atmega_hal::{
    clock::MHz16,
    delay::Delay,
    pac::Peripherals,
    port::mode::{Input, OpenDrain, PullUp},
    port::{Pin, PB0, PB4, PB5, PB6, PC7, PD0, PD4, PD5, PD6, PD7, PF0, PF1, PF4, PF5, PF6, PF7},
};
use core::convert::Infallible;
use embedded_hal::blocking::delay::DelayUs;

pub const ROWS: usize = 4;
pub const COLS: usize = 12;

pub type ClockSpeed = MHz16;

pub type WriteLines = Direct<(
    Pin<OpenDrain, PD0>,
    Pin<OpenDrain, PD5>,
    Pin<OpenDrain, PB5>,
    Pin<OpenDrain, PB6>,
)>;
pub type ReadLines = Direct<(
    Pin<Input<PullUp>, PF1>,
    Pin<Input<PullUp>, PF0>,
    Pin<Input<PullUp>, PB0>,
    Pin<Input<PullUp>, PC7>,
    Pin<Input<PullUp>, PF4>,
    Pin<Input<PullUp>, PF5>,
    Pin<Input<PullUp>, PF6>,
    Pin<Input<PullUp>, PF7>,
    Pin<Input<PullUp>, PD4>,
    Pin<Input<PullUp>, PD6>,
    Pin<Input<PullUp>, PB4>,
    Pin<Input<PullUp>, PD7>,
)>;
pub type ScanDelay = fn();
pub type Diodes = ColToRow;
pub type Scanner = ScanMatrix<WriteLines, ReadLines, ScanDelay, Diodes, ROWS, COLS>;

fn scan_delay() {
    let mut delay = Delay::<ClockSpeed>::new();
    delay.delay_us(30_u8);
}

// We can't use the `usb-device` framework, because there isn't a working
// implementation for AVR yet, and it relies on trait objects anyway, which are
// miscompiled in AVR at the moment.
pub struct Uplink {}

impl crate::uplink::Uplink for Uplink {
    type Error = Infallible;

    fn poll(&mut self) -> Result<(), Self::Error> {
        todo!()
    }

    fn send(&mut self, event: KeyEvent) -> Result<(), Self::Error> {
        todo!()
    }
}

pub type System<K> = crate::system::System<K, Scanner, Uplink, ROWS, COLS>;

pub fn build_system<K>(dp: Peripherals, keymap: K) -> System<K>
where
    K: Keymap<ROWS, COLS>,
{
    let pins = atmega_hal::pins!(dp);

    let write_lines = Direct((
        pins.pd0.into_opendrain_high(),
        pins.pd5.into_opendrain_high(),
        pins.pb5.into_opendrain_high(),
        pins.pb6.into_opendrain_high(),
    ));
    let read_lines = Direct((
        pins.pf1.into_pull_up_input(),
        pins.pf0.into_pull_up_input(),
        pins.pb0.into_pull_up_input(),
        pins.pc7.into_pull_up_input(),
        pins.pf4.into_pull_up_input(),
        pins.pf5.into_pull_up_input(),
        pins.pf6.into_pull_up_input(),
        pins.pf7.into_pull_up_input(),
        pins.pd4.into_pull_up_input(),
        pins.pd6.into_pull_up_input(),
        pins.pb4.into_pull_up_input(),
        pins.pd7.into_pull_up_input(),
    ));
    let scanner = Scanner::new(write_lines, read_lines, scan_delay);

    //TODO
    let uplink = Uplink {};

    let system = System::new(keymap, scanner, uplink);

    // Turn status LED on
    let mut status_led = pins.pe6.into_output();
    status_led.set_high();

    system
}
