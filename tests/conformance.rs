//! Conformance to India Post's May 2026 standard and reference algorithm.
//!
//! Baseline: INDIAPOST-gov/digipin commit
//! a15339e8fdea93b13ae0d7336e567c9ae6ceac8e.
//! Specification: docs/DIGIPIN_Technical_Document.md, sections 2 and 4.1–4.5.
//! Reference: src/digipin.js. Coordinate centers retain full f64 precision;
//! the JavaScript API's six decimal places are only presentation rounding.

use digipin::{
    CELL_WIDTH_DEGREES, COVERAGE, CellBounds, Coordinates, DIGIPIN_LENGTH, Digipin, DigipinError,
    decode, encode,
};
use std::collections::{BTreeSet, HashSet};

const GRID: [[u8; 4]; 4] = [*b"FC98", *b"J327", *b"K456", *b"LMPT"];

// These positive-coordinate helpers work on the MSRV without f64::next_up/down.
/// Returns the immediately preceding representable float for a finite positive value.
fn below(value: f64) -> f64 {
    assert!(value.is_finite() && value > 0.0);
    f64::from_bits(value.to_bits() - 1)
}

/// Returns the immediately following representable float for a finite positive value.
fn above(value: f64) -> f64 {
    assert!(value.is_finite() && value > 0.0);
    f64::from_bits(value.to_bits() + 1)
}

/// Builds a coordinate pair without restricting values to DIGIPIN coverage.
fn coordinates(latitude: f64, longitude: f64) -> Coordinates {
    Coordinates {
        latitude,
        longitude,
    }
}

/// Independent iterative implementation of the official reference algorithm.
/// Each step subdivides the previous cell, avoiding production index arithmetic.
fn reference_encode(latitude: f64, longitude: f64) -> String {
    let (mut south, mut north) = (2.5, 38.5);
    let (mut west, mut east) = (63.5, 99.5);
    let mut code = String::with_capacity(10);
    for _ in 0..10 {
        let latitude_step = (north - south) / 4.0;
        let longitude_step = (east - west) / 4.0;
        let from_south = ((latitude - south) / latitude_step).floor() as usize;
        let row = 3 - from_south.min(3);
        let column = (((longitude - west) / longitude_step).floor() as usize).min(3);
        code.push(char::from(GRID[row][column]));
        north = south + latitude_step * (4 - row) as f64;
        south += latitude_step * (3 - row) as f64;
        west += longitude_step * column as f64;
        east = west + longitude_step;
    }
    code
}

/// Decodes a valid canonical code by recursively subdividing the official grid.
fn reference_bounds(code: &str) -> CellBounds {
    let (mut south, mut north) = (2.5, 38.5);
    let (mut west, mut east) = (63.5, 99.5);
    for symbol in code.bytes() {
        let (row, column) = GRID
            .iter()
            .enumerate()
            .find_map(|(row, cells)| {
                cells
                    .iter()
                    .position(|&value| value == symbol)
                    .map(|column| (row, column))
            })
            .unwrap();
        let latitude_step = (north - south) / 4.0;
        let longitude_step = (east - west) / 4.0;
        let new_south = north - latitude_step * (row + 1) as f64;
        north -= latitude_step * row as f64;
        south = new_south;
        let new_east = west + longitude_step * (column + 1) as f64;
        west += longitude_step * column as f64;
        east = new_east;
    }
    CellBounds {
        south,
        west,
        north,
        east,
    }
}

/// Checks reference agreement, exact cell geometry, membership, and center precision.
fn assert_conforms(latitude: f64, longitude: f64) {
    let expected_code = reference_encode(latitude, longitude);
    let pin = encode(latitude, longitude).unwrap();
    assert_eq!(pin.as_str(), expected_code, "at ({latitude}, {longitude})");
    let expected_bounds = reference_bounds(&expected_code);
    let bounds = pin.bounds();
    assert_eq!(bounds.south, expected_bounds.south);
    assert_eq!(bounds.north, expected_bounds.north);
    assert_eq!(bounds.west, expected_bounds.west);
    assert_eq!(bounds.east, expected_bounds.east);
    assert_eq!(bounds.north - bounds.south, CELL_WIDTH_DEGREES);
    assert_eq!(bounds.east - bounds.west, CELL_WIDTH_DEGREES);
    assert_eq!(bounds.width_degrees(), CELL_WIDTH_DEGREES);
    assert_eq!(bounds.height_degrees(), CELL_WIDTH_DEGREES);
    assert!(bounds.contains(coordinates(latitude, longitude)));

    let center = pin.center();
    assert_eq!(center.latitude, (bounds.south + bounds.north) / 2.0);
    assert_eq!(center.longitude, (bounds.west + bounds.east) / 2.0);
    assert!(bounds.contains(center));
    assert_eq!(decode(&expected_code).unwrap(), center);
    assert_eq!(encode(center.latitude, center.longitude).unwrap(), pin);
    assert!((center.latitude - latitude).abs() <= CELL_WIDTH_DEGREES / 2.0);
    assert!((center.longitude - longitude).abs() <= CELL_WIDTH_DEGREES / 2.0);
}

