use crate::{backlight::Backlight, scanner::Scanner, uplink::Uplink};

pub trait Keyboard<const ROWS: usize, const COLS: usize> {
    type Scanner: Scanner<{ ROWS }, { COLS }>;
    type Uplink: Uplink;
    type Backlight: Backlight;

    fn scanner(&mut self) -> &mut Self::Scanner;

    fn uplink(&mut self) -> &mut Self::Uplink;

    fn backlight(&mut self) -> &mut Self::Backlight;
}
