use core::fmt;

/// Errors produced by coordinate validation and DIGIPIN parsing.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum DigipinError {
    /// Latitude is not finite or is outside the inclusive range `2.5..=38.5`.
    LatitudeOutOfRange(f64),
    /// Longitude is not finite or is outside the inclusive range `63.5..=99.5`.
    LongitudeOutOfRange(f64),
    /// The input is not exactly ten ASCII bytes long.
    InvalidLength {
        /// Actual input length in bytes.
        actual: usize,
    },
    /// A character is not a member of the DIGIPIN alphabet.
    InvalidCharacter {
        /// Zero-based byte offset of the invalid character in the canonical input.
        index: usize,
        /// The invalid Unicode scalar value, preserved for diagnostics.
        character: char,
    },
    /// Display input is neither a continuous code nor an exact `XXX XXXX XXX` grouping.
    InvalidDisplayFormat,
}

impl fmt::Display for DigipinError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LatitudeOutOfRange(value) => write!(
                f,
                "latitude {value} must be finite and between 2.5 and 38.5 degrees"
            ),
            Self::LongitudeOutOfRange(value) => write!(
                f,
                "longitude {value} must be finite and between 63.5 and 99.5 degrees"
            ),
            Self::InvalidLength { actual } => {
                write!(f, "invalid DIGIPIN length: {actual} bytes (expected 10)")
            }
            Self::InvalidCharacter { index, character } => {
                write!(f, "invalid DIGIPIN character {character:?} at byte {index}")
            }
            Self::InvalidDisplayFormat => {
                f.write_str("expected a continuous DIGIPIN or display format XXX XXXX XXX")
            }
        }
    }
}

impl core::error::Error for DigipinError {}

/// Result type used by fallible DIGIPIN operations.
pub type DigipinResult<T> = Result<T, DigipinError>;
