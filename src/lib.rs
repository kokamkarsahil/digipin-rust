#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// DIGIPIN grid used for encoding/decoding
const DIGIPIN_GRID: [[char; 4]; 4] = [
    ['F', 'C', '9', '8'],
    ['J', '3', '2', '7'],
    ['K', '4', '5', '6'],
    ['L', 'M', 'P', 'T'],
];

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
        Self { latitude, longitude }
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
    if latitude < BOUNDS.min_lat || latitude > BOUNDS.max_lat {
        return Err(DigipinError::LatitudeOutOfRange(latitude));
    }
    if longitude < BOUNDS.min_lon || longitude > BOUNDS.max_lon {
        return Err(DigipinError::LongitudeOutOfRange(longitude));
    }

    let mut min_lat = BOUNDS.min_lat;
    let mut max_lat = BOUNDS.max_lat;
    let mut min_lon = BOUNDS.min_lon;
    let mut max_lon = BOUNDS.max_lon;

    let mut digipin = String::with_capacity(12); // 10 chars + 2 hyphens

    for level in 1..=10 {
        let lat_div = (max_lat - min_lat) / 4.0;
        let lon_div = (max_lon - min_lon) / 4.0;

        // REVERSED row logic (to match original JavaScript implementation)
        let row = 3 - ((latitude - min_lat) / lat_div).floor() as usize;
        let col = ((longitude - min_lon) / lon_div).floor() as usize;

        let row = row.min(3);
        let col = col.min(3);

        digipin.push(DIGIPIN_GRID[row][col]);

        if level == 3 || level == 6 {
            digipin.push('-');
        }

        // Update bounds (reverse logic for row)
        max_lat = min_lat + lat_div * (4 - row) as f64;
        min_lat = min_lat + lat_div * (3 - row) as f64;

        min_lon = min_lon + lon_div * col as f64;
        max_lon = min_lon + lon_div;
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
    let pin: String = digipin.chars().filter(|&c| c != '-').collect();
    
    if pin.len() != 10 {
        return Err(DigipinError::InvalidLength(pin.len()));
    }

    let mut min_lat = BOUNDS.min_lat;
    let mut max_lat = BOUNDS.max_lat;
    let mut min_lon = BOUNDS.min_lon;
    let mut max_lon = BOUNDS.max_lon;

    for ch in pin.chars() {
        let (ri, ci) = find_char_in_grid(ch)?;

        let lat_div = (max_lat - min_lat) / 4.0;
        let lon_div = (max_lon - min_lon) / 4.0;

        let lat1 = max_lat - lat_div * (ri + 1) as f64;
        let lat2 = max_lat - lat_div * ri as f64;
        let lon1 = min_lon + lon_div * ci as f64;
        let lon2 = min_lon + lon_div * (ci + 1) as f64;

        // Update bounding box for next level
        min_lat = lat1;
        max_lat = lat2;
        min_lon = lon1;
        max_lon = lon2;
    }

    let center_lat = (min_lat + max_lat) / 2.0;
    let center_lon = (min_lon + max_lon) / 2.0;

    Ok(Coordinates::new(center_lat, center_lon))
}

/// Find the position of a character in the DIGIPIN grid
fn find_char_in_grid(ch: char) -> DigipinResult<(usize, usize)> {
    for (row_idx, row) in DIGIPIN_GRID.iter().enumerate() {
        for (col_idx, &grid_char) in row.iter().enumerate() {
            if grid_char == ch {
                return Ok((row_idx, col_idx));
            }
        }
    }
    Err(DigipinError::InvalidCharacter(ch))
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