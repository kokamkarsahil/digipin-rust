use crate::{
    constants::{BOUNDS, INV_POWER_MUL_SPAN, LOOKUP},
    coordinates::Coordinates,
    error::DigipinResult,
};

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

/// Find the position of a character in the DIGIPIN grid
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
