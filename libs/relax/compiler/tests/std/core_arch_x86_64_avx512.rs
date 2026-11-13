// core::arch::x86_64 differential coverage (core_diff.sh): each probe hashes outputs
// over edge-case inputs and prints "| name tag count hash"; the lines must equal real
// std's under rustc --test. AVX-512 is probed through CPUID/XCR0 directly (feat in
// arch/x86_simd.rs) since relax's detect reports no AVX-512 by policy while the
// emulated intrinsics compute regardless.
#![allow(unused_unsafe, non_snake_case)]

include!("arch/common.rs");
include!("arch/x86_simd.rs");
use core::arch::x86_64::*;
const I1: i32 = 1;
const I8: i32 = 8;



    #[inline(never)]
    unsafe fn probe__mm512_setzero_ps(_n: usize, h: &mut Digest) {
        let out = _mm512_setzero_ps();
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe__mm512_set1_ps(n: usize, h: &mut Digest) {
        let a: f32 = 1.5 + (n % 7) as f32;
        let out = _mm512_set1_ps(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe__mm512_set1_epi8(n: usize, h: &mut Digest) {
        let a: i8 = (n % 251) as u8 as i8;
        let out = _mm512_set1_epi8(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe__mm512_set1_epi32(n: usize, h: &mut Digest) {
        let a: i32 = (n as u32).wrapping_mul(2654435761) as i32;
        let out = _mm512_set1_epi32(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe__mm512_set1_epi64(n: usize, h: &mut Digest) {
        let a: i64 = (n as u64).wrapping_mul(0x9e3779b97f4a7c15) as i64;
        let out = _mm512_set1_epi64(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe__mm512_add_ps(n: usize, h: &mut Digest) {
        let a: __m512 = float_input(n + 0);
        let b: __m512 = float_input(n + 5);
        let out = _mm512_add_ps(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe__mm512_sub_ps(n: usize, h: &mut Digest) {
        let a: __m512 = float_input(n + 0);
        let b: __m512 = float_input(n + 5);
        let out = _mm512_sub_ps(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe__mm512_mul_ps(n: usize, h: &mut Digest) {
        let a: __m512 = float_input(n + 0);
        let b: __m512 = float_input(n + 5);
        let out = _mm512_mul_ps(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe__mm512_div_ps(n: usize, h: &mut Digest) {
        let a: __m512 = float_input(n + 0);
        let b: __m512 = float_input(n + 5);
        let out = _mm512_div_ps(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe__mm512_andnot_ps(n: usize, h: &mut Digest) {
        let a: __m512 = float_input(n + 0);
        let b: __m512 = float_input(n + 5);
        let out = _mm512_andnot_ps(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe__mm512_fmadd_ps(n: usize, h: &mut Digest) {
        let a: __m512 = float_input(n + 0);
        let b: __m512 = float_input(n + 5);
        let c: __m512 = float_input(n + 9);
        let out = _mm512_fmadd_ps(a, b, c);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe__mm512_fnmadd_ps(n: usize, h: &mut Digest) {
        let a: __m512 = float_input(n + 0);
        let b: __m512 = float_input(n + 5);
        let c: __m512 = float_input(n + 9);
        let out = _mm512_fnmadd_ps(a, b, c);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe__mm512_scalef_ps(n: usize, h: &mut Digest) {
        let a: __m512 = float_input(n + 0);
        let b: __m512 = float_input(n + 5);
        let out = _mm512_scalef_ps(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe__mm512_cvtps_ph_0(n: usize, h: &mut Digest) {
        let a: __m512 = float_input(n + 0);
        let out = _mm512_cvtps_ph::<0>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe__mm512_cvtps_ph_1(n: usize, h: &mut Digest) {
        let a: __m512 = float_input(n + 0);
        let out = _mm512_cvtps_ph::<1>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe__mm512_cvtps_ph_2(n: usize, h: &mut Digest) {
        let a: __m512 = float_input(n + 0);
        let out = _mm512_cvtps_ph::<2>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe__mm512_cvtps_ph_3(n: usize, h: &mut Digest) {
        let a: __m512 = float_input(n + 0);
        let out = _mm512_cvtps_ph::<3>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe__mm512_cvtps_ph_8(n: usize, h: &mut Digest) {
        let a: __m512 = float_input(n + 0);
        let out = _mm512_cvtps_ph::<8>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe__mm512_cvtph_ps(n: usize, h: &mut Digest) {
        let a: __m256i = input(n + 0);
        let out = _mm512_cvtph_ps(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe__mm512_cvttps_epi32(n: usize, h: &mut Digest) {
        let a: __m512 = float_input(n + 0);
        let out = _mm512_cvttps_epi32(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe__mm512_cvtepi32_ps(n: usize, h: &mut Digest) {
        let a: __m512i = input(n + 0);
        let out = _mm512_cvtepi32_ps(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe__mm512_reduce_add_ps(n: usize, h: &mut Digest) {
        let a: __m512 = float_input(n + 0);
        let out = _mm512_reduce_add_ps(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe__mm512_reduce_add_epi32(n: usize, h: &mut Digest) {
        let a: __m512i = input(n + 0);
        let out = _mm512_reduce_add_epi32(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe__mm512_and_si512(n: usize, h: &mut Digest) {
        let a: __m512i = input(n + 0);
        let b: __m512i = input(n + 5);
        let out = _mm512_and_si512(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe__mm512_or_si512(n: usize, h: &mut Digest) {
        let a: __m512i = input(n + 0);
        let b: __m512i = input(n + 5);
        let out = _mm512_or_si512(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe__mm512_sub_epi8(n: usize, h: &mut Digest) {
        let a: __m512i = input(n + 0);
        let b: __m512i = input(n + 5);
        let out = _mm512_sub_epi8(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe__mm512_sub_epi32(n: usize, h: &mut Digest) {
        let a: __m512i = input(n + 0);
        let b: __m512i = input(n + 5);
        let out = _mm512_sub_epi32(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe__mm512_mullo_epi32(n: usize, h: &mut Digest) {
        let a: __m512i = input(n + 0);
        let b: __m512i = input(n + 5);
        let out = _mm512_mullo_epi32(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe__mm512_abs_epi8(n: usize, h: &mut Digest) {
        let a: __m512i = input(n + 0);
        let out = _mm512_abs_epi8(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe__mm512_cvtepu8_epi32(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm512_cvtepu8_epi32(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe__mm512_cvtepi8_epi32(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm512_cvtepi8_epi32(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe__mm512_cvtepu16_epi32(n: usize, h: &mut Digest) {
        let a: __m256i = input(n + 0);
        let out = _mm512_cvtepu16_epi32(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe__mm512_cvtepi16_epi32(n: usize, h: &mut Digest) {
        let a: __m256i = input(n + 0);
        let out = _mm512_cvtepi16_epi32(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe__mm512_sllv_epi64(n: usize, h: &mut Digest) {
        let a: __m512i = input(n + 0);
        let c: __m512i = input(n + 5);
        let out = _mm512_sllv_epi64(a, c);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe__mm512_srlv_epi64(n: usize, h: &mut Digest) {
        let a: __m512i = input(n + 0);
        let c: __m512i = input(n + 5);
        let out = _mm512_srlv_epi64(a, c);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe__mm512_extractf32x8_ps_0(n: usize, h: &mut Digest) {
        let a: __m512 = float_input(n + 0);
        let out = _mm512_extractf32x8_ps::<0>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe__mm512_extractf32x8_ps_1(n: usize, h: &mut Digest) {
        let a: __m512 = float_input(n + 0);
        let out = _mm512_extractf32x8_ps::<1>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe__mm512_insertf32x8_0(n: usize, h: &mut Digest) {
        let a: __m512 = float_input(n + 0);
        let b: __m256 = float_input(n + 5);
        let out = _mm512_insertf32x8::<0>(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe__mm512_insertf32x8_1(n: usize, h: &mut Digest) {
        let a: __m512 = float_input(n + 0);
        let b: __m256 = float_input(n + 5);
        let out = _mm512_insertf32x8::<1>(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe__mm512_inserti64x4_0(n: usize, h: &mut Digest) {
        let a: __m512i = input(n + 0);
        let b: __m256i = input(n + 5);
        let out = _mm512_inserti64x4::<0>(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe__mm512_inserti64x4_1(n: usize, h: &mut Digest) {
        let a: __m512i = input(n + 0);
        let b: __m256i = input(n + 5);
        let out = _mm512_inserti64x4::<1>(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe__mm512_castps256_ps512(n: usize, h: &mut Digest) {
        let a: __m256 = float_input(n + 0);
        let out = _mm512_castps256_ps512(a);
        let lo = _mm512_castps512_ps256(out);
        mix(h, &lo);
    }

    #[inline(never)]
    unsafe fn probe__mm512_castps512_ps256(n: usize, h: &mut Digest) {
        let a: __m512 = float_input(n + 0);
        let out = _mm512_castps512_ps256(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe__mm512_castsi256_si512(n: usize, h: &mut Digest) {
        let a: __m256i = input(n + 0);
        let out = _mm512_castsi256_si512(a);
        let lo = _mm512_castsi512_si256(out);
        mix(h, &lo);
    }

    #[inline(never)]
    unsafe fn probe__mm512_castsi512_si256(n: usize, h: &mut Digest) {
        let a: __m512i = input(n + 0);
        let out = _mm512_castsi512_si256(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe__mm512_castsi512_ps(n: usize, h: &mut Digest) {
        let a: __m512i = input(n + 0);
        let out = _mm512_castsi512_ps(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe__mm512_loadu_ps(n: usize, h: &mut Digest) {
        let a: __m512 = float_input(n + 0);
        let mut buf = [0.0f32; 16];
        _mm512_storeu_ps(buf.as_mut_ptr(), a);
        let out = _mm512_loadu_ps(buf.as_ptr());
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe__mm512_storeu_ps(n: usize, h: &mut Digest) {
        let a: __m512 = float_input(n + 0);
        let mut buf = [0.0f32; 16];
        _mm512_storeu_ps(buf.as_mut_ptr(), a);
        mix(h, &buf);
    }

    #[inline(never)]
    unsafe fn probe_mm512_abs_pd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_abs_pd(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_abs_ps(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_abs_ps(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_add_round_pd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_add_round_pd::<4>(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_add_round_ps(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_add_round_ps::<4>(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_alignr_epi32(n: usize, h: &mut Digest) {
        if feat("avx512bw") {
            let out = _mm512_alignr_epi32::<9>(input(n+0), input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_alignr_epi64(n: usize, h: &mut Digest) {
        if feat("avx512bw") {
            let out = _mm512_alignr_epi64::<9>(input(n+0), input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_broadcast_f32x2(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_broadcast_f32x2(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_broadcast_f32x4(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_broadcast_f32x4(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_broadcast_f32x8(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_broadcast_f32x8(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_broadcast_f64x2(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_broadcast_f64x2(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_broadcast_f64x4(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_broadcast_f64x4(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_broadcast_i32x2(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_broadcast_i32x2(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_broadcast_i32x8(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_broadcast_i32x8(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_broadcast_i64x2(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_broadcast_i64x2(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_broadcast_i64x4(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_broadcast_i64x4(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_broadcastmb_epi64(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_broadcastmb_epi64(kbits(n) as __mmask8);
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_broadcastmw_epi32(n: usize, h: &mut Digest) {
        if feat("avx512cd") {
            let out = _mm512_broadcastmw_epi32(kbits(n) as __mmask16);
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_broadcastsd_pd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_broadcastsd_pd(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_broadcastss_ps(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_broadcastss_ps(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_bslli_epi128(n: usize, h: &mut Digest) {
        if feat("avx2") {
            let out = _mm512_bslli_epi128::<5>(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_bsrli_epi128(n: usize, h: &mut Digest) {
        if feat("avx2") {
            let out = _mm512_bsrli_epi128::<5>(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_castpd128_pd512(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_castpd128_pd512(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_castpd256_pd512(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_castpd256_pd512(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_castpd512_pd128(n: usize, h: &mut Digest) {
        if feat("avx512vl") {
            let out = _mm512_castpd512_pd128(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_castpd512_pd256(n: usize, h: &mut Digest) {
        if feat("avx512vl") {
            let out = _mm512_castpd512_pd256(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_castpd_ps(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_castpd_ps(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_castpd_si512(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_castpd_si512(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_castps128_ps512(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_castps128_ps512(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_castps512_ps128(n: usize, h: &mut Digest) {
        if feat("avx512vl") {
            let out = _mm512_castps512_ps128(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_castps_pd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_castps_pd(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_castps_si512(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_castps_si512(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_castsi512_pd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_castsi512_pd(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_conflict_epi32(n: usize, h: &mut Digest) {
        if feat("avx512cd") {
            let out = _mm512_conflict_epi32(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_conflict_epi64(n: usize, h: &mut Digest) {
        if feat("avx512cd") {
            let out = _mm512_conflict_epi64(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvt_roundepi32_ps(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_cvt_roundepi32_ps::<8>(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvt_roundepi64_pd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_cvt_roundepi64_pd::<8>(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvt_roundepi64_ps(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_cvt_roundepi64_ps::<8>(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvt_roundepu32_ps(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_cvt_roundepu32_ps::<8>(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvt_roundepu64_pd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_cvt_roundepu64_pd::<8>(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvt_roundepu64_ps(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_cvt_roundepu64_ps::<8>(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvt_roundpd_epi32(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_cvt_roundpd_epi32::<4>(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvt_roundpd_epi64(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_cvt_roundpd_epi64::<4>(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvt_roundpd_epu32(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_cvt_roundpd_epu32::<4>(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvt_roundpd_epu64(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_cvt_roundpd_epu64::<4>(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvt_roundpd_ps(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_cvt_roundpd_ps::<8>(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvt_roundps_epi32(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_cvt_roundps_epi32::<4>(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvt_roundps_epi64(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_cvt_roundps_epi64::<4>(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvt_roundps_epu32(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_cvt_roundps_epu32::<4>(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvt_roundps_epu64(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_cvt_roundps_epu64::<4>(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvt_roundps_pd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_cvt_roundps_pd::<8>(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvtepi16_epi64(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_cvtepi16_epi64(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvtepi16_epi8(n: usize, h: &mut Digest) {
        if feat("avx512bw") {
            let out = _mm512_cvtepi16_epi8(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvtepi32_epi16(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_cvtepi32_epi16(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvtepi32_epi64(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_cvtepi32_epi64(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvtepi32_epi8(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_cvtepi32_epi8(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvtepi32_pd(n: usize, h: &mut Digest) {
        if feat("avx") {
            let out = _mm512_cvtepi32_pd(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvtepi32lo_pd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_cvtepi32lo_pd(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvtepi64_epi16(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_cvtepi64_epi16(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvtepi64_epi32(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_cvtepi64_epi32(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvtepi64_epi8(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_cvtepi64_epi8(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvtepi64_pd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_cvtepi64_pd(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvtepi64_ps(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_cvtepi64_ps(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvtepi8_epi16(n: usize, h: &mut Digest) {
        if feat("avx512bw") {
            let out = _mm512_cvtepi8_epi16(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvtepi8_epi64(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_cvtepi8_epi64(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvtepu16_epi64(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_cvtepu16_epi64(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvtepu32_epi64(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_cvtepu32_epi64(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvtepu32_pd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_cvtepu32_pd(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvtepu32_ps(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_cvtepu32_ps(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvtepu32lo_pd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_cvtepu32lo_pd(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvtepu64_pd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_cvtepu64_pd(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvtepu64_ps(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_cvtepu64_ps(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvtepu8_epi16(n: usize, h: &mut Digest) {
        if feat("avx512bw") {
            let out = _mm512_cvtepu8_epi16(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvtepu8_epi64(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_cvtepu8_epi64(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvtpd_epi32(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_cvtpd_epi32(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvtpd_epi64(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_cvtpd_epi64(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvtpd_epu32(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_cvtpd_epu32(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvtpd_epu64(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_cvtpd_epu64(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvtpd_ps(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_cvtpd_ps(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvtpd_pslo(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_cvtpd_pslo(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvtps_epi32(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_cvtps_epi32(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvtps_epi64(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_cvtps_epi64(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvtps_epu32(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_cvtps_epu32(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvtps_epu64(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_cvtps_epu64(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvtps_pd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_cvtps_pd(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvtpslo_pd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_cvtpslo_pd(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvtsd_f64(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_cvtsd_f64(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvtsepi16_epi8(n: usize, h: &mut Digest) {
        if feat("avx512bw") {
            let out = _mm512_cvtsepi16_epi8(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvtsepi32_epi16(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_cvtsepi32_epi16(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvtsepi32_epi8(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_cvtsepi32_epi8(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvtsepi64_epi16(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_cvtsepi64_epi16(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvtsepi64_epi32(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_cvtsepi64_epi32(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvtsepi64_epi8(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_cvtsepi64_epi8(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvtsi512_si32(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_cvtsi512_si32(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvtss_f32(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_cvtss_f32(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvtt_roundpd_epi32(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_cvtt_roundpd_epi32::<4>(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvtt_roundpd_epi64(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_cvtt_roundpd_epi64::<4>(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvtt_roundpd_epu32(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_cvtt_roundpd_epu32::<4>(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvtt_roundpd_epu64(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_cvtt_roundpd_epu64::<4>(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvtt_roundps_epi32(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_cvtt_roundps_epi32::<4>(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvtt_roundps_epi64(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_cvtt_roundps_epi64::<4>(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvtt_roundps_epu32(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_cvtt_roundps_epu32::<4>(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvtt_roundps_epu64(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_cvtt_roundps_epu64::<4>(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvttpd_epi32(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_cvttpd_epi32(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvttpd_epi64(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_cvttpd_epi64(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvttpd_epu32(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_cvttpd_epu32(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvttpd_epu64(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_cvttpd_epu64(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvttps_epi64(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_cvttps_epi64(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvttps_epu32(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_cvttps_epu32(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvttps_epu64(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_cvttps_epu64(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvtusepi16_epi8(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_cvtusepi16_epi8(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvtusepi32_epi16(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_cvtusepi32_epi16(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvtusepi32_epi8(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_cvtusepi32_epi8(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvtusepi64_epi16(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_cvtusepi64_epi16(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvtusepi64_epi32(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_cvtusepi64_epi32(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_cvtusepi64_epi8(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_cvtusepi64_epi8(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_dbsad_epu8(n: usize, h: &mut Digest) {
        if feat("avx512bw") {
            let out = _mm512_dbsad_epu8::<0xa5>(input(n+0), input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_div_round_pd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_div_round_pd::<4>(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_div_round_ps(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_div_round_ps::<4>(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_extractf32x4_ps(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_extractf32x4_ps::<1>(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_extractf64x2_pd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_extractf64x2_pd::<1>(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_extractf64x4_pd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_extractf64x4_pd::<1>(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_extracti32x8_epi32(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_extracti32x8_epi32::<1>(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_extracti64x2_epi64(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_extracti64x2_epi64::<1>(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_extracti64x4_epi64(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_extracti64x4_epi64::<1>(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_fixupimm_pd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_fixupimm_pd::<0x1e>(float_input(n+0), float_input(n+1), input(n+2));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_fixupimm_ps(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_fixupimm_ps::<0x1e>(float_input(n+0), float_input(n+1), input(n+2));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_fixupimm_round_pd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_fixupimm_round_pd::<0x1e, 8>(float_input(n+0), float_input(n+1), input(n+2));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_fixupimm_round_ps(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_fixupimm_round_ps::<0x1e, 8>(float_input(n+0), float_input(n+1), input(n+2));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_fmadd_round_pd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_fmadd_round_pd::<8>(float_input(n+0), float_input(n+1), float_input(n+2));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_fmadd_round_ps(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_fmadd_round_ps::<8>(float_input(n+0), float_input(n+1), float_input(n+2));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_fmaddsub_round_pd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_fmaddsub_round_pd::<8>(float_input(n+0), float_input(n+1), float_input(n+2));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_fmaddsub_round_ps(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_fmaddsub_round_ps::<8>(float_input(n+0), float_input(n+1), float_input(n+2));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_fmsub_round_pd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_fmsub_round_pd::<8>(float_input(n+0), float_input(n+1), float_input(n+2));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_fmsub_round_ps(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_fmsub_round_ps::<8>(float_input(n+0), float_input(n+1), float_input(n+2));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_fmsubadd_round_pd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_fmsubadd_round_pd::<8>(float_input(n+0), float_input(n+1), float_input(n+2));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_fmsubadd_round_ps(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_fmsubadd_round_ps::<8>(float_input(n+0), float_input(n+1), float_input(n+2));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_fnmadd_round_pd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_fnmadd_round_pd::<8>(float_input(n+0), float_input(n+1), float_input(n+2));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_fnmadd_round_ps(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_fnmadd_round_ps::<8>(float_input(n+0), float_input(n+1), float_input(n+2));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_fnmsub_round_pd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_fnmsub_round_pd::<8>(float_input(n+0), float_input(n+1), float_input(n+2));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_fnmsub_round_ps(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_fnmsub_round_ps::<8>(float_input(n+0), float_input(n+1), float_input(n+2));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_getexp_round_pd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_getexp_round_pd::<8>(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_getexp_round_ps(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_getexp_round_ps::<8>(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_getmant_round_pd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_getmant_round_pd::<1, 0, 8>(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_getmant_round_ps(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_getmant_round_ps::<1, 0, 8>(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_i32logather_epi64(n: usize, h: &mut Digest) {
        if feat("avx512vl") {
            let out = _mm512_i32logather_epi64::<8>(_mm512_and_si512(input(n+1), _mm512_set1_epi32(7)), loadptr(n).cast::<i64>());
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_i32logather_pd(n: usize, h: &mut Digest) {
        if feat("avx512vl") {
            let out = _mm512_i32logather_pd::<8>(_mm512_and_si512(input(n+1), _mm512_set1_epi32(7)), loadptr(n).cast::<f64>());
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_i32loscatter_epi64(n: usize, h: &mut Digest) {
        if feat("avx512vl") {
            let out = _mm512_i32loscatter_epi64::<8>(storeptr(n).cast::<i64>(), _mm512_and_si512(input(n+1), _mm512_set1_epi32(7)), input(n+2));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_i32loscatter_pd(n: usize, h: &mut Digest) {
        if feat("avx512vl") {
            let out = _mm512_i32loscatter_pd::<8>(storeptr(n).cast::<f64>(), _mm512_and_si512(input(n+1), _mm512_set1_epi32(7)), float_input(n+2));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_int2mask(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_int2mask((n%97) as i32);
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_kand(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_kand(kbits(n) as __mmask16, kbits(n+5) as __mmask16);
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_kandn(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_kandn(kbits(n) as __mmask16, kbits(n+5) as __mmask16);
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_knot(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_knot(kbits(n) as __mmask16);
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_kor(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_kor(kbits(n) as __mmask16, kbits(n+5) as __mmask16);
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_kortestc(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_kortestc(kbits(n) as __mmask16, kbits(n+5) as __mmask16);
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_kortestz(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_kortestz(kbits(n) as __mmask16, kbits(n+5) as __mmask16);
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_kunpackb(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_kunpackb(kbits(n) as __mmask16, kbits(n+7) as __mmask16);
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_kunpackd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_kunpackd(kbits(n) as __mmask64, kbits(n+7) as __mmask64);
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_kunpackw(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_kunpackw(kbits(n) as __mmask32, kbits(n+7) as __mmask32);
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_kxnor(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_kxnor(kbits(n) as __mmask16, kbits(n+5) as __mmask16);
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_kxor(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_kxor(kbits(n) as __mmask16, kbits(n+5) as __mmask16);
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_max_round_pd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_max_round_pd::<4>(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_max_round_ps(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_max_round_ps::<4>(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_min_round_pd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_min_round_pd::<4>(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_min_round_ps(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_min_round_ps::<4>(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_movedup_pd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_movedup_pd(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_movehdup_ps(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_movehdup_ps(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_moveldup_ps(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_moveldup_ps(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_mul_epi32(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_mul_epi32(input(n+0), input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_mul_epu32(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_mul_epu32(input(n+0), input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_mul_round_pd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_mul_round_pd::<4>(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_mul_round_ps(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_mul_round_ps::<4>(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_mulhi_epi16(n: usize, h: &mut Digest) {
        if feat("avx512bw") {
            let out = _mm512_mulhi_epi16(input(n+0), input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_mulhi_epu16(n: usize, h: &mut Digest) {
        if feat("avx512bw") {
            let out = _mm512_mulhi_epu16(input(n+0), input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_mulhrs_epi16(n: usize, h: &mut Digest) {
        if feat("avx512bw") {
            let out = _mm512_mulhrs_epi16(input(n+0), input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_mullox_epi64(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_mullox_epi64(input(n+0), input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_permute_pd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_permute_pd::<2>(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_permute_ps(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_permute_ps::<0x1b>(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_permutevar_epi32(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_permutevar_epi32(input(n+0), input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_permutevar_pd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_permutevar_pd(float_input(n+0), input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_permutevar_ps(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_permutevar_ps(float_input(n+0), input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_permutex_epi64(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_permutex_epi64::<0x4e>(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_permutex_pd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_permutex_pd::<0x4e>(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_range_pd(n: usize, h: &mut Digest) {
        if feat("avx512dq") {
            let out = _mm512_range_pd::<9>(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_range_ps(n: usize, h: &mut Digest) {
        if feat("avx512dq") {
            let out = _mm512_range_ps::<9>(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_range_round_pd(n: usize, h: &mut Digest) {
        if feat("avx512dq") {
            let out = _mm512_range_round_pd::<9, 8>(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_range_round_ps(n: usize, h: &mut Digest) {
        if feat("avx512dq") {
            let out = _mm512_range_round_ps::<9, 8>(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_rcp14_pd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_rcp14_pd(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_rcp14_ps(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_rcp14_ps(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_reduce_pd(n: usize, h: &mut Digest) {
        if feat("avx512dq") {
            let out = _mm512_reduce_pd::<0x19>(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_reduce_ps(n: usize, h: &mut Digest) {
        if feat("avx512dq") {
            let out = _mm512_reduce_ps::<0x19>(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_reduce_round_pd(n: usize, h: &mut Digest) {
        if feat("avx512dq") {
            let out = _mm512_reduce_round_pd::<0x19, 8>(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_reduce_round_ps(n: usize, h: &mut Digest) {
        if feat("avx512dq") {
            let out = _mm512_reduce_round_ps::<0x19, 8>(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_roundscale_pd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_roundscale_pd::<9>(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_roundscale_ps(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_roundscale_ps::<9>(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_roundscale_round_pd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_roundscale_round_pd::<0x19, 8>(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_roundscale_round_ps(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_roundscale_round_ps::<0x19, 8>(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_rsqrt14_pd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_rsqrt14_pd(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_rsqrt14_ps(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_rsqrt14_ps(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_scalef_round_pd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_scalef_round_pd::<4>(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_scalef_round_ps(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_scalef_round_ps::<4>(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_set4_epi32(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_set4_epi32((n%97) as i32, (n%110) as i32, (n%123) as i32, (n%136) as i32);
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_set4_epi64(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_set4_epi64((n*97) as i64, (n*110) as i64, (n*123) as i64, (n*136) as i64);
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_set4_pd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_set4_pd((n%97) as f64, (n%110) as f64, (n%123) as f64, (n%136) as f64);
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_set4_ps(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_set4_ps((n%97) as f32, (n%110) as f32, (n%123) as f32, (n%136) as f32);
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_setr4_epi32(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_setr4_epi32((n%97) as i32, (n%110) as i32, (n%123) as i32, (n%136) as i32);
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_setr4_epi64(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_setr4_epi64((n*97) as i64, (n*110) as i64, (n*123) as i64, (n*136) as i64);
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_setr4_pd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_setr4_pd((n%97) as f64, (n%110) as f64, (n%123) as f64, (n%136) as f64);
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_setr4_ps(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_setr4_ps((n%97) as f32, (n%110) as f32, (n%123) as f32, (n%136) as f32);
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_setzero(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_setzero();
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_setzero_epi32(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_setzero_epi32();
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_shuffle_f32x4(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_shuffle_f32x4::<0xb1>(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_shuffle_f64x2(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_shuffle_f64x2::<0xb1>(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_shuffle_i64x2(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_shuffle_i64x2::<0xb1>(input(n+0), input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_shufflehi_epi16(n: usize, h: &mut Digest) {
        if feat("avx512bw") {
            let out = _mm512_shufflehi_epi16::<0x1b>(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_shufflelo_epi16(n: usize, h: &mut Digest) {
        if feat("avx512bw") {
            let out = _mm512_shufflelo_epi16::<0x1b>(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_sqrt_pd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_sqrt_pd(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_sqrt_ps(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_sqrt_ps(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_sqrt_round_pd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_sqrt_round_pd::<8>(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_sqrt_round_ps(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_sqrt_round_ps::<8>(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_stream_load_si512(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_stream_load_si512(loadptr(n).cast::<__m512i>());
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_sub_round_pd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_sub_round_pd::<4>(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_sub_round_ps(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_sub_round_ps::<4>(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_undefined(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_undefined();
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_undefined_epi32(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_undefined_epi32();
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_undefined_pd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_undefined_pd();
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_undefined_ps(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_undefined_ps();
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_zextpd128_pd512(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_zextpd128_pd512(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_zextpd256_pd512(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_zextpd256_pd512(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_zextps128_ps512(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_zextps128_ps512(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_zextps256_ps512(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_zextps256_ps512(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_zextsi128_si512(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_zextsi128_si512(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm512_zextsi256_si512(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm512_zextsi256_si512(input(n+0));
            mix(h, &out);
        }
    }
#[test]
fn a0001__mm512_setzero_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe__mm512_setzero_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_setzero_ps", "none");
}

#[test]
fn a0002__mm512_set1_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe__mm512_set1_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_set1_ps", "none");
}

#[test]
fn a0003__mm512_set1_epi8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe__mm512_set1_epi8(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_set1_epi8", "none");
}

#[test]
fn a0004__mm512_set1_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe__mm512_set1_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_set1_epi32", "none");
}

#[test]
fn a0005__mm512_set1_epi64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe__mm512_set1_epi64(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_set1_epi64", "none");
}

#[test]
fn a0006__mm512_add_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe__mm512_add_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_add_ps", "none");
}

#[test]
fn a0007__mm512_sub_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe__mm512_sub_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_sub_ps", "none");
}

#[test]
fn a0008__mm512_mul_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe__mm512_mul_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_mul_ps", "none");
}

#[test]
fn a0009__mm512_div_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe__mm512_div_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_div_ps", "none");
}

#[test]
fn a0010__mm512_andnot_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe__mm512_andnot_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_andnot_ps", "none");
}

#[test]
fn a0011__mm512_fmadd_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe__mm512_fmadd_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_fmadd_ps", "none");
}

#[test]
fn a0012__mm512_fnmadd_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe__mm512_fnmadd_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_fnmadd_ps", "none");
}

#[test]
fn a0013__mm512_scalef_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe__mm512_scalef_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_scalef_ps", "none");
}

#[test]
fn a0014__mm512_cvtps_ph_0() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe__mm512_cvtps_ph_0(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvtps_ph_0", "none");
}

#[test]
fn a0015__mm512_cvtps_ph_1() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe__mm512_cvtps_ph_1(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvtps_ph_1", "none");
}

#[test]
fn a0016__mm512_cvtps_ph_2() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe__mm512_cvtps_ph_2(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvtps_ph_2", "none");
}

#[test]
fn a0017__mm512_cvtps_ph_3() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe__mm512_cvtps_ph_3(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvtps_ph_3", "none");
}

#[test]
fn a0018__mm512_cvtps_ph_8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe__mm512_cvtps_ph_8(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvtps_ph_8", "none");
}

#[test]
fn a0019__mm512_cvtph_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe__mm512_cvtph_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvtph_ps", "none");
}

#[test]
fn a0020__mm512_cvttps_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe__mm512_cvttps_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvttps_epi32", "none");
}

#[test]
fn a0021__mm512_cvtepi32_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe__mm512_cvtepi32_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvtepi32_ps", "none");
}

#[test]
fn a0022__mm512_reduce_add_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe__mm512_reduce_add_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_reduce_add_ps", "none");
}

#[test]
fn a0023__mm512_reduce_add_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe__mm512_reduce_add_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_reduce_add_epi32", "none");
}

#[test]
fn a0024__mm512_and_si512() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe__mm512_and_si512(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_and_si512", "none");
}

#[test]
fn a0025__mm512_or_si512() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe__mm512_or_si512(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_or_si512", "none");
}

#[test]
fn a0026__mm512_sub_epi8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe__mm512_sub_epi8(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_sub_epi8", "none");
}

#[test]
fn a0027__mm512_sub_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe__mm512_sub_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_sub_epi32", "none");
}

#[test]
fn a0028__mm512_mullo_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe__mm512_mullo_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_mullo_epi32", "none");
}

#[test]
fn a0029__mm512_abs_epi8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe__mm512_abs_epi8(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_abs_epi8", "none");
}

#[test]
fn a0030__mm512_cvtepu8_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe__mm512_cvtepu8_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvtepu8_epi32", "none");
}

#[test]
fn a0031__mm512_cvtepi8_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe__mm512_cvtepi8_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvtepi8_epi32", "none");
}

#[test]
fn a0032__mm512_cvtepu16_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe__mm512_cvtepu16_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvtepu16_epi32", "none");
}

#[test]
fn a0033__mm512_cvtepi16_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe__mm512_cvtepi16_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvtepi16_epi32", "none");
}

#[test]
fn a0034__mm512_sllv_epi64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe__mm512_sllv_epi64(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_sllv_epi64", "none");
}

#[test]
fn a0035__mm512_srlv_epi64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe__mm512_srlv_epi64(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_srlv_epi64", "none");
}

#[test]
fn a0036__mm512_extractf32x8_ps_0() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe__mm512_extractf32x8_ps_0(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_extractf32x8_ps_0", "none");
}

#[test]
fn a0037__mm512_extractf32x8_ps_1() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe__mm512_extractf32x8_ps_1(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_extractf32x8_ps_1", "none");
}

#[test]
fn a0038__mm512_insertf32x8_0() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe__mm512_insertf32x8_0(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_insertf32x8_0", "none");
}

#[test]
fn a0039__mm512_insertf32x8_1() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe__mm512_insertf32x8_1(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_insertf32x8_1", "none");
}

#[test]
fn a0040__mm512_inserti64x4_0() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe__mm512_inserti64x4_0(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_inserti64x4_0", "none");
}

#[test]
fn a0041__mm512_inserti64x4_1() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe__mm512_inserti64x4_1(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_inserti64x4_1", "none");
}

#[test]
fn a0042__mm512_castps256_ps512() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe__mm512_castps256_ps512(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_castps256_ps512", "none");
}

#[test]
fn a0043__mm512_castps512_ps256() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe__mm512_castps512_ps256(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_castps512_ps256", "none");
}

#[test]
fn a0044__mm512_castsi256_si512() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe__mm512_castsi256_si512(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_castsi256_si512", "none");
}

#[test]
fn a0045__mm512_castsi512_si256() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe__mm512_castsi512_si256(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_castsi512_si256", "none");
}

#[test]
fn a0046__mm512_castsi512_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe__mm512_castsi512_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_castsi512_ps", "none");
}

#[test]
fn a0047__mm512_loadu_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe__mm512_loadu_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_loadu_ps", "none");
}

#[test]
fn a0048__mm512_storeu_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe__mm512_storeu_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_storeu_ps", "none");
}

#[test]
fn a0049_mm512_abs_pd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_abs_pd(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_abs_pd", "none");
}

#[test]
fn a0050_mm512_abs_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_abs_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_abs_ps", "none");
}

#[test]
fn a0051_mm512_add_round_pd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_add_round_pd(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_add_round_pd", "none");
}

#[test]
fn a0052_mm512_add_round_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_add_round_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_add_round_ps", "none");
}

#[test]
fn a0053_mm512_alignr_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_alignr_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_alignr_epi32", "none");
}

#[test]
fn a0054_mm512_alignr_epi64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_alignr_epi64(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_alignr_epi64", "none");
}

#[test]
fn a0055_mm512_broadcast_f32x2() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_broadcast_f32x2(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_broadcast_f32x2", "none");
}

#[test]
fn a0056_mm512_broadcast_f32x4() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_broadcast_f32x4(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_broadcast_f32x4", "none");
}

#[test]
fn a0057_mm512_broadcast_f32x8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_broadcast_f32x8(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_broadcast_f32x8", "none");
}

#[test]
fn a0058_mm512_broadcast_f64x2() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_broadcast_f64x2(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_broadcast_f64x2", "none");
}

#[test]
fn a0059_mm512_broadcast_f64x4() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_broadcast_f64x4(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_broadcast_f64x4", "none");
}

#[test]
fn a0060_mm512_broadcast_i32x2() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_broadcast_i32x2(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_broadcast_i32x2", "none");
}

#[test]
fn a0061_mm512_broadcast_i32x8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_broadcast_i32x8(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_broadcast_i32x8", "none");
}

#[test]
fn a0062_mm512_broadcast_i64x2() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_broadcast_i64x2(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_broadcast_i64x2", "none");
}

#[test]
fn a0063_mm512_broadcast_i64x4() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_broadcast_i64x4(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_broadcast_i64x4", "none");
}

#[test]
fn a0064_mm512_broadcastmb_epi64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_broadcastmb_epi64(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_broadcastmb_epi64", "none");
}

#[test]
fn a0065_mm512_broadcastmw_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_broadcastmw_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_broadcastmw_epi32", "none");
}

#[test]
fn a0066_mm512_broadcastsd_pd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_broadcastsd_pd(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_broadcastsd_pd", "none");
}

#[test]
fn a0067_mm512_broadcastss_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_broadcastss_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_broadcastss_ps", "none");
}

#[test]
fn a0068_mm512_bslli_epi128() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_bslli_epi128(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_bslli_epi128", "none");
}

#[test]
fn a0069_mm512_bsrli_epi128() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_bsrli_epi128(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_bsrli_epi128", "none");
}

#[test]
fn a0070_mm512_castpd128_pd512() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_castpd128_pd512(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_castpd128_pd512", "none");
}

#[test]
fn a0071_mm512_castpd256_pd512() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_castpd256_pd512(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_castpd256_pd512", "none");
}

#[test]
fn a0072_mm512_castpd512_pd128() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_castpd512_pd128(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_castpd512_pd128", "none");
}

#[test]
fn a0073_mm512_castpd512_pd256() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_castpd512_pd256(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_castpd512_pd256", "none");
}

#[test]
fn a0074_mm512_castpd_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_castpd_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_castpd_ps", "none");
}

#[test]
fn a0075_mm512_castpd_si512() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_castpd_si512(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_castpd_si512", "none");
}

#[test]
fn a0076_mm512_castps128_ps512() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_castps128_ps512(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_castps128_ps512", "none");
}

#[test]
fn a0077_mm512_castps512_ps128() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_castps512_ps128(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_castps512_ps128", "none");
}

#[test]
fn a0078_mm512_castps_pd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_castps_pd(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_castps_pd", "none");
}

#[test]
fn a0079_mm512_castps_si512() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_castps_si512(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_castps_si512", "none");
}

#[test]
fn a0080_mm512_castsi512_pd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_castsi512_pd(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_castsi512_pd", "none");
}

#[test]
fn a0081_mm512_conflict_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_conflict_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_conflict_epi32", "none");
}

#[test]
fn a0082_mm512_conflict_epi64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_conflict_epi64(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_conflict_epi64", "none");
}

#[test]
fn a0083_mm512_cvt_roundepi32_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvt_roundepi32_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvt_roundepi32_ps", "none");
}

#[test]
fn a0084_mm512_cvt_roundepi64_pd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvt_roundepi64_pd(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvt_roundepi64_pd", "none");
}

#[test]
fn a0085_mm512_cvt_roundepi64_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvt_roundepi64_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvt_roundepi64_ps", "none");
}

#[test]
fn a0086_mm512_cvt_roundepu32_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvt_roundepu32_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvt_roundepu32_ps", "none");
}

#[test]
fn a0087_mm512_cvt_roundepu64_pd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvt_roundepu64_pd(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvt_roundepu64_pd", "none");
}

#[test]
fn a0088_mm512_cvt_roundepu64_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvt_roundepu64_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvt_roundepu64_ps", "none");
}

#[test]
fn a0089_mm512_cvt_roundpd_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvt_roundpd_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvt_roundpd_epi32", "none");
}

#[test]
fn a0090_mm512_cvt_roundpd_epi64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvt_roundpd_epi64(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvt_roundpd_epi64", "none");
}

#[test]
fn a0091_mm512_cvt_roundpd_epu32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvt_roundpd_epu32(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvt_roundpd_epu32", "none");
}

#[test]
fn a0092_mm512_cvt_roundpd_epu64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvt_roundpd_epu64(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvt_roundpd_epu64", "none");
}

#[test]
fn a0093_mm512_cvt_roundpd_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvt_roundpd_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvt_roundpd_ps", "none");
}

#[test]
fn a0094_mm512_cvt_roundps_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvt_roundps_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvt_roundps_epi32", "none");
}

#[test]
fn a0095_mm512_cvt_roundps_epi64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvt_roundps_epi64(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvt_roundps_epi64", "none");
}

#[test]
fn a0096_mm512_cvt_roundps_epu32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvt_roundps_epu32(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvt_roundps_epu32", "none");
}

#[test]
fn a0097_mm512_cvt_roundps_epu64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvt_roundps_epu64(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvt_roundps_epu64", "none");
}

#[test]
fn a0098_mm512_cvt_roundps_pd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvt_roundps_pd(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvt_roundps_pd", "none");
}

#[test]
fn a0099_mm512_cvtepi16_epi64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvtepi16_epi64(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvtepi16_epi64", "none");
}

#[test]
fn a0100_mm512_cvtepi16_epi8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvtepi16_epi8(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvtepi16_epi8", "none");
}

#[test]
fn a0101_mm512_cvtepi32_epi16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvtepi32_epi16(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvtepi32_epi16", "none");
}

#[test]
fn a0102_mm512_cvtepi32_epi64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvtepi32_epi64(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvtepi32_epi64", "none");
}

#[test]
fn a0103_mm512_cvtepi32_epi8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvtepi32_epi8(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvtepi32_epi8", "none");
}

#[test]
fn a0104_mm512_cvtepi32_pd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvtepi32_pd(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvtepi32_pd", "none");
}

#[test]
fn a0105_mm512_cvtepi32lo_pd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvtepi32lo_pd(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvtepi32lo_pd", "none");
}

#[test]
fn a0106_mm512_cvtepi64_epi16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvtepi64_epi16(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvtepi64_epi16", "none");
}

#[test]
fn a0107_mm512_cvtepi64_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvtepi64_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvtepi64_epi32", "none");
}

#[test]
fn a0108_mm512_cvtepi64_epi8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvtepi64_epi8(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvtepi64_epi8", "none");
}

#[test]
fn a0109_mm512_cvtepi64_pd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvtepi64_pd(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvtepi64_pd", "none");
}

#[test]
fn a0110_mm512_cvtepi64_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvtepi64_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvtepi64_ps", "none");
}

#[test]
fn a0111_mm512_cvtepi8_epi16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvtepi8_epi16(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvtepi8_epi16", "none");
}

#[test]
fn a0112_mm512_cvtepi8_epi64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvtepi8_epi64(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvtepi8_epi64", "none");
}

#[test]
fn a0113_mm512_cvtepu16_epi64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvtepu16_epi64(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvtepu16_epi64", "none");
}

#[test]
fn a0114_mm512_cvtepu32_epi64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvtepu32_epi64(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvtepu32_epi64", "none");
}

#[test]
fn a0115_mm512_cvtepu32_pd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvtepu32_pd(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvtepu32_pd", "none");
}

#[test]
fn a0116_mm512_cvtepu32_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvtepu32_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvtepu32_ps", "none");
}

#[test]
fn a0117_mm512_cvtepu32lo_pd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvtepu32lo_pd(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvtepu32lo_pd", "none");
}

#[test]
fn a0118_mm512_cvtepu64_pd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvtepu64_pd(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvtepu64_pd", "none");
}

#[test]
fn a0119_mm512_cvtepu64_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvtepu64_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvtepu64_ps", "none");
}

#[test]
fn a0120_mm512_cvtepu8_epi16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvtepu8_epi16(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvtepu8_epi16", "none");
}

#[test]
fn a0121_mm512_cvtepu8_epi64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvtepu8_epi64(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvtepu8_epi64", "none");
}

#[test]
fn a0122_mm512_cvtpd_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvtpd_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvtpd_epi32", "none");
}

#[test]
fn a0123_mm512_cvtpd_epi64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvtpd_epi64(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvtpd_epi64", "none");
}

#[test]
fn a0124_mm512_cvtpd_epu32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvtpd_epu32(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvtpd_epu32", "none");
}

#[test]
fn a0125_mm512_cvtpd_epu64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvtpd_epu64(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvtpd_epu64", "none");
}

#[test]
fn a0126_mm512_cvtpd_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvtpd_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvtpd_ps", "none");
}

#[test]
fn a0127_mm512_cvtpd_pslo() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvtpd_pslo(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvtpd_pslo", "none");
}

#[test]
fn a0128_mm512_cvtps_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvtps_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvtps_epi32", "none");
}

#[test]
fn a0129_mm512_cvtps_epi64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvtps_epi64(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvtps_epi64", "none");
}

#[test]
fn a0130_mm512_cvtps_epu32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvtps_epu32(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvtps_epu32", "none");
}

#[test]
fn a0131_mm512_cvtps_epu64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvtps_epu64(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvtps_epu64", "none");
}

#[test]
fn a0132_mm512_cvtps_pd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvtps_pd(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvtps_pd", "none");
}

#[test]
fn a0133_mm512_cvtpslo_pd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvtpslo_pd(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvtpslo_pd", "none");
}

#[test]
fn a0134_mm512_cvtsd_f64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvtsd_f64(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvtsd_f64", "none");
}

#[test]
fn a0135_mm512_cvtsepi16_epi8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvtsepi16_epi8(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvtsepi16_epi8", "none");
}

#[test]
fn a0136_mm512_cvtsepi32_epi16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvtsepi32_epi16(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvtsepi32_epi16", "none");
}

#[test]
fn a0137_mm512_cvtsepi32_epi8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvtsepi32_epi8(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvtsepi32_epi8", "none");
}

#[test]
fn a0138_mm512_cvtsepi64_epi16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvtsepi64_epi16(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvtsepi64_epi16", "none");
}

#[test]
fn a0139_mm512_cvtsepi64_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvtsepi64_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvtsepi64_epi32", "none");
}

#[test]
fn a0140_mm512_cvtsepi64_epi8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvtsepi64_epi8(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvtsepi64_epi8", "none");
}

#[test]
fn a0141_mm512_cvtsi512_si32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvtsi512_si32(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvtsi512_si32", "none");
}

#[test]
fn a0142_mm512_cvtss_f32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvtss_f32(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvtss_f32", "none");
}

#[test]
fn a0143_mm512_cvtt_roundpd_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvtt_roundpd_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvtt_roundpd_epi32", "none");
}

#[test]
fn a0144_mm512_cvtt_roundpd_epi64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvtt_roundpd_epi64(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvtt_roundpd_epi64", "none");
}

#[test]
fn a0145_mm512_cvtt_roundpd_epu32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvtt_roundpd_epu32(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvtt_roundpd_epu32", "none");
}

#[test]
fn a0146_mm512_cvtt_roundpd_epu64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvtt_roundpd_epu64(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvtt_roundpd_epu64", "none");
}

#[test]
fn a0147_mm512_cvtt_roundps_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvtt_roundps_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvtt_roundps_epi32", "none");
}

#[test]
fn a0148_mm512_cvtt_roundps_epi64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvtt_roundps_epi64(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvtt_roundps_epi64", "none");
}

#[test]
fn a0149_mm512_cvtt_roundps_epu32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvtt_roundps_epu32(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvtt_roundps_epu32", "none");
}

#[test]
fn a0150_mm512_cvtt_roundps_epu64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvtt_roundps_epu64(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvtt_roundps_epu64", "none");
}

#[test]
fn a0151_mm512_cvttpd_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvttpd_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvttpd_epi32", "none");
}

#[test]
fn a0152_mm512_cvttpd_epi64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvttpd_epi64(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvttpd_epi64", "none");
}

#[test]
fn a0153_mm512_cvttpd_epu32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvttpd_epu32(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvttpd_epu32", "none");
}

#[test]
fn a0154_mm512_cvttpd_epu64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvttpd_epu64(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvttpd_epu64", "none");
}

#[test]
fn a0155_mm512_cvttps_epi64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvttps_epi64(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvttps_epi64", "none");
}

#[test]
fn a0156_mm512_cvttps_epu32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvttps_epu32(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvttps_epu32", "none");
}

#[test]
fn a0157_mm512_cvttps_epu64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvttps_epu64(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvttps_epu64", "none");
}

#[test]
fn a0158_mm512_cvtusepi16_epi8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvtusepi16_epi8(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvtusepi16_epi8", "none");
}

#[test]
fn a0159_mm512_cvtusepi32_epi16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvtusepi32_epi16(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvtusepi32_epi16", "none");
}

#[test]
fn a0160_mm512_cvtusepi32_epi8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvtusepi32_epi8(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvtusepi32_epi8", "none");
}

#[test]
fn a0161_mm512_cvtusepi64_epi16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvtusepi64_epi16(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvtusepi64_epi16", "none");
}

#[test]
fn a0162_mm512_cvtusepi64_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvtusepi64_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvtusepi64_epi32", "none");
}

#[test]
fn a0163_mm512_cvtusepi64_epi8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_cvtusepi64_epi8(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_cvtusepi64_epi8", "none");
}

#[test]
fn a0164_mm512_dbsad_epu8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_dbsad_epu8(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_dbsad_epu8", "none");
}

#[test]
fn a0165_mm512_div_round_pd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_div_round_pd(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_div_round_pd", "none");
}

#[test]
fn a0166_mm512_div_round_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_div_round_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_div_round_ps", "none");
}

#[test]
fn a0167_mm512_extractf32x4_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_extractf32x4_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_extractf32x4_ps", "none");
}

#[test]
fn a0168_mm512_extractf64x2_pd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_extractf64x2_pd(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_extractf64x2_pd", "none");
}

#[test]
fn a0169_mm512_extractf64x4_pd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_extractf64x4_pd(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_extractf64x4_pd", "none");
}

#[test]
fn a0170_mm512_extracti32x8_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_extracti32x8_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_extracti32x8_epi32", "none");
}

#[test]
fn a0171_mm512_extracti64x2_epi64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_extracti64x2_epi64(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_extracti64x2_epi64", "none");
}

#[test]
fn a0172_mm512_extracti64x4_epi64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_extracti64x4_epi64(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_extracti64x4_epi64", "none");
}

#[test]
fn a0173_mm512_fixupimm_pd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_fixupimm_pd(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_fixupimm_pd", "none");
}

#[test]
fn a0174_mm512_fixupimm_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_fixupimm_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_fixupimm_ps", "none");
}

#[test]
fn a0175_mm512_fixupimm_round_pd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_fixupimm_round_pd(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_fixupimm_round_pd", "none");
}

#[test]
fn a0176_mm512_fixupimm_round_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_fixupimm_round_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_fixupimm_round_ps", "none");
}

#[test]
fn a0177_mm512_fmadd_round_pd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_fmadd_round_pd(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_fmadd_round_pd", "none");
}

#[test]
fn a0178_mm512_fmadd_round_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_fmadd_round_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_fmadd_round_ps", "none");
}

#[test]
fn a0179_mm512_fmaddsub_round_pd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_fmaddsub_round_pd(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_fmaddsub_round_pd", "none");
}

#[test]
fn a0180_mm512_fmaddsub_round_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_fmaddsub_round_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_fmaddsub_round_ps", "none");
}

#[test]
fn a0181_mm512_fmsub_round_pd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_fmsub_round_pd(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_fmsub_round_pd", "none");
}

#[test]
fn a0182_mm512_fmsub_round_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_fmsub_round_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_fmsub_round_ps", "none");
}

#[test]
fn a0183_mm512_fmsubadd_round_pd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_fmsubadd_round_pd(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_fmsubadd_round_pd", "none");
}

#[test]
fn a0184_mm512_fmsubadd_round_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_fmsubadd_round_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_fmsubadd_round_ps", "none");
}

#[test]
fn a0185_mm512_fnmadd_round_pd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_fnmadd_round_pd(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_fnmadd_round_pd", "none");
}

#[test]
fn a0186_mm512_fnmadd_round_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_fnmadd_round_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_fnmadd_round_ps", "none");
}

#[test]
fn a0187_mm512_fnmsub_round_pd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_fnmsub_round_pd(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_fnmsub_round_pd", "none");
}

#[test]
fn a0188_mm512_fnmsub_round_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_fnmsub_round_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_fnmsub_round_ps", "none");
}

#[test]
fn a0189_mm512_getexp_round_pd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_getexp_round_pd(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_getexp_round_pd", "none");
}

#[test]
fn a0190_mm512_getexp_round_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_getexp_round_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_getexp_round_ps", "none");
}

#[test]
fn a0191_mm512_getmant_round_pd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_getmant_round_pd(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_getmant_round_pd", "none");
}

#[test]
fn a0192_mm512_getmant_round_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_getmant_round_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_getmant_round_ps", "none");
}

#[test]
fn a0193_mm512_i32logather_epi64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_i32logather_epi64(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_i32logather_epi64", "none");
}

#[test]
fn a0194_mm512_i32logather_pd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_i32logather_pd(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_i32logather_pd", "none");
}

#[test]
fn a0195_mm512_i32loscatter_epi64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_i32loscatter_epi64(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_i32loscatter_epi64", "none");
}

#[test]
fn a0196_mm512_i32loscatter_pd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_i32loscatter_pd(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_i32loscatter_pd", "none");
}

#[test]
fn a0197_mm512_int2mask() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_int2mask(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_int2mask", "none");
}

#[test]
fn a0198_mm512_kand() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_kand(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_kand", "none");
}

#[test]
fn a0199_mm512_kandn() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_kandn(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_kandn", "none");
}

#[test]
fn a0200_mm512_knot() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_knot(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_knot", "none");
}

#[test]
fn a0201_mm512_kor() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_kor(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_kor", "none");
}

#[test]
fn a0202_mm512_kortestc() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_kortestc(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_kortestc", "none");
}

#[test]
fn a0203_mm512_kortestz() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_kortestz(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_kortestz", "none");
}

#[test]
fn a0204_mm512_kunpackb() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_kunpackb(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_kunpackb", "none");
}

#[test]
fn a0205_mm512_kunpackd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_kunpackd(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_kunpackd", "none");
}

#[test]
fn a0206_mm512_kunpackw() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_kunpackw(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_kunpackw", "none");
}

#[test]
fn a0207_mm512_kxnor() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_kxnor(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_kxnor", "none");
}

#[test]
fn a0208_mm512_kxor() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_kxor(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_kxor", "none");
}

#[test]
fn a0209_mm512_max_round_pd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_max_round_pd(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_max_round_pd", "none");
}

#[test]
fn a0210_mm512_max_round_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_max_round_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_max_round_ps", "none");
}

#[test]
fn a0211_mm512_min_round_pd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_min_round_pd(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_min_round_pd", "none");
}

#[test]
fn a0212_mm512_min_round_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_min_round_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_min_round_ps", "none");
}

#[test]
fn a0213_mm512_movedup_pd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_movedup_pd(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_movedup_pd", "none");
}

#[test]
fn a0214_mm512_movehdup_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_movehdup_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_movehdup_ps", "none");
}

#[test]
fn a0215_mm512_moveldup_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_moveldup_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_moveldup_ps", "none");
}

#[test]
fn a0216_mm512_mul_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_mul_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_mul_epi32", "none");
}

#[test]
fn a0217_mm512_mul_epu32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_mul_epu32(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_mul_epu32", "none");
}

#[test]
fn a0218_mm512_mul_round_pd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_mul_round_pd(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_mul_round_pd", "none");
}

#[test]
fn a0219_mm512_mul_round_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_mul_round_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_mul_round_ps", "none");
}

#[test]
fn a0220_mm512_mulhi_epi16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_mulhi_epi16(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_mulhi_epi16", "none");
}

#[test]
fn a0221_mm512_mulhi_epu16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_mulhi_epu16(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_mulhi_epu16", "none");
}

#[test]
fn a0222_mm512_mulhrs_epi16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_mulhrs_epi16(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_mulhrs_epi16", "none");
}

#[test]
fn a0223_mm512_mullox_epi64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_mullox_epi64(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_mullox_epi64", "none");
}

#[test]
fn a0224_mm512_permute_pd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_permute_pd(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_permute_pd", "none");
}

#[test]
fn a0225_mm512_permute_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_permute_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_permute_ps", "none");
}

#[test]
fn a0226_mm512_permutevar_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_permutevar_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_permutevar_epi32", "none");
}

#[test]
fn a0227_mm512_permutevar_pd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_permutevar_pd(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_permutevar_pd", "none");
}

#[test]
fn a0228_mm512_permutevar_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_permutevar_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_permutevar_ps", "none");
}

#[test]
fn a0229_mm512_permutex_epi64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_permutex_epi64(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_permutex_epi64", "none");
}

#[test]
fn a0230_mm512_permutex_pd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_permutex_pd(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_permutex_pd", "none");
}

#[test]
fn a0231_mm512_range_pd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_range_pd(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_range_pd", "none");
}

#[test]
fn a0232_mm512_range_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_range_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_range_ps", "none");
}

#[test]
fn a0233_mm512_range_round_pd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_range_round_pd(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_range_round_pd", "none");
}

#[test]
fn a0234_mm512_range_round_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_range_round_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_range_round_ps", "none");
}

#[test]
fn a0235_mm512_rcp14_pd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_rcp14_pd(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_rcp14_pd", "none");
}

#[test]
fn a0236_mm512_rcp14_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_rcp14_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_rcp14_ps", "none");
}

#[test]
fn a0237_mm512_reduce_pd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_reduce_pd(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_reduce_pd", "none");
}

#[test]
fn a0238_mm512_reduce_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_reduce_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_reduce_ps", "none");
}

#[test]
fn a0239_mm512_reduce_round_pd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_reduce_round_pd(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_reduce_round_pd", "none");
}

#[test]
fn a0240_mm512_reduce_round_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_reduce_round_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_reduce_round_ps", "none");
}

#[test]
fn a0241_mm512_roundscale_pd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_roundscale_pd(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_roundscale_pd", "none");
}

#[test]
fn a0242_mm512_roundscale_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_roundscale_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_roundscale_ps", "none");
}

#[test]
fn a0243_mm512_roundscale_round_pd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_roundscale_round_pd(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_roundscale_round_pd", "none");
}

#[test]
fn a0244_mm512_roundscale_round_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_roundscale_round_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_roundscale_round_ps", "none");
}

#[test]
fn a0245_mm512_rsqrt14_pd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_rsqrt14_pd(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_rsqrt14_pd", "none");
}

#[test]
fn a0246_mm512_rsqrt14_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_rsqrt14_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_rsqrt14_ps", "none");
}

#[test]
fn a0247_mm512_scalef_round_pd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_scalef_round_pd(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_scalef_round_pd", "none");
}

#[test]
fn a0248_mm512_scalef_round_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_scalef_round_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_scalef_round_ps", "none");
}

#[test]
fn a0249_mm512_set4_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_set4_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_set4_epi32", "none");
}

#[test]
fn a0250_mm512_set4_epi64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_set4_epi64(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_set4_epi64", "none");
}

#[test]
fn a0251_mm512_set4_pd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_set4_pd(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_set4_pd", "none");
}

#[test]
fn a0252_mm512_set4_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_set4_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_set4_ps", "none");
}

#[test]
fn a0253_mm512_setr4_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_setr4_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_setr4_epi32", "none");
}

#[test]
fn a0254_mm512_setr4_epi64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_setr4_epi64(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_setr4_epi64", "none");
}

#[test]
fn a0255_mm512_setr4_pd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_setr4_pd(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_setr4_pd", "none");
}

#[test]
fn a0256_mm512_setr4_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_setr4_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_setr4_ps", "none");
}

#[test]
fn a0257_mm512_setzero() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_setzero(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_setzero", "none");
}

#[test]
fn a0258_mm512_setzero_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_setzero_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_setzero_epi32", "none");
}

#[test]
fn a0259_mm512_shuffle_f32x4() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_shuffle_f32x4(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_shuffle_f32x4", "none");
}

#[test]
fn a0260_mm512_shuffle_f64x2() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_shuffle_f64x2(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_shuffle_f64x2", "none");
}

#[test]
fn a0261_mm512_shuffle_i64x2() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_shuffle_i64x2(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_shuffle_i64x2", "none");
}

#[test]
fn a0262_mm512_shufflehi_epi16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_shufflehi_epi16(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_shufflehi_epi16", "none");
}

#[test]
fn a0263_mm512_shufflelo_epi16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_shufflelo_epi16(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_shufflelo_epi16", "none");
}

#[test]
fn a0264_mm512_sqrt_pd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_sqrt_pd(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_sqrt_pd", "none");
}

#[test]
fn a0265_mm512_sqrt_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_sqrt_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_sqrt_ps", "none");
}

#[test]
fn a0266_mm512_sqrt_round_pd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_sqrt_round_pd(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_sqrt_round_pd", "none");
}

#[test]
fn a0267_mm512_sqrt_round_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_sqrt_round_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_sqrt_round_ps", "none");
}

#[test]
fn a0268_mm512_stream_load_si512() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_stream_load_si512(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_stream_load_si512", "none");
}

#[test]
fn a0269_mm512_sub_round_pd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_sub_round_pd(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_sub_round_pd", "none");
}

#[test]
fn a0270_mm512_sub_round_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_sub_round_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_sub_round_ps", "none");
}

#[test]
fn a0271_mm512_undefined() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_undefined(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_undefined", "none");
}

#[test]
fn a0272_mm512_undefined_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_undefined_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_undefined_epi32", "none");
}

#[test]
fn a0273_mm512_undefined_pd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_undefined_pd(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_undefined_pd", "none");
}

#[test]
fn a0274_mm512_undefined_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_undefined_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_undefined_ps", "none");
}

#[test]
fn a0275_mm512_zextpd128_pd512() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_zextpd128_pd512(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_zextpd128_pd512", "none");
}

#[test]
fn a0276_mm512_zextpd256_pd512() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_zextpd256_pd512(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_zextpd256_pd512", "none");
}

#[test]
fn a0277_mm512_zextps128_ps512() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_zextps128_ps512(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_zextps128_ps512", "none");
}

#[test]
fn a0278_mm512_zextps256_ps512() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_zextps256_ps512(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_zextps256_ps512", "none");
}

#[test]
fn a0279_mm512_zextsi128_si512() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_zextsi128_si512(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_zextsi128_si512", "none");
}

#[test]
fn a0280_mm512_zextsi256_si512() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm512_zextsi256_si512(core::hint::black_box(n), &mut d); } }
    d.line("_mm512_zextsi256_si512", "none");
}
