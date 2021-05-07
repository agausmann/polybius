//! Interfaces to the physical hardware of the keyboard.
//!
//! This module defines types that implement common physical layouts of keyboards, how the key
//! switches are wired up and the software required to poll those switches for their press state.

use crate::group::{InputGroup, TriStateGroup};

/// An implementation of a "scan matrix".
///
/// The columns are scanned by driving them low one at a time, and the keyswitch states
/// for each key in the column are polled by checking which row pins are low.
pub struct ScanMatrix<R, C, const ROWS: usize, const COLS: usize> {
    rows: R,
    cols: C,
    state: [[bool; COLS]; ROWS],
}

impl<R, C, const ROWS: usize, const COLS: usize> ScanMatrix<R, C, ROWS, COLS>
where
    R: Rows<ROWS>,
    C: Cols<COLS, Error = R::Error>,
{
    pub fn new(rows: R, cols: C) -> Self {
        Self {
            rows,
            cols,
            state: [[false; COLS]; ROWS],
        }
    }

    pub fn poll(&mut self) -> Result<(), R::Error> {
        for col in 0..COLS {
            self.cols.set(col)?;
            //TODO delay needed here?
            for row in 0..ROWS {
                self.state[row][col] = self.rows.poll(row)?;
            }
        }
        Ok(())
    }

    pub fn is_pressed(&self, row: usize, col: usize) -> bool {
        self.state[row][col]
    }
}

pub trait Rows<const LEN: usize> {
    type Error;

    fn poll(&mut self, index: usize) -> Result<bool, Self::Error>;
}

pub trait Cols<const LEN: usize> {
    type Error;

    fn set(&mut self, index: usize) -> Result<(), Self::Error>;
}

/// Each pin in the group directly corresponds to a row or column.
///
/// When used as columns, it will drive the selected column pin low and set the
/// rest of the columns to floating.
///
/// When this is used to represent rows, it is assumed that the pins are pulled
/// high by default and driven low when connected to a selected column.
pub struct Direct<Group>(Group);

impl<Group, const LEN: usize> Rows<LEN> for Direct<Group>
where
    Group: InputGroup<LEN>,
{
    type Error = Group::Error;

    fn poll(&mut self, index: usize) -> Result<bool, Self::Error> {
        self.0.is_low(index)
    }
}

impl<Group, const LEN: usize> Cols<LEN> for Direct<Group>
where
    Group: TriStateGroup<LEN>,
{
    type Error = Group::Error;

    fn set(&mut self, index: usize) -> Result<(), Self::Error> {
        for i in 0..LEN {
            self.0.set_floating(i)?;
        }
        self.0.set_low(index)
    }
}
