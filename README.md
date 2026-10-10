# digipin

A typed, allocation-free Rust implementation of India Post's DIGIPIN location code.
Encode coordinates, parse validated codes, and recover a cell's center and bounds.
The library supports `no_std` with no default runtime dependencies.

**Development status:** this checkout contains unreleased breaking changes. The
package version remains `0.0.5`; the examples below describe the new API. See
[MIGRATION.md](MIGRATION.md) when upgrading from the published API.

## Installation

The current package version is:

```toml
[dependencies]
digipin = "0.0.5"
```

The published `0.0.5` release uses the older API. To use the unreleased API in this
checkout, use a local path dependency instead:

```toml
[dependencies]
digipin = { path = "../digipin-rust" }
```

Rust **1.85 or newer** is required. The crate uses edition 2024.

## Usage

```rust
use digipin::{decode, encode, Coordinates, Digipin};

fn main() -> Result<(), digipin::DigipinError> {
    let location = Coordinates::new(28.6139, 77.2090);
    let pin = encode(location.latitude, location.longitude)?;

    assert_eq!(pin.as_str(), "39J438TJC7");
    assert_eq!(location.encode()?, pin);
    assert!(pin.bounds().contains(location));
    println!("{}", pin.display_grouped()); // 39J 438T JC7

    let parsed: Digipin = "39j438tjc7".parse()?;
    assert_eq!(parsed, pin);
    assert_eq!(decode(pin.as_str())?, pin.center());
    assert_eq!(pin.center().encode()?, pin);
    Ok(())
}
```

`Digipin` stores ten validated ASCII characters and implements `Copy`, `Eq`,
`Hash`, and `Ord`. Pass codes as typed values after parsing once. `as_str()` and
`Display` expose the uppercase canonical code; obtaining a `String` with
`to_string()` is optional and allocates in the caller.

### Parsing and display

Canonical codes contain exactly ten characters from `23456789CFJKLMPT`.
`FromStr` and `decode` accept lowercase ASCII letters and normalize them to
uppercase. They reject separators, whitespace, non-ASCII characters, and invalid
lengths.

```rust
use digipin::Digipin;

fn main() -> Result<(), digipin::DigipinError> {
    let pin: Digipin = "39j438tjc7".parse()?;
    assert_eq!(pin.as_str(), "39J438TJC7");
    assert!("39J-438-TJC7".parse::<Digipin>().is_err());

    // Accept human-readable input explicitly: raw ten characters, or 3–4–3 groups
    // separated by exactly one ASCII space each.
    assert_eq!(Digipin::parse_display("39J 438T JC7")?, pin);
    assert_eq!(Digipin::parse_display("39J438TJC7")?, pin);
    assert!(Digipin::parse_display("39J  438T JC7").is_err());
    println!("{}", pin.display_grouped()); // 39J 438T JC7
    Ok(())
}
```

`display_grouped()` formats without allocating. The official display grouping is
`XXX XXXX XXX`; hyphenated legacy codes require an explicit migration.

### Geographic behavior

Encoding accepts finite WGS 84 latitude and longitude in decimal degrees within
the inclusive coverage rectangle:

| Axis | Minimum | Maximum |
| --- | ---: | ---: |
| Latitude | 2.5° N | 38.5° N |
| Longitude | 63.5° E | 99.5° E |

This rectangle covers India and surrounding areas; successful encoding does not
establish that a location is inside India's political boundary.
`Coordinates::new` simply stores its values; `encode` performs validation.

Each code identifies a cell of width and height
`0.000034332275390625°` (`36° / 4^10`), about 3.8 meters north to south.
East-to-west distance varies with latitude. Decoding returns the **cell center**,
so the original coordinate is quantized rather than recovered exactly.

```rust
use digipin::Coordinates;

fn main() -> Result<(), digipin::DigipinError> {
    let original = Coordinates::new(28.622788, 77.213033);
    let pin = original.encode()?;
    assert_eq!(pin.as_str(), "39J49LL8T4"); // Dak Bhawan, New Delhi
    let cell = pin.bounds();
    assert!(cell.contains(original));
    assert!(cell.contains(pin.center()));
    assert!(cell.south < cell.north);
    assert!(cell.west < cell.east);
    Ok(())
}
```

`CellBounds` exposes `south`, `west`, `north`, and `east`. On a shared grid edge,
the cell to the **north or east** owns the coordinate. Accordingly, `contains`
includes the south and west edges and excludes the north and east edges, except
that the maximum outer coverage edges remain included.

### Optional Serde support

Enable `serde` on your dependency:

```toml
[dependencies]
digipin = { path = "../digipin-rust", features = ["serde"] }
serde_json = "1.0"
```

`Digipin` serializes as a canonical string. Deserialization uses strict parsing,
validates the input, and normalizes lowercase ASCII. `Coordinates` serializes as
an object with `latitude` and `longitude` fields; coordinate coverage validation
still happens when encoding.

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
# #[cfg(feature = "serde")]
# {
    use digipin::{Coordinates, Digipin};

    let pin: Digipin = "39j438tjc7".parse()?;
    assert_eq!(serde_json::to_string(&pin)?, "\"39J438TJC7\"");
    let restored: Digipin = serde_json::from_str("\"39j438tjc7\"")?;
    assert_eq!(restored, pin);
    assert!(serde_json::from_str::<Digipin>("\"39J 438T JC7\"").is_err());

    let location = Coordinates::new(28.6139, 77.2090);
    assert_eq!(
        serde_json::to_string(&location)?,
        r#"{"latitude":28.6139,"longitude":77.209}"#,
    );
# }
    Ok(())
}
```

Both the default crate and the `serde` feature support `no_std` without requiring
an allocator. `serde_json` is only an example format and is not a library
dependency.

## Specification and development

The implementation follows the [India Post DIGIPIN technical document, May
2026](https://github.com/INDIAPOST-gov/digipin/blob/a15339e8fdea93b13ae0d7336e567c9ae6ceac8e/docs/DIGIPIN_Technical_Document.md).
The source is pinned so that the documented boundary and display rules remain
reviewable. [Published API documentation](https://docs.rs/digipin/0.0.5/digipin/)
describes release `0.0.5`; generate documentation locally for this checkout.

```sh
cargo run --example basic_usage
cargo test --locked
cargo test --locked --all-features
cargo fmt --all -- --check
cargo clippy --locked --all-targets --all-features -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps --all-features
cargo bench --locked --bench digipin_benches
```

CI also checks Rust 1.85 and builds the library for the bare-metal
`thumbv7em-none-eabi` target, with and without Serde.

Licensed under [MIT](LICENSE).
