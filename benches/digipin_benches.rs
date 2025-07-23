use digipin::{get_digipin, get_coordinates_from_digipin, get_bounds_from_digipin};
use divan::black_box;

#[divan::bench]
fn encode() {
    let lat = black_box(28.6139f64);
    let lon = black_box(77.2090f64);
    get_digipin(lat, lon).unwrap();
}

#[divan::bench]
fn decode() {
    let pin = black_box("FCJ-3F9-8273");
    get_coordinates_from_digipin(pin).unwrap();
}

#[divan::bench]
fn bounds() {
    let pin = black_box("FCJ-3F9-8273");
    get_bounds_from_digipin(pin).unwrap();
}

fn main() {
    divan::main();
} 