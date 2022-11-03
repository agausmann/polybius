# Polybius

Keyboard firmware experiments on the bleeding edge of Rust's const-eval.

**NOTE: This crate is not being maintained anymore.** I've encountered
difficulties porting other architectures and keyboards (especially split)
to this design, and so instead of trying to port everything to this and
adapt it, I have decided to move on.

I will eventually buiild a new framework, but first, I will be building
a lot of firmwares from scratch. Abstractions will come later.

## Features

- **Modular design.** Polybius is intended to be a simple "glue" library, the
bridge between keyboard support packages and user-defined keymaps. Both of
these can and should be implemented as separate crates.

- **Type-checked keymaps and matrix I/O.** Uses Rust's powerful type system
and compile-time evaluation to validate that:
  - The user's keymap matches the layout specified by the hardware.
  - The correct number of I/O for rows and columns is provided by the hardware
  support package.
  - The I/O direction of rows and columns matches the diode configuration.

## Featured Crates

- [my_keyboards][agausmann/my_keyboards] by [Adam Gausmann][agausmann] -
Examples of end-user code using Polybius; keymaps for the keyboards I own.

[agausmann]: https://github.com/agausmann
[agausmann/my_keyboards]: https://github.com/agausmann/my_keyboards
