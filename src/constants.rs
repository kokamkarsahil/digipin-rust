/// DIGIPIN grid used for encoding/decoding
pub(crate) const DIGIPIN_GRID: [[char; 4]; 4] = [
    ['F', 'C', '9', '8'],
    ['J', '3', '2', '7'],
    ['K', '4', '5', '6'],
    ['L', 'M', 'P', 'T'],
];
pub(crate) const LOOKUP: [Option<(u8, u8)>; 128] = {
    let mut arr = [None; 128];
    arr['F' as u8 as usize] = Some((0, 0));
    arr['C' as u8 as usize] = Some((0, 1));
    arr['9' as u8 as usize] = Some((0, 2));
    arr['8' as u8 as usize] = Some((0, 3));
    arr['J' as u8 as usize] = Some((1, 0));
    arr['3' as u8 as usize] = Some((1, 1));
    arr['2' as u8 as usize] = Some((1, 2));
    arr['7' as u8 as usize] = Some((1, 3));
    arr['K' as u8 as usize] = Some((2, 0));
    arr['4' as u8 as usize] = Some((2, 1));
    arr['5' as u8 as usize] = Some((2, 2));
    arr['6' as u8 as usize] = Some((2, 3));
    arr['L' as u8 as usize] = Some((3, 0));
    arr['M' as u8 as usize] = Some((3, 1));
    arr['P' as u8 as usize] = Some((3, 2));
    arr['T' as u8 as usize] = Some((3, 3));
    arr['f' as u8 as usize] = Some((0, 0));
    arr['c' as u8 as usize] = Some((0, 1));
    arr['j' as u8 as usize] = Some((1, 0));
    arr['k' as u8 as usize] = Some((2, 0));
    arr['l' as u8 as usize] = Some((3, 0));
    arr['m' as u8 as usize] = Some((3, 1));
    arr['p' as u8 as usize] = Some((3, 2));
    arr['t' as u8 as usize] = Some((3, 3));
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