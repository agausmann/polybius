//! Typestate for declaring diode presence and direction.

pub struct KeyPosition {
    pub row: usize,
    pub col: usize,
}

pub struct ScanPosition {
    pub write_index: usize,
    pub read_index: usize,
}

pub trait DiodeConfiguration<const ROWS: usize, const COLS: usize> {
    const CAN_GHOST: bool;
    const WRITE_LINES: usize;
    const READ_LINES: usize;

    fn scan_position(pos: KeyPosition) -> ScanPosition;

    fn key_position(pos: ScanPosition) -> KeyPosition;
}

/// Diodes are present; current is allowed to flow from rows to columns.
pub struct RowToCol {
    _private: (),
}

impl<const ROWS: usize, const COLS: usize> DiodeConfiguration<ROWS, COLS> for RowToCol {
    const CAN_GHOST: bool = false;
    const WRITE_LINES: usize = COLS;
    const READ_LINES: usize = ROWS;

    #[inline]
    fn scan_position(pos: KeyPosition) -> ScanPosition {
        ScanPosition {
            read_index: pos.row,
            write_index: pos.col,
        }
    }

    #[inline]
    fn key_position(pos: ScanPosition) -> KeyPosition {
        KeyPosition {
            row: pos.read_index,
            col: pos.write_index,
        }
    }
}

/// Diodes are present; current is allowed to flow from columns to rows.
pub struct ColToRow {
    _private: (),
}

impl<const ROWS: usize, const COLS: usize> DiodeConfiguration<ROWS, COLS> for ColToRow {
    const CAN_GHOST: bool = false;
    const WRITE_LINES: usize = ROWS;
    const READ_LINES: usize = COLS;

    #[inline]
    fn scan_position(pos: KeyPosition) -> ScanPosition {
        ScanPosition {
            read_index: pos.col,
            write_index: pos.row,
        }
    }

    #[inline]
    fn key_position(pos: ScanPosition) -> KeyPosition {
        KeyPosition {
            row: pos.write_index,
            col: pos.read_index,
        }
    }
}

/// No diodes are present; current can flow in any direction.
pub struct NoDiodes {
    _private: (),
}

impl<const ROWS: usize, const COLS: usize> DiodeConfiguration<ROWS, COLS> for NoDiodes {
    const CAN_GHOST: bool = true;
    const WRITE_LINES: usize = ROWS;
    const READ_LINES: usize = COLS;

    #[inline]
    fn scan_position(pos: KeyPosition) -> ScanPosition {
        ScanPosition {
            read_index: pos.col,
            write_index: pos.row,
        }
    }

    #[inline]
    fn key_position(pos: ScanPosition) -> KeyPosition {
        KeyPosition {
            row: pos.write_index,
            col: pos.read_index,
        }
    }
}
