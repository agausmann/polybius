use crate::keyboard::Keyboard;
use crate::keycode::KeyAction;
use crate::keymap::Keymap;
use crate::scanner::Scanner;
use crate::uplink::Uplink;

pub struct System<K, B, const ROWS: usize, const COLS: usize> {
    keymap: K,
    keyboard: B,
}

impl<K, B, const ROWS: usize, const COLS: usize> System<K, B, ROWS, COLS>
where
    K: Keymap<ROWS, COLS>,
    B: Keyboard<ROWS, COLS>,
{
    pub fn new(keymap: K, keyboard: B) -> Self {
        Self { keymap, keyboard }
    }

    pub fn poll(
        &mut self,
    ) -> Result<(), Error<<B::Scanner as Scanner<ROWS, COLS>>::Error, <B::Uplink as Uplink>::Error>>
    {
        self.keyboard.scanner().poll().map_err(Error::Scanner)?;
        for row in 0..ROWS {
            for col in 0..COLS {
                if self.keyboard.scanner().just_pressed(row, col) {
                    let keycode = self.keymap.get(row, col);
                    let action = KeyAction::Pressed;
                    self.keymap.key_event(keycode, action);
                    self.keyboard
                        .uplink()
                        .key_event(keycode, action)
                        .map_err(Error::Uplink)?;
                }
                if self.keyboard.scanner().just_released(row, col) {
                    let keycode = self.keymap.get(row, col);
                    let action = KeyAction::Released;
                    self.keymap.key_event(keycode, action);
                    self.keyboard
                        .uplink()
                        .key_event(keycode, action)
                        .map_err(Error::Uplink)?;
                }
            }
        }
        self.keyboard.uplink().poll().map_err(Error::Uplink)?;
        Ok(())
    }
}

pub enum Error<S, U> {
    Scanner(S),
    Uplink(U),
}
