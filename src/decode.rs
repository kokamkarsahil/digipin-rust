use crate::{
    constants::{BOUNDS, INV_POWER_MUL_SPAN, LOOKUP},
    coordinates::Coordinates,
    error::DigipinResult,
};

/// Decode a DIGIPIN string into the geographic coordinates of its cell center.
///
/// The input may include hyphens for readability; exactly 10 DIGIPIN symbols (excluding hyphens)
/// are required. Invalid characters or incorrect symbol counts produce an error.
///
/// # Arguments
///
/// * `digipin` - A DIGIPIN string (hyphens are allowed and ignored).
///
/// # Returns
///
/// A `Coordinates` struct with `latitude` and `longitude` representing the cell center.
///
/// # Errors
///
/// Returns `DigipinError::InvalidLength(n)` when the number of non-hyphen symbols is not exactly 10,
/// or `DigipinError::InvalidCharacter(ch)` for characters not present in the DIGIPIN grid.
///
/// # Examples
///
/// ```
/// use digipin::{get_coordinates_from_digipin, Coordinates};
///
/// let coords = get_coordinates_from_digipin("FCJ-3F9-8273").unwrap();
/// assert!(coords.latitude.abs() <= 90.0);
/// assert!(coords.longitude.abs() <= 180.0);
/// ```
pub fn get_coordinates_from_digipin(digipin: &str) -> DigipinResult<Coordinates> {
    let mut idx_lat: u32 = 0;
    let mut idx_lon: u32 = 0;
    let mut count = 0;

    for ch in digipin.chars() {
        if ch == '-' {
            continue;
        }
        if count >= 10 {
            return Err(crate::error::DigipinError::InvalidLength(count + 1));
        }

        let (row, col) = find_char_in_grid(ch)?;
        idx_lat = (idx_lat << 2) | row as u32;
        idx_lon = (idx_lon << 2) | col as u32;
        count += 1;
    }

    if count != 10 {
        return Err(crate::error::DigipinError::InvalidLength(count));
    }

    let center_lat = BOUNDS.max_lat - (idx_lat as f64 + 0.5) * INV_POWER_MUL_SPAN;
    let center_lon = BOUNDS.min_lon + (idx_lon as f64 + 0.5) * INV_POWER_MUL_SPAN;

    Ok(Coordinates {
        latitude: center_lat,
        longitude: center_lon,
    })
}

/// Map a DIGIPIN character to its (row, column) coordinates in the DIGIPIN grid.
///
/// Returns `Ok((row, col))` when `ch` is an ASCII character with a defined entry in the internal lookup table;
/// returns `Err(DigipinError::InvalidCharacter(ch))` for non-ASCII characters or characters not present in the lookup.
///
/// # Examples
///
/// ```
/// // Succeeds for ASCII characters that exist in the DIGIPIN alphabet.
/// assert!(crate::decode::find_char_in_grid('A').is_ok());
///
/// // Non-ASCII characters are rejected.
/// assert!(crate::decode::find_char_in_grid('ß').is_err());
/// ```
fn find_char_in_grid(ch: char) -> DigipinResult<(usize, usize)> {
    if !ch.is_ascii() {
        return Err(crate::error::DigipinError::InvalidCharacter(ch));
    }
    let b = ch as u8;
    match LOOKUP[b as usize] {
        Some((row, col)) => Ok((row as usize, col as usize)),
        None => Err(crate::error::DigipinError::InvalidCharacter(ch)),
    }
}
