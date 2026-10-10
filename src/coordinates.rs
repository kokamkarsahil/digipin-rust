/// A latitude and longitude pair in decimal degrees (WGS 84 / EPSG:4326).
///
/// This is a general coordinate value: construction does not validate the
/// coordinates or restrict them to the DIGIPIN coverage area. [`Self::encode`]
/// checks that both values are finite and within [`crate::COVERAGE`].
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Coordinates {
    /// Latitude in decimal degrees, increasing northward.
    pub latitude: f64,
    /// Longitude in decimal degrees, increasing eastward.
    pub longitude: f64,
}

impl Coordinates {
    /// Creates a coordinate pair without validation.
    ///
    /// # Examples
    ///
    /// ```
    /// use digipin::Coordinates;
    ///
    /// let c = Coordinates::new(12.34, 56.78);
    /// assert_eq!(c.latitude, 12.34);
    /// assert_eq!(c.longitude, 56.78);
    /// ```
    pub const fn new(latitude: f64, longitude: f64) -> Self {
        Self {
            latitude,
            longitude,
        }
    }

    /// Encodes this location as a DIGIPIN.
    ///
    /// # Errors
    ///
    /// Returns an error when either coordinate is nonfinite or outside the
    /// inclusive DIGIPIN coverage limits: latitude 2.5–38.5° north and
    /// longitude 63.5–99.5° east.
    ///
    /// # Examples
    ///
    /// ```
    /// use digipin::Coordinates;
    ///
    /// let pin = Coordinates::new(28.6139, 77.2090).encode()?;
    /// assert_eq!(pin.as_str().len(), 10);
    /// # Ok::<(), digipin::DigipinError>(())
    /// ```
    pub fn encode(self) -> crate::DigipinResult<crate::Digipin> {
        crate::encode(self.latitude, self.longitude)
    }
}
