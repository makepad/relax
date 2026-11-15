// core::arch::x86_64 differential coverage (core_diff.sh): each probe hashes outputs
// over edge-case inputs and prints "| name tag count hash"; the lines must equal real
// std's under rustc --test. AVX-512 is probed through CPUID/XCR0 directly (feat in
// arch/x86_simd.rs) since relax's detect reports no AVX-512 by policy while the
// emulated intrinsics compute regardless.
#![allow(unused_unsafe, non_snake_case)]

include!("arch/common.rs");
include!("arch/x86_simd.rs");
use core::arch::x86_64::*;


    #[inline(never)]
    unsafe fn probe_mm256_sha512rnds2_epi64 (n: usize, h: &mut Digest) {
        if feat("sha512") {
            let out = _mm256_sha512rnds2_epi64(input(n+1), input(n+2), input(n+3)); mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_sha512msg1_epi64 (n: usize, h: &mut Digest) {
        if feat("sha512") {
            let out = _mm256_sha512msg1_epi64(input(n+1), input(n+2)); mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_sha512msg2_epi64 (n: usize, h: &mut Digest) {
        if feat("sha512") {
            let out = _mm256_sha512msg2_epi64(input(n+1), input(n+2)); mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_gf2p8affine_epi64_epi8 (n: usize, h: &mut Digest) {
        if feat("gfni") {
            let out = _mm_gf2p8affine_epi64_epi8::<0x71>(input(n+1), input(n+2)); mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_gf2p8affineinv_epi64_epi8 (n: usize, h: &mut Digest) {
        if feat("gfni") {
            let out = _mm_gf2p8affineinv_epi64_epi8::<0x71>(input(n+1), input(n+2)); mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_gf2p8mul_epi8 (n: usize, h: &mut Digest) {
        if feat("gfni") {
            let out = _mm_gf2p8mul_epi8(input(n+1), input(n+2)); mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_gf2p8affine_epi64_epi8 (n: usize, h: &mut Digest) {
        if feat("gfni") {
            let out = _mm256_gf2p8affine_epi64_epi8::<0x71>(input(n+1), input(n+2)); mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_gf2p8affineinv_epi64_epi8 (n: usize, h: &mut Digest) {
        if feat("gfni") {
            let out = _mm256_gf2p8affineinv_epi64_epi8::<0x71>(input(n+1), input(n+2)); mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_gf2p8mul_epi8 (n: usize, h: &mut Digest) {
        if feat("gfni") {
            let out = _mm256_gf2p8mul_epi8(input(n+1), input(n+2)); mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_gf2p8affine_epi64_epi8 (n: usize, h: &mut Digest) {
        if feat("gfni") {
            let out = _mm512_gf2p8affine_epi64_epi8::<0x71>(input(n+1), input(n+2)); mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_gf2p8affineinv_epi64_epi8 (n: usize, h: &mut Digest) {
        if feat("gfni") {
            let out = _mm512_gf2p8affineinv_epi64_epi8::<0x71>(input(n+1), input(n+2)); mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_gf2p8mul_epi8 (n: usize, h: &mut Digest) {
        if feat("gfni") {
            let out = _mm512_gf2p8mul_epi8(input(n+1), input(n+2)); mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe__mm512_dpbusd_epi32(n: usize, h: &mut Digest) {
        let s: __m512i = input(n + 0);
        let a: __m512i = input(n + 5);
        let b: __m512i = input(n + 9);
        let out = _mm512_dpbusd_epi32(s, a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe__mm256_dpbusd_epi32(n: usize, h: &mut Digest) {
        let s: __m256i = input(n + 0);
        let a: __m256i = input(n + 5);
        let b: __m256i = input(n + 9);
        let out = _mm256_dpbusd_epi32(s, a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe__mm512_permutexvar_ps(n: usize, h: &mut Digest) {
        let i: __m512i = input(n + 0);
        let a: __m512 = float_input(n + 5);
        let out = _mm512_permutexvar_ps(i, a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe__mm256_permutexvar_ps(n: usize, h: &mut Digest) {
        let i: __m256i = input(n + 0);
        let a: __m256 = float_input(n + 5);
        let out = _mm256_permutexvar_ps(i, a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe__mm512_permutexvar_epi16(n: usize, h: &mut Digest) {
        let i: __m512i = input(n + 0);
        let a: __m512i = input(n + 5);
        let out = _mm512_permutexvar_epi16(i, a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm256_dpbssd_epi32(n: usize, h: &mut Digest) {
        if feat("avxvnniint8") {
            let out = _mm256_dpbssd_epi32(input(n+3), input(n+0), input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_dpbssds_epi32(n: usize, h: &mut Digest) {
        if feat("avxvnniint8") {
            let out = _mm256_dpbssds_epi32(input(n+3), input(n+0), input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_dpbusd_avx_epi32(n: usize, h: &mut Digest) {
        if feat("avxvnni") {
            let out = _mm256_dpbusd_avx_epi32(input(n+3), input(n+0), input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_dpbusds_avx_epi32(n: usize, h: &mut Digest) {
        if feat("avxvnni") {
            let out = _mm256_dpbusds_avx_epi32(input(n+3), input(n+0), input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_dpbusds_epi32(n: usize, h: &mut Digest) {
        if feat("avx512vnni") {
            let out = _mm256_dpbusds_epi32(input(n+3), input(n+0), input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_dpwssd_avx_epi32(n: usize, h: &mut Digest) {
        if feat("avxvnni") {
            let out = _mm256_dpwssd_avx_epi32(input(n+3), input(n+0), input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_dpwssd_epi32(n: usize, h: &mut Digest) {
        if feat("avx512vnni") {
            let out = _mm256_dpwssd_epi32(input(n+3), input(n+0), input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_dpwssds_avx_epi32(n: usize, h: &mut Digest) {
        if feat("avxvnni") {
            let out = _mm256_dpwssds_avx_epi32(input(n+3), input(n+0), input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_dpwssds_epi32(n: usize, h: &mut Digest) {
        if feat("avx512vnni") {
            let out = _mm256_dpwssds_epi32(input(n+3), input(n+0), input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_madd52hi_avx_epu64(n: usize, h: &mut Digest) {
        if feat("avxifma") {
            let out = _mm256_madd52hi_avx_epu64(input(n+0), input(n+1), input(n+2));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_madd52lo_avx_epu64(n: usize, h: &mut Digest) {
        if feat("avxifma") {
            let out = _mm256_madd52lo_avx_epu64(input(n+0), input(n+1), input(n+2));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_multishift_epi64_epi8(n: usize, h: &mut Digest) {
        if feat("avx512vbmi") {
            let out = _mm256_multishift_epi64_epi8(input(n+0), input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_popcnt_epi16(n: usize, h: &mut Digest) {
        if feat("avx512bitalg") {
            let out = _mm256_popcnt_epi16(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_popcnt_epi8(n: usize, h: &mut Digest) {
        if feat("avx512bitalg") {
            let out = _mm256_popcnt_epi8(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_sm4key4_epi32(n: usize, h: &mut Digest) {
        if feat("sm4") {
            let out = _mm256_sm4key4_epi32(input(n+0), input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_sm4rnds4_epi32(n: usize, h: &mut Digest) {
        if feat("sm4") {
            let out = _mm256_sm4rnds4_epi32(input(n+0), input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_dpbusds_epi32(n: usize, h: &mut Digest) {
        if feat("avx512vnni") {
            let out = _mm512_dpbusds_epi32(input(n+3), input(n+0), input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_dpwssd_epi32(n: usize, h: &mut Digest) {
        if feat("avx512vnni") {
            let out = _mm512_dpwssd_epi32(input(n+3), input(n+0), input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_dpwssds_epi32(n: usize, h: &mut Digest) {
        if feat("avx512vnni") {
            let out = _mm512_dpwssds_epi32(input(n+3), input(n+0), input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_madd52hi_epu64(n: usize, h: &mut Digest) {
        if feat("avx512ifma") {
            let out = _mm512_madd52hi_epu64(input(n+0), input(n+1), input(n+2));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_madd52lo_epu64(n: usize, h: &mut Digest) {
        if feat("avx512ifma") {
            let out = _mm512_madd52lo_epu64(input(n+0), input(n+1), input(n+2));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_multishift_epi64_epi8(n: usize, h: &mut Digest) {
        if feat("avx512vbmi") {
            let out = _mm512_multishift_epi64_epi8(input(n+0), input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_popcnt_epi16(n: usize, h: &mut Digest) {
        if feat("avx512bitalg") {
            let out = _mm512_popcnt_epi16(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_popcnt_epi8(n: usize, h: &mut Digest) {
        if feat("avx512bitalg") {
            let out = _mm512_popcnt_epi8(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_dpbssd_epi32(n: usize, h: &mut Digest) {
        if feat("avxvnniint8") {
            let out = _mm_dpbssd_epi32(input(n+3), input(n+0), input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_dpbssds_epi32(n: usize, h: &mut Digest) {
        if feat("avxvnniint8") {
            let out = _mm_dpbssds_epi32(input(n+3), input(n+0), input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_dpbusd_avx_epi32(n: usize, h: &mut Digest) {
        if feat("avxvnni") {
            let out = _mm_dpbusd_avx_epi32(input(n+3), input(n+0), input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_dpbusd_epi32(n: usize, h: &mut Digest) {
        if feat("avx512vnni") {
            let out = _mm_dpbusd_epi32(input(n+3), input(n+0), input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_dpbusds_avx_epi32(n: usize, h: &mut Digest) {
        if feat("avxvnni") {
            let out = _mm_dpbusds_avx_epi32(input(n+3), input(n+0), input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_dpbusds_epi32(n: usize, h: &mut Digest) {
        if feat("avx512vnni") {
            let out = _mm_dpbusds_epi32(input(n+3), input(n+0), input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_dpwssd_avx_epi32(n: usize, h: &mut Digest) {
        if feat("avxvnni") {
            let out = _mm_dpwssd_avx_epi32(input(n+3), input(n+0), input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_dpwssd_epi32(n: usize, h: &mut Digest) {
        if feat("avx512vnni") {
            let out = _mm_dpwssd_epi32(input(n+3), input(n+0), input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_dpwssds_avx_epi32(n: usize, h: &mut Digest) {
        if feat("avxvnni") {
            let out = _mm_dpwssds_avx_epi32(input(n+3), input(n+0), input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_dpwssds_epi32(n: usize, h: &mut Digest) {
        if feat("avx512vnni") {
            let out = _mm_dpwssds_epi32(input(n+3), input(n+0), input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_madd52hi_avx_epu64(n: usize, h: &mut Digest) {
        if feat("avxifma") {
            let out = _mm_madd52hi_avx_epu64(input(n+0), input(n+1), input(n+2));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_madd52hi_epu64(n: usize, h: &mut Digest) {
        if feat("avxifma") {
            let out = _mm_madd52hi_epu64(input(n+0), input(n+1), input(n+2));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_madd52lo_avx_epu64(n: usize, h: &mut Digest) {
        if feat("avxifma") {
            let out = _mm_madd52lo_avx_epu64(input(n+0), input(n+1), input(n+2));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_madd52lo_epu64(n: usize, h: &mut Digest) {
        if feat("avxifma") {
            let out = _mm_madd52lo_epu64(input(n+0), input(n+1), input(n+2));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_multishift_epi64_epi8(n: usize, h: &mut Digest) {
        if feat("avx512vbmi") {
            let out = _mm_multishift_epi64_epi8(input(n+0), input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_popcnt_epi16(n: usize, h: &mut Digest) {
        if feat("avx512bitalg") {
            let out = _mm_popcnt_epi16(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_popcnt_epi8(n: usize, h: &mut Digest) {
        if feat("avx512bitalg") {
            let out = _mm_popcnt_epi8(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_sm3msg1_epi32(n: usize, h: &mut Digest) {
        if feat("sm3") {
            let out = _mm_sm3msg1_epi32(input(n+0), input(n+1), input(n+2));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_sm3msg2_epi32(n: usize, h: &mut Digest) {
        if feat("sm3") {
            let out = _mm_sm3msg2_epi32(input(n+0), input(n+1), input(n+2));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_sm3rnds2_epi32(n: usize, h: &mut Digest) {
        if feat("sm3") {
            let out = _mm_sm3rnds2_epi32::<0x32>(input(n+0), input(n+1), input(n+2));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_sm4key4_epi32(n: usize, h: &mut Digest) {
        if feat("sm4") {
            let out = _mm_sm4key4_epi32(input(n+0), input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_sm4rnds4_epi32(n: usize, h: &mut Digest) {
        if feat("sm4") {
            let out = _mm_sm4rnds4_epi32(input(n+0), input(n+1));
            mix(h, &out);
        }
    }
#[test]
fn a0001_mm256_sha512rnds2_epi64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_sha512rnds2_epi64(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_sha512rnds2_epi64", "none");
}

#[test]
fn a0002_mm256_sha512msg1_epi64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_sha512msg1_epi64(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_sha512msg1_epi64", "none");
}

#[test]
fn a0003_mm256_sha512msg2_epi64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_sha512msg2_epi64(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_sha512msg2_epi64", "none");
}

#[test]
fn a0004_mm_gf2p8affine_epi64_epi8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_gf2p8affine_epi64_epi8(core::hint::black_box(n), &mut d); } }
    d.line("_mm_gf2p8affine_epi64_epi8", "none");
}

#[test]
fn a0005_mm_gf2p8affineinv_epi64_epi8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_gf2p8affineinv_epi64_epi8(core::hint::black_box(n), &mut d); } }
    d.line("_mm_gf2p8affineinv_epi64_epi8", "none");
}

#[test]
fn a0006_mm_gf2p8mul_epi8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_gf2p8mul_epi8(core::hint::black_box(n), &mut d); } }
    d.line("_mm_gf2p8mul_epi8", "none");
}

#[test]
fn a0007_mm256_gf2p8affine_epi64_epi8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_gf2p8affine_epi64_epi8(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_gf2p8affine_epi64_epi8", "none");
}

#[test]
fn a0008_mm256_gf2p8affineinv_epi64_epi8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_gf2p8affineinv_epi64_epi8(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_gf2p8affineinv_epi64_epi8", "none");
}

#[test]
fn a0009_mm256_gf2p8mul_epi8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_gf2p8mul_epi8(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_gf2p8mul_epi8", "none");
}

#[test]
fn a0010_mm512_gf2p8affine_epi64_epi8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_gf2p8affine_epi64_epi8(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_gf2p8affine_epi64_epi8", "none");
}

#[test]
fn a0011_mm512_gf2p8affineinv_epi64_epi8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_gf2p8affineinv_epi64_epi8(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_gf2p8affineinv_epi64_epi8", "none");
}

#[test]
fn a0012_mm512_gf2p8mul_epi8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_gf2p8mul_epi8(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_gf2p8mul_epi8", "none");
}

#[test]
fn a0013__mm512_dpbusd_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe__mm512_dpbusd_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_dpbusd_epi32", "none");
}

#[test]
fn a0014__mm256_dpbusd_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe__mm256_dpbusd_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_dpbusd_epi32", "none");
}

#[test]
fn a0015__mm512_permutexvar_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe__mm512_permutexvar_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_permutexvar_ps", "none");
}

#[test]
fn a0016__mm256_permutexvar_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe__mm256_permutexvar_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_permutexvar_ps", "none");
}

#[test]
fn a0017__mm512_permutexvar_epi16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe__mm512_permutexvar_epi16(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_permutexvar_epi16", "none");
}

#[test]
fn a0018_mm256_dpbssd_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_dpbssd_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_dpbssd_epi32", "none");
}

#[test]
fn a0019_mm256_dpbssds_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_dpbssds_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_dpbssds_epi32", "none");
}

#[test]
fn a0020_mm256_dpbusd_avx_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_dpbusd_avx_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_dpbusd_avx_epi32", "none");
}

#[test]
fn a0021_mm256_dpbusds_avx_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_dpbusds_avx_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_dpbusds_avx_epi32", "none");
}

#[test]
fn a0022_mm256_dpbusds_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_dpbusds_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_dpbusds_epi32", "none");
}

#[test]
fn a0023_mm256_dpwssd_avx_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_dpwssd_avx_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_dpwssd_avx_epi32", "none");
}

#[test]
fn a0024_mm256_dpwssd_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_dpwssd_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_dpwssd_epi32", "none");
}

#[test]
fn a0025_mm256_dpwssds_avx_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_dpwssds_avx_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_dpwssds_avx_epi32", "none");
}

#[test]
fn a0026_mm256_dpwssds_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_dpwssds_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_dpwssds_epi32", "none");
}

#[test]
fn a0027_mm256_madd52hi_avx_epu64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_madd52hi_avx_epu64(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_madd52hi_avx_epu64", "none");
}

#[test]
fn a0028_mm256_madd52lo_avx_epu64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_madd52lo_avx_epu64(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_madd52lo_avx_epu64", "none");
}

#[test]
fn a0029_mm256_multishift_epi64_epi8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_multishift_epi64_epi8(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_multishift_epi64_epi8", "none");
}

#[test]
fn a0030_mm256_popcnt_epi16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_popcnt_epi16(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_popcnt_epi16", "none");
}

#[test]
fn a0031_mm256_popcnt_epi8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_popcnt_epi8(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_popcnt_epi8", "none");
}

#[test]
fn a0032_mm256_sm4key4_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_sm4key4_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_sm4key4_epi32", "none");
}

#[test]
fn a0033_mm256_sm4rnds4_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_sm4rnds4_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_sm4rnds4_epi32", "none");
}

#[test]
fn a0034_mm512_dpbusds_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_dpbusds_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_dpbusds_epi32", "none");
}

#[test]
fn a0035_mm512_dpwssd_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_dpwssd_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_dpwssd_epi32", "none");
}

#[test]
fn a0036_mm512_dpwssds_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_dpwssds_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_dpwssds_epi32", "none");
}

#[test]
fn a0037_mm512_madd52hi_epu64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_madd52hi_epu64(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_madd52hi_epu64", "none");
}

#[test]
fn a0038_mm512_madd52lo_epu64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_madd52lo_epu64(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_madd52lo_epu64", "none");
}

#[test]
fn a0039_mm512_multishift_epi64_epi8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_multishift_epi64_epi8(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_multishift_epi64_epi8", "none");
}

#[test]
fn a0040_mm512_popcnt_epi16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_popcnt_epi16(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_popcnt_epi16", "none");
}

#[test]
fn a0041_mm512_popcnt_epi8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_popcnt_epi8(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_popcnt_epi8", "none");
}

#[test]
fn a0042_mm_dpbssd_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_dpbssd_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_dpbssd_epi32", "none");
}

#[test]
fn a0043_mm_dpbssds_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_dpbssds_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_dpbssds_epi32", "none");
}

#[test]
fn a0044_mm_dpbusd_avx_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_dpbusd_avx_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_dpbusd_avx_epi32", "none");
}

#[test]
fn a0045_mm_dpbusd_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_dpbusd_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_dpbusd_epi32", "none");
}

#[test]
fn a0046_mm_dpbusds_avx_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_dpbusds_avx_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_dpbusds_avx_epi32", "none");
}

#[test]
fn a0047_mm_dpbusds_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_dpbusds_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_dpbusds_epi32", "none");
}

#[test]
fn a0048_mm_dpwssd_avx_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_dpwssd_avx_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_dpwssd_avx_epi32", "none");
}

#[test]
fn a0049_mm_dpwssd_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_dpwssd_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_dpwssd_epi32", "none");
}

#[test]
fn a0050_mm_dpwssds_avx_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_dpwssds_avx_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_dpwssds_avx_epi32", "none");
}

#[test]
fn a0051_mm_dpwssds_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_dpwssds_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_dpwssds_epi32", "none");
}

#[test]
fn a0052_mm_madd52hi_avx_epu64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_madd52hi_avx_epu64(core::hint::black_box(n), &mut d); } }
    d.line("_mm_madd52hi_avx_epu64", "none");
}

#[test]
fn a0053_mm_madd52hi_epu64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_madd52hi_epu64(core::hint::black_box(n), &mut d); } }
    d.line("_mm_madd52hi_epu64", "none");
}

#[test]
fn a0054_mm_madd52lo_avx_epu64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_madd52lo_avx_epu64(core::hint::black_box(n), &mut d); } }
    d.line("_mm_madd52lo_avx_epu64", "none");
}

#[test]
fn a0055_mm_madd52lo_epu64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_madd52lo_epu64(core::hint::black_box(n), &mut d); } }
    d.line("_mm_madd52lo_epu64", "none");
}

#[test]
fn a0056_mm_multishift_epi64_epi8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_multishift_epi64_epi8(core::hint::black_box(n), &mut d); } }
    d.line("_mm_multishift_epi64_epi8", "none");
}

#[test]
fn a0057_mm_popcnt_epi16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_popcnt_epi16(core::hint::black_box(n), &mut d); } }
    d.line("_mm_popcnt_epi16", "none");
}

#[test]
fn a0058_mm_popcnt_epi8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_popcnt_epi8(core::hint::black_box(n), &mut d); } }
    d.line("_mm_popcnt_epi8", "none");
}

#[test]
fn a0059_mm_sm3msg1_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_sm3msg1_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_sm3msg1_epi32", "none");
}

#[test]
fn a0060_mm_sm3msg2_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_sm3msg2_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_sm3msg2_epi32", "none");
}

#[test]
fn a0061_mm_sm3rnds2_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_sm3rnds2_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_sm3rnds2_epi32", "none");
}

#[test]
fn a0062_mm_sm4key4_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_sm4key4_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_sm4key4_epi32", "none");
}

#[test]
fn a0063_mm_sm4rnds4_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_sm4rnds4_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_sm4rnds4_epi32", "none");
}
