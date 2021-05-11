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
    pub layer_mask: u32,
    pub layers: [Simple<ROWS, COLS>; LAYERS],
}

impl<const ROWS: usize, const COLS: usize, const LAYERS: usize> Layered<ROWS, COLS, LAYERS> {
    pub fn is_layer_enabled(&self, layer: usize) -> bool {
        (self.layer_mask & (1 << layer)) != 0
    }

    pub fn enable_layer(&mut self, layer: usize) {
        self.layer_mask |= 1 << layer;
    }

    pub fn disable_layer(&mut self, layer: usize) {
        self.layer_mask &= !(1 << layer);
    }
}

impl<const ROWS: usize, const COLS: usize, const LAYERS: usize> Keymap<ROWS, COLS>
    for Layered<ROWS, COLS, LAYERS>
{
    fn get(&self, row: usize, col: usize) -> Keycode {
        for i in (0..LAYERS).rev() {
            if !self.is_layer_enabled(i) {
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
