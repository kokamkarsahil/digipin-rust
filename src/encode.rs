use crate::{constants::{BOUNDS, DIGIPIN_GRID, POWER, SPAN}, error::DigipinResult};

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

    let frac_lat = (BOUNDS.max_lat - latitude) / SPAN;
    // We use `min(POWER - 1)` to handle the edge case where latitude is exactly `BOUNDS.min_lat`.
    // In that case, `frac_lat` is 1.0, and `(frac_lat * POWER)` would be `POWER`, which is out of bounds for a 0-indexed system.
    let idx_lat = ((frac_lat * (POWER as f64)) as u32).min(POWER - 1);

    let frac_lon = (longitude - BOUNDS.min_lon) / SPAN;
    // We use `min(POWER - 1)` to handle the edge case where longitude is exactly `BOUNDS.max_lon`.
    // In that case, `frac_lon` is 1.0, and `(frac_lon * POWER)` would be `POWER`, which is out of bounds for a 0-indexed system.
    let idx_lon = ((frac_lon * (POWER as f64)) as u32).min(POWER - 1);

    let mut chars = ['\0'; 10];
    for level in 0..10 {
        let shift = 18 - 2 * level;
        let row = ((idx_lat >> shift) & 3) as usize;
        let col = ((idx_lon >> shift) & 3) as usize;
        chars[level] = DIGIPIN_GRID[row][col];
    }

    let mut s = String::with_capacity(12);
    s.extend(&chars[0..3]);
    s.push('-');
    s.extend(&chars[3..6]);
    s.push('-');
    s.extend(&chars[6..10]);

    Ok(s)
} 