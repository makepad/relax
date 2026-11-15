// core::arch::x86_64 differential coverage (core_diff.sh): each probe hashes outputs
// over edge-case inputs and prints "| name tag count hash"; the lines must equal real
// std's under rustc --test. AVX512BF16 is probed through CPUID/XCR0 directly (feat in
// arch/x86_simd.rs) since relax's detect reports no AVX-512 by policy while the
// emulated intrinsics compute regardless.
#![allow(unused_unsafe, non_snake_case)]

include!("arch/common.rs");
include!("arch/x86_simd.rs");
use core::arch::x86_64::*;

// f32 operand bits for the bf16 conversions: signed zeros, infinities, quiet and
// signaling NaNs with payloads that outlive the truncation, denormals, the largest
// finite f32, and every round-to-nearest-even boundary (halfway even/odd, just
// below, just above) including the boundary that rounds up to bf16 infinity.
fn f32_bits_fixture(n: usize) -> u32 {
    let bits = [
        0x00000000u32, 0x80000000, 0x3f800000, 0xbf800000, 0x7f800000, 0xff800000,
        0x7fc00000, 0xffc00000, 0x7fa01234, 0xffa00001, 0x7fc00001, 0x00000001,
        0x80000001, 0x007fffff, 0x00800000, 0x7f7fffff, 0x3f808000, 0x3f807fff,
        0x3f808001, 0x3f818000, 0x3f817fff, 0x3f818001, 0xbf808000, 0xbf818000,
        0x7f7f8000, 0x7f7f7fff, 0x7f7e8000, 0x00408000, 0x00418000, 0x3f800001,
        0xbf800001, 0x477fff80,
    ];
    bits[n % bits.len()]
}

// A vector whose f32 lanes walk the fixture table.
fn fvec<T: Copy>(n: usize) -> T {
    let count = (core::mem::size_of::<T>() + 3) / 4;
    let mut words = [0u32; 16];
    for i in 0..count {
        words[i] = f32_bits_fixture(n + i * (n / 16));
    }
    unsafe { core::ptr::read_unaligned(words.as_ptr() as *const T) }
}

// bf16 operand lanes: zeros, ones, infinities, quiet and signaling NaNs, denormals,
// the largest finite bf16, and negative variants.
fn u16_bits_fixture(n: usize) -> u16 {
    let bits = [
        0x0000u16, 0x8000, 0x3f80, 0xbf80, 0x7f80, 0xff80, 0x7fc0, 0xffc0,
        0x7fa0, 0xffa0, 0x7fc1, 0x7f81, 0x0001, 0x8001, 0x007f, 0x0080,
        0x7f7f, 0xff7f, 0x4000, 0xc000, 0x3f81, 0xbf7f, 0x0002, 0xffff,
    ];
    bits[n % bits.len()]
}

fn bh<T: Copy>(n: usize) -> T {
    let count = (core::mem::size_of::<T>() + 1) / 2;
    let mut words = [0u16; 32];
    for i in 0..count {
        words[i] = u16_bits_fixture(n + i * (n / 16));
    }
    unsafe { core::ptr::read_unaligned(words.as_ptr() as *const T) }
}

#[inline(never)]
unsafe fn probe_mm_cvtne2ps_pbh(n: usize, h: &mut Digest) {
    if feat("avx512bf16") && feat("avx512vl") {
        let out = _mm_cvtne2ps_pbh(fvec(n + 1), fvec(n + 2));
        mix(h, &out);
    }
}

#[inline(never)]
unsafe fn probe_mm_mask_cvtne2ps_pbh(n: usize, h: &mut Digest) {
    if feat("avx512bf16") && feat("avx512vl") {
        let out = _mm_mask_cvtne2ps_pbh(bh(n + 1), kbits(n + 2) as __mmask8, fvec(n + 3), fvec(n + 4));
        mix(h, &out);
    }
}

