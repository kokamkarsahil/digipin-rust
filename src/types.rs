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
    /// Create a new coordinate pair
    pub fn new(latitude: f64, longitude: f64) -> Self {
        Self {
            latitude,
            longitude,
        }
    }
}

/// Geographic bounds structure
#[derive(Debug, Clone, Copy)]
pub(crate) struct Bounds {
    pub(crate) min_lat: f64,
    pub(crate) max_lat: f64,
    pub(crate) min_lon: f64,
    pub(crate) max_lon: f64,
} 