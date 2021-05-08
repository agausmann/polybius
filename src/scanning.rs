//! Key scanning.
//!
//! Defines how the key switches are wired up and how to scan
//! those switches for their press state.

use crate::pin_group::{InputGroup, OutputGroup};

pub type ScanRow = u32;

pub struct ScanPosition {
    pub write_index: u8,
    pub read_index: u8,
}

/// An implementation of a "scan matrix".
pub struct ScanMatrix<W, R, D, const M: usize, const N: usize> {
    write_lines: W,
    read_lines: R,
    scan_delay: D,
    old_state: [ScanRow; M],
    new_state: [ScanRow; M],
}

impl<W, R, D, const M: usize, const N: usize> ScanMatrix<W, R, D, M, N>
where
    W: WriteLines<M>,
    R: ReadLines<N, Error = W::Error>,
    D: FnMut(),
{
    pub fn new(write_lines: W, read_lines: R, scan_delay: D) -> Self {
        Self {
            write_lines,
            read_lines,
            scan_delay,
            old_state: [Default::default(); M],
            new_state: [Default::default(); M],
        }
    }

    pub fn poll(&mut self) -> Result<(), W::Error> {
        //TODO ghosting
        for i in 0..M {
            self.write_lines.set(i)?;
            (self.scan_delay)();
            self.old_state[i] = self.new_state[i];
            self.new_state[i] = self.read_lines.poll()?;
        }
        Ok(())
    }

    pub fn is_pressed(&self, pos: ScanPosition) -> bool {
        (self.new_state[pos.write_index as usize] & (1 << pos.read_index)) != 0
    }

    pub fn just_pressed(&self, pos: ScanPosition) -> bool {
        ((self.new_state[pos.write_index as usize] & !self.old_state[pos.write_index as usize])
            & (1 << pos.read_index))
            != 0
    }

    pub fn just_released(&self, pos: ScanPosition) -> bool {
        ((!self.new_state[pos.write_index as usize] & self.old_state[pos.write_index as usize])
            & (1 << pos.read_index))
            != 0
    }
}

pub trait ReadLines<const LEN: usize> {
    type Error;

    fn poll(&mut self) -> Result<ScanRow, Self::Error>;
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

    fn poll(&mut self) -> Result<ScanRow, Self::Error> {
        let mut acc = 0;
        for i in 0..LEN {
            if self.0.is_low(i)? {
                acc |= 1 << i;
            }
        }
        Ok(acc)
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
