//! Mapping physical keys to keycodes.

use crate::keycode::qmk::{KC_NO, KC_TRANSPARENT};
use crate::keycode::{KeyAction, Keycode, LayerAction};
use crate::system;

pub trait Keymap<const ROWS: usize, const COLS: usize> {
    fn get(&self, row: usize, col: usize) -> Keycode;

    fn key_event(&mut self, keycode: Keycode, action: KeyAction) {
        let _ = (keycode, action);
    }
}

pub struct Simple<const ROWS: usize, const COLS: usize>(pub &'static [[Keycode; COLS]; ROWS]);

impl<const ROWS: usize, const COLS: usize> Keymap<ROWS, COLS> for Simple<ROWS, COLS> {
    fn get(&self, row: usize, col: usize) -> Keycode {
        self.0[row][col]
    }
}

pub struct Layered<const ROWS: usize, const COLS: usize, const LAYERS: usize> {
    layer_mask: u32,
    layers: &'static [[[Keycode; COLS]; ROWS]; LAYERS],
}

impl<const ROWS: usize, const COLS: usize, const LAYERS: usize> Layered<ROWS, COLS, LAYERS> {
    pub fn new(layers: &'static [[[Keycode; COLS]; ROWS]; LAYERS]) -> Self {
        Self {
            layer_mask: 1,
            layers,
        }
    }
    pub fn is_layer_enabled(&self, layer: u8) -> bool {
        (self.layer_mask & (1 << layer)) != 0
    }

    pub fn enable_layer(&mut self, layer: u8) {
        self.layer_mask |= 1 << layer;
        system::clear_keyboard_but_mods();
    }

    pub fn disable_layer(&mut self, layer: u8) {
        self.layer_mask &= !(1 << layer);
        system::clear_keyboard_but_mods();
    }

    pub fn toggle_layer(&mut self, layer: u8) {
        self.layer_mask ^= 1 << layer;
        system::clear_keyboard_but_mods();
    }
}

impl<const ROWS: usize, const COLS: usize, const LAYERS: usize> Keymap<ROWS, COLS>
    for Layered<ROWS, COLS, LAYERS>
{
    fn get(&self, row: usize, col: usize) -> Keycode {
        for i in (0..LAYERS).rev() {
            if !self.is_layer_enabled(i as u8) {
                continue;
            }
            match self.layers[i][row][col] {
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

    fn key_event(&mut self, keycode: Keycode, action: KeyAction) {
        match keycode {
            Keycode::Layer(layer_key) => match layer_key.action() {
                LayerAction::Momentary => {
                    if action.is_pressed() {
                        self.enable_layer(layer_key.layer());
                    } else {
                        self.disable_layer(layer_key.layer());
                    }
                }
                LayerAction::Oneshot => {
                    //TODO
                }
                LayerAction::Toggle => {
                    if action.is_pressed() {
                        self.toggle_layer(layer_key.layer());
                    }
                }
                LayerAction::To => {
                    if action.is_pressed() {
                        // TODO preserve default layers
                        self.layer_mask = 1 << layer_key.layer();
                    }
                }
            },
            _ => {}
        }
    }
}
