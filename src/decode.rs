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

    while let Some(ch) = char_iter.next() {
        if count == 10 {
            // Too many characters
            return Err(crate::error::DigipinError::InvalidLength(11));
        }
        let (row, col) = find_char_in_grid(ch)?;
        idx_lat = (idx_lat << 2) | row as u32;
        idx_lon = (idx_lon << 2) | col as u32;
        count += 1;
    }

    if count != 10 {
        return Err(crate::error::DigipinError::InvalidLength(count));
    }

    let frac_lat = (idx_lat as f64 + 0.5) / (POWER as f64);
    let center_lat = BOUNDS.max_lat - frac_lat * SPAN;
    let frac_lon = (idx_lon as f64 + 0.5) / (POWER as f64);
    let center_lon = BOUNDS.min_lon + frac_lon * SPAN;

    Ok(Coordinates::new(center_lat, center_lon))
}

/// Find the position of a character in the DIGIPIN grid
const fn find_char_in_grid(ch: char) -> DigipinResult<(usize, usize)> {
    let idx = ch as u32;
    if idx > 127 {
        return Err(crate::error::DigipinError::InvalidCharacter(ch));
    }
    match LOOKUP[idx as usize] {
        Some((row, col)) => Ok((row as usize, col as usize)),
        None => Err(crate::error::DigipinError::InvalidCharacter(ch)),
    }
} 