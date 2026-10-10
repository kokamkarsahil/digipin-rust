use crate::Coordinates;

/// Geographic bounds of a DIGIPIN cell, in decimal degrees (WGS 84).
///
/// DIGIPIN cells include their south and west edges and exclude their north
/// and east edges. This assigns locations on grid lines to the adjoining
/// north or east cell. Cells along the outer coverage limits also include
/// the north edge at 38.5° and the east edge at 99.5°.
///
/// The fields are public for convenient geographic calculations. Constructing
/// or modifying a value does not validate the bounds.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CellBounds {
    /// Southern latitude in decimal degrees.
    pub south: f64,
    /// Western longitude in decimal degrees.
    pub west: f64,
    /// Northern latitude in decimal degrees.
    pub north: f64,
    /// Eastern longitude in decimal degrees.
    pub east: f64,
}

impl CellBounds {
    /// Stores south, west, north, and east bounds in degrees without validation.
    pub(crate) const fn new(south: f64, west: f64, north: f64, east: f64) -> Self {
        Self {
            south,
            west,
            north,
            east,
        }
    }

    /// Returns the center of these bounds.
    ///
    /// Decoding a DIGIPIN returns this location, rather than recovering the
    /// original coordinates that were encoded.
    pub const fn center(self) -> Coordinates {
        Coordinates::new(
            (self.south + self.north) / 2.0,
            (self.west + self.east) / 2.0,
        )
    }

    /// Checks whether a location belongs to these bounds.
    ///
    /// South and west edges are included. North and east edges are excluded,
    /// except at the outer coverage limits of 38.5° north and 99.5° east.
    /// Nonfinite coordinates are outside DIGIPIN cell bounds.
    pub fn contains(self, coordinates: Coordinates) -> bool {
        coordinates.latitude >= self.south
            && (coordinates.latitude < self.north
                || (self.north == 38.5 && coordinates.latitude == self.north))
            && coordinates.longitude >= self.west
            && (coordinates.longitude < self.east
                || (self.east == 99.5 && coordinates.longitude == self.east))
    }

    /// Returns the east–west extent in degrees of longitude.
    ///
    /// Its physical distance depends on latitude; it is not a distance in
    /// meters.
    pub const fn width_degrees(self) -> f64 {
        self.east - self.west
    }

    /// Returns the north–south extent in degrees of latitude.
    pub const fn height_degrees(self) -> f64 {
        self.north - self.south
    }
}
