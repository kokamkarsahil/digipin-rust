pub(crate) const DIGIPIN_GRID: [[u8; 4]; 4] = [
    [b'F', b'C', b'9', b'8'],
    [b'J', b'3', b'2', b'7'],
    [b'K', b'4', b'5', b'6'],
    [b'L', b'M', b'P', b'T'],
];

pub(crate) const SPAN: f64 = 36.0;
pub(crate) const POWER_F: f64 = 1_048_576.0;

pub(crate) const BOUNDS: crate::types::Bounds = crate::types::Bounds {
    min_lat: 2.5,
    max_lat: 38.5,
    min_lon: 63.5,
    max_lon: 99.5,
}; 