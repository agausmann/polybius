/// Backlight interface for keyboard hardware.
///
/// For keyboards that do not support backlight, the type [`NoBacklight`]
/// provides a no-op implementation.
pub trait Backlight {
    /// Number of backlight levels/states, including the zero/off state.
    ///
    /// If there is no backlight, then the only allowed state is zero, and
    /// `num_levels` should be 1.
    fn num_levels(&self) -> u8;

    /// The current backlight level.
    fn level(&self) -> u8;

    /// Set the current level to `level`.
    ///
    /// Allowable values are in the range `0..self.num_levels()` (maximum is
    /// `self.num_levels() - 1`). Values outside this range should be clamped.
    fn set_level(&mut self, level: u8);

    /// Increments the current level by one. Does nothing if backlight is
    /// already at the max level.
    fn increase(&mut self) {
        let level = self.level();
        if level < self.num_levels() - 1 {
            self.set_level(level + 1);
        }
    }

    /// Decrements the current level by one. Does nothing if the backlight is
    /// already at the zero level.
    fn decrease(&mut self) {
        let level = self.level();
        if level > 0 {
            self.set_level(level - 1);
        }
    }

    /// Cycles through backlight levels in increasing order. Loops around to
    /// zero after the maximum level is reached.
    fn cycle_step(&mut self) {
        self.set_level((self.level() + 1) % self.num_levels());
    }
}

pub struct NoBacklight;

impl Backlight for NoBacklight {
    fn num_levels(&self) -> u8 {
        1
    }

    fn level(&self) -> u8 {
        0
    }

    fn set_level(&mut self, level: u8) {
        let _ = level;
    }
}
