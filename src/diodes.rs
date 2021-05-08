//! Typestate for declaring diode presence and direction.

use crate::scanning::ScanPosition;

pub struct KeyPosition {
    pub row: u8,
    pub col: u8,
}

pub trait DiodeConfiguration {
    const CAN_GHOST: bool;

    fn scan_position(pos: KeyPosition) -> ScanPosition;
}

/// Diodes are present; current is allowed to flow from rows to columns.
pub struct RowToCol {
    _private: (),
}

impl DiodeConfiguration for RowToCol {
    const CAN_GHOST: bool = false;

    #[inline]
    fn scan_position(pos: KeyPosition) -> ScanPosition {
        ScanPosition {
            read_index: pos.row,
            write_index: pos.col,
        }
    }
}

/// Diodes are present; current is allowed to flow from columns to rows.
pub struct ColToRow {
    _private: (),
}

impl DiodeConfiguration for ColToRow {
    const CAN_GHOST: bool = false;

    #[inline]
    fn scan_position(pos: KeyPosition) -> ScanPosition {
        ScanPosition {
            read_index: pos.col,
            write_index: pos.row,
        }
    }
}

/// No diodes are present; current can flow in any direction.
pub struct NoDiodes {
    _private: (),
}

impl DiodeConfiguration for NoDiodes {
    const CAN_GHOST: bool = true;

    #[inline]
    fn scan_position(pos: KeyPosition) -> ScanPosition {
        ScanPosition {
            read_index: pos.col,
            write_index: pos.row,
        }
    }
}
