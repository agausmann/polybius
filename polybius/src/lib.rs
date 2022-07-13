#![no_std]
#![feature(generic_const_exprs)]
#![deny(unsafe_op_in_unsafe_fn)]

pub mod backlight;
pub mod diodes;
pub mod keyboard;
pub mod keycode;
pub mod keymap;
pub mod mutex;
pub mod pin_group;
pub mod scanner;
pub mod system;
pub mod uplink;

pub mod arch;
