# KBForge

An experimental library for developing custom keyboard firmwares in Rust.

## Related work

- [keebrs](https://crates.io/crates/keebrs)
- [keyberon](https://crates.io/crates/keyberon)

## Goals

- **Increased type safety** / stronger compile-time guarantees compared to
  keebrs/keyberon/other Rust frameworks.  Because of recent improvements to
fixed-size arrays, and const generics more generally, it's possible to
statically guarantee that the various components (e.g. scanner and keymap) have
the same understanding of the keyboard's physical layout. If there's a size
mismatch anywhere, then it will be caught as a compile-time error, instead of
becoming a runtime error which is more difficult to debug on embedded hardware.

- Modularity, mainly so that the immutable physical details of a particular
  keyboard model can be easily separated from user-customizable features like
the keymap.

- Mimic some of the basic features of [QMK](https://qmk.fm), such as layering
  and custom keycodes. And to some extent, make keymaps easy to make if you
have prior experience working with QMK at the low level. **Anti-goal: KBForge
is not intended to have complete feature parity with QMK or be a replacement
for QMK.**
