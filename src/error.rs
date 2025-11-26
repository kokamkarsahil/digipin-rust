use std::fmt;

/// Represents the possible errors that can occur during DIGIPIN encoding or decoding.
#[derive(Debug, Clone, PartialEq)]
pub enum DigipinError {
    /// Returned when the latitude is outside the valid range of `6.0..=38.0`.
    LatitudeOutOfRange(f64),
    /// Returned when the longitude is outside the valid range of `68.0..=98.0`.
    LongitudeOutOfRange(f64),
    /// Returned when the DIGIPIN string has a length other than 10 characters
    /// (excluding hyphens).
    InvalidLength(usize),
    /// Returned when the DIGIPIN string contains a character that is not part of the
    /// valid charset.
    InvalidCharacter(char),
}

impl fmt::Display for DigipinError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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