//! Encode and decode India Post's DIGIPIN addressing grid.
//!
//! The core is allocation-free, works without `std`, and has no dependencies.
//! A [`Digipin`] stores ten validated ASCII symbols, so decoding an existing
//! value cannot fail. Input coordinates use WGS84 latitude/longitude in degrees.
//!
//! ```
//! use digipin::{decode, encode, Digipin};
//!
//! let pin = encode(12.9716, 77.5946)?;
//! assert_eq!(pin.as_str(), "4P3JK852C9");
//! assert_eq!(pin.display_grouped().to_string(), "4P3 JK85 2C9");
//! assert_eq!(decode(pin.as_str())?, pin.center());
//! assert_eq!(pin.center().encode()?, pin);
//!
//! let parsed: Digipin = "4p3jk852c9".parse()?;
//! assert_eq!(parsed, pin);
//! # Ok::<(), digipin::DigipinError>(())
//! ```
//!
//! # Format and precision
//!
//! Canonical codes are continuous uppercase strings, following the
//! [May 2026 specification](https://github.com/INDIAPOST-gov/digipin/blob/a15339e8fdea93b13ae0d7336e567c9ae6ceac8e/docs/DIGIPIN_Technical_Document.md).
//! Parsing also accepts ASCII lowercase; punctuation and whitespace are rejected.
//! Use [`Digipin::parse_display`] explicitly for the optional `3-4-3` grouping.
//!
//! Every code identifies a cell of width [`CELL_WIDTH_DEGREES`], approximately
//! 3.8 metres north/south. East/west distance varies with latitude. Decoding returns
//! the cell center, retaining full `f64` precision rather than rounding to six
//! decimal places. Exact grid lines belong to the north/east cell; the outer
//! north/east edges belong to the final cell. [`COVERAGE`] is a rectangle, not a
//! test of national borders.
//!
//! # Features
//!
//! The optional `serde` feature serializes a [`Digipin`] as its canonical string
//! and validates input during deserialization. [`Coordinates`] serialize as an
//! object with `latitude` and `longitude` fields. Serde support also works
//! without `std` or an allocator.

#![no_std]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod bounds;
mod constants;
mod coordinates;
mod decode;
mod digipin;
mod encode;
mod error;
#[cfg(feature = "serde")]
mod serde_impl;

// Compile the guides as doctests without duplicating them in generated API docs.
#[cfg(doctest)]
#[doc = include_str!("../README.md")]
#[doc = include_str!("../MIGRATION.md")]
mod usage_examples {}

pub use bounds::CellBounds;
pub use coordinates::Coordinates;
pub use decode::decode;
pub use digipin::{Digipin, GroupedDigipin};
pub use encode::encode;
pub use error::{DigipinError, DigipinResult};

/// Number of ASCII symbols in a full-resolution DIGIPIN.
pub const DIGIPIN_LENGTH: usize = 10;

/// Width of a full-resolution cell in latitude and longitude degrees: `36 / 4^10`.
pub const CELL_WIDTH_DEGREES: f64 = 36.0 / constants::CELLS_PER_AXIS as f64;

/// Inclusive geographic coverage of the DIGIPIN grid, in WGS84 degrees.
///
/// This rectangle extends beyond India's borders; membership does not establish
/// whether a coordinate is within India.
pub const COVERAGE: CellBounds = CellBounds::new(2.5, 63.5, 38.5, 99.5);
