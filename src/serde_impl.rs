use core::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Visitor};

use crate::Digipin;

impl Serialize for Digipin {
    /// Serializes the validated code as a continuous uppercase string.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for Digipin {
    /// Deserializes a string through DIGIPIN parsing, validating and normalizing it.
    ///
    /// Rejects non-string values and strings containing invalid symbols,
    /// separators, whitespace, or an incorrect byte length.
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct DigipinVisitor;

        impl Visitor<'_> for DigipinVisitor {
            type Value = Digipin;

            /// Describes the string format expected in deserialization diagnostics.
            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a string of ten DIGIPIN symbols without separators")
            }

            /// Parses a borrowed string and translates validation errors into Serde errors.
            fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<Digipin, E> {
                value.parse().map_err(E::custom)
            }
        }

        deserializer.deserialize_str(DigipinVisitor)
    }
}
