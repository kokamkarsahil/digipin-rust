use crate::{
    CELL_WIDTH_DEGREES, COVERAGE, DIGIPIN_LENGTH, Digipin, DigipinError, DigipinResult,
    constants::{CELLS_PER_AXIS, GRID},
};

/// Encodes WGS84 latitude and longitude in degrees into a full-resolution DIGIPIN.
///
/// Exact grid lines belong to the north/east cell. The maximum latitude and
/// longitude are included in the last cell on their respective axes.
///
/// # Errors
///
/// Returns [`DigipinError::LatitudeOutOfRange`] or
/// [`DigipinError::LongitudeOutOfRange`] for coordinates outside [`COVERAGE`],
/// including NaN and infinity. Latitude is checked first.
///
/// ```
/// let pin = digipin::encode(28.622788, 77.213033)?;
/// assert_eq!(pin.as_str(), "39J49LL8T4");
/// # Ok::<(), digipin::DigipinError>(())
/// ```
pub fn encode(latitude: f64, longitude: f64) -> DigipinResult<Digipin> {
    if !(COVERAGE.south..=COVERAGE.north).contains(&latitude) {
        return Err(DigipinError::LatitudeOutOfRange(latitude));
    }
    if !(COVERAGE.west..=COVERAGE.east).contains(&longitude) {
        return Err(DigipinError::LongitudeOutOfRange(longitude));
    }

    // The cell width is an exact dyadic fraction. Dividing by it avoids a
    // pre-rounded reciprocal. Validated, nonnegative casts truncate like floor
    // without requiring libm; the inclusive outer edges are clamped.
    let south_index =
        (((latitude - COVERAGE.south) / CELL_WIDTH_DEGREES) as u32).min(CELLS_PER_AXIS - 1);
    let west_index =
        (((longitude - COVERAGE.west) / CELL_WIDTH_DEGREES) as u32).min(CELLS_PER_AXIS - 1);
    let north_index = CELLS_PER_AXIS - 1 - south_index;

    let mut symbols = [0; DIGIPIN_LENGTH];
    for (level, symbol) in symbols.iter_mut().enumerate() {
        let shift = 2 * (DIGIPIN_LENGTH - level - 1);
        let row = ((north_index >> shift) & 3) as usize;
        let col = ((west_index >> shift) & 3) as usize;
        *symbol = GRID[row][col];
    }
    Ok(Digipin::from_symbols(symbols))
}
