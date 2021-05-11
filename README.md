# KBForge

An experimental library for developing custom keyboard firmwares in Rust.

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

- Allow components to be implemented "out-of-tree," in contrast to QMK's build
  system where keyboard and keymap support has to be in-tree / in the same
project. This is done by leveraging Rust's build tool, Cargo, and the ability
for individual projects ("crates") to depend on and extend each other. As an
example, see [my_kbforge](https://github.com/agausmann/my_kbforge), which is
where I maintain my personal KBForge keymaps for my keyboards.
