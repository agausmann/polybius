//! Architecture-specific utilities.

#[cfg(any(test, target_arch = "avr"))]
pub mod avr;
