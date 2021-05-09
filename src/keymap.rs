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

pub struct Layered<const ROWS: usize, const COLS: usize> {
    layer_mask: u32,
    layers: &'static [Simple<ROWS, COLS>],
}

impl<const ROWS: usize, const COLS: usize> Keymap<ROWS, COLS> for Layered<ROWS, COLS> {
    fn get(&self, row: usize, col: usize) -> Keycode {
        for i in (0..self.layers.len()).rev() {
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
