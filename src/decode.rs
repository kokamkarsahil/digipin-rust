//! # Decode Module
//!
//! This module handles the decoding of a DIGIPIN string back into geographical
//! coordinates. It includes logic for validating the format of the DIGIPIN string
//! and converting it back to a `Coordinates` struct.

use crate::{constants::{BOUNDS, LOOKUP, POWER, SPAN}, coordinates::Coordinates, error::DigipinResult};

/// Decodes a DIGIPIN string back into its central latitude and longitude coordinates.
///
/// # Arguments
/// * `digipin` - A DIGIPIN string (with or without hyphens)
///
/// # Returns
/// A `Coordinates` struct containing the decoded latitude and longitude
///
/// # Errors
/// Returns `DigipinError` if the DIGIPIN is invalid.
///
/// # Example
/// ```
/// use digipin::get_coordinates_from_digipin;
///
/// let coords = get_coordinates_from_digipin("FCJ-3F9-8273")?;
/// println!("Latitude: {}, Longitude: {}", coords.latitude, coords.longitude);
/// # Ok::<(), digipin::DigipinError>(())
/// ```
pub fn get_coordinates_from_digipin(digipin: &str) -> DigipinResult<Coordinates> {
    let mut char_iter = digipin.chars().filter(|&c| c != '-');
    let mut idx_lat: u32 = 0;
    let mut idx_lon: u32 = 0;
    let mut count = 0;

    for _ in 0..10 {
        match char_iter.next() {
            Some(ch) => {
                let (row, col) = find_char_in_grid(ch)?;
                idx_lat = (idx_lat << 2) | row as u32;
                idx_lon = (idx_lon << 2) | col as u32;
                count += 1;
            }
            None => return Err(crate::error::DigipinError::InvalidLength(count)),
        }
    }

    if char_iter.next().is_some() {
        return Err(crate::error::DigipinError::InvalidLength(count + 1));
    }

    // Convert the integer indices back to fractional coordinates.
    // The `+ 0.5` is added to find the center of the grid cell, improving accuracy.
    let frac_lat = (idx_lat as f64 + 0.5) / (POWER as f64);
    let center_lat = BOUNDS.max_lat - frac_lat * SPAN;
    let frac_lon = (idx_lon as f64 + 0.5) / (POWER as f64);
    let center_lon = BOUNDS.min_lon + frac_lon * SPAN;

    Ok(Coordinates::new(center_lat, center_lon))
}

/// Find the position of a character in the DIGIPIN grid
fn find_char_in_grid(ch: char) -> DigipinResult<(usize, usize)> {
    let idx = ch as u32;
    if idx > 127 {
        return Err(crate::error::DigipinError::InvalidCharacter(ch));
    }
    match LOOKUP[idx as usize] {
        Some((row, col)) => Ok((row as usize, col as usize)),
        None => Err(crate::error::DigipinError::InvalidCharacter(ch)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::DigipinError;

    #[test]
    fn test_invalid_digipin() {
        // Test invalid length
        assert!(matches!(
            get_coordinates_from_digipin("FCJ-3F9"),
            Err(DigipinError::InvalidLength(_))
        ));

        // Test invalid character
        assert!(matches!(
            get_coordinates_from_digipin("FCJ-3F9-82Z3"),
            Err(DigipinError::InvalidCharacter('Z'))
        ));
    }

    #[test]
    fn test_digipin_with_hyphens() {
        let coords = get_coordinates_from_digipin("FCJ-3F9-8273").unwrap();
        let coords_no_hyphens = get_coordinates_from_digipin("FCJ3F98273").unwrap();

        assert_eq!(coords.latitude, coords_no_hyphens.latitude);
        assert_eq!(coords.longitude, coords_no_hyphens.longitude);
    }
}