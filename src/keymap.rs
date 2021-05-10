//! Mapping physical keys to keycodes.

use crate::keycode::qmk::{KC_NO, KC_TRANSPARENT};
use crate::keycode::Keycode;

pub trait Keymap<const ROWS: usize, const COLS: usize> {
    fn get(&self, row: usize, col: usize) -> Keycode;
}

pub type Simple<const ROWS: usize, const COLS: usize> = [[Keycode; COLS]; ROWS];

impl<const ROWS: usize, const COLS: usize> Keymap<ROWS, COLS> for Simple<ROWS, COLS> {
    fn get(&self, row: usize, col: usize) -> Keycode {
        self[row][col]
    }
}

pub struct Layered<const ROWS: usize, const COLS: usize, const LAYERS: usize> {
    layer_mask: u32,
    layers: [Simple<ROWS, COLS>; LAYERS],
}

impl<const ROWS: usize, const COLS: usize, const LAYERS: usize> Keymap<ROWS, COLS>
    for Layered<ROWS, COLS, LAYERS>
{
    fn get(&self, row: usize, col: usize) -> Keycode {
        for i in (0..LAYERS).rev() {
            if (self.layer_mask & (1 << i)) == 0 {
                continue;
            }
            match self.layers[i].get(row, col) {
                KC_TRANSPARENT => {
                    continue;
                }
                other => {
                    return other;
                }
            }
        }
        KC_NO
    }
}
