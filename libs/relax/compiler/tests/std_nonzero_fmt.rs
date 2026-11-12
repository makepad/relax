//! NonZero integers format with every integer format trait, as real core's NonZero types do.
use std::num::{NonZeroI32, NonZeroU32};

#[test]
fn nonzero_formats() {
    let n = NonZeroI32::new(5).unwrap();
    assert_eq!(format!("{} {:?} {:b} {:o} {:x} {:X}", n, n, n, n, n, n), "5 5 101 5 5 5");
    let u = NonZeroU32::new(255).unwrap();
    assert_eq!(format!("{} {:x} {:X} {:b}", u, u, u, u), "255 ff FF 11111111");
}
