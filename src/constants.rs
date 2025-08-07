/// DIGIPIN grid used for encoding/decoding
pub(crate) const DIGIPIN_GRID: [[char; 4]; 4] = [
    ['F', 'C', '9', '8'],
    ['J', '3', '2', '7'],
    ['K', '4', '5', '6'],
    ['L', 'M', 'P', 'T'],
];
pub(crate) const LOOKUP: [Option<(u8, u8)>; 128] = {
    let mut arr = [None; 128];
    arr[b'F' as usize] = Some((0, 0));
    arr[b'C' as usize] = Some((0, 1));
    arr[b'9' as usize] = Some((0, 2));
    arr[b'8' as usize] = Some((0, 3));
    arr[b'J' as usize] = Some((1, 0));
    arr[b'3' as usize] = Some((1, 1));
    arr[b'2' as usize] = Some((1, 2));
    arr[b'7' as usize] = Some((1, 3));
    arr[b'K' as usize] = Some((2, 0));
    arr[b'4' as usize] = Some((2, 1));
    arr[b'5' as usize] = Some((2, 2));
    arr[b'6' as usize] = Some((2, 3));
    arr[b'L' as usize] = Some((3, 0));
    arr[b'M' as usize] = Some((3, 1));
    arr[b'P' as usize] = Some((3, 2));
    arr[b'T' as usize] = Some((3, 3));

    arr
};
pub(crate) const SPAN: f64 = 36.0;
pub(crate) const POWER: u32 = 1 << 20;

/// Geographic bounds structure
#[derive(Debug, Clone, Copy)]
pub(crate) struct Bounds {
    pub(crate) min_lat: f64,
    pub(crate) max_lat: f64,
    pub(crate) min_lon: f64,
    pub(crate) max_lon: f64,
}

/// Geographic bounds for DIGIPIN encoding (covers India)
pub(crate) const BOUNDS: Bounds = Bounds {
    min_lat: 2.5,
    max_lat: 38.5,
    min_lon: 63.5,
    max_lon: 99.5,
}; 