#[inline(never)]
unsafe fn probe_mm_maskz_cvtne2ps_pbh(n: usize, h: &mut Digest) {
    if feat("avx512bf16") && feat("avx512vl") {
        let out = _mm_maskz_cvtne2ps_pbh(kbits(n + 2) as __mmask8, fvec(n + 3), fvec(n + 4));
        mix(h, &out);
    }
}

#[inline(never)]
unsafe fn probe_mm256_cvtne2ps_pbh(n: usize, h: &mut Digest) {
    if feat("avx512bf16") && feat("avx512vl") {
        let out = _mm256_cvtne2ps_pbh(fvec(n + 1), fvec(n + 2));
        mix(h, &out);
    }
}

#[inline(never)]
unsafe fn probe_mm256_mask_cvtne2ps_pbh(n: usize, h: &mut Digest) {
    if feat("avx512bf16") && feat("avx512vl") {
        let out = _mm256_mask_cvtne2ps_pbh(bh(n + 1), kbits(n + 2) as __mmask16, fvec(n + 3), fvec(n + 4));
        mix(h, &out);
    }
}

#[inline(never)]
unsafe fn probe_mm256_maskz_cvtne2ps_pbh(n: usize, h: &mut Digest) {
    if feat("avx512bf16") && feat("avx512vl") {
        let out = _mm256_maskz_cvtne2ps_pbh(kbits(n + 2) as __mmask16, fvec(n + 3), fvec(n + 4));
        mix(h, &out);
    }
}

#[inline(never)]
unsafe fn probe_mm512_cvtne2ps_pbh(n: usize, h: &mut Digest) {
    if feat("avx512bf16") {
        let out = _mm512_cvtne2ps_pbh(fvec(n + 1), fvec(n + 2));
        mix(h, &out);
    }
}

#[inline(never)]
unsafe fn probe_mm512_mask_cvtne2ps_pbh(n: usize, h: &mut Digest) {
    if feat("avx512bf16") {
        let out = _mm512_mask_cvtne2ps_pbh(bh(n + 1), kbits(n + 2) as __mmask32, fvec(n + 3), fvec(n + 4));
        mix(h, &out);
    }
}

#[inline(never)]
unsafe fn probe_mm512_maskz_cvtne2ps_pbh(n: usize, h: &mut Digest) {
    if feat("avx512bf16") {
        let out = _mm512_maskz_cvtne2ps_pbh(kbits(n + 2) as __mmask32, fvec(n + 3), fvec(n + 4));
        mix(h, &out);
    }
}

#[inline(never)]
unsafe fn probe_mm_cvtneps_pbh(n: usize, h: &mut Digest) {
    if feat("avx512bf16") && feat("avx512vl") {
        let out = _mm_cvtneps_pbh(fvec(n + 1));
        mix(h, &out);
    }
}

#[inline(never)]
unsafe fn probe_mm_mask_cvtneps_pbh(n: usize, h: &mut Digest) {
    if feat("avx512bf16") && feat("avx512vl") {
        let out = _mm_mask_cvtneps_pbh(bh(n + 1), kbits(n + 2) as __mmask8, fvec(n + 3));
        mix(h, &out);
    }
}

#[inline(never)]
unsafe fn probe_mm_maskz_cvtneps_pbh(n: usize, h: &mut Digest) {
    if feat("avx512bf16") && feat("avx512vl") {
        let out = _mm_maskz_cvtneps_pbh(kbits(n + 2) as __mmask8, fvec(n + 3));
        mix(h, &out);
    }
}

#[inline(never)]
unsafe fn probe_mm256_cvtneps_pbh(n: usize, h: &mut Digest) {
    if feat("avx512bf16") && feat("avx512vl") {
        let out = _mm256_cvtneps_pbh(fvec(n + 1));
        mix(h, &out);
    }
}

#[inline(never)]
unsafe fn probe_mm256_mask_cvtneps_pbh(n: usize, h: &mut Digest) {
    if feat("avx512bf16") && feat("avx512vl") {
        let out = _mm256_mask_cvtneps_pbh(bh(n + 1), kbits(n + 2) as __mmask8, fvec(n + 3));
        mix(h, &out);
    }
}

