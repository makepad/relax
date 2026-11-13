#[cfg(test)]
mod tests {
    macro_rules! unsigned_const {
        ($name:ident, $int:ty) => {
            mod $name {
                use std::num::NonZero;

                const CHECKED: Option<NonZero<$int>> = NonZero::<$int>::MIN.checked_add(2);
                const UNCHANGED: Option<NonZero<$int>> = NonZero::<$int>::MIN.checked_add(0);
                const OVERFLOW: Option<NonZero<$int>> = NonZero::<$int>::MAX.checked_add(1);
                const ADDED: NonZero<$int> = NonZero::<$int>::MIN.saturating_add(2);
                const SATURATED: NonZero<$int> = NonZero::<$int>::MAX.saturating_add(1);
                const POWER: Option<NonZero<$int>> = ADDED.checked_next_power_of_two();
                const POWER_ONE: Option<NonZero<$int>> = NonZero::<$int>::MIN.checked_next_power_of_two();
                const POWER_OVERFLOW: Option<NonZero<$int>> = NonZero::<$int>::MAX.checked_next_power_of_two();
                const LOG_ONE: u32 = NonZero::<$int>::MIN.ilog10();
                const LOG: u32 = NonZero::<$int>::new(100).unwrap().ilog10();
                const ROOT_ONE: NonZero<$int> = NonZero::<$int>::MIN.isqrt();
                const ROOT: NonZero<$int> = NonZero::<$int>::new(80).unwrap().isqrt();

                #[test]
                fn const_arithmetic() {
                    assert_eq!(CHECKED.unwrap().get(), 3);
                    assert_eq!(UNCHANGED.unwrap().get(), 1);
                    assert_eq!(OVERFLOW, None);
                    assert_eq!(ADDED.get(), 3);
                    assert_eq!(SATURATED.get(), <$int>::MAX);
                    assert_eq!(POWER.unwrap().get(), 4);
                    assert_eq!(POWER_ONE.unwrap().get(), 1);
                    assert_eq!(POWER_OVERFLOW, None);
                    assert_eq!(LOG_ONE, 0);
                    assert_eq!(LOG, 2);
                    assert_eq!(ROOT_ONE.get(), 1);
                    assert_eq!(ROOT.get(), 8);
                }
            }
        };
    }

    unsigned_const!(u8_const, u8);
    unsigned_const!(u16_const, u16);
    unsigned_const!(u32_const, u32);
    unsigned_const!(u64_const, u64);
    unsigned_const!(u128_const, u128);
    unsigned_const!(usize_const, usize);
}
