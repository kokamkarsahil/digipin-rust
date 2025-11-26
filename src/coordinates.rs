#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Represents a geographical coordinate pair.
///
/// This struct holds the latitude and longitude in decimal degrees.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Coordinates {
    /// The latitude, in decimal degrees.
    pub latitude: f64,
    /// The longitude, in decimal degrees.
    pub longitude: f64,
}
