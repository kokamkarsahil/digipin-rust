# digipin

[![Latest Version](https://img.shields.io/crates/v/digipin.svg)](https://crates.io/crates/digipin)
[![Docs.rs](https://docs.rs/digipin/badge.svg)](https://docs.rs/digipin)
[![License](https://img.shields.io/crates/l/digipin.svg)](https://github.com/kokamkarsahil/digipin-rust/blob/main/LICENSE)

A Rust library for encoding and decoding latitude and longitude coordinates into 10-digit alphanumeric **DIGIPIN** codes.

For more information about DIGIPIN, read the [official technical document](https://www.indiapost.gov.in/VAS/DOP_PDFFiles/DIGIPIN%20Technical%20document.pdf) and visit the [India Post DIGIPIN website](https://www.indiapost.gov.in/VAS/Pages/digipin.aspx).

## What is DIGIPIN?

DIGIPIN is a geocoding system developed by India Post that represents any location within India using a 10-digit alphanumeric code. It offers a consistent, human-readable, and precise alternative to traditional latitude and longitude coordinates.

**Key Benefits**:
- **High Precision**: Provides accuracy down to a few meters.
- **Human-Readable**: Easy to remember, share, and input.
- **Offline Capable**: Encoding and decoding can be performed without an internet connection.
- **Uniform**: Covers the entire Indian subcontinent with a consistent grid.

## 🚀 Quick Start

```rust
use digipin::{get_digipin, get_coordinates_from_digipin};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Encode New Delhi's coordinates to a DIGIPIN
    let digipin = get_digipin(28.6139, 77.2090)?;
    println!("📍 DIGIPIN: {}", digipin); // Example: 39J-438-TJC7

    // Decode the DIGIPIN back to coordinates
    let coords = get_coordinates_from_digipin(&digipin)?;
    println!("📍 Location: {:.4}°N, {:.4}°E", coords.latitude, coords.longitude);
    
    Ok(())
}
```

## 📦 Installation

Add this library to your `Cargo.toml`:

```toml
[dependencies]
digipin = "0.0.4"
```

### With Serde

To enable JSON serialization and deserialization for the `Coordinates` struct, add the `serde` feature:

```toml
[dependencies]
digipin = { version = "0.0.4", features = ["serde"] }
```

## ✨ Features

- **`serde`**: Implements `serde::Serialize` and `serde::Deserialize` for the `Coordinates` struct, allowing easy integration with JSON and other formats.

## API

### `get_digipin(latitude: f64, longitude: f64) -> DigipinResult<String>`

Encodes latitude and longitude into a formatted DIGIPIN string. Returns an error if the coordinates are outside the valid bounds for India.

### `get_coordinates_from_digipin(digipin: &str) -> DigipinResult<Coordinates>`

Decodes a DIGIPIN string into `Coordinates`. Handles both hyphenated (`XXX-XXX-XXXX`) and non-hyphenated formats. Returns an error for invalid characters or incorrect length.

### `struct Coordinates`

A simple struct to hold `latitude` and `longitude` values.

### `enum DigipinError`

Represents potential errors during encoding or decoding, such as:
- `LatitudeOutOfRange`: Latitude is outside the `6.0..=38.0` range.
- `LongitudeOutOfRange`: Longitude is outside the `68.0..=98.0` range.
- `InvalidLength`: DIGIPIN string is not 10 characters long (excluding hyphens).
- `InvalidCharacter`: An invalid character was found in the DIGIPIN string.

## 📈 Benchmarking

The library uses `divan` for benchmarking. To run the benchmarks, use `cargo bench`:

```bash
cargo bench
```

## 🤝 Contributing

Contributions are welcome! Whether it's adding features, improving documentation, or reporting bugs, please feel free to open an issue or submit a pull request.

### Development Setup

1. **Clone the repository**:
   ```bash
   git clone https://github.com/kokamkarsahil/digipin-rust.git
   cd digipin-rust
   ```

2. **Run tests**:
   ```bash
   cargo test
   ```

3. **Check formatting and lint**:
   ```bash
   cargo fmt
   cargo clippy
   ```

## 📜 License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.
