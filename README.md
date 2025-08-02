# digipin-rust

DIGIPIN Encoder and Decoder Library - Encode latitude & longitude into 10-digit alphanumeric DIGIPIN codes

For more information about DIGIPIN, read [Technical document DIGIPIN ](https://www.indiapost.gov.in/VAS/DOP_PDFFiles/DIGIPIN%20Technical%20document.pdf).
and visit [India Post DIGIPIN](https://www.indiapost.gov.in/VAS/Pages/digipin.aspx)

## 🚀 Quick Start

```rust
use digipin::{get_digipin, get_coordinates_from_digipin};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Encode New Delhi coordinates to DIGIPIN
    let digipin = get_digipin(28.6139, 77.2090)?;
    println!("📍 DIGIPIN: {}", digipin); // Example: FCJ-3F9-8273
    
    // Decode back to coordinates
    let coords = get_coordinates_from_digipin(&digipin)?;
    println!("📍 Location: {:.4}°N, {:.4}°E", coords.latitude, coords.longitude);
    
    Ok(())
}
```

Documentation: https://docs.rs/digipin/latest/digipin/

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
digipin = "0.1.0"

# For Serde support
digipin = { version = "0.1.0", features = ["serde"] }
```toml
[dependencies]
digipin = "0.1.0"

# With Serde support for JSON serialization
digipin = { version = "0.1.0", features = ["serde"] }
```

## Usage

### Digipin Converstion

```rust
use digipin::{get_digipin};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Major Indian cities
    let cities = vec![
        ("New Delhi", 28.6139, 77.2090),
        ("Mumbai", 19.0760, 72.8777),
        ("Bangalore", 12.9716, 77.5946),
        ("Chennai", 13.0827, 80.2707),
        ("Kolkata", 22.5726, 88.3639),
        ("Hyderabad", 17.3850, 78.4867),
    ];
    
    for (city, lat, lon) in cities {
        let digipin = get_digipin(lat, lon)?;
        println!("{:10} → {}", city, digipin);
    }
    
    Ok(())
}
```

**Output:**
```
New Delhi  → 39J-438-TJC7
Mumbai     → 4FK-595-8823
Bangalore  → 4P3-JK8-52C9
Chennai    → 4T3-84L-L5L9
Kolkata    → 2TF-J7F-86MM
Hyderabad  → 422-594-J546
```

### Bidirectional Conversion

```rust
use digipin::{get_digipin, get_coordinates_from_digipin, Coordinates};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Start with coordinates
    let original = Coordinates::new(28.6139, 77.2090);
    println!("Original: {:.4}°N, {:.4}°E", original.latitude, original.longitude);
    
    // Encode to DIGIPIN
    let digipin = get_digipin(original.latitude, original.longitude)?;
    println!("DIGIPIN:  {}", digipin);
    
    // Decode back to coordinates
    let decoded = get_coordinates_from_digipin(&digipin)?;
    println!("Decoded:  {:.4}°N, {:.4}°E", decoded.latitude, decoded.longitude);
    
    // Calculate precision
    let accuracy = ((original.latitude - decoded.latitude).powi(2) + 
                   (original.longitude - decoded.longitude).powi(2)).sqrt();
    println!("Accuracy: ~{:.0}m", accuracy * 111_000.0); // Convert degrees to meters
    
    Ok(())
}
```

**Output:**
```
Original: 28.6139°N, 77.2090°E
DIGIPIN:  39J-438-TJC7
Decoded:  28.6139°N, 77.2090°E
Accuracy: ~0m
```

### Error Handling

```rust
use digipin::{get_digipin, get_coordinates_from_digipin, DigipinError};

fn handle_location(lat: f64, lon: f64, digipin: &str) {
    // Handle encoding errors
    match get_digipin(lat, lon) {
        Ok(pin) => println!("Encoded: {}", pin),
        Err(DigipinError::LatitudeOutOfRange(lat)) => {
            println!("Latitude {:.2}° is outside India", lat);
        }
        Err(DigipinError::LongitudeOutOfRange(lon)) => {
            println!("Longitude {:.2}° is outside India", lon);
        }
        Err(e) => println!("Error: {}", e),
    }
    
    // Handle decoding errors
    match get_coordinates_from_digipin(digipin) {
        Ok(coords) => println!("Decoded: {:.4}°N, {:.4}°E", 
                               coords.latitude, coords.longitude),
        Err(DigipinError::InvalidLength(len)) => {
            println!("Invalid length: {} (expected 10 characters)", len);
        }
        Err(DigipinError::InvalidCharacter(ch)) => {
            println!("Invalid character: '{}'", ch);
        }
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    // Test with invalid coordinates (outside India)
    handle_location(51.5074, -0.1278, ""); // London - outside range
    
    // Test with invalid DIGIPIN
    handle_location(0.0, 0.0, "INVALID-123"); // Invalid characters
    handle_location(0.0, 0.0, "FCJ-3F9"); // Too short
}
```

### Format Flexibility

```rust
use digipin::get_coordinates_from_digipin;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let digipin_with_hyphens = "FCJ-3F9-8273";
    let digipin_without_hyphens = "FCJ3F98273";
    
    // Both formats work identically
    let coords1 = get_coordinates_from_digipin(digipin_with_hyphens)?;
    let coords2 = get_coordinates_from_digipin(digipin_without_hyphens)?;
    
    assert_eq!(coords1.latitude, coords2.latitude);
    assert_eq!(coords1.longitude, coords2.longitude);
    
    println!("✅ Both formats produce identical results!");
    Ok(())
}
```

## Advanced Usage

### With Serde (JSON Support)

Enable the `serde` feature in your `Cargo.toml`:

```toml
[dependencies]
digipin = { version = "0.1.0", features = ["serde"] }
serde_json = "1.0"
```

```rust
use digipin::Coordinates;
use serde_json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let coords = Coordinates::new(28.6139, 77.2090);
    
    // Serialize to JSON
    let json = serde_json::to_string(&coords)?;
    println!("📤 JSON: {}", json);
    // Output: {"latitude":28.6139,"longitude":77.2090}
    
    // Deserialize from JSON
    let deserialized: Coordinates = serde_json::from_str(&json)?;
    println!("📥 Restored: {:.4}°N, {:.4}°E", 
             deserialized.latitude, deserialized.longitude);
    
    Ok(())
}
```

### Batch Processing

```rust
use digipin::{get_digipin, Coordinates};
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let locations: HashMap<&str, Coordinates> = [
        ("Gateway of India", Coordinates::new(18.9220, 72.8347)),
        ("India Gate", Coordinates::new(28.6129, 77.2295)),
        ("Charminar", Coordinates::new(17.3616, 78.4747)),
        ("Howrah Bridge", Coordinates::new(22.5851, 88.3467)),
    ].iter().cloned().collect();
    
    println!("Indian Landmarks and their DIGIPINs:");
    println!("{:-<50}", "");
    
    for (name, coords) in locations {
        match get_digipin(coords.latitude, coords.longitude) {
            Ok(digipin) => println!("{:18} → {}", name, digipin),
            Err(e) => println!("{:18} → Error: {}", name, e),
        }
    }
    
    Ok(())
}
```

### Running Examples and Tests

```bash
# Clone the repository
git clone https://github.com/your-username/digipin-rust.git
cd digipin-rust

# Run the basic example
cargo run --example basic_usage

# Run all tests
cargo test

# Run tests with all features
cargo test --all-features

# Run tests with verbose output
cargo test -- --nocapture

# Check documentation
cargo doc --open
```

# License

This project is licensed under the MIT License. See the `LICENSE` file for more details.
