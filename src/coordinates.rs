#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Represents a coordinate pair with latitude and longitude
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Coordinates {
    pub latitude: f64,
    pub longitude: f64,
}

impl Coordinates {
    /// Creates a new Coordinates instance with the given latitude and longitude
    pub fn new(latitude: f64, longitude: f64) -> Self {
        Self {
            latitude,
            longitude,
        }
    }
}
