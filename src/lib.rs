#![no_std]
#![feature(generic_const_exprs)]
#![cfg_attr(feature = "planck_rev2", feature(asm_experimental_arch))]

pub mod diodes;
pub mod keycode;
pub mod keymap;
pub mod pin_group;
pub mod scanner;
pub mod system;
pub mod uplink;

pub mod board;
