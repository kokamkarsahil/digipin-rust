use core::{
    borrow::Borrow,
    fmt,
    hash::{Hash, Hasher},
    str::FromStr,
};

use crate::{
    CellBounds, Coordinates, DIGIPIN_LENGTH, DigipinError, DigipinResult,
    constants::{INVALID_SYMBOL, LOOKUP},
};

/// A validated ten-symbol DIGIPIN, stored inline without an allocation.
///
/// [`FromStr`] and [`TryFrom<&str>`] accept ten ASCII DIGIPIN symbols,
/// normalizing lowercase to uppercase. Whitespace, punctuation, and non-ASCII
/// characters are rejected. Length is checked before character validity.
/// Ordinary display formatting and Serde serialization produce the canonical
/// uppercase string. Use [`Self::parse_display`] for the optional `3-4-3` space grouping.
///
/// ```
/// use digipin::Digipin;
/// let pin: Digipin = "4p3jk852c9".parse()?;
/// assert_eq!(pin.as_str(), "4P3JK852C9");
/// assert!("4P3-JK8-52C9".parse::<Digipin>().is_err());
/// # Ok::<(), digipin::DigipinError>(())
/// ```
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Digipin([u8; DIGIPIN_LENGTH]);

impl Digipin {
    /// Creates a DIGIPIN from WGS84 latitude and longitude in degrees.
    ///
    /// # Errors
    ///
    /// See [`crate::encode`] for coordinate validation errors.
    pub fn encode(latitude: f64, longitude: f64) -> DigipinResult<Self> {
        crate::encode(latitude, longitude)
    }

    /// Borrows the canonical uppercase string without allocating.
    pub fn as_str(&self) -> &str {
        // All constructors validate ASCII; keep the invariant in safe Rust.
        core::str::from_utf8(&self.0).expect("DIGIPIN symbols are ASCII")
    }

    /// Borrows the ten canonical ASCII symbols.
    pub const fn as_bytes(&self) -> &[u8; DIGIPIN_LENGTH] {
        &self.0
    }

    /// Returns the full-precision WGS84 coordinates of the cell center.
    #[must_use]
    pub fn center(self) -> Coordinates {
        self.bounds().center()
    }

    /// Returns the cell's WGS84 bounds in degrees.
    #[must_use]
    pub fn bounds(self) -> CellBounds {
        crate::decode::bounds(self)
    }

    /// Returns a formatter for the optional `XXX XXXX XXX` display grouping.
    ///
    /// The formatter borrows the code and does not allocate. Its output is for
    /// display; [`Self::as_str`] and [`fmt::Display`] give the canonical code.
    pub const fn display_grouped(&self) -> GroupedDigipin<'_> {
        GroupedDigipin(self)
    }

    /// Parses either a continuous code or exactly `XXX XXXX XXX` with ASCII spaces.
    ///
    /// ASCII lowercase is normalized. Extra spaces, other grouping patterns,
    /// hyphens, tabs, and leading/trailing whitespace are rejected.
    ///
    /// # Errors
    ///
    /// Returns [`DigipinError::InvalidDisplayFormat`] for an incorrect grouping,
    /// or a character validation error with its byte offset in the ungrouped code.
    ///
    /// ```
    /// let pin = digipin::Digipin::parse_display("4P3 JK85 2C9")?;
    /// assert_eq!(pin.as_str(), "4P3JK852C9");
    /// # Ok::<(), digipin::DigipinError>(())
    /// ```
    pub fn parse_display(input: &str) -> DigipinResult<Self> {
        if input.len() == DIGIPIN_LENGTH {
            return input.parse();
        }
        let bytes = input.as_bytes();
        if bytes.len() != DIGIPIN_LENGTH + 2 || bytes[3] != b' ' || bytes[8] != b' ' {
            return Err(DigipinError::InvalidDisplayFormat);
        }
        let mut symbols = [0; DIGIPIN_LENGTH];
        symbols[..3].copy_from_slice(&bytes[..3]);
        symbols[3..7].copy_from_slice(&bytes[4..8]);
        symbols[7..].copy_from_slice(&bytes[9..]);
        let canonical =
            core::str::from_utf8(&symbols).map_err(|_| DigipinError::InvalidDisplayFormat)?;
        canonical.parse()
    }

    /// Internal constructor; callers must supply only canonical GRID symbols.
    pub(crate) const fn from_symbols(symbols: [u8; DIGIPIN_LENGTH]) -> Self {
        Self(symbols)
    }
}

impl FromStr for Digipin {
    type Err = DigipinError;

    fn from_str(input: &str) -> DigipinResult<Self> {
        if input.len() != DIGIPIN_LENGTH {
            return Err(DigipinError::InvalidLength {
                actual: input.len(),
            });
        }
        let mut symbols = [0; DIGIPIN_LENGTH];
        for (index, character) in input.char_indices() {
            if !character.is_ascii() {
                return Err(DigipinError::InvalidCharacter { index, character });
            }
            let symbol = (character as u8).to_ascii_uppercase();
            if LOOKUP[symbol as usize] == INVALID_SYMBOL {
                return Err(DigipinError::InvalidCharacter { index, character });
            }
            symbols[index] = symbol;
        }
        Ok(Self(symbols))
    }
}

impl TryFrom<&str> for Digipin {
    type Error = DigipinError;

    fn try_from(input: &str) -> DigipinResult<Self> {
        input.parse()
    }
}

impl TryFrom<Coordinates> for Digipin {
    type Error = DigipinError;

    fn try_from(coordinates: Coordinates) -> DigipinResult<Self> {
        coordinates.encode()
    }
}

impl AsRef<str> for Digipin {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Borrow<str> for Digipin {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl Hash for Digipin {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Borrow<str> requires the same hash as the borrowed string.
        self.as_str().hash(state);
    }
}

impl fmt::Display for Digipin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(self.as_str())
    }
}

impl fmt::Debug for Digipin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Digipin").field(&self.as_str()).finish()
    }
}

/// Borrowed formatter for a DIGIPIN grouped with spaces in the `3-4-3` pattern.
///
/// Created by [`Digipin::display_grouped`].
#[derive(Debug, Clone, Copy)]
pub struct GroupedDigipin<'a>(&'a Digipin);

impl fmt::Display for GroupedDigipin<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbols = self.0.as_bytes();
        let mut bytes = [b' '; DIGIPIN_LENGTH + 2];
        bytes[..3].copy_from_slice(&symbols[..3]);
        bytes[4..8].copy_from_slice(&symbols[3..7]);
        bytes[9..].copy_from_slice(&symbols[7..]);
        f.pad(core::str::from_utf8(&bytes).expect("DIGIPIN display is ASCII"))
    }
}