#[inline(never)]
unsafe fn probe_mm256_maskz_cvtneps_pbh(n: usize, h: &mut Digest) {
    if feat("avx512bf16") && feat("avx512vl") {
        let out = _mm256_maskz_cvtneps_pbh(kbits(n + 2) as __mmask8, fvec(n + 3));
        mix(h, &out);
    }
}

#[inline(never)]
unsafe fn probe_mm512_cvtneps_pbh(n: usize, h: &mut Digest) {
    if feat("avx512bf16") {
        let out = _mm512_cvtneps_pbh(fvec(n + 1));
        mix(h, &out);
    }
}

#[inline(never)]
unsafe fn probe_mm512_mask_cvtneps_pbh(n: usize, h: &mut Digest) {
    if feat("avx512bf16") {
        let out = _mm512_mask_cvtneps_pbh(bh(n + 1), kbits(n + 2) as __mmask16, fvec(n + 3));
        mix(h, &out);
    }
}

#[inline(never)]
unsafe fn probe_mm512_maskz_cvtneps_pbh(n: usize, h: &mut Digest) {
    if feat("avx512bf16") {
        let out = _mm512_maskz_cvtneps_pbh(kbits(n + 2) as __mmask16, fvec(n + 3));
        mix(h, &out);
    }
}

#[inline(never)]
unsafe fn probe_mm_cvtpbh_ps(n: usize, h: &mut Digest) {
    if feat("avx512bf16") && feat("avx512vl") {
        let out = _mm_cvtpbh_ps(bh(n + 1));
        mix(h, &out);
    }
}

#[inline(never)]
unsafe fn probe_mm_mask_cvtpbh_ps(n: usize, h: &mut Digest) {
    if feat("avx512bf16") && feat("avx512vl") {
        let out = _mm_mask_cvtpbh_ps(fvec(n + 1), kbits(n + 2) as __mmask8, bh(n + 3));
        mix(h, &out);
    }
}

#[inline(never)]
unsafe fn probe_mm_maskz_cvtpbh_ps(n: usize, h: &mut Digest) {
    if feat("avx512bf16") && feat("avx512vl") {
        let out = _mm_maskz_cvtpbh_ps(kbits(n + 2) as __mmask8, bh(n + 3));
        mix(h, &out);
    }
}

#[inline(never)]
unsafe fn probe_mm256_cvtpbh_ps(n: usize, h: &mut Digest) {
    if feat("avx512bf16") && feat("avx512vl") {
        let out = _mm256_cvtpbh_ps(bh(n + 1));
        mix(h, &out);
    }
}

#[inline(never)]
unsafe fn probe_mm256_mask_cvtpbh_ps(n: usize, h: &mut Digest) {
    if feat("avx512bf16") && feat("avx512vl") {
        let out = _mm256_mask_cvtpbh_ps(fvec(n + 1), kbits(n + 2) as __mmask8, bh(n + 3));
        mix(h, &out);
    }
}

#[inline(never)]
unsafe fn probe_mm256_maskz_cvtpbh_ps(n: usize, h: &mut Digest) {
    if feat("avx512bf16") && feat("avx512vl") {
        let out = _mm256_maskz_cvtpbh_ps(kbits(n + 2) as __mmask8, bh(n + 3));
        mix(h, &out);
    }
}

#[inline(never)]
unsafe fn probe_mm512_cvtpbh_ps(n: usize, h: &mut Digest) {
    if feat("avx512bf16") {
        let out = _mm512_cvtpbh_ps(bh(n + 1));
        mix(h, &out);
    }
}

#[inline(never)]
unsafe fn probe_mm512_mask_cvtpbh_ps(n: usize, h: &mut Digest) {
    if feat("avx512bf16") {
        let out = _mm512_mask_cvtpbh_ps(fvec(n + 1), kbits(n + 2) as __mmask16, bh(n + 3));
        mix(h, &out);
    }
}

