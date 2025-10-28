use digipin::{get_digipin, get_coordinates_from_digipin};
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

const BATCH_SIZE: usize = 1_000;

#[divan::bench]
fn process_batch() {
    let mut coordinates = Vec::with_capacity(BATCH_SIZE);
    for i in 0..BATCH_SIZE {
        let lat = 28.6139 + (i as f64 * 0.0001);
        let lon = 77.2090 + (i as f64 * 0.0001);
        coordinates.push((lat, lon));
    }

    let digipins: Vec<String> = coordinates
        .into_iter()
        .map(|(lat, lon)| get_digipin(lat, lon).unwrap())
        .collect();

    for pin in digipins {
        black_box(get_coordinates_from_digipin(&pin).unwrap());
    }
}

fn main() {
    divan::main();
} 