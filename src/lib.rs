#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// DIGIPIN grid used for encoding/decoding
const DIGIPIN_GRID: [[char; 4]; 4] = [
    ['F', 'C', '9', '8'],
    ['J', '3', '2', '7'],
    ['K', '4', '5', '6'],
    ['L', 'M', 'P', 'T'],
];

const SPAN: f64 = 36.0;
const POWER_F: f64 = 1_048_576.0;

/// Geographic bounds for DIGIPIN encoding (covers India)
const BOUNDS: Bounds = Bounds {
    min_lat: 2.5,
    max_lat: 38.5,
    min_lon: 63.5,
    max_lon: 99.5,
};

/// Geographic bounds structure
#[derive(Debug, Clone, Copy)]
struct Bounds {
    min_lat: f64,
    max_lat: f64,
    min_lon: f64,
    max_lon: f64,
}

/// Represents a coordinate pair with latitude and longitude
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Coordinates {
    pub latitude: f64,
    pub longitude: f64,
}

impl Coordinates {
    /// Create a new coordinate pair
    pub fn new(latitude: f64, longitude: f64) -> Self {
        Self {
            latitude,
            longitude,
        }
    }
}

/// Errors that can occur during DIGIPIN operations
#[derive(Debug, Clone, PartialEq)]
pub enum DigipinError {
    /// Latitude is outside the valid range (2.5 to 38.5)
    LatitudeOutOfRange(f64),
    /// Longitude is outside the valid range (63.5 to 99.5)
    LongitudeOutOfRange(f64),
    /// DIGIPIN string has invalid length (must be 10 characters excluding hyphens)
    InvalidLength(usize),
    /// DIGIPIN contains invalid characters
    InvalidCharacter(char),
}

impl std::fmt::Display for DigipinError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DigipinError::LatitudeOutOfRange(lat) => {
                write!(f, "Latitude {} is out of range (2.5 to 38.5)", lat)
            }
            DigipinError::LongitudeOutOfRange(lon) => {
                write!(f, "Longitude {} is out of range (63.5 to 99.5)", lon)
            }
            DigipinError::InvalidLength(len) => {
                write!(f, "Invalid DIGIPIN length: {} (expected 10)", len)
            }
            DigipinError::InvalidCharacter(ch) => {
                write!(f, "Invalid character '{}' in DIGIPIN", ch)
            }
        }
    }
}

impl std::error::Error for DigipinError {}

/// Result type for DIGIPIN operations
pub type DigipinResult<T> = Result<T, DigipinError>;

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

    let mut digipin = String::with_capacity(12);

    for i in (0..10).rev() {
        let shift = 2 * i;
        let row = ((idx_lat >> shift) & 3) as usize;
        let col = ((idx_lon >> shift) & 3) as usize;
        digipin.push(DIGIPIN_GRID[row][col]);
        if i == 7 || i == 4 {
            digipin.push('-');
        }
    }

    Ok(digipin)
}

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
    match ch {
        'F' => Ok((0, 0)),
        'C' => Ok((0, 1)),
        '9' => Ok((0, 2)),
        '8' => Ok((0, 3)),
        'J' => Ok((1, 0)),
        '3' => Ok((1, 1)),
        '2' => Ok((1, 2)),
        '7' => Ok((1, 3)),
        'K' => Ok((2, 0)),
        '4' => Ok((2, 1)),
        '5' => Ok((2, 2)),
        '6' => Ok((2, 3)),
        'L' => Ok((3, 0)),
        'M' => Ok((3, 1)),
        'P' => Ok((3, 2)),
        'T' => Ok((3, 3)),
        _ => Err(DigipinError::InvalidCharacter(ch)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_decode_roundtrip() {
        let original_lat = 28.6139;
        let original_lon = 77.2090;

        let digipin = get_digipin(original_lat, original_lon).unwrap();
        let decoded = get_coordinates_from_digipin(&digipin).unwrap();

        // Should be close to original (within DIGIPIN precision)
        assert!((decoded.latitude - original_lat).abs() < 0.01);
        assert!((decoded.longitude - original_lon).abs() < 0.01);
    }

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