#[inline(never)]
unsafe fn probe_mm512_maskz_cvtpbh_ps(n: usize, h: &mut Digest) {
    if feat("avx512bf16") {
        let out = _mm512_maskz_cvtpbh_ps(kbits(n + 2) as __mmask16, bh(n + 3));
        mix(h, &out);
    }
}

#[inline(never)]
unsafe fn probe_mm_dpbf16_ps(n: usize, h: &mut Digest) {
    if feat("avx512bf16") && feat("avx512vl") {
        let out = _mm_dpbf16_ps(fvec(n + 1), bh(n + 2), bh(n + 3));
        mix(h, &out);
    }
}

#[inline(never)]
unsafe fn probe_mm_mask_dpbf16_ps(n: usize, h: &mut Digest) {
    if feat("avx512bf16") && feat("avx512vl") {
        let out = _mm_mask_dpbf16_ps(fvec(n + 1), kbits(n + 2) as __mmask8, bh(n + 3), bh(n + 4));
        mix(h, &out);
    }
}

#[inline(never)]
unsafe fn probe_mm_maskz_dpbf16_ps(n: usize, h: &mut Digest) {
    if feat("avx512bf16") && feat("avx512vl") {
        let out = _mm_maskz_dpbf16_ps(kbits(n + 2) as __mmask8, fvec(n + 1), bh(n + 3), bh(n + 4));
        mix(h, &out);
    }
}

#[inline(never)]
unsafe fn probe_mm256_dpbf16_ps(n: usize, h: &mut Digest) {
    if feat("avx512bf16") && feat("avx512vl") {
        let out = _mm256_dpbf16_ps(fvec(n + 1), bh(n + 2), bh(n + 3));
        mix(h, &out);
    }
}

#[inline(never)]
unsafe fn probe_mm256_mask_dpbf16_ps(n: usize, h: &mut Digest) {
    if feat("avx512bf16") && feat("avx512vl") {
        let out = _mm256_mask_dpbf16_ps(fvec(n + 1), kbits(n + 2) as __mmask8, bh(n + 3), bh(n + 4));
        mix(h, &out);
    }
}

#[inline(never)]
unsafe fn probe_mm256_maskz_dpbf16_ps(n: usize, h: &mut Digest) {
    if feat("avx512bf16") && feat("avx512vl") {
        let out = _mm256_maskz_dpbf16_ps(kbits(n + 2) as __mmask8, fvec(n + 1), bh(n + 3), bh(n + 4));
        mix(h, &out);
    }
}

#[inline(never)]
unsafe fn probe_mm512_dpbf16_ps(n: usize, h: &mut Digest) {
    if feat("avx512bf16") {
        let out = _mm512_dpbf16_ps(fvec(n + 1), bh(n + 2), bh(n + 3));
        mix(h, &out);
    }
}

#[inline(never)]
unsafe fn probe_mm512_mask_dpbf16_ps(n: usize, h: &mut Digest) {
    if feat("avx512bf16") {
        let out = _mm512_mask_dpbf16_ps(fvec(n + 1), kbits(n + 2) as __mmask16, bh(n + 3), bh(n + 4));
        mix(h, &out);
    }
}

#[inline(never)]
unsafe fn probe_mm512_maskz_dpbf16_ps(n: usize, h: &mut Digest) {
    if feat("avx512bf16") {
        let out = _mm512_maskz_dpbf16_ps(kbits(n + 2) as __mmask16, fvec(n + 1), bh(n + 3), bh(n + 4));
        mix(h, &out);
    }
}

macro_rules! bf16_test {
    ($name:ident, $probe:ident, $intr:literal) => {
        #[test]
        fn $name() {
            let mut d = Digest::new();
            unsafe {
                for n in 0..512 {
                    $probe(core::hint::black_box(n), &mut d);
                }
            }
            d.line($intr, "none");
        }
    };
}

