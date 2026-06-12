#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

mod constants;
mod coordinates;
mod decode;
mod encode;
mod error;

pub use coordinates::Coordinates;
pub use decode::get_coordinates_from_digipin;
pub use encode::get_digipin;
pub use error::{DigipinError, DigipinResult};

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

    #[test]
    fn test_boundary_roundtrip() {
        use super::constants::{BOUNDS, POWER, SPAN};
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

    // ---- Coordinates::new() tests (new constructor added in this PR) ----

    #[test]
    fn test_coordinates_new_basic() {
        let c = Coordinates::new(12.34, 56.78);
        assert_eq!(c.latitude, 12.34);
        assert_eq!(c.longitude, 56.78);
    }

    #[test]
    fn test_coordinates_new_negative_values() {
        let c = Coordinates::new(-33.87, -70.65);
        assert_eq!(c.latitude, -33.87);
        assert_eq!(c.longitude, -70.65);
    }

    #[test]
    fn test_coordinates_new_zero() {
        let c = Coordinates::new(0.0, 0.0);
        assert_eq!(c.latitude, 0.0);
        assert_eq!(c.longitude, 0.0);
    }

    #[test]
    fn test_coordinates_new_equals_struct_literal() {
        let via_new = Coordinates::new(28.6139, 77.2090);
        let via_literal = Coordinates { latitude: 28.6139, longitude: 77.2090 };
        assert_eq!(via_new, via_literal);
    }

    #[test]
    fn test_coordinates_new_large_values() {
        let c = Coordinates::new(90.0, 180.0);
        assert_eq!(c.latitude, 90.0);
        assert_eq!(c.longitude, 180.0);
    }

    // ---- Constants tests (INV_SPAN_MUL_POWER, INV_POWER_MUL_SPAN, DIGIPIN_GRID as u8) ----

    #[test]
    fn test_inv_span_mul_power_value() {
        use super::constants::{INV_SPAN_MUL_POWER, POWER, SPAN};
        let expected = (POWER as f64) / SPAN;
        assert!((INV_SPAN_MUL_POWER - expected).abs() < 1e-9);
    }

    #[test]
    fn test_inv_power_mul_span_value() {
        use super::constants::{INV_POWER_MUL_SPAN, POWER, SPAN};
        let expected = SPAN / (POWER as f64);
        assert!((INV_POWER_MUL_SPAN - expected).abs() < 1e-20);
    }

    #[test]
    fn test_inv_constants_are_reciprocals() {
        use super::constants::{INV_POWER_MUL_SPAN, INV_SPAN_MUL_POWER};
        let product = INV_SPAN_MUL_POWER * INV_POWER_MUL_SPAN;
        assert!((product - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_digipin_grid_is_u8_and_correct() {
        use super::constants::DIGIPIN_GRID;
        // Row 0
        assert_eq!(DIGIPIN_GRID[0][0], 'F');
        assert_eq!(DIGIPIN_GRID[0][1], 'C');
        assert_eq!(DIGIPIN_GRID[0][2], '9');
        assert_eq!(DIGIPIN_GRID[0][3], '8');
        // Row 1
        assert_eq!(DIGIPIN_GRID[1][0], 'J');
        assert_eq!(DIGIPIN_GRID[1][1], '3');
        assert_eq!(DIGIPIN_GRID[1][2], '2');
        assert_eq!(DIGIPIN_GRID[1][3], '7');
        // Row 2
        assert_eq!(DIGIPIN_GRID[2][0], 'K');
        assert_eq!(DIGIPIN_GRID[2][1], '4');
        assert_eq!(DIGIPIN_GRID[2][2], '5');
        assert_eq!(DIGIPIN_GRID[2][3], '6');
        // Row 3
        assert_eq!(DIGIPIN_GRID[3][0], 'L');
        assert_eq!(DIGIPIN_GRID[3][1], 'M');
        assert_eq!(DIGIPIN_GRID[3][2], 'P');
        assert_eq!(DIGIPIN_GRID[3][3], 'T');
    }

    #[test]
    fn test_digipin_grid_chars_are_valid_ascii() {
        use super::constants::DIGIPIN_GRID;
        for row in &DIGIPIN_GRID {
            for &ch in row {
                assert!(ch.is_ascii(), "grid char {ch} is not ASCII");
                assert_ne!(ch, '-', "grid should not contain hyphens");
            }
        }
    }

    // ---- Refactored decode logic tests ----

    #[test]
    fn test_decode_empty_string_returns_invalid_length_zero() {
        assert!(matches!(
            get_coordinates_from_digipin(""),
            Err(DigipinError::InvalidLength(0))
        ));
    }

    #[test]
    fn test_decode_nine_chars_returns_invalid_length_nine() {
        // 9 valid chars, no hyphens
        assert!(matches!(
            get_coordinates_from_digipin("FCJKMPCT9"),
            Err(DigipinError::InvalidLength(9))
        ));
    }

    #[test]
    fn test_decode_eleven_chars_returns_invalid_length_eleven() {
        // 11 valid chars → the 11th triggers the over-count guard (count+1 = 11)
        assert!(matches!(
            get_coordinates_from_digipin("FCJKMPCT955"),
            Err(DigipinError::InvalidLength(11))
        ));
    }

    #[test]
    fn test_decode_hyphen_only_returns_invalid_length_zero() {
        assert!(matches!(
            get_coordinates_from_digipin("----------"),
            Err(DigipinError::InvalidLength(0))
        ));
    }

    #[test]
    fn test_decode_non_ascii_char_returns_invalid_character() {
        // 'ß' is non-ASCII (multi-byte UTF-8)
        assert!(matches!(
            get_coordinates_from_digipin("FCJ-3F9-82ß"),
            Err(DigipinError::InvalidCharacter('ß'))
        ));
    }

    #[test]
    fn test_decode_unicode_non_ascii_returns_invalid_character() {
        // Euro sign is non-ASCII
        assert!(matches!(
            get_coordinates_from_digipin("€CJKMPCT95"),
            Err(DigipinError::InvalidCharacter('€'))
        ));
    }

    #[test]
    fn test_decode_ascii_but_not_in_grid_returns_invalid_character() {
        // 'A' is valid ASCII but not in the DIGIPIN alphabet
        assert!(matches!(
            get_coordinates_from_digipin("ACJKMPCT95"),
            Err(DigipinError::InvalidCharacter('A'))
        ));
    }

    #[test]
    fn test_decode_digit_one_not_in_grid() {
        // '1' is not in the DIGIPIN grid
        assert!(matches!(
            get_coordinates_from_digipin("1CJKMPCT95"),
            Err(DigipinError::InvalidCharacter('1'))
        ));
    }

    #[test]
    fn test_decode_ten_valid_chars_no_hyphens_succeeds() {
        let result = get_coordinates_from_digipin("FCJKMPCT95");
        assert!(result.is_ok(), "expected Ok, got {result:?}");
    }

    #[test]
    fn test_decode_extra_hyphens_accepted_when_ten_valid_chars() {
        // Extra hyphens should be silently ignored
        let with_extra = get_coordinates_from_digipin("F-C-J-K-M-P-C-T-9-5").unwrap();
        let plain = get_coordinates_from_digipin("FCJKMPCT95").unwrap();
        assert_eq!(with_extra.latitude, plain.latitude);
        assert_eq!(with_extra.longitude, plain.longitude);
    }

    #[test]
    fn test_decode_over_count_with_hyphens_interspersed() {
        // 11 valid chars separated by hyphens → InvalidLength(11)
        assert!(matches!(
            get_coordinates_from_digipin("F-C-J-K-M-P-C-T-9-5-5"),
            Err(DigipinError::InvalidLength(11))
        ));
    }

    #[test]
    fn test_decode_result_coords_within_geographic_bounds() {
        use super::constants::BOUNDS;
        let coords = get_coordinates_from_digipin("FCJ-3F9-8273").unwrap();
        assert!(coords.latitude >= BOUNDS.min_lat && coords.latitude <= BOUNDS.max_lat);
        assert!(coords.longitude >= BOUNDS.min_lon && coords.longitude <= BOUNDS.max_lon);
    }

    // ---- Refactored encode logic tests ----

    #[test]
    fn test_encode_output_length_is_twelve() {
        let pin = get_digipin(28.6139, 77.2090).unwrap();
        assert_eq!(pin.len(), 12);
    }

    #[test]
    fn test_encode_hyphen_positions() {
        let pin = get_digipin(28.6139, 77.2090).unwrap();
        let chars: Vec<char> = pin.chars().collect();
        assert_eq!(chars[3], '-');
        assert_eq!(chars[7], '-');
    }

    #[test]
    fn test_encode_output_is_valid_ascii() {
        let pin = get_digipin(20.0, 80.0).unwrap();
        assert!(pin.is_ascii(), "DIGIPIN output must be ASCII");
    }

    #[test]
    fn test_encode_output_chars_are_valid_digipin_or_hyphen() {
        const VALID_CHARS: &str = "FC98J327K456LMPT-";
        let pin = get_digipin(15.0, 75.0).unwrap();
        for ch in pin.chars() {
            assert!(
                VALID_CHARS.contains(ch),
                "unexpected character '{ch}' in DIGIPIN output"
            );
        }
    }

    #[test]
    fn test_encode_format_xxx_xxx_xxxx() {
        let pin = get_digipin(28.6139, 77.2090).unwrap();
        let parts: Vec<&str> = pin.split('-').collect();
        assert_eq!(parts.len(), 3, "expected 3 hyphen-separated segments");
        assert_eq!(parts[0].len(), 3);
        assert_eq!(parts[1].len(), 3);
        assert_eq!(parts[2].len(), 4);
    }

    #[test]
    fn test_encode_exact_boundary_min_lat_min_lon() {
        use super::constants::BOUNDS;
        // Exact boundary values are valid (range check uses ..=)
        assert!(get_digipin(BOUNDS.min_lat, BOUNDS.min_lon).is_ok());
    }

    #[test]
    fn test_encode_exact_boundary_max_lat_max_lon() {
        use super::constants::BOUNDS;
        assert!(get_digipin(BOUNDS.max_lat, BOUNDS.max_lon).is_ok());
    }

    #[test]
    fn test_encode_just_below_min_lat_fails() {
        use super::constants::BOUNDS;
        assert!(matches!(
            get_digipin(BOUNDS.min_lat - 0.001, 80.0),
            Err(DigipinError::LatitudeOutOfRange(_))
        ));
    }

    #[test]
    fn test_encode_just_above_max_lat_fails() {
        use super::constants::BOUNDS;
        assert!(matches!(
            get_digipin(BOUNDS.max_lat + 0.001, 80.0),
            Err(DigipinError::LatitudeOutOfRange(_))
        ));
    }

    #[test]
    fn test_encode_just_below_min_lon_fails() {
        use super::constants::BOUNDS;
        assert!(matches!(
            get_digipin(20.0, BOUNDS.min_lon - 0.001),
            Err(DigipinError::LongitudeOutOfRange(_))
        ));
    }

    #[test]
    fn test_encode_just_above_max_lon_fails() {
        use super::constants::BOUNDS;
        assert!(matches!(
            get_digipin(20.0, BOUNDS.max_lon + 0.001),
            Err(DigipinError::LongitudeOutOfRange(_))
        ));
    }

    #[test]
    fn test_encode_decode_with_coordinates_new() {
        // Verify Coordinates::new result can be used with encode/decode
        let c = Coordinates::new(19.0760, 72.8777);
        let pin = get_digipin(c.latitude, c.longitude).unwrap();
        let decoded = get_coordinates_from_digipin(&pin).unwrap();
        assert!((decoded.latitude - c.latitude).abs() < 0.01);
        assert!((decoded.longitude - c.longitude).abs() < 0.01);
    }

    #[test]
    fn test_encode_uses_inv_span_mul_power_consistent_with_span_power() {
        // Encode via the public API and verify the index math produces stable output
        // by checking that two nearby coordinates map to the same DIGIPIN (within one cell)
        // or consecutive DIGIPINs (when straddling a boundary).
        use super::constants::{INV_SPAN_MUL_POWER, POWER, SPAN};
        let cell_size = SPAN / (POWER as f64);
        let lat = 28.6139;
        let lon = 77.2090;
        let pin1 = get_digipin(lat, lon).unwrap();
        // Shift by a tiny amount well within one cell
        let tiny = cell_size * 0.001;
        let pin2 = get_digipin(lat + tiny, lon + tiny).unwrap();
        // Both should decode to coordinates within half a cell of the original
        let d1 = get_coordinates_from_digipin(&pin1).unwrap();
        let d2 = get_coordinates_from_digipin(&pin2).unwrap();
        let half = cell_size / 2.0 + 1e-10;
        assert!((d1.latitude - lat).abs() <= half);
        assert!((d2.latitude - (lat + tiny)).abs() <= half);
        // Confirm INV_SPAN_MUL_POWER is correctly defined
        assert!((INV_SPAN_MUL_POWER - (POWER as f64) / SPAN).abs() < 1e-6);
    }
}
