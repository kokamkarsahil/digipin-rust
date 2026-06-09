use crate::{constants::{BOUNDS, DIGIPIN_GRID, POWER, SPAN}, error::DigipinResult};

/// Encodes geographical coordinates into a 10-digit alphanumeric DIGIPIN code.
///
/// The function takes a latitude and longitude and returns a formatted DIGIPIN string
/// (e.g., "39J-438-TJC7").
///
/// # Arguments
///
/// * `latitude` - The latitude, in decimal degrees. Must be within `6.0..=38.0`.
/// * `longitude` - The longitude, in decimal degrees. Must be within `68.0..=98.0`.
///
/// # Returns
///
/// A `DigipinResult` containing the formatted DIGIPIN string on success, or a
/// `DigipinError` if the coordinates are out of bounds.
///
/// # Example
///
/// ```
/// use digipin::get_digipin;
///
/// let digipin = get_digipin(28.6139, 77.2090).unwrap(); // New Delhi
/// assert_eq!(digipin, "39J-438-TJC7");
/// ```
pub fn get_digipin(latitude: f64, longitude: f64) -> DigipinResult<String> {
    if !(BOUNDS.min_lat..=BOUNDS.max_lat).contains(&latitude) {
        return Err(crate::error::DigipinError::LatitudeOutOfRange(latitude));
    }
    if !(BOUNDS.min_lon..=BOUNDS.max_lon).contains(&longitude) {
        return Err(crate::error::DigipinError::LongitudeOutOfRange(longitude));
    }

    let frac_lat = (BOUNDS.max_lat - latitude) / SPAN;
    let idx_lat = ((frac_lat * (POWER as f64)) as u32).min(POWER - 1);
    let frac_lon = (longitude - BOUNDS.min_lon) / SPAN;
    let idx_lon = ((frac_lon * (POWER as f64)) as u32).min(POWER - 1);

    let mut digipin = String::with_capacity(12);
    for level in 0..10 {
        let shift = 18 - 2 * level;
        let row = ((idx_lat >> shift) & 3) as usize;
        let col = ((idx_lon >> shift) & 3) as usize;
        digipin.push(DIGIPIN_GRID[row][col]);
        if level == 2 || level == 5 {
            digipin.push('-');
        }
    }

    Ok(digipin)
} 