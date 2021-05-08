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
///
/// The columns are scanned by driving them low one at a time, and the keyswitch states
/// for each key in the column are polled by checking which row pins are low.
pub struct ScanMatrix<W, R, D, const M: usize, const N: usize> {
    write_group: W,
    read_group: R,
    scan_delay: D,
    old_state: [ScanRow; M],
    new_state: [ScanRow; M],
}

impl<W, R, D, const M: usize, const N: usize> ScanMatrix<W, R, D, M, N>
where
    W: WriteGroup<M>,
    R: ReadGroup<N, Error = W::Error>,
    D: FnMut(),
{
    pub fn new(write_group: W, read_group: R, scan_delay: D) -> Self {
        Self {
            write_group,
            read_group,
            scan_delay,
            old_state: [Default::default(); M],
            new_state: [Default::default(); M],
        }
    }

    pub fn poll(&mut self) -> Result<(), W::Error> {
        //TODO ghosting
        for i in 0..M {
            self.write_group.set(i)?;
            (self.scan_delay)();
            self.old_state[i] = self.new_state[i];
            self.new_state[i] = self.read_group.poll()?;
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

pub trait ReadGroup<const LEN: usize> {
    type Error;

    fn poll(&mut self) -> Result<ScanRow, Self::Error>;
}

pub trait WriteGroup<const LEN: usize> {
    type Error;

    fn set(&mut self, index: usize) -> Result<(), Self::Error>;
}

/// Each pin in the group directly corresponds to a row or column.
///
/// When used as columns, it will drive the selected column pin low and set the
/// rest of the columns high (or floating if the output is open-drain).
///
/// When this is used to represent rows, it is assumed that the pins are pulled
/// high by default and driven low when connected to a selected column.
pub struct Direct<Group>(Group);

impl<Group, const LEN: usize> ReadGroup<LEN> for Direct<Group>
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

impl<Group, const LEN: usize> WriteGroup<LEN> for Direct<Group>
where
    Group: OutputGroup<LEN>,
{
    type Error = Group::Error;

    fn set(&mut self, index: usize) -> Result<(), Self::Error> {
        for i in 0..LEN {
            self.0.set_high(i)?;
        }
        self.0.set_low(index)
    }
}
