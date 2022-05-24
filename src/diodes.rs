//! Typestate for declaring diode presence and direction.

/// The logical/visual position of a key, in terms of the row and column.
pub struct KeyPosition {
    pub row: usize,
    pub col: usize,
}

/// The position of a key on the switch matrix, in terms of the pins that would
/// be read and written.
pub struct ScanPosition {
    pub write_index: usize,
    pub read_index: usize,
}

/// Mapping between logical and electrical key positions that accounts for the
/// presence and direction of any diodes in the matrix.
///
/// # FIXME: Current flow and logic level assumptions
///
/// The default implementors of this trait (`ColToRow` and `RowToCol`) assume
/// that the write lines will be driven (or pulled) high when _not_ being
/// checked, and that write line currently being checked will be driven low.
/// Likewise, it assumes that read lines are pulled high in their default state
/// and then driven low when a switch connects them to the write line currently
/// being checked. While this is the logic used by the Polybius scanner, perhaps
/// it may not always be true?
///
/// # TODO: Generalizations
///
/// The logic of this trait could be generalized to many other logical <->
/// electrical position mappings. For example, in some keyboards where the
/// number of rows is much less than the number of columns, they optimize the
/// number of matrix pins by combining adjacent columns, effectively halving the
/// number of column pins and doubling row pins in the electrical switch matrix.
///
/// As far as I know, in QMK, they only remap these at the keymap level by
/// reordering the keycodes using macros. However, Rust's const-eval
/// capabilities in theory allow us to perform this mapping at the
/// matrix/scanner level by implementing a custom "DiodeConfiguration" that also
/// incorporates such a mapping.
pub trait DiodeConfiguration<const ROWS: usize, const COLS: usize> {
    /// Whether this matrix is susceptible to ghosting.
    const CAN_GHOST: bool;

    /// The number of write lines in the electrical matrix.
    const WRITE_LINES: usize;

    /// The number of read lines in the electrical matrix.
    const READ_LINES: usize;

    /// Maps a logical position to its corresponding matrix position.
    ///
    /// This should be the inverse of [`key_position`].
    fn scan_position(pos: KeyPosition) -> ScanPosition;

    /// Maps a matrix position to its corresponding logical position.
    ///
    /// This should be the inverse [`scan_position`].
    fn key_position(pos: ScanPosition) -> KeyPosition;
}

/// Diodes are present; current is allowed to flow from rows to columns.
///
/// This diode configuration defines the matrix columns as the write lines and
/// the matrix rows as the read lines.
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
///
/// This diode configuration defines the matrix columns as the write lines and
/// the matrix rows as the read lines.
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
///
/// Although somewhat arbitrary in this case, this configuration defines the
/// matrix columns as the write lines and the matrix rows as the read lines.
///
/// This configuration will also indicate to the scanner that this matrix can
/// ghost, so that the scanner can detect and account for such cases.
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