bf16_test!(a0001_mm_cvtne2ps_pbh, probe_mm_cvtne2ps_pbh, "_mm_cvtne2ps_pbh");
bf16_test!(a0002_mm_mask_cvtne2ps_pbh, probe_mm_mask_cvtne2ps_pbh, "_mm_mask_cvtne2ps_pbh");
bf16_test!(a0003_mm_maskz_cvtne2ps_pbh, probe_mm_maskz_cvtne2ps_pbh, "_mm_maskz_cvtne2ps_pbh");
bf16_test!(a0004_mm256_cvtne2ps_pbh, probe_mm256_cvtne2ps_pbh, "_mm256_cvtne2ps_pbh");
bf16_test!(a0005_mm256_mask_cvtne2ps_pbh, probe_mm256_mask_cvtne2ps_pbh, "_mm256_mask_cvtne2ps_pbh");
bf16_test!(a0006_mm256_maskz_cvtne2ps_pbh, probe_mm256_maskz_cvtne2ps_pbh, "_mm256_maskz_cvtne2ps_pbh");
bf16_test!(a0007_mm512_cvtne2ps_pbh, probe_mm512_cvtne2ps_pbh, "_mm512_cvtne2ps_pbh");
bf16_test!(a0008_mm512_mask_cvtne2ps_pbh, probe_mm512_mask_cvtne2ps_pbh, "_mm512_mask_cvtne2ps_pbh");
bf16_test!(a0009_mm512_maskz_cvtne2ps_pbh, probe_mm512_maskz_cvtne2ps_pbh, "_mm512_maskz_cvtne2ps_pbh");

bf16_test!(a0010_mm_cvtneps_pbh, probe_mm_cvtneps_pbh, "_mm_cvtneps_pbh");
bf16_test!(a0011_mm_mask_cvtneps_pbh, probe_mm_mask_cvtneps_pbh, "_mm_mask_cvtneps_pbh");
bf16_test!(a0012_mm_maskz_cvtneps_pbh, probe_mm_maskz_cvtneps_pbh, "_mm_maskz_cvtneps_pbh");
bf16_test!(a0013_mm256_cvtneps_pbh, probe_mm256_cvtneps_pbh, "_mm256_cvtneps_pbh");
bf16_test!(a0014_mm256_mask_cvtneps_pbh, probe_mm256_mask_cvtneps_pbh, "_mm256_mask_cvtneps_pbh");
bf16_test!(a0015_mm256_maskz_cvtneps_pbh, probe_mm256_maskz_cvtneps_pbh, "_mm256_maskz_cvtneps_pbh");
bf16_test!(a0016_mm512_cvtneps_pbh, probe_mm512_cvtneps_pbh, "_mm512_cvtneps_pbh");
bf16_test!(a0017_mm512_mask_cvtneps_pbh, probe_mm512_mask_cvtneps_pbh, "_mm512_mask_cvtneps_pbh");
bf16_test!(a0018_mm512_maskz_cvtneps_pbh, probe_mm512_maskz_cvtneps_pbh, "_mm512_maskz_cvtneps_pbh");

bf16_test!(a0019_mm_cvtpbh_ps, probe_mm_cvtpbh_ps, "_mm_cvtpbh_ps");
bf16_test!(a0020_mm_mask_cvtpbh_ps, probe_mm_mask_cvtpbh_ps, "_mm_mask_cvtpbh_ps");
bf16_test!(a0021_mm_maskz_cvtpbh_ps, probe_mm_maskz_cvtpbh_ps, "_mm_maskz_cvtpbh_ps");
bf16_test!(a0022_mm256_cvtpbh_ps, probe_mm256_cvtpbh_ps, "_mm256_cvtpbh_ps");
bf16_test!(a0023_mm256_mask_cvtpbh_ps, probe_mm256_mask_cvtpbh_ps, "_mm256_mask_cvtpbh_ps");
bf16_test!(a0024_mm256_maskz_cvtpbh_ps, probe_mm256_maskz_cvtpbh_ps, "_mm256_maskz_cvtpbh_ps");
bf16_test!(a0025_mm512_cvtpbh_ps, probe_mm512_cvtpbh_ps, "_mm512_cvtpbh_ps");
bf16_test!(a0026_mm512_mask_cvtpbh_ps, probe_mm512_mask_cvtpbh_ps, "_mm512_mask_cvtpbh_ps");
bf16_test!(a0027_mm512_maskz_cvtpbh_ps, probe_mm512_maskz_cvtpbh_ps, "_mm512_maskz_cvtpbh_ps");

