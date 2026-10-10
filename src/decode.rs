use crate::{
    CELL_WIDTH_DEGREES, COVERAGE, CellBounds, Coordinates, Digipin, DigipinResult,
    constants::LOOKUP,
};

/// Parses a DIGIPIN and returns its cell center at full `f64` precision.
///
/// ASCII lowercase is normalized to uppercase. Separators and whitespace are
/// rejected; use [`Digipin::parse_display`] for a code grouped with spaces.
/// For an already validated [`Digipin`], use [`Digipin::center`] directly.
///
/// # Errors
///
/// Returns the parsing errors documented on [`Digipin`].
///
/// ```
/// let coordinates = digipin::decode("4P3JK852C9")?;
/// assert_eq!(coordinates.latitude, 12.971601486206055);
/// assert_eq!(coordinates.longitude, 77.59458351135254);
/// # Ok::<(), digipin::DigipinError>(())
/// ```
pub fn decode(input: &str) -> DigipinResult<Coordinates> {
    input.parse::<Digipin>().map(|pin| pin.center())
}

pub(crate) fn bounds(pin: Digipin) -> CellBounds {
    let mut north_index = 0u32;
    let mut west_index = 0u32;
    for &symbol in pin.as_bytes() {
        let position = LOOKUP[symbol as usize];
        north_index = (north_index << 2) | u32::from(position >> 2);
        west_index = (west_index << 2) | u32::from(position & 3);
    }

    let north = COVERAGE.north - f64::from(north_index) * CELL_WIDTH_DEGREES;
    let west = COVERAGE.west + f64::from(west_index) * CELL_WIDTH_DEGREES;
    CellBounds::new(
        north - CELL_WIDTH_DEGREES,
        west,
        north,
        west + CELL_WIDTH_DEGREES,
    )
}
