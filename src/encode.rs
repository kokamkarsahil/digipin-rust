use crate::{
    constants::{BOUNDS, DIGIPIN_GRID, ENCODE_SCALE_FACTOR, POWER},
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

    let idx_lat = (((BOUNDS.max_lat - latitude) * ENCODE_SCALE_FACTOR) as u32).min(POWER - 1);
    let idx_lon = (((longitude - BOUNDS.min_lon) * ENCODE_SCALE_FACTOR) as u32).min(POWER - 1);

    let mut buffer = [0u8; 12];
    let mut i = 0;

    for level in 0..10 {
        let shift = 18 - 2 * level;
        let row = ((idx_lat >> shift) & 3) as usize;
        let col = ((idx_lon >> shift) & 3) as usize;
        buffer[i] = DIGIPIN_GRID[row][col] as u8;
        i += 1;

        if level == 2 || level == 5 {
            buffer[i] = b'-';
            i += 1;
        }
    }

    // This is safe because DIGIPIN_GRID contains only ASCII characters.
    Ok(unsafe { std::str::from_utf8_unchecked(&buffer) }.to_string())
} 