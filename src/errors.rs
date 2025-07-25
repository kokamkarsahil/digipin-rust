use std::fmt;

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

pub type DigipinResult<T> = Result<T, DigipinError>; 