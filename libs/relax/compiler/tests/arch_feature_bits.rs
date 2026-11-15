#[path = "../std/std/arch/feature_bits.rs"]
mod feature_bits;

#[test]
fn cache_word_boundaries_and_reserved_bits() {
    use feature_bits::{feature_bit, feature_location, INIT};

    assert_eq!(feature_location(62), (0, 1u64 << 62));
    assert_eq!(feature_location(63), (0, 1u64 << 63));
    assert_eq!(feature_location(64), (1, 1));
    assert_eq!(feature_location(65), (1, 2));
    assert_eq!(feature_location(127), (1, (INIT >> 64) as u64));
    assert_eq!(feature_bit(64, true), 1u128 << 64);
    assert_eq!(feature_bit(65, true), 1u128 << 65);
    assert_eq!(feature_bit(106, true), 1u128 << 106);
    assert_eq!(feature_bit(107, true), 1u128 << 107);
    assert_eq!(feature_bit(64, false), 0);
    assert_eq!(feature_bit(65, false), 0);
}

#[cfg(relax)]
#[test]
fn feature_cache_uses_the_expected_indices() {
    use std::arch::detect::{check, Feature};

    assert_eq!(Feature::Bti as u8, 64);
    assert_eq!(Feature::Cssc as u8, 65);
    assert_ne!(Feature::Sm4 as u8, Feature::Sm4x86 as u8);
    assert_eq!(Feature::Avx512vp2intersect as u8, 62);
    assert_eq!(Feature::Sha512 as u8, 106);
    assert_eq!(Feature::Tbm as u8, 107);
    for feature in [Feature::Avx512vp2intersect, Feature::Sha512, Feature::Tbm] {
        assert_eq!(check(feature), check(feature));
    }
}
