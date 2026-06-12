use crate::{
    constants::{BOUNDS, DIGIPIN_GRID, INV_SPAN_MUL_POWER, POWER},
    error::DigipinResult,
};

/// Encodes latitude and longitude into a 10-character alphanumeric DIGIPIN formatted with hyphens.
///
/// Latitude must be between 2.5 and 38.5; longitude must be between 63.5 and 99.5. The returned
/// string contains 10 characters from the DIGIPIN alphabet with hyphens after the 3rd and 6th
/// characters (format: "XXX-XXX-XXXX").
///
/// # Errors
///
/// Returns `DigipinError::LatitudeOutOfRange(latitude)` or
/// `DigipinError::LongitudeOutOfRange(longitude)` when the inputs fall outside the valid bounds.
///
/// # Examples
///
/// ```
/// let p = get_digipin(28.6139, 77.2090).unwrap(); // New Delhi coordinates
/// assert_eq!(p.len(), 12);
/// assert_eq!(p.chars().nth(3).unwrap(), '-');
/// assert_eq!(p.chars().nth(7).unwrap(), '-');
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
