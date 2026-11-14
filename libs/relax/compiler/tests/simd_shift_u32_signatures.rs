#![cfg(target_arch = "x86_64")]
#![allow(unused_unsafe, non_snake_case)]
use core::arch::x86_64::*;

fn features() -> (bool, bool, bool) {
    unsafe {
        let p = __cpuid(1);
        if p.ecx & (3 << 26) != (3 << 26) || _xgetbv(0) & 0xe6 != 0xe6 {
            return (false, false, false);
        }
        let e = __cpuid_count(7, 0);
        (
            e.ebx & (1 << 16) != 0,
            e.ebx & (1 << 30) != 0,
            e.ebx & (1 << 31) != 0,
        )
    }
}

fn shifted(value: u64, bits: u32, count: u32, op: u8) -> u64 {
    let mask = if bits == 64 {
        u64::MAX
    } else {
        (1u64 << bits) - 1
    };
    match op {
        0 => {
            if count >= bits {
                0
            } else {
                (value << count) & mask
            }
        }
        1 => {
            if count >= bits {
                0
            } else {
                value >> count
            }
        }
        _ => {
            let signed = if bits == 16 {
                value as i16 as i64
            } else if bits == 32 {
                value as i32 as i64
            } else {
                value as i64
            };
            (signed >> count.min(bits - 1)) as u64 & mask
        }
    }
}

macro_rules! samples {
    ($ty:ty, $n:expr) => {{
        let mut a = [0 as $ty; $n];
        let mut src = [0 as $ty; $n];
        let mut i = 0;
        while i < $n {
            a[i] = (0x9e3779b97f4a7c15u64.wrapping_mul(i as u64 + 1)
                ^ (1u64 << ((core::mem::size_of::<$ty>() * 8) - 1))) as $ty;
            src[i] = (0xabcdef0145672839u64.wrapping_mul(i as u64 + 7)) as $ty;
            i += 1;
        }
        (core::hint::black_box(a), core::hint::black_box(src))
    }};
}

macro_rules! check_unmasked {
    ($name:ident, $vec:ty, $ty:ty, $n:expr, $op:expr) => {{
        let (a, _) = samples!($ty, $n);
        let input: $vec = core::mem::transmute(a);
        let actual: [$ty; $n] = core::mem::transmute($name::<N>(input));
        let mut i = 0;
        while i < $n {
            let expected = shifted(
                a[i] as u64,
                (core::mem::size_of::<$ty>() * 8) as u32,
                N,
                $op,
            ) as $ty;
            assert_eq!(
                actual[i],
                expected,
                "{} count {} lane {}",
                stringify!($name),
                N,
                i
            );
            i += 1;
        }
    }};
}

macro_rules! check_mask {
    ($name:ident, $vec:ty, $ty:ty, $n:expr, $maskty:ty, $op:expr) => {{
        let (a, src) = samples!($ty, $n);
        let input: $vec = core::mem::transmute(a);
        let source: $vec = core::mem::transmute(src);
        for mask in [0u64, u64::MAX, 0xaaaaaaaaaaaaaaaau64, 0x5555555555555555u64] {
            let mask = core::hint::black_box(mask as $maskty);
            let actual: [$ty; $n] = core::mem::transmute($name::<N>(source, mask, input));
            let mut i = 0;
            while i < $n {
                let expected = if ((mask as u64 >> i) & 1) != 0 {
                    shifted(
                        a[i] as u64,
                        (core::mem::size_of::<$ty>() * 8) as u32,
                        N,
                        $op,
                    ) as $ty
                } else {
                    src[i]
                };
                assert_eq!(
                    actual[i],
                    expected,
                    "{} count {} mask {} lane {}",
                    stringify!($name),
                    N,
                    mask,
                    i
                );
                i += 1;
            }
        }
    }};
}