/// Verifies the published Chennai, Bangalore, and Dak Bhawan codes and exact centers.
#[test]
fn official_published_examples() {
    // README: Chennai and Bangalore. Specification section 6: Dak Bhawan.
    let vectors = [
        (
            13.11179621,
            80.20264269,
            "4T396F42L7",
            13.111780166625977,
            80.20263481140137,
        ),
        (
            12.9716,
            77.5946,
            "4P3JK852C9",
            12.971601486206055,
            77.59458351135254,
        ),
        (
            28.622788,
            77.213033,
            "39J49LL8T4",
            28.622793197631836,
            77.21304893493652,
        ),
    ];
    for (latitude, longitude, code, center_latitude, center_longitude) in vectors {
        let pin = encode(latitude, longitude).unwrap();
        assert_eq!(pin.as_str(), code);
        assert_eq!(pin.center(), coordinates(center_latitude, center_longitude));
        assert_conforms(latitude, longitude);
    }
}

/// Checks the final standard's coverage rectangle, code length, and cell width.
#[test]
fn coverage_and_precision_match_the_final_standard() {
    assert_eq!(DIGIPIN_LENGTH, 10);
    assert_eq!(COVERAGE.south, 2.5);
    assert_eq!(COVERAGE.north, 38.5);
    assert_eq!(COVERAGE.west, 63.5);
    assert_eq!(COVERAGE.east, 99.5);
    assert_eq!(CELL_WIDTH_DEGREES, 36.0 / 1_048_576.0);
}

/// Confirms all coverage corners and sampled outer edges belong to their expected cells.
#[test]
fn coverage_corners_and_outer_edges_are_included() {
    for (latitude, longitude, code) in [
        (2.5, 63.5, "LLLLLLLLLL"),
        (38.5, 63.5, "FFFFFFFFFF"),
        (2.5, 99.5, "TTTTTTTTTT"),
        (38.5, 99.5, "8888888888"),
        (20.5, 63.5, "JLLLLLLLLL"),
        (2.5, 81.5, "PLLLLLLLLL"),
        (38.5, 81.5, "9FFFFFFFFF"),
        (20.5, 99.5, "7TTTTTTTTT"),
    ] {
        assert_eq!(encode(latitude, longitude).unwrap().as_str(), code);
        assert!(COVERAGE.contains(coordinates(latitude, longitude)));
        assert_conforms(latitude, longitude);
    }
}

/// Verifies exact grid intersections select the northeast cell, including at finest depth.
#[test]
fn intersections_belong_to_the_northeast_cell() {
    for (latitude, longitude, code) in [
        (20.5, 81.5, "2LLLLLLLLL"),
        (11.5, 72.5, "4LLLLLLLLL"),
        (29.5, 90.5, "8LLLLLLLLL"),
        (
            2.5 + CELL_WIDTH_DEGREES,
            63.5 + CELL_WIDTH_DEGREES,
            "LLLLLLLLL4",
        ),
    ] {
        assert_eq!(encode(latitude, longitude).unwrap().as_str(), code);
        assert_conforms(latitude, longitude);
    }
}

/// Checks all sixteen first-level symbols against the official north-to-south grid order.
#[test]
fn first_level_grid_has_the_official_orientation() {
    for (row, cells) in GRID.iter().enumerate() {
        for (column, &symbol) in cells.iter().enumerate() {
            let latitude = 38.5 - (row as f64 + 0.5) * 9.0;
            let longitude = 63.5 + (column as f64 + 0.5) * 9.0;
            let pin = encode(latitude, longitude).unwrap();
            assert_eq!(pin.as_str().as_bytes()[0], symbol);
            assert_conforms(latitude, longitude);
        }
    }
}

