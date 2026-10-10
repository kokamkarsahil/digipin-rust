/// Rows are ordered north to south; columns west to east.
pub(crate) const GRID: [[u8; 4]; 4] = [*b"FC98", *b"J327", *b"K456", *b"LMPT"];

pub(crate) const CELLS_PER_AXIS: u32 = 1 << (2 * crate::DIGIPIN_LENGTH);
pub(crate) const INVALID_SYMBOL: u8 = u8::MAX;

/// The high two bits encode the row, and the low two bits encode the column.
/// Generated from GRID so encoding and decoding cannot drift apart.
pub(crate) const LOOKUP: [u8; 128] = {
    let mut lookup = [INVALID_SYMBOL; 128];
    let mut row = 0;
    while row < 4 {
        let mut col = 0;
        while col < 4 {
            lookup[GRID[row][col] as usize] = (row * 4 + col) as u8;
            col += 1;
        }
        row += 1;
    }
    lookup
};
