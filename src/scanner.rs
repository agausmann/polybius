//! Key scanning.
//!
//! Defines how the key switches are wired up and how to scan
//! those switches for their press state.

use crate::diodes::{DiodeConfiguration, KeyPosition, ScanPosition};
use crate::pin_group::{InputGroup, OutputGroup};
use core::marker::PhantomData;

pub trait Scanner<const ROWS: usize, const COLS: usize> {
    type Error;

    fn poll(&mut self) -> Result<(), Self::Error>;

    fn is_pressed(&self, row: usize, col: usize) -> bool;

    fn just_pressed(&self, row: usize, col: usize) -> bool;

    fn just_released(&self, row: usize, col: usize) -> bool;
}

pub type ScanRow = u32;

/// An implementation of a "scan matrix".
pub struct ScanMatrix<W, R, D, C, const ROWS: usize, const COLS: usize> {
    write_lines: W,
    read_lines: R,
    scan_delay: D,
    _diodes: PhantomData<C>,
    old_state: [ScanRow; ROWS],
    new_state: [ScanRow; ROWS],
}

impl<W, R, D, C, const ROWS: usize, const COLS: usize> ScanMatrix<W, R, D, C, ROWS, COLS>
where
    C: DiodeConfiguration<ROWS, COLS>,
    W: WriteLines<{ C::WRITE_LINES }>,
    R: ReadLines<{ C::READ_LINES }, Error = W::Error>,
    D: FnMut(),
{
    pub fn new(write_lines: W, read_lines: R, scan_delay: D) -> Self {
        Self {
            write_lines,
            read_lines,
            scan_delay,
            _diodes: PhantomData,
            old_state: [Default::default(); ROWS],
            new_state: [Default::default(); ROWS],
        }
    }
}

impl<W, R, D, C, const ROWS: usize, const COLS: usize> Scanner<ROWS, COLS>
    for ScanMatrix<W, R, D, C, ROWS, COLS>
where
    C: DiodeConfiguration<ROWS, COLS>,
    W: WriteLines<{ C::WRITE_LINES }>,
    R: ReadLines<{ C::READ_LINES }, Error = W::Error>,
    D: FnMut(),
{
    type Error = W::Error;

    fn poll(&mut self) -> Result<(), Self::Error> {
        self.old_state = self.new_state;
        self.new_state = [Default::default(); ROWS];

        //TODO ghosting
        for i in 0..C::WRITE_LINES {
            self.write_lines.set(i)?;
            (self.scan_delay)();
            for j in 0..C::READ_LINES {
                if self.read_lines.poll(j)? {
                    let KeyPosition { row, col } = C::key_position(ScanPosition {
                        write_index: i,
                        read_index: j,
                    });

                    self.new_state[row] |= 1 << col;
                }
            }
        }
        Ok(())
    }

    fn is_pressed(&self, row: usize, col: usize) -> bool {
        (self.new_state[row] & (1 << col)) != 0
    }

    fn just_pressed(&self, row: usize, col: usize) -> bool {
        ((self.new_state[row] & !self.old_state[row]) & (1 << col)) != 0
    }

    fn just_released(&self, row: usize, col: usize) -> bool {
        ((!self.new_state[row] & self.old_state[row]) & (1 << col)) != 0
    }
}

pub trait ReadLines<const LEN: usize> {
    type Error;

    fn poll(&mut self, index: usize) -> Result<bool, Self::Error>;
}

pub trait WriteLines<const LEN: usize> {
    type Error;

    fn set(&mut self, index: usize) -> Result<(), Self::Error>;
}

/// A WriteLines or ReadLines, made from an OutputGroup or InputGroup, where
/// each pin in the group corresponds directly to a read or write line in the
/// matrix.
///
/// When used as a [`WriteLines`], it will drive the selected line low and set the
/// rest of the lines high (or floating if the output is open-drain).
///
/// When used as [`ReadLines`], it is assumed that the lines are pulled high by
/// default and connected to a write line when the corresponding key is
/// pressed, therefore being driven low when that write line is selected.
pub struct Direct<Group>(pub Group);

impl<Group, const LEN: usize> ReadLines<LEN> for Direct<Group>
where
    Group: InputGroup<LEN>,
{
    type Error = Group::Error;

    fn poll(&mut self, index: usize) -> Result<bool, Self::Error> {
        self.0.is_low(index)
    }
}

impl<Group, const LEN: usize> WriteLines<LEN> for Direct<Group>
where
    Group: OutputGroup<LEN>,
{
    type Error = Group::Error;

    fn set(&mut self, index: usize) -> Result<(), Self::Error> {
        for i in 0..LEN {
            self.0.set_high(i)?;
        }
        self.0.set_low(index)?;
        Ok(())
    }
}
