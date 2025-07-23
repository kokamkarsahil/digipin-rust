use crate::{constants::*, errors::*, types::*};

const LOOKUP: [Option<(usize, usize)>; 128] = {
    let mut lookup = [None; 128];
    lookup['F' as usize] = Some((0, 0));
    lookup['C' as usize] = Some((0, 1));
    lookup['9' as usize] = Some((0, 2));
    lookup['8' as usize] = Some((0, 3));
    lookup['J' as usize] = Some((1, 0));
    lookup['3' as usize] = Some((1, 1));
    lookup['2' as usize] = Some((1, 2));
    lookup['7' as usize] = Some((1, 3));
    lookup['K' as usize] = Some((2, 0));
    lookup['4' as usize] = Some((2, 1));
    lookup['5' as usize] = Some((2, 2));
    lookup['6' as usize] = Some((2, 3));
    lookup['L' as usize] = Some((3, 0));
    lookup['M' as usize] = Some((3, 1));
    lookup['P' as usize] = Some((3, 2));
    lookup['T' as usize] = Some((3, 3));
    lookup
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
            None => return Err(DigipinError::InvalidLength(count)),
        }
    }

    if char_iter.next().is_some() {
        return Err(DigipinError::InvalidLength(count + 1));
    }

    let frac_lat = (idx_lat as f64 + 0.5) / POWER_F;
    let center_lat = BOUNDS.max_lat - frac_lat * SPAN;
    let frac_lon = (idx_lon as f64 + 0.5) / POWER_F;
    let center_lon = BOUNDS.min_lon + frac_lon * SPAN;

    Ok(Coordinates::new(center_lat, center_lon))
}

/// Find the position of a character in the DIGIPIN grid
fn find_char_in_grid(ch: char) -> DigipinResult<(usize, usize)> {
    if ch.is_ascii() {
        let idx = ch as usize;
        if idx < 128 {
            if let Some(pos) = LOOKUP[idx] {
                return Ok(pos);
            }
        }
    }
    Err(DigipinError::InvalidCharacter(ch))
}

/// Gets the geographic bounds for a given DIGIPIN.
///
/// # Arguments
/// * `digipin` - A DIGIPIN string (with or without hyphens)
///
/// # Returns
/// A `GeoBounds` struct containing the min/max latitude and longitude of the area
///
/// # Errors
/// Returns `DigipinError` if the DIGIPIN is invalid.
///
/// # Example
/// ```
/// use digipin::get_bounds_from_digipin;
///
/// let bounds = get_bounds_from_digipin("FCJ-3F9-8273").unwrap();
/// println!("Lat range: {} to {}", bounds.min_latitude, bounds.max_latitude);
/// # Ok::<(), digipin::DigipinError>(())
/// ```
pub fn get_bounds_from_digipin(digipin: &str) -> DigipinResult<GeoBounds> {
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
            None => return Err(DigipinError::InvalidLength(count)),
        }
    }

    if char_iter.next().is_some() {
        return Err(DigipinError::InvalidLength(count + 1));
    }

    let power = POWER_F;
    let min_frac_lat = idx_lat as f64 / power;
    let max_frac_lat = (idx_lat as f64 + 1.0) / power;
    let max_lat = BOUNDS.max_lat - min_frac_lat * SPAN;
    let min_lat = BOUNDS.max_lat - max_frac_lat * SPAN;
    let min_frac_lon = idx_lon as f64 / power;
    let max_frac_lon = (idx_lon as f64 + 1.0) / power;
    let min_lon = BOUNDS.min_lon + min_frac_lon * SPAN;
    let max_lon = BOUNDS.min_lon + max_frac_lon * SPAN;

    Ok(GeoBounds {
        min_latitude: min_lat,
        max_latitude: max_lat,
        min_longitude: min_lon,
        max_longitude: max_lon,
    })
} 