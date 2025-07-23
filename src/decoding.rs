use crate::{constants::*, errors::*, types::*};

const LOOKUP: [Option<(u8, u8)>; 128] = {
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
    let (idx_lat, idx_lon) = if digipin.is_ascii() {
        parse_ascii(digipin)?
    } else {
        parse_unicode(digipin)?
    };
    let frac_lat = (idx_lat as f64 + 0.5) / POWER_F;
    let center_lat = BOUNDS.max_lat - frac_lat * SPAN;
    let frac_lon = (idx_lon as f64 + 0.5) / POWER_F;
    let center_lon = BOUNDS.min_lon + frac_lon * SPAN;
    Ok(Coordinates::new(center_lat, center_lon))
}

/// Find the position of a character in the DIGIPIN grid
fn find_char_in_grid(ch: char) -> DigipinResult<(u8, u8)> {
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

fn parse_ascii(digipin: &str) -> DigipinResult<(u32, u32)> {
    debug_assert!(digipin.is_ascii());
    let bytes = digipin.as_bytes();
    let len = bytes.len();

    if len == 10 {
        let mut idx_lat = 0u32;
        let mut idx_lon = 0u32;
        for i in 0..10 {
            let b = bytes[i];
            let (row, col) = LOOKUP[b as usize].ok_or(DigipinError::InvalidCharacter(b as char))?;
            idx_lat = (idx_lat << 2) | row as u32;
            idx_lon = (idx_lon << 2) | col as u32;
        }
        return Ok((idx_lat, idx_lon));
    } else if len == 12 && bytes[3] == b'-' && bytes[7] == b'-' {
        let mut idx_lat = 0u32;
        let mut idx_lon = 0u32;
        let indices = [0, 1, 2, 4, 5, 6, 8, 9, 10, 11];
        for &i in &indices {
            let b = bytes[i];
            let (row, col) = LOOKUP[b as usize].ok_or(DigipinError::InvalidCharacter(b as char))?;
            idx_lat = (idx_lat << 2) | row as u32;
            idx_lon = (idx_lon << 2) | col as u32;
        }
        return Ok((idx_lat, idx_lon));
    }

    let mut i = 0;
    let mut idx_lat: u32 = 0;
    let mut idx_lon: u32 = 0;
    let mut count = 0;
    while i < len && count < 10 {
        let b = bytes[i];
        i += 1;
        if b == b'-' {
            continue;
        }
        let (row, col) = LOOKUP[b as usize].ok_or(DigipinError::InvalidCharacter(b as char))?;
        idx_lat = (idx_lat << 2) | row as u32;
        idx_lon = (idx_lon << 2) | col as u32;
        count += 1;
    }
    if count < 10 {
        return Err(DigipinError::InvalidLength(count));
    }
    while i < len {
        let b = bytes[i];
        i += 1;
        if b != b'-' {
            return Err(DigipinError::InvalidLength(count + 1));
        }
    }
    Ok((idx_lat, idx_lon))
}

fn parse_unicode(digipin: &str) -> DigipinResult<(u32, u32)> {
    let mut char_iter = digipin.chars().filter(|&c| c != '-');
    let mut idx_lat: u32 = 0;
    let mut idx_lon: u32 = 0;
    let mut count = 0;
    for _ in 0..10 {
        let ch = char_iter.next().ok_or(DigipinError::InvalidLength(count))?;
        let (row, col) = find_char_in_grid(ch)?;
        idx_lat = (idx_lat << 2) | row as u32;
        idx_lon = (idx_lon << 2) | col as u32;
        count += 1;
    }
    if char_iter.next().is_some() {
        Err(DigipinError::InvalidLength(11))
    } else {
        Ok((idx_lat, idx_lon))
    }
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
    let (idx_lat, idx_lon) = if digipin.is_ascii() {
        parse_ascii(digipin)?
    } else {
        parse_unicode(digipin)?
    };
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