macro_rules! check_maskz {
    ($name:ident, $vec:ty, $ty:ty, $n:expr, $maskty:ty, $op:expr) => {{
        let (a, _) = samples!($ty, $n);
        let input: $vec = core::mem::transmute(a);
        for mask in [0u64, u64::MAX, 0xaaaaaaaaaaaaaaaau64, 0x5555555555555555u64] {
            let mask = core::hint::black_box(mask as $maskty);
            let actual: [$ty; $n] = core::mem::transmute($name::<N>(mask, input));
            let mut i = 0;
            while i < $n {
                let expected = if ((mask as u64 >> i) & 1) != 0 {
                    shifted(
                        a[i] as u64,
                        (core::mem::size_of::<$ty>() * 8) as u32,
                        N,
                        $op,
                    ) as $ty
                } else {
                    0
                };
                assert_eq!(
                    actual[i],
                    expected,
                    "{} count {} mask {} lane {}",
                    stringify!($name),
                    N,
                    mask,
                    i
                );
                i += 1;
            }
        }
    }};
}

#[inline(never)]
unsafe fn check_all<const N: u32>() {
    let (f, bw, vl) = features();
    if f && vl {
        check_unmasked!(_mm_srai_epi64, __m128i, u64, 2, 2);
    }
    if f && vl {
        check_unmasked!(_mm256_srai_epi64, __m256i, u64, 4, 2);
    }
    if f && bw {
        check_unmasked!(_mm512_srai_epi16, __m512i, u16, 32, 2);
    }
    if f {
        check_unmasked!(_mm512_srai_epi32, __m512i, u32, 16, 2);
    }
    if f {
        check_unmasked!(_mm512_srai_epi64, __m512i, u64, 8, 2);
    }
    if f && bw {
        check_unmasked!(_mm512_srli_epi16, __m512i, u16, 32, 1);
    }
    if f {
        check_unmasked!(_mm512_srli_epi32, __m512i, u32, 16, 1);
    }
    if f {
        check_unmasked!(_mm512_srli_epi64, __m512i, u64, 8, 1);
    }
    if f && bw && vl {
        check_mask!(_mm256_mask_slli_epi16, __m256i, u16, 16, __mmask16, 0);
    }
    if f && vl {
        check_mask!(_mm256_mask_slli_epi32, __m256i, u32, 8, __mmask8, 0);
    }
    if f && vl {
        check_mask!(_mm256_mask_slli_epi64, __m256i, u64, 4, __mmask8, 0);
    }
    if f && bw && vl {
        check_mask!(_mm256_mask_srai_epi16, __m256i, u16, 16, __mmask16, 2);
    }
    if f && vl {
        check_mask!(_mm256_mask_srai_epi32, __m256i, u32, 8, __mmask8, 2);
    }
    if f && vl {
        check_mask!(_mm256_mask_srai_epi64, __m256i, u64, 4, __mmask8, 2);
    }
    if f && vl {
        check_mask!(_mm256_mask_srli_epi32, __m256i, u32, 8, __mmask8, 1);
    }
    if f && vl {
        check_mask!(_mm256_mask_srli_epi64, __m256i, u64, 4, __mmask8, 1);
    }
    if f && bw && vl {
        check_maskz!(_mm256_maskz_slli_epi16, __m256i, u16, 16, __mmask16, 0);
    }
    if f && vl {
        check_maskz!(_mm256_maskz_slli_epi32, __m256i, u32, 8, __mmask8, 0);
    }
    if f && vl {
        check_maskz!(_mm256_maskz_slli_epi64, __m256i, u64, 4, __mmask8, 0);
    }
    if f && bw && vl {
        check_maskz!(_mm256_maskz_srai_epi16, __m256i, u16, 16, __mmask16, 2);
    }
    if f && vl {
        check_maskz!(_mm256_maskz_srai_epi32, __m256i, u32, 8, __mmask8, 2);
    }
    if f && vl {
        check_maskz!(_mm256_maskz_srai_epi64, __m256i, u64, 4, __mmask8, 2);
    }
    if f && vl {
        check_maskz!(_mm256_maskz_srli_epi32, __m256i, u32, 8, __mmask8, 1);
    }
    if f && vl {
        check_maskz!(_mm256_maskz_srli_epi64, __m256i, u64, 4, __mmask8, 1);
    }
    if f && bw {
        check_mask!(_mm512_mask_slli_epi16, __m512i, u16, 32, __mmask32, 0);
    }
    if f {
        check_mask!(_mm512_mask_slli_epi32, __m512i, u32, 16, __mmask16, 0);
    }
    if f {
        check_mask!(_mm512_mask_slli_epi64, __m512i, u64, 8, __mmask8, 0);
    }
    if f && bw {
        check_mask!(_mm512_mask_srai_epi16, __m512i, u16, 32, __mmask32, 2);
    }
    if f {
        check_mask!(_mm512_mask_srai_epi32, __m512i, u32, 16, __mmask16, 2);
    }
    if f {
        check_mask!(_mm512_mask_srai_epi64, __m512i, u64, 8, __mmask8, 2);
    }
    if f && bw {
        check_mask!(_mm512_mask_srli_epi16, __m512i, u16, 32, __mmask32, 1);
    }
    if f {
        check_mask!(_mm512_mask_srli_epi32, __m512i, u32, 16, __mmask16, 1);
    }
    if f {
        check_mask!(_mm512_mask_srli_epi64, __m512i, u64, 8, __mmask8, 1);
    }
    if f && bw {
        check_maskz!(_mm512_maskz_slli_epi16, __m512i, u16, 32, __mmask32, 0);
    }
    if f {
        check_maskz!(_mm512_maskz_slli_epi32, __m512i, u32, 16, __mmask16, 0);
    }
    if f {
        check_maskz!(_mm512_maskz_slli_epi64, __m512i, u64, 8, __mmask8, 0);
    }
    if f && bw {
        check_maskz!(_mm512_maskz_srai_epi16, __m512i, u16, 32, __mmask32, 2);
    }
    if f {
        check_maskz!(_mm512_maskz_srai_epi32, __m512i, u32, 16, __mmask16, 2);
    }
    if f {
        check_maskz!(_mm512_maskz_srai_epi64, __m512i, u64, 8, __mmask8, 2);
    }
    if f {
        check_maskz!(_mm512_maskz_srli_epi32, __m512i, u32, 16, __mmask16, 1);
    }
    if f {
        check_maskz!(_mm512_maskz_srli_epi64, __m512i, u64, 8, __mmask8, 1);
    }
    if f && bw && vl {
        check_mask!(_mm_mask_slli_epi16, __m128i, u16, 8, __mmask8, 0);
    }
    if f && vl {
        check_mask!(_mm_mask_slli_epi32, __m128i, u32, 4, __mmask8, 0);
    }
    if f && vl {
        check_mask!(_mm_mask_slli_epi64, __m128i, u64, 2, __mmask8, 0);
    }
    if f && bw && vl {
        check_mask!(_mm_mask_srai_epi16, __m128i, u16, 8, __mmask8, 2);
    }
    if f && vl {
        check_mask!(_mm_mask_srai_epi32, __m128i, u32, 4, __mmask8, 2);
    }
    if f && vl {
        check_mask!(_mm_mask_srai_epi64, __m128i, u64, 2, __mmask8, 2);
    }
    if f && vl {
        check_mask!(_mm_mask_srli_epi32, __m128i, u32, 4, __mmask8, 1);
    }
    if f && vl {
        check_mask!(_mm_mask_srli_epi64, __m128i, u64, 2, __mmask8, 1);
    }
    if f && bw && vl {
        check_maskz!(_mm_maskz_slli_epi16, __m128i, u16, 8, __mmask8, 0);
    }
    if f && vl {
        check_maskz!(_mm_maskz_slli_epi32, __m128i, u32, 4, __mmask8, 0);
    }
    if f && vl {
        check_maskz!(_mm_maskz_slli_epi64, __m128i, u64, 2, __mmask8, 0);
    }
    if f && bw && vl {
        check_maskz!(_mm_maskz_srai_epi16, __m128i, u16, 8, __mmask8, 2);
    }
    if f && vl {
        check_maskz!(_mm_maskz_srai_epi32, __m128i, u32, 4, __mmask8, 2);
    }
    if f && vl {
        check_maskz!(_mm_maskz_srai_epi64, __m128i, u64, 2, __mmask8, 2);
    }
    if f && vl {
        check_maskz!(_mm_maskz_srli_epi32, __m128i, u32, 4, __mmask8, 1);
    }
    if f && vl {
        check_maskz!(_mm_maskz_srli_epi64, __m128i, u64, 2, __mmask8, 1);
    }
}

#[test]
fn every_shift_accepts_u32_and_matches_lane_semantics() {
    unsafe {
        check_all::<0>();
        check_all::<1>();
        check_all::<15>();
        check_all::<16>();
        check_all::<31>();
        check_all::<32>();
        check_all::<63>();
        check_all::<64>();
        check_all::<255>();
    }
}
