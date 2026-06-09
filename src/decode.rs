use crate::{constants::{BOUNDS, LOOKUP, POWER, SPAN}, coordinates::Coordinates, error::DigipinResult};

/// Decodes a DIGIPIN string into its corresponding geographical coordinates.
///
/// This function accepts DIGIPIN codes both with and without hyphens. It returns the
/// center coordinates of the geographical area the DIGIPIN represents.
///
/// # Arguments
///
/// * `digipin` - The DIGIPIN string to decode.
///
/// # Returns
///
/// A `DigipinResult` containing the `Coordinates` on success, or a `DigipinError` if
/// the DIGIPIN string is invalid (e.g., wrong length, invalid characters).
///
/// # Example
///
/// ```
/// use digipin::get_coordinates_from_digipin;
///
/// let coords = get_coordinates_from_digipin("39J-438-TJC7").unwrap();
/// assert!((coords.latitude - 28.6139).abs() < 1e-4);
/// assert!((coords.longitude - 77.2090).abs() < 1e-4);
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

    let frac_lat = (idx_lat as f64 + 0.5) / (POWER as f64);
    let center_lat = BOUNDS.max_lat - frac_lat * SPAN;
    let frac_lon = (idx_lon as f64 + 0.5) / (POWER as f64);
    let center_lon = BOUNDS.min_lon + frac_lon * SPAN;

    Ok(Coordinates { latitude: center_lat, longitude: center_lon })
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