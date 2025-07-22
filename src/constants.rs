pub(crate) const DIGIPIN_GRID: [[char; 4]; 4] = [
    ['F', 'C', '9', '8'],
    ['J', '3', '2', '7'],
    ['K', '4', '5', '6'],
    ['L', 'M', 'P', 'T'],
];

pub(crate) const SPAN: f64 = 36.0;
pub(crate) const POWER_F: f64 = 1_048_576.0;

pub(crate) const BOUNDS: crate::types::Bounds = crate::types::Bounds {
    min_lat: 2.5,
    max_lat: 38.5,
    min_lon: 63.5,
    max_lon: 99.5,
}; 