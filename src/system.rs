use crate::keymap::Keymap;
use crate::scanner::Scanner;
use crate::uplink::{KeyAction, KeyEvent, Uplink};

pub struct System<K, S, U, const ROWS: usize, const COLS: usize> {
    keymap: K,
    scanner: S,
    uplink: U,
}

impl<K, S, U, const ROWS: usize, const COLS: usize> System<K, S, U, ROWS, COLS>
where
    K: Keymap<ROWS, COLS>,
    S: Scanner<ROWS, COLS>,
    U: Uplink,
{
    pub fn new(keymap: K, scanner: S, uplink: U) -> Self {
        Self {
            keymap,
            scanner,
            uplink,
        }
    }

    pub fn poll(&mut self) -> Result<(), Error<S::Error, U::Error>> {
        self.scanner.poll().map_err(Error::Scanner)?;
        for row in 0..ROWS {
            for col in 0..COLS {
                if self.scanner.just_pressed(row, col) {
                    let event = KeyEvent {
                        keycode: self.keymap.get(row, col),
                        action: KeyAction::Pressed,
                    };
                    self.keymap.handle_key(event.keycode, event.action);
                    self.uplink.send(event).map_err(Error::Uplink)?;
                }
                if self.scanner.just_released(row, col) {
                    let event = KeyEvent {
                        keycode: self.keymap.get(row, col),
                        action: KeyAction::Released,
                    };
                    self.keymap.handle_key(event.keycode, event.action);
                    self.uplink.send(event).map_err(Error::Uplink)?;
                }
            }
        }
        self.uplink.poll().map_err(Error::Uplink)?;
        Ok(())
    }
}

pub enum Error<S, U> {
    Scanner(S),
    Uplink(U),
}
