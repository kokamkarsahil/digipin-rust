use digipin::{Coordinates, Digipin};
use divan::black_box;

/// Measures encoding New Delhi coordinates into a validated DIGIPIN.
#[divan::bench]
fn encode() -> Digipin {
    digipin::encode(black_box(28.6139), black_box(77.2090)).unwrap()
}

/// Measures validating and parsing a ten-character uppercase canonical code.
#[divan::bench]
fn parse_canonical() -> Digipin {
    black_box("39J438TJC7").parse().unwrap()
}

/// Measures validating lowercase input and normalizing it to a canonical code.
#[divan::bench]
fn parse_lowercase() -> Digipin {
    black_box("39j438tjc7").parse().unwrap()
}

/// Measures validating and parsing a code with the official 3–4–3 space grouping.
#[divan::bench]
fn parse_display() -> Digipin {
    Digipin::parse_display(black_box("39J 438T JC7")).unwrap()
}

/// Measures computing the center of a validated DIGIPIN cell.
/// Parsing occurs before the measured closure and is excluded from the timing.
#[divan::bench]
fn center(bencher: divan::Bencher) {
    let pin: Digipin = "39J438TJC7".parse().unwrap();
    bencher.bench(|| black_box(pin).center());
}

/// Measures computing the bounds of a validated DIGIPIN cell.
/// Parsing occurs before the measured closure and is excluded from the timing.
#[divan::bench]
fn bounds(bencher: divan::Bencher) {
    let pin: Digipin = "39J438TJC7".parse().unwrap();
    bencher.bench(|| black_box(pin).bounds());
}

/// Measures strict string parsing followed by computing the decoded cell center.
#[divan::bench]
fn decode() -> Coordinates {
    digipin::decode(black_box("39J438TJC7")).unwrap()
}

/// Runs the Divan benchmark harness for DIGIPIN encoding, parsing, and decoding.
fn main() {
    divan::main();
}
