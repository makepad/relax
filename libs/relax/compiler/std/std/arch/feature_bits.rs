// The high cache word reserves bit 127 as the initialization flag for both words.
pub(super) const INIT: u128 = 1 << 127;

pub(super) const fn feature_location(index: u8) -> (u8, u64) {
    (index / 64, 1u64 << (index % 64))
}

pub(super) const fn feature_bit(index: u8, enabled: bool) -> u128 {
    if enabled {
        let (word, mask) = feature_location(index);
        (mask as u128) << (word as u32 * 64)
    } else {
        0
    }
}
