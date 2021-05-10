use crate::diodes::ColToRow;
use crate::keymap::Keymap;
use crate::scanner::{Direct, ScanMatrix};
use crate::uplink::KeyEvent;
use atmega32u4_hal::pac::Peripherals;
use atmega32u4_hal::port::mode::{PullUp, TriState};
use atmega32u4_hal::port::portb::{PB0, PB4, PB5, PB6};
use atmega32u4_hal::port::portc::PC7;
use atmega32u4_hal::port::portd::{PD0, PD4, PD5, PD6, PD7};
use atmega32u4_hal::port::portf::{PF0, PF1, PF4, PF5, PF6, PF7};
use core::convert::Infallible;

pub const ROWS: usize = 4;
pub const COLS: usize = 12;

pub type WriteLines = Direct<(PD0<TriState>, PD5<TriState>, PB5<TriState>, PB6<TriState>)>;
pub type ReadLines = Direct<(
    PF1<PullUp>,
    PF0<PullUp>,
    PB0<PullUp>,
    PC7<PullUp>,
    PF4<PullUp>,
    PF5<PullUp>,
    PF6<PullUp>,
    PF7<PullUp>,
    PD4<PullUp>,
    PD6<PullUp>,
    PB4<PullUp>,
    PD7<PullUp>,
)>;
pub type Delay = ();
pub type Diodes = ColToRow;
pub type Scanner = ScanMatrix<WriteLines, ReadLines, Delay, Diodes, ROWS, COLS>;

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
    todo!()
}
