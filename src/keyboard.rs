use crate::{backlight::Backlight, scanner::Scanner, uplink::Uplink};

/// Collection of various features that may be provided by keyboard hardware.
///
/// # Implementation notes
///
/// It's recommended to implement all the components of the keyboard as separate
/// types implementing the various component traits. Then, the type that
/// implements this `Keyboard` trait can compose and provide instances of all of
/// the other components within a single instance.
///
/// The components provided by this trait are intentionally designed as
/// associated types and accessors instead of as supertraits (which would
/// provide an equivalent requirement to fulfill but would be much less
/// ergonomic).
///
/// This design encourages modularity and code reuse, as it is very convenient
/// to use an existing implementation of a component. Polybius provides at least
/// one implementation of each component, and accepts contributions for
/// additional implementations if they could be useful for other keyboard
/// implementors.
pub trait Keyboard<const ROWS: usize, const COLS: usize> {
    type Scanner: Scanner<ROWS, COLS>;
    type Uplink: Uplink;
    type Backlight: Backlight;

    fn scanner(&mut self) -> &mut Self::Scanner;

    fn uplink(&mut self) -> &mut Self::Uplink;

    fn backlight(&mut self) -> &mut Self::Backlight;
}
