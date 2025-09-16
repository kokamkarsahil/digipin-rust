#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

mod constants;
mod coordinates;
mod decode;
mod encode;
mod error;

pub use coordinates::Coordinates;
pub use error::{DigipinError, DigipinResult};
pub use encode::get_digipin;
pub use decode::get_coordinates_from_digipin;

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
    fn test_boundary_roundtrip() {
        use super::constants::{BOUNDS, SPAN, POWER};
        let half_cell = (SPAN / (POWER as f64)) / 2.0;
        let corners = [
            (BOUNDS.min_lat, BOUNDS.min_lon),
            (BOUNDS.min_lat, BOUNDS.max_lon),
            (BOUNDS.max_lat, BOUNDS.min_lon),
            (BOUNDS.max_lat, BOUNDS.max_lon),
        ];
        for &(orig_lat, orig_lon) in &corners {
            let digipin = get_digipin(orig_lat, orig_lon).unwrap();
            let decoded = get_coordinates_from_digipin(&digipin).unwrap();
            assert!((decoded.latitude - orig_lat).abs() <= half_cell + 1e-10);
            assert!((decoded.longitude - orig_lon).abs() <= half_cell + 1e-10);
        }
    }
}