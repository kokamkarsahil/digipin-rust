use digipin::{Coordinates, Digipin, DigipinError, decode};

/// Prints canonical and grouped codes, cell centers, and bounds for three locations.
/// Checks containment and center round trips, then demonstrates lowercase and
/// grouped parsing, propagating any encoding or parsing error to the caller.
fn main() -> Result<(), DigipinError> {
    let locations = [
        ("New Delhi", Coordinates::new(28.6139, 77.2090)),
        ("Bangalore", Coordinates::new(12.9716, 77.5946)),
        ("Dak Bhawan", Coordinates::new(28.622788, 77.213033)),
    ];

    for (name, location) in locations {
        let pin = location.encode()?;
        let center = pin.center();
        let bounds = pin.bounds();

        println!("{name}: {pin} ({})", pin.display_grouped());
        println!("  Center: {:.8}, {:.8}", center.latitude, center.longitude);
        println!(
            "  Bounds: south={:.8}, west={:.8}, north={:.8}, east={:.8}",
            bounds.south, bounds.west, bounds.north, bounds.east
        );

        assert!(bounds.contains(location));
        assert_eq!(center.encode()?, pin);
        assert_eq!(decode(pin.as_str())?, center);
    }

    let canonical: Digipin = "39j438tjc7".parse()?;
    let displayed = Digipin::parse_display("39J 438T JC7")?;
    assert_eq!(canonical, displayed);
    assert_eq!(canonical.as_str(), "39J438TJC7");
    println!("Parsed lowercase and grouped input: {canonical}");

    Ok(())
}
