//! # Encode Module
//!
//! This module provides the functionality to encode geographical coordinates (latitude and longitude)
//! into a DIGIPIN string. It handles validation to ensure the coordinates fall within the
//! accepted bounds for India.

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

    // Normalize latitude and longitude to a fraction between 0.0 and 1.0
    let frac_lat = (BOUNDS.max_lat - latitude) / SPAN;
    let frac_lon = (longitude - BOUNDS.min_lon) / SPAN;

    // Scale the fractional coordinates to integer indices based on the grid precision.
    // `.min(POWER - 1)` handles the edge case where the coordinate is exactly on the boundary.
    let idx_lat = ((frac_lat * (POWER as f64)) as u32).min(POWER - 1);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::DigipinError;

    #[test]
    fn test_invalid_coordinates() {
        // Test latitude out of range
        assert!(matches!(
            get_digipin(50.0, 77.0),
            Err(DigipinError::LatitudeOutOfRange(_))
        ));

        // Test longitude out of range
        assert!(matches!(
            get_digipin(28.0, 120.0),
            Err(DigipinError::LongitudeOutOfRange(_))
        ));
    }
}