/// Exercises boundary ties and adjacent floats at every subdivision depth.
#[test]
fn every_partition_depth_uses_north_and_east_boundary_ties() {
    for depth in 1..=10 {
        let partitions = 4_u32.pow(depth);
        let step = 36.0 / f64::from(partitions);
        // Exercise every line at small depths and sample dispersed lines at
        // larger depths, including lines adjoining both outer coverage edges.
        let lines: Vec<u32> = if partitions <= 256 {
            (1..partitions).collect()
        } else {
            vec![
                1,
                2,
                partitions / 4,
                partitions / 3,
                partitions / 2,
                partitions * 3 / 4,
                partitions - 2,
                partitions - 1,
            ]
        };
        for line in lines {
            let latitude = 2.5 + f64::from(line) * step;
            let longitude = 63.5 + f64::from(line) * step;
            let northeast = encode(latitude, longitude).unwrap();
            assert_eq!(
                encode(above(latitude), above(longitude)).unwrap(),
                northeast,
                "northeast tie at depth {depth}, line {line}"
            );
            assert_ne!(encode(below(latitude), longitude).unwrap(), northeast);
            assert_ne!(encode(latitude, below(longitude)).unwrap(), northeast);
            for lat in [below(latitude), latitude, above(latitude)] {
                for lon in [below(longitude), longitude, above(longitude)] {
                    assert_conforms(lat, lon);
                }
            }
        }
    }
}