bf16_test!(a0028_mm_dpbf16_ps, probe_mm_dpbf16_ps, "_mm_dpbf16_ps");
bf16_test!(a0029_mm_mask_dpbf16_ps, probe_mm_mask_dpbf16_ps, "_mm_mask_dpbf16_ps");
bf16_test!(a0030_mm_maskz_dpbf16_ps, probe_mm_maskz_dpbf16_ps, "_mm_maskz_dpbf16_ps");
bf16_test!(a0031_mm256_dpbf16_ps, probe_mm256_dpbf16_ps, "_mm256_dpbf16_ps");
bf16_test!(a0032_mm256_mask_dpbf16_ps, probe_mm256_mask_dpbf16_ps, "_mm256_mask_dpbf16_ps");
bf16_test!(a0033_mm256_maskz_dpbf16_ps, probe_mm256_maskz_dpbf16_ps, "_mm256_maskz_dpbf16_ps");
bf16_test!(a0034_mm512_dpbf16_ps, probe_mm512_dpbf16_ps, "_mm512_dpbf16_ps");
bf16_test!(a0035_mm512_mask_dpbf16_ps, probe_mm512_mask_dpbf16_ps, "_mm512_mask_dpbf16_ps");
bf16_test!(a0036_mm512_maskz_dpbf16_ps, probe_mm512_maskz_dpbf16_ps, "_mm512_maskz_dpbf16_ps");

#[test]
fn a0037_dpbf16_rounding_edges() {
    if !feat("avx512bf16") || !feat("avx512vl") {
        return;
    }
    // Negative zero must survive both accumulation stages. Include negative
    // underflow and cancellation at the normal/subnormal rounding boundary.
    let cases = [
        (0x80000000u32, 0x8000u16, 0x3f80u16, 0x8000u16, 0x3f80u16),
        (0x80000000, 0x8000, 0x3f80, 0x8080, 0x3880),
        (0x00800000, 0x8000, 0x3f80, 0x8080, 0x3380),
        (0x80800000, 0x0000, 0x3f80, 0x0080, 0x3380),
    ];
    let mut d = Digest::new();
    for (s, a0, b0, a1, b1) in cases {
        unsafe {
            let src: __m128 = core::mem::transmute([f32::from_bits(s); 4]);
            let a: __m128bh = core::mem::transmute([a0, a1, a0, a1, a0, a1, a0, a1]);
            let b: __m128bh = core::mem::transmute([b0, b1, b0, b1, b0, b1, b0, b1]);
            let out = _mm_dpbf16_ps(core::hint::black_box(src), a, b);
            mix(&mut d, &out);
        }
    }
    d.line("_mm_dpbf16_ps", "rounding_edges");
}

#[test]
fn a0038_dpbf16_random_bits() {
    if !feat("avx512bf16") || !feat("avx512vl") {
        return;
    }
    let mut rng = Rng(0xdeadbeef12345678);
    let mut d = Digest::new();
    for _ in 0..RANDOM_CASES {
        let mut src = [0.0f32; 4];
        let mut a = [0u16; 8];
        let mut b = [0u16; 8];
        for i in 0..4 {
            src[i] = f32::from_bits(rng.next() as u32);
        }
        for i in 0..8 {
            a[i] = rng.next() as u16;
            b[i] = rng.next() as u16;
        }
        unsafe {
            let out = _mm_dpbf16_ps(core::mem::transmute(src), core::mem::transmute(a), core::mem::transmute(b));
            mix(&mut d, &out);
        }
    }
    d.line("_mm_dpbf16_ps", "random_bits");
}
