use digipin::{Coordinates, Digipin};
use divan::black_box;

#[divan::bench]
fn encode() -> Digipin {
    digipin::encode(black_box(28.6139), black_box(77.2090)).unwrap()
}

#[divan::bench]
fn parse_canonical() -> Digipin {
    black_box("39J438TJC7").parse().unwrap()
}

#[divan::bench]
fn parse_lowercase() -> Digipin {
    black_box("39j438tjc7").parse().unwrap()
}

#[divan::bench]
fn parse_display() -> Digipin {
    Digipin::parse_display(black_box("39J 438T JC7")).unwrap()
}

#[divan::bench]
fn center(bencher: divan::Bencher) {
    let pin: Digipin = "39J438TJC7".parse().unwrap();
    bencher.bench(|| black_box(pin).center());
}

#[divan::bench]
fn bounds(bencher: divan::Bencher) {
    let pin: Digipin = "39J438TJC7".parse().unwrap();
    bencher.bench(|| black_box(pin).bounds());
}

#[divan::bench]
fn decode() -> Coordinates {
    digipin::decode(black_box("39J438TJC7")).unwrap()
}

fn main() {
    divan::main();
}
