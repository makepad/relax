#![cfg(target_arch = "x86_64")]
#![allow(deprecated, unused_unsafe)]

use core::arch::x86_64::*;

fn has_avx512() -> bool {
    unsafe {
        let p = __cpuid(1);
        p.ecx & (3 << 26) == (3 << 26)
            && _xgetbv(0) & 0xe6 == 0xe6
            && __cpuid_count(7, 0).ebx & (1 << 16) != 0
    }
}

#[inline(never)]
#[target_feature(enable = "avx512f")]
unsafe fn rounded_add<const R: i32>(
    a: [f32; 4],
    b: [f32; 4],
    c: [f64; 4],
    d: [f64; 4],
) -> ([u32; 4], [u32; 4], [u64; 4], [u64; 4]) {
    let mut scalar32 = [0; 4];
    let mut scalar64 = [0; 4];
    for i in 0..4 {
        scalar32[i] =
            _mm_cvtss_f32(_mm_add_round_ss::<R>(_mm_set_ss(a[i]), _mm_set_ss(b[i]))).to_bits();
        scalar64[i] =
            _mm_cvtsd_f64(_mm_add_round_sd::<R>(_mm_set_sd(c[i]), _mm_set_sd(d[i]))).to_bits();
    }
    let va = _mm512_setr_ps(
        a[0], a[1], a[2], a[3], a[0], a[1], a[2], a[3], a[0], a[1], a[2], a[3], a[0], a[1], a[2],
        a[3],
    );
    let vb = _mm512_setr_ps(
        b[0], b[1], b[2], b[3], b[0], b[1], b[2], b[3], b[0], b[1], b[2], b[3], b[0], b[1], b[2],
        b[3],
    );
    let vc = _mm512_setr_pd(c[0], c[1], c[2], c[3], c[0], c[1], c[2], c[3]);
    let vd = _mm512_setr_pd(d[0], d[1], d[2], d[3], d[0], d[1], d[2], d[3]);
    let p: [f32; 16] = core::mem::transmute(_mm512_add_round_ps::<R>(va, vb));
    let q: [f64; 8] = core::mem::transmute(_mm512_add_round_pd::<R>(vc, vd));
    (
        scalar32,
        [
            p[0].to_bits(),
            p[1].to_bits(),
            p[2].to_bits(),
            p[3].to_bits(),
        ],
        scalar64,
        [
            q[0].to_bits(),
            q[1].to_bits(),
            q[2].to_bits(),
            q[3].to_bits(),
        ],
    )
}

#[test]
fn embedded_add_rounding_midpoints_and_overflow() {
    if !has_avx512() {
        return;
    }
    unsafe {
        let a = [1.0, -1.0, f32::MAX, -f32::MAX];
        let b = [
            f32::from_bits(0x33800000),
            -f32::from_bits(0x33800000),
            f32::MAX,
            -f32::MAX,
        ];
        let c = [1.0, -1.0, f64::MAX, -f64::MAX];
        let d = [
            f64::from_bits(0x3ca0000000000000),
            -f64::from_bits(0x3ca0000000000000),
            f64::MAX,
            -f64::MAX,
        ];
        let old = _mm_getcsr();
        // Compile all Relax entry points before clearing exception flags.
        let _ = rounded_add::<8>(a, b, c, d);
        let _ = rounded_add::<9>(a, b, c, d);
        let _ = rounded_add::<10>(a, b, c, d);
        let _ = rounded_add::<11>(a, b, c, d);
        _mm_setcsr((old | 0x1f80) & !0x603f | 4);
        let a = core::hint::black_box(a);
        let b = core::hint::black_box(b);
        let c = core::hint::black_box(c);
        let d = core::hint::black_box(d);
        let result = [
            rounded_add::<8>(a, b, c, d),
            rounded_add::<9>(a, b, c, d),
            rounded_add::<10>(a, b, c, d),
            rounded_add::<11>(a, b, c, d),
        ];
        let flags = _mm_getcsr() & 0x3f;
        _mm_setcsr(old);
        let expected32 = [
            [0x3f800000, 0xbf800000, 0x7f800000, 0xff800000],
            [0x3f800000, 0xbf800001, 0x7f7fffff, 0xff800000],
            [0x3f800001, 0xbf800000, 0x7f800000, 0xff7fffff],
            [0x3f800000, 0xbf800000, 0x7f7fffff, 0xff7fffff],
        ];
        let expected64 = [
            [
                0x3ff0000000000000,
                0xbff0000000000000,
                0x7ff0000000000000,
                0xfff0000000000000,
            ],
            [
                0x3ff0000000000000,
                0xbff0000000000001,
                0x7fefffffffffffff,
                0xfff0000000000000,
            ],
            [
                0x3ff0000000000001,
                0xbff0000000000000,
                0x7ff0000000000000,
                0xffefffffffffffff,
            ],
            [
                0x3ff0000000000000,
                0xbff0000000000000,
                0x7fefffffffffffff,
                0xffefffffffffffff,
            ],
        ];
        for mode in 0..4 {
            assert_eq!(result[mode].0, expected32[mode], "scalar f32 mode {mode}");
            assert_eq!(result[mode].1, expected32[mode], "packed f32 mode {mode}");
            assert_eq!(result[mode].2, expected64[mode], "scalar f64 mode {mode}");
            assert_eq!(result[mode].3, expected64[mode], "packed f64 mode {mode}");
        }
        assert_eq!(flags, 4, "SAE must preserve existing exception flags");
    }
}

#[inline(never)]
unsafe fn convert32(x: f32) -> (i32, i32) {
    let v = _mm_set_ss(x);
    (_mm_cvtss_si32(v), _mm_cvttss_si32(v))
}

#[inline(never)]
unsafe fn convert64(x: f64) -> (i32, i32) {
    let v = _mm_set_sd(x);
    (_mm_cvtsd_si32(v), _mm_cvttsd_si32(v))
}

#[test]
fn conversions_use_current_mxcsr() {
    unsafe {
        let old = _mm_getcsr();
        let _ = convert32(1.0);
        let _ = convert64(1.0);
        let mut f32_results = [[(0, 0); 2]; 4];
        let mut f64_results = [[(0, 0); 2]; 4];
        for mode in 0..4 {
            _mm_setcsr((old | 0x1f80) & !(3 << 13) | ((mode as u32) << 13));
            for (i, x) in [1.5, -1.5].iter().enumerate() {
                f32_results[mode][i] = convert32(core::hint::black_box(*x));
                f64_results[mode][i] = convert64(core::hint::black_box(*x as f64));
            }
        }
        _mm_setcsr(old);
        let expected = [
            [(2, 1), (-2, -1)],
            [(1, 1), (-2, -1)],
            [(2, 1), (-1, -1)],
            [(1, 1), (-1, -1)],
        ];
        assert_eq!(f32_results, expected);
        assert_eq!(f64_results, expected);
    }
}
