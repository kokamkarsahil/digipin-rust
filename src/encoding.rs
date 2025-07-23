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

    let mut vec_u8 = Vec::with_capacity(12);

    vec_u8.push(DIGIPIN_GRID[((idx_lat >> 18) & 3) as usize][((idx_lon >> 18) & 3) as usize]);
    vec_u8.push(DIGIPIN_GRID[((idx_lat >> 16) & 3) as usize][((idx_lon >> 16) & 3) as usize]);
    vec_u8.push(DIGIPIN_GRID[((idx_lat >> 14) & 3) as usize][((idx_lon >> 14) & 3) as usize]);
    vec_u8.push(b'-');
    vec_u8.push(DIGIPIN_GRID[((idx_lat >> 12) & 3) as usize][((idx_lon >> 12) & 3) as usize]);
    vec_u8.push(DIGIPIN_GRID[((idx_lat >> 10) & 3) as usize][((idx_lon >> 10) & 3) as usize]);
    vec_u8.push(DIGIPIN_GRID[((idx_lat >> 8) & 3) as usize][((idx_lon >> 8) & 3) as usize]);
    vec_u8.push(b'-');
    vec_u8.push(DIGIPIN_GRID[((idx_lat >> 6) & 3) as usize][((idx_lon >> 6) & 3) as usize]);
    vec_u8.push(DIGIPIN_GRID[((idx_lat >> 4) & 3) as usize][((idx_lon >> 4) & 3) as usize]);
    vec_u8.push(DIGIPIN_GRID[((idx_lat >> 2) & 3) as usize][((idx_lon >> 2) & 3) as usize]);
    vec_u8.push(DIGIPIN_GRID[(idx_lat & 3) as usize][(idx_lon & 3) as usize]);

    Ok(unsafe { String::from_utf8_unchecked(vec_u8) })
} 