use fullhouse::Deque;

use crate::backlight::Backlight;
use crate::keyboard::Keyboard;
use crate::keycode::{KeyAction, Keycode, SystemKeycode};
use crate::keymap::Keymap;
use crate::mutex::Mutex;
use crate::scanner::Scanner;
use crate::uplink::Uplink;

#[derive(Clone)]
enum Request {
    ClearKeyboardButMods,
}

static REQUESTS: Mutex<Deque<Request, 16>> = Mutex::new(Deque::new());

fn try_send(req: Request) {
    if let Some(mut deque) = REQUESTS.try_lock() {
        let _ = deque.push_back(req);
    }
}

pub fn clear_keyboard_but_mods() {
    try_send(Request::ClearKeyboardButMods);
}

/// Top-level system implementation that polls components and dispatches events.
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
                    self.key_event(self.keymap.get(row, col), KeyAction::Pressed)?;
                }
                if self.keyboard.scanner().just_released(row, col) {
                    self.key_event(self.keymap.get(row, col), KeyAction::Released)?;
                }
            }
        }
        self.keyboard.uplink().poll().map_err(Error::Uplink)?;
        while let Some(request) = REQUESTS.try_lock().and_then(|mut deque| deque.pop_front()) {
            match request {
                Request::ClearKeyboardButMods => {
                    self.keyboard
                        .uplink()
                        .clear_keyboard_but_mods()
                        .map_err(Error::Uplink)?;
                }
            }
        }
        Ok(())
    }

    fn key_event(
        &mut self,
        keycode: Keycode,
        action: KeyAction,
    ) -> Result<(), Error<<B::Scanner as Scanner<ROWS, COLS>>::Error, <B::Uplink as Uplink>::Error>>
    {
        match keycode {
            Keycode::System(SystemKeycode::BacklightDown) if action.is_pressed() => {
                self.keyboard.backlight().decrease();
            }
            Keycode::System(SystemKeycode::BacklightUp) if action.is_pressed() => {
                self.keyboard.backlight().increase();
            }
            Keycode::System(SystemKeycode::BacklightStep) if action.is_pressed() => {
                self.keyboard.backlight().cycle_step();
            }
            _ => {}
        }
        self.keymap.key_event(keycode, action);
        self.keyboard
            .uplink()
            .key_event(keycode, action)
            .map_err(Error::Uplink)?;
        Ok(())
    }
}

pub enum Error<S, U> {
    Scanner(S),
    Uplink(U),
}
