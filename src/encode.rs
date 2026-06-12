use crate::{
    constants::{BOUNDS, DIGIPIN_GRID, INV_SPAN_MUL_POWER, POWER},
    error::DigipinResult,
};

/// Encodes latitude and longitude coordinates into a 10-digit alphanumeric DIGIPIN.
///
/// # Arguments
/// * `latitude` - Latitude coordinate (must be between 2.5 and 38.5)
/// * `longitude` - Longitude coordinate (must be between 63.5 and 99.5)
///
/// # Returns
/// A formatted DIGIPIN string with hyphens (e.g., "FCJ-3F9-8273")
///
/// # Errors
/// Returns `DigipinError` if coordinates are outside the valid range.
///
/// # Example
/// ```
/// use digipin::get_digipin;
///
/// let digipin = get_digipin(28.6139, 77.2090)?; // New Delhi coordinates
/// println!("DIGIPIN: {}", digipin);
/// # Ok::<(), digipin::DigipinError>(())
/// ```
pub fn get_digipin(latitude: f64, longitude: f64) -> DigipinResult<String> {
    if !(BOUNDS.min_lat..=BOUNDS.max_lat).contains(&latitude) {
        return Err(crate::error::DigipinError::LatitudeOutOfRange(latitude));
    }
    if !(BOUNDS.min_lon..=BOUNDS.max_lon).contains(&longitude) {
        return Err(crate::error::DigipinError::LongitudeOutOfRange(longitude));
    }

    let idx_lat = (((BOUNDS.max_lat - latitude) * INV_SPAN_MUL_POWER) as u32).min(POWER - 1);
    let idx_lon = (((longitude - BOUNDS.min_lon) * INV_SPAN_MUL_POWER) as u32).min(POWER - 1);

    let mut buf = vec![0u8; 12];
    let mut i = 0;
    for level in 0..10 {
        let shift = 18 - 2 * level;
        let row = ((idx_lat >> shift) & 3) as usize;
        let col = ((idx_lon >> shift) & 3) as usize;
        buf[i] = DIGIPIN_GRID[row][col];
        i += 1;
        if level == 2 || level == 5 {
            buf[i] = b'-';
            i += 1;
        }
    }

    // SAFETY: `buf` is populated exclusively with valid ASCII bytes from `DIGIPIN_GRID` and `b'-'`.
    Ok(unsafe { String::from_utf8_unchecked(buf) })
}
