# Polybius

Developing keyboard firmwares with Rust!

**Note: This is very unstable!** There will be bugs, and the API may change
drastically before the first published version.

Hardware support is also lacking, but that can be improved! I am slowly adding
support for my keyboards (including split ones), and I encourage you to write
your own driver crates too. Feel free to open a pull-request, and I will do my best
to keep it up-to-date with the latest changes in the core.

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
