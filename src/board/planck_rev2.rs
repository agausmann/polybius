use crate::diodes::ColToRow;
use crate::keymap::Keymap;
use crate::scanner::{Direct, ScanMatrix};
use crate::uplink::KeyEvent;
use atmega32u4_hal::clock::MHz16;
use atmega32u4_hal::delay::Delay;
use atmega32u4_hal::pac::Peripherals;
use atmega32u4_hal::port::mode::{Input, PullUp, TriState};
use atmega32u4_hal::port::portb::{PB0, PB4, PB5, PB6};
use atmega32u4_hal::port::portc::PC7;
use atmega32u4_hal::port::portd::{PD0, PD4, PD5, PD6, PD7};
use atmega32u4_hal::port::portf::{PF0, PF1, PF4, PF5, PF6, PF7};
use atmega32u4_hal::port::PortExt;
use core::convert::Infallible;
use embedded_hal::blocking::delay::DelayUs;
use embedded_hal::digital::v2::OutputPin;

pub const ROWS: usize = 4;
pub const COLS: usize = 12;

pub type ClockSpeed = MHz16;

pub type WriteLines = Direct<(PD0<TriState>, PD5<TriState>, PB5<TriState>, PB6<TriState>)>;
pub type ReadLines = Direct<(
    PF1<Input<PullUp>>,
    PF0<Input<PullUp>>,
    PB0<Input<PullUp>>,
    PC7<Input<PullUp>>,
    PF4<Input<PullUp>>,
    PF5<Input<PullUp>>,
    PF6<Input<PullUp>>,
    PF7<Input<PullUp>>,
    PD4<Input<PullUp>>,
    PD6<Input<PullUp>>,
    PB4<Input<PullUp>>,
    PD7<Input<PullUp>>,
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
    let portb = dp.PORTB.split();
    let portc = dp.PORTC.split();
    let portd = dp.PORTD.split();
    let porte = dp.PORTE.split();
    let portf = dp.PORTF.split();

    let write_lines = Direct((
        portd.pd0.into_tri_state(&portd.ddr),
        portd.pd5.into_tri_state(&portd.ddr),
        portb.pb5.into_tri_state(&portb.ddr),
        portb.pb6.into_tri_state(&portb.ddr),
    ));
    let read_lines = Direct((
        portf.pf1.into_pull_up_input(&portf.ddr),
        portf.pf0.into_pull_up_input(&portf.ddr),
        portb.pb0.into_pull_up_input(&portb.ddr),
        portc.pc7.into_pull_up_input(&portc.ddr),
        portf.pf4.into_pull_up_input(&portf.ddr),
        portf.pf5.into_pull_up_input(&portf.ddr),
        portf.pf6.into_pull_up_input(&portf.ddr),
        portf.pf7.into_pull_up_input(&portf.ddr),
        portd.pd4.into_pull_up_input(&portd.ddr),
        portd.pd6.into_pull_up_input(&portd.ddr),
        portb.pb4.into_pull_up_input(&portb.ddr),
        portd.pd7.into_pull_up_input(&portd.ddr),
    ));
    let scanner = Scanner::new(write_lines, read_lines, scan_delay);

    //TODO
    let uplink = Uplink {};

    let system = System::new(keymap, scanner, uplink);

    // Turn status LED on
    let mut status_led = porte.pe6.into_output(&porte.ddr);
    status_led.set_high().ok();

    system
}
