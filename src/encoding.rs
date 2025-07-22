use crate::{constants::*, errors::*};

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
        return Err(DigipinError::LatitudeOutOfRange(latitude));
    }
    if !(BOUNDS.min_lon..=BOUNDS.max_lon).contains(&longitude) {
        return Err(DigipinError::LongitudeOutOfRange(longitude));
    }

    let frac_lat = (BOUNDS.max_lat - latitude) / SPAN;
    let idx_lat = (frac_lat * POWER_F).floor().min(POWER_F - 1.0) as u32;
    let frac_lon = (longitude - BOUNDS.min_lon) / SPAN;
    let idx_lon = (frac_lon * POWER_F).floor().min(POWER_F - 1.0) as u32;

    let mut digipin = String::with_capacity(12);

    for i in (0..10).rev() {
        let shift = 2 * i;
        let row = ((idx_lat >> shift) & 3) as usize;
        let col = ((idx_lon >> shift) & 3) as usize;
        digipin.push(DIGIPIN_GRID[row][col]);
        if i == 7 || i == 4 {
            digipin.push('-');
        }
    }

    Ok(digipin)
} 