/// Compares twenty thousand reproducible points across coverage with the reference algorithm.
#[test]
fn seeded_full_domain_samples_match_the_independent_reference() {
    /// Advances SplitMix64 and maps its upper 53 bits into the interval [0, 1).
    fn uniform(state: &mut u64) -> f64 {
        // SplitMix64, with 53 random bits mapped into [0, 1).
        *state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut value = *state;
        value = (value ^ (value >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        value = (value ^ (value >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        value ^= value >> 31;
        (value >> 11) as f64 / ((1_u64 << 53) as f64)
    }
    let mut state = 0x4449_4749_5049_4e32;
    for _ in 0..20_000 {
        let latitude = 2.5 + uniform(&mut state) * 36.0;
        let longitude = 63.5 + uniform(&mut state) * 36.0;
        assert_conforms(latitude, longitude);
    }
}

/// Checks half-open cell membership, outer-edge exceptions, and nonfinite rejection.
#[test]
fn cell_membership_uses_closed_south_west_and_open_north_east_edges() {
    let pin = encode(20.5, 81.5).unwrap();
    let bounds = pin.bounds();
    assert!(bounds.contains(coordinates(bounds.south, bounds.west)));
    assert!(bounds.contains(coordinates(below(bounds.north), below(bounds.east))));
    assert!(!bounds.contains(coordinates(bounds.north, bounds.west)));
    assert!(!bounds.contains(coordinates(bounds.south, bounds.east)));
    assert!(!bounds.contains(coordinates(below(bounds.south), bounds.west)));
    assert!(!bounds.contains(coordinates(bounds.south, below(bounds.west))));
    assert!(!bounds.contains(coordinates(f64::NAN, bounds.west)));
    assert!(!bounds.contains(coordinates(bounds.south, f64::NAN)));
    assert!(!bounds.contains(coordinates(f64::INFINITY, bounds.west)));
    assert!(!bounds.contains(coordinates(bounds.south, f64::NEG_INFINITY)));

    for (latitude, longitude) in [(38.5, 81.5), (20.5, 99.5), (38.5, 99.5)] {
        assert!(
            encode(latitude, longitude)
                .unwrap()
                .bounds()
                .contains(coordinates(latitude, longitude))
        );
    }
}

/// Verifies invalid coordinates preserve their offending value in the corresponding error.
#[test]
fn coordinates_outside_coverage_and_nonfinite_values_are_rejected() {
    for latitude in [
        below(2.5),
        above(38.5),
        1.5, // Obsolete beta-version bounds must not be used.
        39.0,
        f64::NAN,
        f64::INFINITY,
        f64::NEG_INFINITY,
    ] {
        match encode(latitude, 81.5) {
            Err(DigipinError::LatitudeOutOfRange(value)) => {
                assert_eq!(value.to_bits(), latitude.to_bits());
            }
            other => panic!("expected latitude error for {latitude}, got {other:?}"),
        }
        assert!(!COVERAGE.contains(coordinates(latitude, 81.5)));
    }
    for longitude in [
        below(63.5),
        above(99.5),
        0.0,
        180.0,
        f64::NAN,
        f64::INFINITY,
        f64::NEG_INFINITY,
    ] {
        match encode(20.5, longitude) {
            Err(DigipinError::LongitudeOutOfRange(value)) => {
                assert_eq!(value.to_bits(), longitude.to_bits());
            }
            other => panic!("expected longitude error for {longitude}, got {other:?}"),
        }
        assert!(!COVERAGE.contains(coordinates(20.5, longitude)));
    }
}

/// Confirms ASCII case normalization and canonical output for the complete approved alphabet.
#[test]
fn raw_parsing_normalizes_ascii_case_and_emits_canonical_identifiers() {
    let expected = encode(28.622788, 77.213033).unwrap();
    for text in ["39J49LL8T4", "39j49ll8t4", "39J49lL8t4"] {
        let pin: Digipin = text.parse().unwrap();
        assert_eq!(pin, expected);
        assert_eq!(pin.as_str(), "39J49LL8T4");
        assert_eq!(pin.to_string(), "39J49LL8T4");
        assert_eq!(decode(text).unwrap(), expected.center());
    }
    for &symbol in GRID.iter().flatten() {
        let text: String = std::iter::repeat_n(char::from(symbol), 10).collect();
        let pin: Digipin = text.parse().unwrap();
        assert_eq!(pin.as_str(), text);
    }
}

/// Checks raw parsing reports byte lengths and invalid symbols without accepting separators.
#[test]
fn raw_parsing_rejects_separators_whitespace_and_wrong_lengths() {
    for text in [
        "",
        "39J49LL8T",
        "39J49LL8T44",
        "39J-49L-L8T4",
        "39J 49LL 8T4",
        " 39J49LL8T4",
        "39J49LL8T4 ",
        "39J49LL8T4\n",
    ] {
        assert!(matches!(
            text.parse::<Digipin>(),
            Err(DigipinError::InvalidLength { actual }) if actual == text.len()
        ));
        assert!(decode(text).is_err());
    }
    for (text, index, character) in [
        ("39J49LL8T-", 9, '-'),
        ("39J49LL8T ", 9, ' '),
        ("39J49LL8T\n", 9, '\n'),
        ("39J49LL8T0", 9, '0'),
        ("39J49LL8T1", 9, '1'),
        ("39J49LL8TG", 9, 'G'),
        ("39J49LL8TW", 9, 'W'),
        ("39J49LL8TX", 9, 'X'),
    ] {
        assert!(matches!(
            text.parse::<Digipin>(),
            Err(DigipinError::InvalidCharacter { index: actual_index, character: actual_character })
                if actual_index == index && actual_character == character
        ));
    }
}

/// Ensures Unicode input is rejected safely with scalar values and byte offsets intact.
#[test]
fn unicode_is_rejected_without_panicking_or_ascii_lookalike_normalization() {
    for (text, index, character) in [
        ("ＦLLLLLLL", 0, 'Ｆ'),
        ("LLLLLLLＦ", 7, 'Ｆ'),
        ("LL😀LLLL", 2, '😀'),
    ] {
        assert_eq!(text.len(), 10);
        assert!(matches!(
            text.parse::<Digipin>(),
            Err(DigipinError::InvalidCharacter { index: actual_index, character: actual_character })
                if actual_index == index && actual_character == character
        ));
        assert!(Digipin::parse_display(text).is_err());
    }
    for text in ["１２３４５６７８９０", "39J49LL8Té", "39J\u{200b}49LL8T4"] {
        assert!(text.parse::<Digipin>().is_err());
        assert!(Digipin::parse_display(text).is_err());
    }
}

/// Verifies optional display parsing accepts only continuous codes or exact 3-4-3 spacing.
#[test]
fn display_grouping_uses_the_may_2026_three_four_three_format() {
    let pin: Digipin = "39J49LL8T4".parse().unwrap();
    assert_eq!(pin.display_grouped().to_string(), "39J 49LL 8T4");
    for text in ["39J49LL8T4", "39J 49LL 8T4", "39j 49ll 8t4"] {
        assert_eq!(Digipin::parse_display(text).unwrap(), pin);
    }
    for text in [
        "39J-49L-L8T4",
        "39J 49L L8T4",
        "39J  49LL 8T4",
        "39J\t49LL 8T4",
        "39J 49LL\n8T4",
        "39J\u{00a0}49LL 8T4",
        " 39J 49LL 8T4",
        "39J 49LL 8T4 ",
    ] {
        assert!(matches!(
            Digipin::parse_display(text),
            Err(DigipinError::InvalidDisplayFormat)
        ));
    }
    assert!(Digipin::parse_display("39J 49LL 8TZ").is_err());
}

/// Checks code value traits, canonical deduplication, and borrowed string collection lookups.
#[test]
fn validated_codes_support_value_semantics_and_ordered_collections() {
    /// Enforces the copyable, comparable, sortable, hashable, and debuggable value contract.
    fn value_traits<T: Copy + Eq + Ord + std::hash::Hash + std::fmt::Debug>() {}
    value_traits::<Digipin>();

    let lowercase: Digipin = "39j49ll8t4".parse().unwrap();
    let uppercase: Digipin = "39J49LL8T4".parse().unwrap();
    let other: Digipin = "4P3JK852C9".parse().unwrap();
    let hashed = HashSet::from([lowercase, uppercase, other]);
    assert_eq!(hashed.len(), 2);
    // Borrow<str> must use the same Hash and Ord contracts as the stored code.
    assert!(hashed.contains("39J49LL8T4"));
    assert!(hashed.contains("4P3JK852C9"));
    assert!(!hashed.contains("39j49ll8t4"));
    let ordered = BTreeSet::from([lowercase, uppercase, other]);
    assert!(ordered.contains("39J49LL8T4"));
    let sorted: Vec<_> = ordered.into_iter().collect();
    assert_eq!(sorted, vec![uppercase, other]);
}

/// Confirms public conversions preserve code validation, canonical borrowing, and compact storage.
#[test]
fn public_conversions_preserve_the_validated_value() {
    let position = Coordinates::new(28.622788, 77.213033);
    let expected: Digipin = "39J49LL8T4".parse().unwrap();
    assert_eq!(position.encode().unwrap(), expected);
    assert_eq!(Digipin::try_from(position).unwrap(), expected);
    assert_eq!(
        Digipin::encode(position.latitude, position.longitude).unwrap(),
        expected
    );
    assert_eq!(Digipin::try_from("39j49ll8t4").unwrap(), expected);
    assert_eq!(expected.as_bytes(), b"39J49LL8T4");
    let borrowed: &str = expected.as_ref();
    assert_eq!(borrowed, expected.as_str());
    assert_eq!(std::mem::size_of::<Digipin>(), DIGIPIN_LENGTH);
    assert_eq!(expected.bounds().center(), expected.center());
    assert!(Digipin::try_from("39J-49L-L8T4").is_err());
    assert!(Digipin::try_from(Coordinates::new(-33.8688, 151.2093)).is_err());
}

/// Checks Serde emits canonical strings and rejects malformed codes or nonstring JSON values.
#[cfg(feature = "serde")]
#[test]
fn serde_uses_validated_canonical_strings() {
    let pin: Digipin = "39J49LL8T4".parse().unwrap();
    assert_eq!(serde_json::to_string(&pin).unwrap(), "\"39J49LL8T4\"");
    assert_eq!(
        serde_json::from_str::<Digipin>("\"39j49ll8t4\"").unwrap(),
        pin
    );
    for text in [
        "\"\"",
        "\"39J49LL8TZ\"",
        "\"39J-49L-L8T4\"",
        "\"39J 49LL 8T4\"",
        "\" 39J49LL8T4\"",
        "\"ＦLLLLLLL\"",
        "null",
        "1234567890",
        "[]",
        "{}",
    ] {
        assert!(
            serde_json::from_str::<Digipin>(text).is_err(),
            "accepted invalid serialized code: {text}"
        );
    }
}

/// Verifies coordinate serialization supports locations beyond the DIGIPIN coverage rectangle.
#[cfg(feature = "serde")]
#[test]
fn coordinate_serialization_keeps_general_geographic_coordinates() {
    // Coordinates are general geographic values; DIGIPIN coverage validation
    // happens only when encoding a code, not when storing coordinate data.
    let position = coordinates(-33.8688, 151.2093);
    let json = serde_json::to_string(&position).unwrap();
    assert_eq!(
        serde_json::from_str::<Coordinates>(&json).unwrap(),
        position
    );
    assert!(encode(position.latitude, position.longitude).is_err());
}
