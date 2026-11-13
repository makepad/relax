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
    unsafe fn probe_lzcnt_32 (n: usize, h: &mut Digest) {
        if feat("lzcnt") {
            let out = _lzcnt_u32((input::<u64>(n+1) as u32)); mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_tzcnt_32 (n: usize, h: &mut Digest) {
        if feat("bmi1") {
            let out = _tzcnt_u32((input::<u64>(n+1) as u32)); mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_andn_32 (n: usize, h: &mut Digest) {
        if feat("sse2") {
            let out = _andn_u32((input::<u64>(n+1) as u32), (input::<u64>(n+2) as u32)); mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_blsr_32 (n: usize, h: &mut Digest) {
        if feat("bmi1") {
            let out = _blsr_u32((input::<u64>(n+1) as u32)); mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_blsi_32 (n: usize, h: &mut Digest) {
        if feat("bmi1") {
            let out = _blsi_u32((input::<u64>(n+1) as u32)); mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_blsmsk_32 (n: usize, h: &mut Digest) {
        if feat("bmi1") {
            let out = _blsmsk_u32((input::<u64>(n+1) as u32)); mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_blsfill_32 (n: usize, h: &mut Digest) {
        if feat("tbm") {
            let out = _blsfill_u32((input::<u64>(n+1) as u32)); mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_blci_32 (n: usize, h: &mut Digest) {
        if feat("tbm") {
            let out = _blci_u32((input::<u64>(n+1) as u32)); mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_blcic_32 (n: usize, h: &mut Digest) {
        if feat("tbm") {
            let out = _blcic_u32((input::<u64>(n+1) as u32)); mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_blcmsk_32 (n: usize, h: &mut Digest) {
        if feat("tbm") {
            let out = _blcmsk_u32((input::<u64>(n+1) as u32)); mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_blcfill_32 (n: usize, h: &mut Digest) {
        if feat("tbm") {
            let out = _blcfill_u32((input::<u64>(n+1) as u32)); mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_blcs_32 (n: usize, h: &mut Digest) {
        if feat("tbm") {
            let out = _blcs_u32((input::<u64>(n+1) as u32)); mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_t1mskc_32 (n: usize, h: &mut Digest) {
        if feat("tbm") {
            let out = _t1mskc_u32((input::<u64>(n+1) as u32)); mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_tzmsk_32 (n: usize, h: &mut Digest) {
        if feat("tbm") {
            let out = _tzmsk_u32((input::<u64>(n+1) as u32)); mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_bzhi_32 (n: usize, h: &mut Digest) {
        if feat("bmi2") {
            let out = _bzhi_u32((input::<u64>(n+1) as u32), (input::<u64>(n+2) as u32)); mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_bextr2_32 (n: usize, h: &mut Digest) {
        if feat("tbm") {
            let out = _bextr2_u32((input::<u64>(n+1) as u32), (input::<u64>(n+2) as u32)); mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_bextr_32 (n: usize, h: &mut Digest) {
        if feat("bmi1") {
            let out = _bextr_u32((input::<u64>(n+1) as u32), (input::<u64>(n+2) as u32), (input::<u64>(n+3) as u32)); mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_bextri_32 (n: usize, h: &mut Digest) {
        if feat("tbm") {
            let out = _bextri_u32::<0x1a08>((input::<u64>(n+1) as u32)); mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_pdep_32 (n: usize, h: &mut Digest) {
        if feat("bmi2") {
            let out = _pdep_u32((input::<u64>(n+1) as u32), (input::<u64>(n+2) as u32)); mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_pext_32 (n: usize, h: &mut Digest) {
        if feat("bmi2") {
            let out = _pext_u32((input::<u64>(n+1) as u32), (input::<u64>(n+2) as u32)); mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mulx_32 (n: usize, h: &mut Digest) {
        if feat("bmi2") {
            let mut o: u32 = 0;
            let out = _mulx_u32((input::<u64>(n+1) as u32), (input::<u64>(n+2) as u32), &mut o); mix(h, &out); mix(h, &o);
        }
    }

    #[inline(never)]
    unsafe fn probe_addcarry_32 (n: usize, h: &mut Digest) {
        if feat("adx") {
            let mut o: u32 = 0;
            let out = _addcarry_u32((input::<u64>(n+1) as u8), (input::<u64>(n+2) as u32), (input::<u64>(n+3) as u32), &mut o); mix(h, &out); mix(h, &o);
        }
    }

    #[inline(never)]
    unsafe fn probe_addcarryx_32 (n: usize, h: &mut Digest) {
        if feat("adx") {
            let mut o: u32 = 0;
            let out = _addcarryx_u32((input::<u64>(n+1) as u8), (input::<u64>(n+2) as u32), (input::<u64>(n+3) as u32), &mut o); mix(h, &out); mix(h, &o);
        }
    }

    #[inline(never)]
    unsafe fn probe_subborrow_32 (n: usize, h: &mut Digest) {
        if feat("adx") {
            let mut o: u32 = 0;
            let out = _subborrow_u32((input::<u64>(n+1) as u8), (input::<u64>(n+2) as u32), (input::<u64>(n+3) as u32), &mut o); mix(h, &out); mix(h, &o);
        }
    }

    #[inline(never)]
    unsafe fn probe_lzcnt_64 (n: usize, h: &mut Digest) {
        if feat("lzcnt") {
            let out = _lzcnt_u64((input::<u64>(n+1) as u64)); mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_tzcnt_64 (n: usize, h: &mut Digest) {
        if feat("bmi1") {
            let out = _tzcnt_u64((input::<u64>(n+1) as u64)); mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_andn_64 (n: usize, h: &mut Digest) {
        if feat("sse2") {
            let out = _andn_u64((input::<u64>(n+1) as u64), (input::<u64>(n+2) as u64)); mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_blsr_64 (n: usize, h: &mut Digest) {
        if feat("bmi1") {
            let out = _blsr_u64((input::<u64>(n+1) as u64)); mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_blsi_64 (n: usize, h: &mut Digest) {
        if feat("bmi1") {
            let out = _blsi_u64((input::<u64>(n+1) as u64)); mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_blsmsk_64 (n: usize, h: &mut Digest) {
        if feat("bmi1") {
            let out = _blsmsk_u64((input::<u64>(n+1) as u64)); mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_blsfill_64 (n: usize, h: &mut Digest) {
        if feat("tbm") {
            let out = _blsfill_u64((input::<u64>(n+1) as u64)); mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_blci_64 (n: usize, h: &mut Digest) {
        if feat("tbm") {
            let out = _blci_u64((input::<u64>(n+1) as u64)); mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_blcic_64 (n: usize, h: &mut Digest) {
        if feat("tbm") {
            let out = _blcic_u64((input::<u64>(n+1) as u64)); mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_blcmsk_64 (n: usize, h: &mut Digest) {
        if feat("tbm") {
            let out = _blcmsk_u64((input::<u64>(n+1) as u64)); mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_blcfill_64 (n: usize, h: &mut Digest) {
        if feat("tbm") {
            let out = _blcfill_u64((input::<u64>(n+1) as u64)); mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_blcs_64 (n: usize, h: &mut Digest) {
        if feat("tbm") {
            let out = _blcs_u64((input::<u64>(n+1) as u64)); mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_t1mskc_64 (n: usize, h: &mut Digest) {
        if feat("tbm") {
            let out = _t1mskc_u64((input::<u64>(n+1) as u64)); mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_tzmsk_64 (n: usize, h: &mut Digest) {
        if feat("tbm") {
            let out = _tzmsk_u64((input::<u64>(n+1) as u64)); mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_bzhi_64 (n: usize, h: &mut Digest) {
        if feat("bmi2") {
            let out = _bzhi_u64((input::<u64>(n+1) as u64), (input::<u64>(n+2) as u32)); mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_bextr2_64 (n: usize, h: &mut Digest) {
        if feat("tbm") {
            let out = _bextr2_u64((input::<u64>(n+1) as u64), (input::<u64>(n+2) as u64)); mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_bextr_64 (n: usize, h: &mut Digest) {
        if feat("bmi1") {
            let out = _bextr_u64((input::<u64>(n+1) as u64), (input::<u64>(n+2) as u32), (input::<u64>(n+3) as u32)); mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_bextri_64 (n: usize, h: &mut Digest) {
        if feat("tbm") {
            let out = _bextri_u64::<0x1a08>((input::<u64>(n+1) as u64)); mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_pdep_64 (n: usize, h: &mut Digest) {
        if feat("bmi2") {
            let out = _pdep_u64((input::<u64>(n+1) as u64), (input::<u64>(n+2) as u64)); mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_pext_64 (n: usize, h: &mut Digest) {
        if feat("bmi2") {
            let out = _pext_u64((input::<u64>(n+1) as u64), (input::<u64>(n+2) as u64)); mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mulx_64 (n: usize, h: &mut Digest) {
        if feat("bmi2") {
            let mut o: u64 = 0;
            let out = _mulx_u64((input::<u64>(n+1) as u64), (input::<u64>(n+2) as u64), &mut o); mix(h, &out); mix(h, &o);
        }
    }

    #[inline(never)]
    unsafe fn probe_addcarry_64 (n: usize, h: &mut Digest) {
        if feat("adx") {
            let mut o: u64 = 0;
            let out = _addcarry_u64((input::<u64>(n+1) as u8), (input::<u64>(n+2) as u64), (input::<u64>(n+3) as u64), &mut o); mix(h, &out); mix(h, &o);
        }
    }

    #[inline(never)]
    unsafe fn probe_addcarryx_64 (n: usize, h: &mut Digest) {
        if feat("adx") {
            let mut o: u64 = 0;
            let out = _addcarryx_u64((input::<u64>(n+1) as u8), (input::<u64>(n+2) as u64), (input::<u64>(n+3) as u64), &mut o); mix(h, &out); mix(h, &o);
        }
    }

    #[inline(never)]
    unsafe fn probe_subborrow_64 (n: usize, h: &mut Digest) {
        if feat("adx") {
            let mut o: u64 = 0;
            let out = _subborrow_u64((input::<u64>(n+1) as u8), (input::<u64>(n+2) as u64), (input::<u64>(n+3) as u64), &mut o); mix(h, &out); mix(h, &o);
        }
    }

    #[inline(never)]
    unsafe fn probe_tzcnt_u16 (n: usize, h: &mut Digest) {
        if feat("bmi1") {
            let out = _tzcnt_u16((input::<u64>(n+1) as u16)); mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_popcnt32 (n: usize, h: &mut Digest) {
        if feat("popcnt") {
            let out = _popcnt32((input::<u64>(n+1) as i32)); mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_popcnt64 (n: usize, h: &mut Digest) {
        if feat("popcnt") {
            let out = _popcnt64((input::<u64>(n+1) as i64)); mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_bswap (n: usize, h: &mut Digest) {
        if feat("sse2") {
            let out = _bswap((input::<u64>(n+1) as i32)); mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_bswap64 (n: usize, h: &mut Digest) {
        if feat("sse2") {
            let out = _bswap64((input::<u64>(n+1) as i64)); mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_bittest (n: usize, h: &mut Digest) {
        if feat("sse2") {
            let a = [input::<i32>(n+1), input::<i32>(n+2), input::<i32>(n+3), input::<i32>(n+4), 0i32, 0, 0, 0]; let out = _bittest(&a[2] as *const i32, (input::<i32>(n+5) & 127) - 64); mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_bittestandset (n: usize, h: &mut Digest) {
        if feat("sse2") {
            let mut a = [input::<i32>(n+1), input::<i32>(n+2), input::<i32>(n+3), input::<i32>(n+4), 0i32, 0, 0, 0]; let out = _bittestandset(&mut a[2] as *mut i32, (input::<i32>(n+5) & 127) - 64); mix(h, &out); mix(h, &a);
        }
    }

    #[inline(never)]
    unsafe fn probe_bittestandreset (n: usize, h: &mut Digest) {
        if feat("sse2") {
            let mut a = [input::<i32>(n+1), input::<i32>(n+2), input::<i32>(n+3), input::<i32>(n+4), 0i32, 0, 0, 0]; let out = _bittestandreset(&mut a[2] as *mut i32, (input::<i32>(n+5) & 127) - 64); mix(h, &out); mix(h, &a);
        }
    }

    #[inline(never)]
    unsafe fn probe_bittestandcomplement (n: usize, h: &mut Digest) {
        if feat("sse2") {
            let mut a = [input::<i32>(n+1), input::<i32>(n+2), input::<i32>(n+3), input::<i32>(n+4), 0i32, 0, 0, 0]; let out = _bittestandcomplement(&mut a[2] as *mut i32, (input::<i32>(n+5) & 127) - 64); mix(h, &out); mix(h, &a);
        }
    }

    #[inline(never)]
    unsafe fn probe_bittest64 (n: usize, h: &mut Digest) {
        if feat("sse2") {
            let a = [input::<i64>(n+1), input::<i64>(n+2), input::<i64>(n+3), input::<i64>(n+4)]; let out = _bittest64(&a[1] as *const i64, (input::<i64>(n+5) & 127) - 64); mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_bittestandset64 (n: usize, h: &mut Digest) {
        if feat("sse2") {
            let mut a = [input::<i64>(n+1), input::<i64>(n+2), input::<i64>(n+3), input::<i64>(n+4)]; let out = _bittestandset64(&mut a[1] as *mut i64, (input::<i64>(n+5) & 127) - 64); mix(h, &out); mix(h, &a);
        }
    }

    #[inline(never)]
    unsafe fn probe_bittestandreset64 (n: usize, h: &mut Digest) {
        if feat("sse2") {
            let mut a = [input::<i64>(n+1), input::<i64>(n+2), input::<i64>(n+3), input::<i64>(n+4)]; let out = _bittestandreset64(&mut a[1] as *mut i64, (input::<i64>(n+5) & 127) - 64); mix(h, &out); mix(h, &a);
        }
    }

    #[inline(never)]
    unsafe fn probe_bittestandcomplement64 (n: usize, h: &mut Digest) {
        if feat("sse2") {
            let mut a = [input::<i64>(n+1), input::<i64>(n+2), input::<i64>(n+3), input::<i64>(n+4)]; let out = _bittestandcomplement64(&mut a[1] as *mut i64, (input::<i64>(n+5) & 127) - 64); mix(h, &out); mix(h, &a);
        }
    }

    #[inline(never)]
    unsafe fn probe_rdtsc (n: usize, h: &mut Digest) {
        if feat("sse2") {
            let out = _rdtsc(); mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_comineq_ss (n: usize, h: &mut Digest) {
        if feat("sse") {
            let out = _mm_comineq_ss(float_input(n+1), float_input(n+2)); mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_comi_round_ss (n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_comi_round_ss::<0, 4>(float_input(n+1), float_input(n+2)); mix(h, &out);
            let out = _mm_comi_round_ss::<1, 4>(float_input(n+1), float_input(n+2)); mix(h, &out);
            let out = _mm_comi_round_ss::<2, 4>(float_input(n+1), float_input(n+2)); mix(h, &out);
            let out = _mm_comi_round_ss::<4, 4>(float_input(n+1), float_input(n+2)); mix(h, &out);
            let out = _mm_comi_round_ss::<7, 4>(float_input(n+1), float_input(n+2)); mix(h, &out);
            let out = _mm_comi_round_ss::<13, 4>(float_input(n+1), float_input(n+2)); mix(h, &out);
            let out = _mm_comi_round_ss::<14, 4>(float_input(n+1), float_input(n+2)); mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_comi_round_sd (n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_comi_round_sd::<0, 4>(float_input(n+1), float_input(n+2)); mix(h, &out);
            let out = _mm_comi_round_sd::<1, 4>(float_input(n+1), float_input(n+2)); mix(h, &out);
            let out = _mm_comi_round_sd::<2, 4>(float_input(n+1), float_input(n+2)); mix(h, &out);
            let out = _mm_comi_round_sd::<4, 4>(float_input(n+1), float_input(n+2)); mix(h, &out);
            let out = _mm_comi_round_sd::<7, 4>(float_input(n+1), float_input(n+2)); mix(h, &out);
            let out = _mm_comi_round_sd::<13, 4>(float_input(n+1), float_input(n+2)); mix(h, &out);
            let out = _mm_comi_round_sd::<14, 4>(float_input(n+1), float_input(n+2)); mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_extracti_si64 (n: usize, h: &mut Digest) {
        if feat("sse4a") {
            let out = _mm_extracti_si64::<8, 4>(input(n+1)); mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_inserti_si64 (n: usize, h: &mut Digest) {
        if feat("sse4a") {
            let out = _mm_inserti_si64::<8, 4>(input(n+1), input(n+2)); mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cmpestra (n: usize, h: &mut Digest) {
        if feat("sse4.2") {
            let out = _mm_cmpestra::<0>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestra::<1>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestra::<8>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestra::<10>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestra::<12>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestra::<14>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestra::<18>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestra::<22>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestra::<26>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestra::<30>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestra::<48>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestra::<52>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestra::<58>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestra::<66>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cmpestro (n: usize, h: &mut Digest) {
        if feat("sse4.2") {
            let out = _mm_cmpestro::<0>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestro::<1>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestro::<8>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestro::<10>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestro::<12>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestro::<14>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestro::<18>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestro::<22>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestro::<26>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestro::<30>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestro::<48>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestro::<52>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestro::<58>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestro::<66>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cmpestrc (n: usize, h: &mut Digest) {
        if feat("sse4.2") {
            let out = _mm_cmpestrc::<0>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestrc::<1>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestrc::<8>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestrc::<10>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestrc::<12>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestrc::<14>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestrc::<18>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestrc::<22>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestrc::<26>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestrc::<30>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestrc::<48>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestrc::<52>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestrc::<58>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestrc::<66>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cmpestrs (n: usize, h: &mut Digest) {
        if feat("sse4.2") {
            let out = _mm_cmpestrs::<0>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestrs::<1>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestrs::<8>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestrs::<10>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestrs::<12>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestrs::<14>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestrs::<18>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestrs::<22>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestrs::<26>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestrs::<30>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestrs::<48>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestrs::<52>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestrs::<58>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestrs::<66>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cmpestrz (n: usize, h: &mut Digest) {
        if feat("sse4.2") {
            let out = _mm_cmpestrz::<0>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestrz::<1>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestrz::<8>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestrz::<10>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestrz::<12>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestrz::<14>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestrz::<18>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestrz::<22>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestrz::<26>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestrz::<30>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestrz::<48>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestrz::<52>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestrz::<58>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestrz::<66>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cmpestri (n: usize, h: &mut Digest) {
        if feat("sse4.2") {
            let out = _mm_cmpestri::<0>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestri::<1>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestri::<8>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestri::<10>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestri::<12>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestri::<14>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestri::<18>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestri::<22>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestri::<26>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestri::<30>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestri::<48>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestri::<52>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestri::<58>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestri::<66>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cmpestrm (n: usize, h: &mut Digest) {
        if feat("sse4.2") {
            let out = _mm_cmpestrm::<0>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestrm::<1>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestrm::<8>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestrm::<10>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestrm::<12>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestrm::<14>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestrm::<18>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestrm::<22>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestrm::<26>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestrm::<30>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestrm::<48>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestrm::<52>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestrm::<58>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
            let out = _mm_cmpestrm::<66>(input::<__m128i>(n+1), (input::<u32>(n+2) % 20) as i32, input::<__m128i>(n+3), (input::<u32>(n+4) % 20) as i32); mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cmpistra (n: usize, h: &mut Digest) {
        if feat("sse4.2") {
            let out = _mm_cmpistra::<0>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistra::<1>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistra::<8>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistra::<10>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistra::<12>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistra::<14>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistra::<18>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistra::<22>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistra::<26>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistra::<30>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistra::<48>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistra::<52>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistra::<58>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistra::<66>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cmpistro (n: usize, h: &mut Digest) {
        if feat("sse4.2") {
            let out = _mm_cmpistro::<0>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistro::<1>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistro::<8>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistro::<10>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistro::<12>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistro::<14>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistro::<18>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistro::<22>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistro::<26>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistro::<30>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistro::<48>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistro::<52>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistro::<58>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistro::<66>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cmpistrc (n: usize, h: &mut Digest) {
        if feat("sse4.2") {
            let out = _mm_cmpistrc::<0>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistrc::<1>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistrc::<8>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistrc::<10>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistrc::<12>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistrc::<14>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistrc::<18>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistrc::<22>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistrc::<26>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistrc::<30>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistrc::<48>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistrc::<52>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistrc::<58>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistrc::<66>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cmpistrs (n: usize, h: &mut Digest) {
        if feat("sse4.2") {
            let out = _mm_cmpistrs::<0>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistrs::<1>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistrs::<8>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistrs::<10>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistrs::<12>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistrs::<14>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistrs::<18>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistrs::<22>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistrs::<26>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistrs::<30>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistrs::<48>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistrs::<52>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistrs::<58>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistrs::<66>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cmpistrz (n: usize, h: &mut Digest) {
        if feat("sse4.2") {
            let out = _mm_cmpistrz::<0>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistrz::<1>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistrz::<8>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistrz::<10>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistrz::<12>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistrz::<14>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistrz::<18>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistrz::<22>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistrz::<26>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistrz::<30>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistrz::<48>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistrz::<52>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistrz::<58>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistrz::<66>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cmpistri (n: usize, h: &mut Digest) {
        if feat("sse4.2") {
            let out = _mm_cmpistri::<0>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistri::<1>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistri::<8>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistri::<10>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistri::<12>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistri::<14>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistri::<18>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistri::<22>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistri::<26>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistri::<30>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistri::<48>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistri::<52>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistri::<58>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistri::<66>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cmpistrm (n: usize, h: &mut Digest) {
        if feat("sse4.2") {
            let out = _mm_cmpistrm::<0>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistrm::<1>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistrm::<8>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistrm::<10>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistrm::<12>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistrm::<14>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistrm::<18>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistrm::<22>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistrm::<26>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistrm::<30>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistrm::<48>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistrm::<52>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistrm::<58>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
            let out = _mm_cmpistrm::<66>(input::<__m128i>(n+1), input::<__m128i>(n+2)); mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_abs_epi16(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_abs_epi16(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_add_epi16(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let b: __m128i = input(n + 5);
        let out = _mm_add_epi16(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_add_epi32(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let b: __m128i = input(n + 5);
        let out = _mm_add_epi32(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_add_epi8(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let b: __m128i = input(n + 5);
        let out = _mm_add_epi8(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_and_si128(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let b: __m128i = input(n + 5);
        let out = _mm_and_si128(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_andnot_si128(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let b: __m128i = input(n + 5);
        let out = _mm_andnot_si128(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_avg_epu8(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let b: __m128i = input(n + 5);
        let out = _mm_avg_epu8(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_blendv_epi8(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let b: __m128i = input(n + 5);
        let mask: __m128i = input(n + 10);
        let out = _mm_blendv_epi8(a, b, mask);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_cmpeq_epi8(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let b: __m128i = input(n + 5);
        let out = _mm_cmpeq_epi8(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_cmpeq_epi16(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let b: __m128i = input(n + 5);
        let out = _mm_cmpeq_epi16(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_cmpgt_epi8(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let b: __m128i = input(n + 5);
        let out = _mm_cmpgt_epi8(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_load_si128(n: usize, h: &mut Digest) {
        #[repr(C, align(16))]
        struct Buffer([u8; 48]);
        let mut buf = Buffer([0xa5u8; 48]);
        let src: [u8; 16] = input(n);
        for i in 0..16 {
            buf.0[16 + i] = src[i];
        }
        let mem_addr = buf.0.as_mut_ptr().add(16) as *const __m128i;
        let out = _mm_load_si128(mem_addr);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_loadu_si128(n: usize, h: &mut Digest) {
        #[repr(C, align(16))]
        struct Buffer([u8; 48]);
        let mut buf = Buffer([0xa5u8; 48]);
        let src: [u8; 16] = input(n);
        for i in 0..16 {
            buf.0[16 + i] = src[i];
        }
        let mem_addr = buf.0.as_mut_ptr().add(17) as *const __m128i;
        let out = _mm_loadu_si128(mem_addr);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_madd_epi16(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let b: __m128i = input(n + 5);
        let out = _mm_madd_epi16(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_maddubs_epi16(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let b: __m128i = input(n + 5);
        let out = _mm_maddubs_epi16(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_min_epi16(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let b: __m128i = input(n + 5);
        let out = _mm_min_epi16(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_or_si128(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let b: __m128i = input(n + 5);
        let out = _mm_or_si128(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_packus_epi16(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let b: __m128i = input(n + 5);
        let out = _mm_packus_epi16(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_sad_epu8(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let b: __m128i = input(n + 5);
        let out = _mm_sad_epu8(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_set1_epi16(n: usize, h: &mut Digest) {
        let a: i16 = input(n + 0);
        let out = _mm_set1_epi16(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_set1_epi8(n: usize, h: &mut Digest) {
        let a: i8 = input(n + 0);
        let out = _mm_set1_epi8(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_setzero_si128(_n: usize, h: &mut Digest) {
        let out = _mm_setzero_si128();
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_shuffle_epi32_0(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_shuffle_epi32::<0>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_shuffle_epi32_1(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_shuffle_epi32::<1>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_shuffle_epi32_7(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_shuffle_epi32::<7>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_shuffle_epi32_15(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_shuffle_epi32::<15>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_shuffle_epi32_16(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_shuffle_epi32::<16>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_shuffle_epi32_31(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_shuffle_epi32::<31>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_shuffle_epi32_32(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_shuffle_epi32::<32>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_shuffle_epi32_63(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_shuffle_epi32::<63>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_shuffle_epi32_64(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_shuffle_epi32::<64>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_shuffle_epi32_255(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_shuffle_epi32::<255>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_shuffle_epi8(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let b: __m128i = input(n + 5);
        let out = _mm_shuffle_epi8(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_slli_epi32_0(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_slli_epi32::<0>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_slli_epi32_1(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_slli_epi32::<1>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_slli_epi32_7(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_slli_epi32::<7>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_slli_epi32_15(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_slli_epi32::<15>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_slli_epi32_16(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_slli_epi32::<16>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_slli_epi32_31(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_slli_epi32::<31>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_slli_epi32_32(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_slli_epi32::<32>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_slli_epi32_63(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_slli_epi32::<63>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_slli_epi32_64(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_slli_epi32::<64>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_slli_epi32_255(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_slli_epi32::<255>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_srli_si128_0(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_srli_si128::<0>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_srli_si128_1(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_srli_si128::<1>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_srli_si128_7(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_srli_si128::<7>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_srli_si128_15(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_srli_si128::<15>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_srli_si128_16(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_srli_si128::<16>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_srli_si128_31(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_srli_si128::<31>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_srli_si128_32(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_srli_si128::<32>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_srli_si128_63(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_srli_si128::<63>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_srli_si128_64(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_srli_si128::<64>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_srli_si128_255(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_srli_si128::<255>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_store_si128(n: usize, h: &mut Digest) {
        #[repr(C, align(16))]
        struct Buffer([u8; 48]);
        let mut buf = Buffer([0xa5u8; 48]);
        let src: [u8; 16] = input(n);
        for i in 0..16 {
            buf.0[16 + i] = src[i];
        }
        let mem_addr = buf.0.as_mut_ptr().add(16) as *mut __m128i;
        let a: __m128i = input(n + 5);
        _mm_store_si128(mem_addr, a);
        mix(h, &buf.0);
    }

    #[inline(never)]
    unsafe fn probe_mm_storeu_si128(n: usize, h: &mut Digest) {
        #[repr(C, align(16))]
        struct Buffer([u8; 48]);
        let mut buf = Buffer([0xa5u8; 48]);
        let src: [u8; 16] = input(n);
        for i in 0..16 {
            buf.0[16 + i] = src[i];
        }
        let mem_addr = buf.0.as_mut_ptr().add(17) as *mut __m128i;
        let a: __m128i = input(n + 5);
        _mm_storeu_si128(mem_addr, a);
        mix(h, &buf.0);
    }

    #[inline(never)]
    unsafe fn probe_mm_sub_epi16(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let b: __m128i = input(n + 5);
        let out = _mm_sub_epi16(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_sub_epi8(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let b: __m128i = input(n + 5);
        let out = _mm_sub_epi8(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_unpackhi_epi64(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let b: __m128i = input(n + 5);
        let out = _mm_unpackhi_epi64(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_unpackhi_epi8(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let b: __m128i = input(n + 5);
        let out = _mm_unpackhi_epi8(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_unpacklo_epi8(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let b: __m128i = input(n + 5);
        let out = _mm_unpacklo_epi8(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_xor_si128(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let b: __m128i = input(n + 5);
        let out = _mm_xor_si128(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_add_ps(n: usize, h: &mut Digest) {
        let a: __m128 = float_input(n + 0);
        let b: __m128 = float_input(n + 5);
        let out = _mm_add_ps(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_and_ps(n: usize, h: &mut Digest) {
        let a: __m128 = float_input(n + 0);
        let b: __m128 = float_input(n + 5);
        let out = _mm_and_ps(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_andnot_ps(n: usize, h: &mut Digest) {
        let a: __m128 = float_input(n + 0);
        let b: __m128 = float_input(n + 5);
        let out = _mm_andnot_ps(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_cmpeq_epi32(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let b: __m128i = input(n + 5);
        let out = _mm_cmpeq_epi32(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_cmpeq_ps(n: usize, h: &mut Digest) {
        let a: __m128 = float_input(n + 0);
        let b: __m128 = float_input(n + 5);
        let out = _mm_cmpeq_ps(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_cmpge_ps(n: usize, h: &mut Digest) {
        let a: __m128 = float_input(n + 0);
        let b: __m128 = float_input(n + 5);
        let out = _mm_cmpge_ps(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_cmpgt_epi16(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let b: __m128i = input(n + 5);
        let out = _mm_cmpgt_epi16(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_cmpgt_epi32(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let b: __m128i = input(n + 5);
        let out = _mm_cmpgt_epi32(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_cmpgt_ps(n: usize, h: &mut Digest) {
        let a: __m128 = float_input(n + 0);
        let b: __m128 = float_input(n + 5);
        let out = _mm_cmpgt_ps(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_cmple_ps(n: usize, h: &mut Digest) {
        let a: __m128 = float_input(n + 0);
        let b: __m128 = float_input(n + 5);
        let out = _mm_cmple_ps(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_cmplt_ps(n: usize, h: &mut Digest) {
        let a: __m128 = float_input(n + 0);
        let b: __m128 = float_input(n + 5);
        let out = _mm_cmplt_ps(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_div_ps(n: usize, h: &mut Digest) {
        let a: __m128 = float_input(n + 0);
        let b: __m128 = float_input(n + 5);
        let out = _mm_div_ps(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_loadu_ps(n: usize, h: &mut Digest) {
        #[repr(C, align(16))]
        struct Buffer([u8; 48]);
        let mut buf = Buffer([0xa5u8; 48]);
        let src: [u8; 16] = input(n);
        for i in 0..16 {
            buf.0[16 + i] = src[i];
        }
        let mem_addr = buf.0.as_mut_ptr().add(17) as *const f32;
        let out = _mm_loadu_ps(mem_addr);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_max_ps(n: usize, h: &mut Digest) {
        let a: __m128 = minmax_input(n % 16);
        let b: __m128 = minmax_input(n / 16);
        let out = _mm_max_ps(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_min_ps(n: usize, h: &mut Digest) {
        let a: __m128 = minmax_input(n % 16);
        let b: __m128 = minmax_input(n / 16);
        let out = _mm_min_ps(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_mul_ps(n: usize, h: &mut Digest) {
        let a: __m128 = float_input(n + 0);
        let b: __m128 = float_input(n + 5);
        let out = _mm_mul_ps(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_or_ps(n: usize, h: &mut Digest) {
        let a: __m128 = float_input(n + 0);
        let b: __m128 = float_input(n + 5);
        let out = _mm_or_ps(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_set1_ps(n: usize, h: &mut Digest) {
        let a: f32 = float_input(n + 0);
        let out = _mm_set1_ps(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_slli_epi16_0(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_slli_epi16::<0>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_slli_epi16_1(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_slli_epi16::<1>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_slli_epi16_7(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_slli_epi16::<7>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_slli_epi16_15(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_slli_epi16::<15>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_slli_epi16_16(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_slli_epi16::<16>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_slli_epi16_31(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_slli_epi16::<31>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_slli_epi16_32(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_slli_epi16::<32>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_slli_epi16_63(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_slli_epi16::<63>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_slli_epi16_64(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_slli_epi16::<64>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_slli_epi16_255(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_slli_epi16::<255>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_slli_epi64_0(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_slli_epi64::<0>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_slli_epi64_1(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_slli_epi64::<1>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_slli_epi64_7(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_slli_epi64::<7>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_slli_epi64_15(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_slli_epi64::<15>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_slli_epi64_16(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_slli_epi64::<16>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_slli_epi64_31(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_slli_epi64::<31>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_slli_epi64_32(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_slli_epi64::<32>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_slli_epi64_63(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_slli_epi64::<63>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_slli_epi64_64(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_slli_epi64::<64>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_slli_epi64_255(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_slli_epi64::<255>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_srai_epi32_0(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_srai_epi32::<0>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_srai_epi32_1(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_srai_epi32::<1>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_srai_epi32_7(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_srai_epi32::<7>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_srai_epi32_15(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_srai_epi32::<15>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_srai_epi32_16(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_srai_epi32::<16>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_srai_epi32_31(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_srai_epi32::<31>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_srai_epi32_32(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_srai_epi32::<32>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_srai_epi32_63(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_srai_epi32::<63>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_srai_epi32_64(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_srai_epi32::<64>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_srai_epi32_255(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_srai_epi32::<255>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_srli_epi16_0(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_srli_epi16::<0>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_srli_epi16_1(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_srli_epi16::<1>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_srli_epi16_7(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_srli_epi16::<7>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_srli_epi16_15(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_srli_epi16::<15>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_srli_epi16_16(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_srli_epi16::<16>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_srli_epi16_31(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_srli_epi16::<31>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_srli_epi16_32(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_srli_epi16::<32>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_srli_epi16_63(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_srli_epi16::<63>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_srli_epi16_64(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_srli_epi16::<64>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_srli_epi16_255(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_srli_epi16::<255>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_srli_epi32_0(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_srli_epi32::<0>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_srli_epi32_1(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_srli_epi32::<1>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_srli_epi32_7(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_srli_epi32::<7>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_srli_epi32_15(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_srli_epi32::<15>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_srli_epi32_16(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_srli_epi32::<16>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_srli_epi32_31(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_srli_epi32::<31>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_srli_epi32_32(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_srli_epi32::<32>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_srli_epi32_63(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_srli_epi32::<63>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_srli_epi32_64(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_srli_epi32::<64>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_srli_epi32_255(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_srli_epi32::<255>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_srli_epi64_0(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_srli_epi64::<0>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_srli_epi64_1(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_srli_epi64::<1>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_srli_epi64_7(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_srli_epi64::<7>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_srli_epi64_15(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_srli_epi64::<15>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_srli_epi64_16(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_srli_epi64::<16>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_srli_epi64_31(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_srli_epi64::<31>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_srli_epi64_32(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_srli_epi64::<32>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_srli_epi64_63(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_srli_epi64::<63>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_srli_epi64_64(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_srli_epi64::<64>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_srli_epi64_255(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_srli_epi64::<255>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_sqrt_ps(n: usize, h: &mut Digest) {
        let a: __m128 = float_input(n + 0);
        let out = _mm_sqrt_ps(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_storeu_ps(n: usize, h: &mut Digest) {
        #[repr(C, align(16))]
        struct Buffer([u8; 48]);
        let mut buf = Buffer([0xa5u8; 48]);
        let src: [u8; 16] = input(n);
        for i in 0..16 {
            buf.0[16 + i] = src[i];
        }
        let mem_addr = buf.0.as_mut_ptr().add(17) as *mut f32;
        let a: __m128 = float_input(n + 5);
        _mm_storeu_ps(mem_addr, a);
        mix(h, &buf.0);
    }

    #[inline(never)]
    unsafe fn probe_mm_sub_ps(n: usize, h: &mut Digest) {
        let a: __m128 = float_input(n + 0);
        let b: __m128 = float_input(n + 5);
        let out = _mm_sub_ps(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_unpackhi_epi16(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let b: __m128i = input(n + 5);
        let out = _mm_unpackhi_epi16(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_unpackhi_epi32(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let b: __m128i = input(n + 5);
        let out = _mm_unpackhi_epi32(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_unpacklo_epi16(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let b: __m128i = input(n + 5);
        let out = _mm_unpacklo_epi16(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_unpacklo_epi32(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let b: __m128i = input(n + 5);
        let out = _mm_unpacklo_epi32(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_unpacklo_epi64(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let b: __m128i = input(n + 5);
        let out = _mm_unpacklo_epi64(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_xor_ps(n: usize, h: &mut Digest) {
        let a: __m128 = float_input(n + 0);
        let b: __m128 = float_input(n + 5);
        let out = _mm_xor_ps(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_add_epi64(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let b: __m128i = input(n + 5);
        let out = _mm_add_epi64(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_sub_epi32(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let b: __m128i = input(n + 5);
        let out = _mm_sub_epi32(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_sub_epi64(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let b: __m128i = input(n + 5);
        let out = _mm_sub_epi64(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_adds_epi8(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let b: __m128i = input(n + 5);
        let out = _mm_adds_epi8(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_adds_epi16(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let b: __m128i = input(n + 5);
        let out = _mm_adds_epi16(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_adds_epu8(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let b: __m128i = input(n + 5);
        let out = _mm_adds_epu8(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_adds_epu16(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let b: __m128i = input(n + 5);
        let out = _mm_adds_epu16(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_subs_epi8(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let b: __m128i = input(n + 5);
        let out = _mm_subs_epi8(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_subs_epi16(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let b: __m128i = input(n + 5);
        let out = _mm_subs_epi16(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_subs_epu8(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let b: __m128i = input(n + 5);
        let out = _mm_subs_epu8(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_subs_epu16(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let b: __m128i = input(n + 5);
        let out = _mm_subs_epu16(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_min_epi8(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let b: __m128i = input(n + 5);
        let out = _mm_min_epi8(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_min_epi32(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let b: __m128i = input(n + 5);
        let out = _mm_min_epi32(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_min_epu8(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let b: __m128i = input(n + 5);
        let out = _mm_min_epu8(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_min_epu16(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let b: __m128i = input(n + 5);
        let out = _mm_min_epu16(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_min_epu32(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let b: __m128i = input(n + 5);
        let out = _mm_min_epu32(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_max_epi8(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let b: __m128i = input(n + 5);
        let out = _mm_max_epi8(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_max_epi16(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let b: __m128i = input(n + 5);
        let out = _mm_max_epi16(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_max_epi32(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let b: __m128i = input(n + 5);
        let out = _mm_max_epi32(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_max_epu8(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let b: __m128i = input(n + 5);
        let out = _mm_max_epu8(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_max_epu16(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let b: __m128i = input(n + 5);
        let out = _mm_max_epu16(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_max_epu32(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let b: __m128i = input(n + 5);
        let out = _mm_max_epu32(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_avg_epu16(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let b: __m128i = input(n + 5);
        let out = _mm_avg_epu16(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_abs_epi8(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_abs_epi8(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_abs_epi32(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_abs_epi32(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_set1_epi32(n: usize, h: &mut Digest) {
        let a: i32 = input(n + 0);
        let out = _mm_set1_epi32(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_set1_epi64x(n: usize, h: &mut Digest) {
        let a: i64 = input(n + 0);
        let out = _mm_set1_epi64x(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_packs_epi16(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let b: __m128i = input(n + 5);
        let out = _mm_packs_epi16(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_packs_epi32(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let b: __m128i = input(n + 5);
        let out = _mm_packs_epi32(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_packus_epi32(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let b: __m128i = input(n + 5);
        let out = _mm_packus_epi32(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_srai_epi16_0(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_srai_epi16::<0>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_srai_epi16_1(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_srai_epi16::<1>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_srai_epi16_7(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_srai_epi16::<7>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_srai_epi16_15(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_srai_epi16::<15>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_srai_epi16_16(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_srai_epi16::<16>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_srai_epi16_31(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_srai_epi16::<31>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_srai_epi16_32(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_srai_epi16::<32>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_srai_epi16_63(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_srai_epi16::<63>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_srai_epi16_64(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_srai_epi16::<64>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_srai_epi16_255(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_srai_epi16::<255>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_cmpneq_ps(n: usize, h: &mut Digest) {
        let a: __m128 = float_input(n + 0);
        let b: __m128 = float_input(n + 5);
        let out = _mm_cmpneq_ps(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_srai_epi64_0(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_srai_epi64::<0>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_srai_epi64_1(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_srai_epi64::<1>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_srai_epi64_7(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_srai_epi64::<7>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_srai_epi64_15(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_srai_epi64::<15>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_srai_epi64_16(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_srai_epi64::<16>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_srai_epi64_31(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_srai_epi64::<31>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_srai_epi64_32(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_srai_epi64::<32>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_srai_epi64_63(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_srai_epi64::<63>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_srai_epi64_64(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_srai_epi64::<64>(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm_srai_epi64_255(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm_srai_epi64::<255>(a);
        mix(h, &out);
}

    #[inline(never)]
    unsafe fn probe__mm256_abs_epi8(n: usize, h: &mut Digest) {
        let a: __m256i = input(n + 0);
        let out = _mm256_abs_epi8(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe__mm256_sign_epi8(n: usize, h: &mut Digest) {
        let a: __m256i = input(n + 0);
        let b: __m256i = input(n + 5);
        let out = _mm256_sign_epi8(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe__mm_hadd_epi32(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let b: __m128i = input(n + 5);
        let out = _mm_hadd_epi32(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe__mm256_cvtepu8_epi32(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm256_cvtepu8_epi32(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe__mm256_cvtepi8_epi16(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm256_cvtepi8_epi16(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe__mm256_cvtepi16_epi32(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm256_cvtepi16_epi32(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe__mm256_cvtepu16_epi32(n: usize, h: &mut Digest) {
        let a: __m128i = input(n + 0);
        let out = _mm256_cvtepu16_epi32(a);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe__mm256_permute2f128_si256_0(n: usize, h: &mut Digest) {
        let a: __m256i = input(n + 0);
        let b: __m256i = input(n + 5);
        let out = _mm256_permute2f128_si256::<0>(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe__mm256_permute2f128_si256_10(n: usize, h: &mut Digest) {
        let a: __m256i = input(n + 0);
        let b: __m256i = input(n + 5);
        let out = _mm256_permute2f128_si256::<16>(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe__mm256_permute2f128_si256_21(n: usize, h: &mut Digest) {
        let a: __m256i = input(n + 0);
        let b: __m256i = input(n + 5);
        let out = _mm256_permute2f128_si256::<33>(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe__mm256_permute2f128_si256_88(n: usize, h: &mut Digest) {
        let a: __m256i = input(n + 0);
        let b: __m256i = input(n + 5);
        let out = _mm256_permute2f128_si256::<136>(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe__mm256_permute2f128_si256_ff(n: usize, h: &mut Digest) {
        let a: __m256i = input(n + 0);
        let b: __m256i = input(n + 5);
        let out = _mm256_permute2f128_si256::<255>(a, b);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe__mm_clflush(_n: usize, h: &mut Digest) {
        let buf = [0xa5u8; 64];
        _mm_clflush(buf.as_ptr());
        mix(h, &buf);
    }

    #[inline(never)]
    unsafe fn probe__mm_mfence(n: usize, h: &mut Digest) {
        _mm_mfence();
        let out = n as u64 ^ 0x5bd1e995;
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe__pext_u32(n: usize, h: &mut Digest) {
        let a: u32 = input(n + 0);
        let m: u32 = input(n + 5);
        let out = _pext_u32(a, m);
        mix(h, &out);
    }

    #[inline(never)]
    unsafe fn probe_mm256_addsub_pd(n: usize, h: &mut Digest) {
        if feat("avx") {
            let out = _mm256_addsub_pd(float_input(n+0), float_input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_addsub_ps(n: usize, h: &mut Digest) {
        if feat("avx") {
            let out = _mm256_addsub_ps(float_input(n+0), float_input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_alignr_epi32(n: usize, h: &mut Digest) {
        if feat("avx2") {
            let out = _mm256_alignr_epi32::<9>(input(n+0), input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_alignr_epi64(n: usize, h: &mut Digest) {
        if feat("avx2") {
            let out = _mm256_alignr_epi64::<9>(input(n+0), input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_blend_epi16(n: usize, h: &mut Digest) {
        if feat("avx2") {
            let out = _mm256_blend_epi16::<165>(input(n+0), input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_blend_ps(n: usize, h: &mut Digest) {
        if feat("avx2") {
            let out = _mm256_blend_ps::<165>(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_broadcast_f32x2(n: usize, h: &mut Digest) {
        if feat("avx512vl") {
            let out = _mm256_broadcast_f32x2(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_broadcast_f32x4(n: usize, h: &mut Digest) {
        if feat("avx512vl") {
            let out = _mm256_broadcast_f32x4(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_broadcast_f64x2(n: usize, h: &mut Digest) {
        if feat("avx512vl") {
            let out = _mm256_broadcast_f64x2(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_broadcast_i32x2(n: usize, h: &mut Digest) {
        if feat("avx512vl") {
            let out = _mm256_broadcast_i32x2(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_broadcast_i64x2(n: usize, h: &mut Digest) {
        if feat("avx512vl") {
            let out = _mm256_broadcast_i64x2(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_broadcast_ps(n: usize, h: &mut Digest) {
        if feat("avx") {
            let out = _mm256_broadcast_ps(&float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_broadcast_ss(n: usize, h: &mut Digest) {
        if feat("avx") {
            let out = _mm256_broadcast_ss(&float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_broadcastmb_epi64(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm256_broadcastmb_epi64(kbits(n) as __mmask8);
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_broadcastmw_epi32(n: usize, h: &mut Digest) {
        if feat("avx512cd") {
            let out = _mm256_broadcastmw_epi32(kbits(n) as __mmask16);
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_broadcastss_ps(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm256_broadcastss_ps(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_bslli_epi128(n: usize, h: &mut Digest) {
        if feat("avx2") {
            let out = _mm256_bslli_epi128::<5>(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_bsrli_epi128(n: usize, h: &mut Digest) {
        if feat("avx2") {
            let out = _mm256_bsrli_epi128::<5>(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_castps128_ps256(n: usize, h: &mut Digest) {
        if feat("avx2") {
            let out = _mm256_castps128_ps256(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_conflict_epi32(n: usize, h: &mut Digest) {
        if feat("avx512cd") {
            let out = _mm256_conflict_epi32(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_conflict_epi64(n: usize, h: &mut Digest) {
        if feat("avx512cd") {
            let out = _mm256_conflict_epi64(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_cvtepi16_epi64(n: usize, h: &mut Digest) {
        if feat("avx2") {
            let out = _mm256_cvtepi16_epi64(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_cvtepi16_epi8(n: usize, h: &mut Digest) {
        if feat("avx2") {
            let out = _mm256_cvtepi16_epi8(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_cvtepi32_epi16(n: usize, h: &mut Digest) {
        if feat("avx2") {
            let out = _mm256_cvtepi32_epi16(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_cvtepi32_epi64(n: usize, h: &mut Digest) {
        if feat("avx2") {
            let out = _mm256_cvtepi32_epi64(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_cvtepi32_epi8(n: usize, h: &mut Digest) {
        if feat("avx2") {
            let out = _mm256_cvtepi32_epi8(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_cvtepi64_epi16(n: usize, h: &mut Digest) {
        if feat("avx2") {
            let out = _mm256_cvtepi64_epi16(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_cvtepi64_epi32(n: usize, h: &mut Digest) {
        if feat("avx2") {
            let out = _mm256_cvtepi64_epi32(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_cvtepi64_epi8(n: usize, h: &mut Digest) {
        if feat("avx2") {
            let out = _mm256_cvtepi64_epi8(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_cvtepi64_pd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm256_cvtepi64_pd(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_cvtepi64_ps(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm256_cvtepi64_ps(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_cvtepi8_epi64(n: usize, h: &mut Digest) {
        if feat("avx2") {
            let out = _mm256_cvtepi8_epi64(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_cvtepu16_epi64(n: usize, h: &mut Digest) {
        if feat("avx2") {
            let out = _mm256_cvtepu16_epi64(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_cvtepu32_epi64(n: usize, h: &mut Digest) {
        if feat("avx2") {
            let out = _mm256_cvtepu32_epi64(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_cvtepu32_pd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm256_cvtepu32_pd(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_cvtepu64_pd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm256_cvtepu64_pd(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_cvtepu64_ps(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm256_cvtepu64_ps(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_cvtepu8_epi64(n: usize, h: &mut Digest) {
        if feat("avx2") {
            let out = _mm256_cvtepu8_epi64(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_cvtpd_epi64(n: usize, h: &mut Digest) {
        if feat("avx512vl") {
            let out = _mm256_cvtpd_epi64(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_cvtpd_epu32(n: usize, h: &mut Digest) {
        if feat("avx512vl") {
            let out = _mm256_cvtpd_epu32(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_cvtpd_epu64(n: usize, h: &mut Digest) {
        if feat("avx512vl") {
            let out = _mm256_cvtpd_epu64(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_cvtps_epi32(n: usize, h: &mut Digest) {
        if feat("avx2") {
            let out = _mm256_cvtps_epi32(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_cvtps_epi64(n: usize, h: &mut Digest) {
        if feat("avx512vl") {
            let out = _mm256_cvtps_epi64(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_cvtps_epu32(n: usize, h: &mut Digest) {
        if feat("avx512vl") {
            let out = _mm256_cvtps_epu32(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_cvtps_epu64(n: usize, h: &mut Digest) {
        if feat("avx512vl") {
            let out = _mm256_cvtps_epu64(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_cvtsd_f64(n: usize, h: &mut Digest) {
        if feat("avx") {
            let out = _mm256_cvtsd_f64(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_cvtsepi16_epi8(n: usize, h: &mut Digest) {
        if feat("avx2") {
            let out = _mm256_cvtsepi16_epi8(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_cvtsepi32_epi16(n: usize, h: &mut Digest) {
        if feat("avx2") {
            let out = _mm256_cvtsepi32_epi16(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_cvtsepi32_epi8(n: usize, h: &mut Digest) {
        if feat("avx2") {
            let out = _mm256_cvtsepi32_epi8(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_cvtsepi64_epi16(n: usize, h: &mut Digest) {
        if feat("avx2") {
            let out = _mm256_cvtsepi64_epi16(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_cvtsepi64_epi32(n: usize, h: &mut Digest) {
        if feat("avx2") {
            let out = _mm256_cvtsepi64_epi32(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_cvtsepi64_epi8(n: usize, h: &mut Digest) {
        if feat("avx2") {
            let out = _mm256_cvtsepi64_epi8(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_cvtsi256_si32(n: usize, h: &mut Digest) {
        if feat("avx") {
            let out = _mm256_cvtsi256_si32(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_cvtss_f32(n: usize, h: &mut Digest) {
        if feat("avx") {
            let out = _mm256_cvtss_f32(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_cvttpd_epi64(n: usize, h: &mut Digest) {
        if feat("avx512vl") {
            let out = _mm256_cvttpd_epi64(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_cvttpd_epu32(n: usize, h: &mut Digest) {
        if feat("avx512vl") {
            let out = _mm256_cvttpd_epu32(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_cvttpd_epu64(n: usize, h: &mut Digest) {
        if feat("avx512vl") {
            let out = _mm256_cvttpd_epu64(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_cvttps_epi64(n: usize, h: &mut Digest) {
        if feat("avx512vl") {
            let out = _mm256_cvttps_epi64(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_cvttps_epu32(n: usize, h: &mut Digest) {
        if feat("avx512vl") {
            let out = _mm256_cvttps_epu32(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_cvttps_epu64(n: usize, h: &mut Digest) {
        if feat("avx512vl") {
            let out = _mm256_cvttps_epu64(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_cvtusepi16_epi8(n: usize, h: &mut Digest) {
        if feat("avx512vl") {
            let out = _mm256_cvtusepi16_epi8(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_cvtusepi32_epi16(n: usize, h: &mut Digest) {
        if feat("avx512vl") {
            let out = _mm256_cvtusepi32_epi16(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_cvtusepi32_epi8(n: usize, h: &mut Digest) {
        if feat("avx512vl") {
            let out = _mm256_cvtusepi32_epi8(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_cvtusepi64_epi16(n: usize, h: &mut Digest) {
        if feat("avx512vl") {
            let out = _mm256_cvtusepi64_epi16(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_cvtusepi64_epi32(n: usize, h: &mut Digest) {
        if feat("avx512vl") {
            let out = _mm256_cvtusepi64_epi32(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_cvtusepi64_epi8(n: usize, h: &mut Digest) {
        if feat("avx512vl") {
            let out = _mm256_cvtusepi64_epi8(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_dbsad_epu8(n: usize, h: &mut Digest) {
        if feat("avx512bw") {
            let out = _mm256_dbsad_epu8::<0xa5>(input(n+0), input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_dp_ps(n: usize, h: &mut Digest) {
        if feat("avx") {
            let out = _mm256_dp_ps::<0xff>(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_dpbsud_epi32(n: usize, h: &mut Digest) {
        if feat("avxvnniint8") {
            let out = _mm256_dpbsud_epi32(input(n+3), input(n+0), input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_dpbsuds_epi32(n: usize, h: &mut Digest) {
        if feat("avxvnniint8") {
            let out = _mm256_dpbsuds_epi32(input(n+3), input(n+0), input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_dpbuud_epi32(n: usize, h: &mut Digest) {
        if feat("avxvnniint8") {
            let out = _mm256_dpbuud_epi32(input(n+3), input(n+0), input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_dpbuuds_epi32(n: usize, h: &mut Digest) {
        if feat("avxvnniint8") {
            let out = _mm256_dpbuuds_epi32(input(n+3), input(n+0), input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_dpwsud_epi32(n: usize, h: &mut Digest) {
        if feat("avxvnniint16") {
            let out = _mm256_dpwsud_epi32(input(n+3), input(n+0), input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_dpwsuds_epi32(n: usize, h: &mut Digest) {
        if feat("avxvnniint16") {
            let out = _mm256_dpwsuds_epi32(input(n+3), input(n+0), input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_dpwusd_epi32(n: usize, h: &mut Digest) {
        if feat("avxvnniint16") {
            let out = _mm256_dpwusd_epi32(input(n+3), input(n+0), input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_dpwusds_epi32(n: usize, h: &mut Digest) {
        if feat("avxvnniint16") {
            let out = _mm256_dpwusds_epi32(input(n+3), input(n+0), input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_dpwuud_epi32(n: usize, h: &mut Digest) {
        if feat("avxvnniint16") {
            let out = _mm256_dpwuud_epi32(input(n+3), input(n+0), input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_dpwuuds_epi32(n: usize, h: &mut Digest) {
        if feat("avxvnniint16") {
            let out = _mm256_dpwuuds_epi32(input(n+3), input(n+0), input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_extract_epi16(n: usize, h: &mut Digest) {
        if feat("avx2") {
            let out = _mm256_extract_epi16::<2>(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_extract_epi8(n: usize, h: &mut Digest) {
        if feat("avx2") {
            let out = _mm256_extract_epi8::<2>(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_extractf32x4_ps(n: usize, h: &mut Digest) {
        if feat("avx512vl") {
            let out = _mm256_extractf32x4_ps::<1>(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_extractf64x2_pd(n: usize, h: &mut Digest) {
        if feat("avx512vl") {
            let out = _mm256_extractf64x2_pd::<1>(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_extracti32x4_epi32(n: usize, h: &mut Digest) {
        if feat("avx512vl") {
            let out = _mm256_extracti32x4_epi32::<1>(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_extracti64x2_epi64(n: usize, h: &mut Digest) {
        if feat("avx512vl") {
            let out = _mm256_extracti64x2_epi64::<1>(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_fixupimm_pd(n: usize, h: &mut Digest) {
        if feat("avx512vl") {
            let out = _mm256_fixupimm_pd::<0x1e>(float_input(n+0), float_input(n+1), input(n+2));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_fixupimm_ps(n: usize, h: &mut Digest) {
        if feat("avx512vl") {
            let out = _mm256_fixupimm_ps::<0x1e>(float_input(n+0), float_input(n+1), input(n+2));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_hadd_epi16(n: usize, h: &mut Digest) {
        if feat("avx2") {
            let out = _mm256_hadd_epi16(input(n+0), input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_hadd_epi32(n: usize, h: &mut Digest) {
        if feat("avx2") {
            let out = _mm256_hadd_epi32(input(n+0), input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_hadd_pd(n: usize, h: &mut Digest) {
        if feat("avx") {
            let out = _mm256_hadd_pd(float_input(n+0), float_input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_hadd_ps(n: usize, h: &mut Digest) {
        if feat("avx") {
            let out = _mm256_hadd_ps(float_input(n+0), float_input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_hadds_epi16(n: usize, h: &mut Digest) {
        if feat("avx2") {
            let out = _mm256_hadds_epi16(input(n+0), input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_hsub_epi16(n: usize, h: &mut Digest) {
        if feat("avx2") {
            let out = _mm256_hsub_epi16(input(n+0), input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_hsub_epi32(n: usize, h: &mut Digest) {
        if feat("avx2") {
            let out = _mm256_hsub_epi32(input(n+0), input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_hsub_pd(n: usize, h: &mut Digest) {
        if feat("avx") {
            let out = _mm256_hsub_pd(float_input(n+0), float_input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_hsub_ps(n: usize, h: &mut Digest) {
        if feat("avx") {
            let out = _mm256_hsub_ps(float_input(n+0), float_input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_hsubs_epi16(n: usize, h: &mut Digest) {
        if feat("avx2") {
            let out = _mm256_hsubs_epi16(input(n+0), input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_insert_epi16(n: usize, h: &mut Digest) {
        if feat("avx2") {
            let out = _mm256_insert_epi16::<2>(input(n+0), (n%97) as i16);
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_insert_epi32(n: usize, h: &mut Digest) {
        if feat("avx2") {
            let out = _mm256_insert_epi32::<2>(input(n+0), (n%97) as i32);
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_insert_epi64(n: usize, h: &mut Digest) {
        if feat("avx2") {
            let out = _mm256_insert_epi64::<2>(input(n+0), (n%97) as i64);
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_insert_epi8(n: usize, h: &mut Digest) {
        if feat("avx2") {
            let out = _mm256_insert_epi8::<2>(input(n+0), (n%97) as i8);
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_insertf128_ps(n: usize, h: &mut Digest) {
        if feat("avx") {
            let out = _mm256_insertf128_ps::<1>(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_loadu2_m128(n: usize, h: &mut Digest) {
        if feat("avx") {
            let out = _mm256_loadu2_m128(loadptr(n+16).cast::<f32>(), loadptr(n).cast::<f32>());
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_loadu2_m128d(n: usize, h: &mut Digest) {
        if feat("avx") {
            let out = _mm256_loadu2_m128d(loadptr(n+8).cast::<f64>(), loadptr(n).cast::<f64>());
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_loadu2_m128i(n: usize, h: &mut Digest) {
        if feat("avx") {
            let out = _mm256_loadu2_m128i(loadptr(n+8).cast::<__m128i>(), loadptr(n).cast::<__m128i>());
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_movehdup_ps(n: usize, h: &mut Digest) {
        if feat("avx") {
            let out = _mm256_movehdup_ps(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_moveldup_ps(n: usize, h: &mut Digest) {
        if feat("avx") {
            let out = _mm256_moveldup_ps(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_mpsadbw_epu8(n: usize, h: &mut Digest) {
        if feat("avx2") {
            let out = _mm256_mpsadbw_epu8::<0x05>(input(n+0), input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_mul_epi32(n: usize, h: &mut Digest) {
        if feat("avx2") {
            let out = _mm256_mul_epi32(input(n+0), input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_mulhi_epi16(n: usize, h: &mut Digest) {
        if feat("avx2") {
            let out = _mm256_mulhi_epi16(input(n+0), input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_mulhrs_epi16(n: usize, h: &mut Digest) {
        if feat("avx2") {
            let out = _mm256_mulhrs_epi16(input(n+0), input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_permute_ps(n: usize, h: &mut Digest) {
        if feat("avx") {
            let out = _mm256_permute_ps::<0x1b>(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_permutevar_ps(n: usize, h: &mut Digest) {
        if feat("avx") {
            let out = _mm256_permutevar_ps(float_input(n+0), input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_permutex_epi64(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm256_permutex_epi64::<0x4e>(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_permutex_pd(n: usize, h: &mut Digest) {
        if feat("avx") {
            let out = _mm256_permutex_pd::<0x4e>(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_range_pd(n: usize, h: &mut Digest) {
        if feat("avx512dq") {
            let out = _mm256_range_pd::<9>(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_range_ps(n: usize, h: &mut Digest) {
        if feat("avx512dq") {
            let out = _mm256_range_ps::<9>(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_rcp14_pd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm256_rcp14_pd(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_rcp14_ps(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm256_rcp14_ps(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_rcp_ps(n: usize, h: &mut Digest) {
        if feat("avx") {
            let out = _mm256_rcp_ps(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_reduce_add_epi16(n: usize, h: &mut Digest) {
        if feat("avx512bw") {
            let out = _mm256_reduce_add_epi16(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_reduce_add_epi8(n: usize, h: &mut Digest) {
        if feat("avx512bw") {
            let out = _mm256_reduce_add_epi8(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_reduce_and_epi16(n: usize, h: &mut Digest) {
        if feat("avx512bw") {
            let out = _mm256_reduce_and_epi16(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_reduce_and_epi8(n: usize, h: &mut Digest) {
        if feat("avx512bw") {
            let out = _mm256_reduce_and_epi8(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_reduce_max_epi16(n: usize, h: &mut Digest) {
        if feat("avx512bw") {
            let out = _mm256_reduce_max_epi16(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_reduce_max_epi8(n: usize, h: &mut Digest) {
        if feat("avx512bw") {
            let out = _mm256_reduce_max_epi8(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_reduce_max_epu16(n: usize, h: &mut Digest) {
        if feat("avx512bw") {
            let out = _mm256_reduce_max_epu16(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_reduce_max_epu8(n: usize, h: &mut Digest) {
        if feat("avx512bw") {
            let out = _mm256_reduce_max_epu8(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_reduce_min_epi16(n: usize, h: &mut Digest) {
        if feat("avx512bw") {
            let out = _mm256_reduce_min_epi16(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_reduce_min_epi8(n: usize, h: &mut Digest) {
        if feat("avx512bw") {
            let out = _mm256_reduce_min_epi8(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_reduce_min_epu16(n: usize, h: &mut Digest) {
        if feat("avx512bw") {
            let out = _mm256_reduce_min_epu16(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_reduce_min_epu8(n: usize, h: &mut Digest) {
        if feat("avx512bw") {
            let out = _mm256_reduce_min_epu8(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_reduce_mul_epi16(n: usize, h: &mut Digest) {
        if feat("avx512bw") {
            let out = _mm256_reduce_mul_epi16(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_reduce_mul_epi8(n: usize, h: &mut Digest) {
        if feat("avx512bw") {
            let out = _mm256_reduce_mul_epi8(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_reduce_or_epi16(n: usize, h: &mut Digest) {
        if feat("avx512bw") {
            let out = _mm256_reduce_or_epi16(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_reduce_or_epi8(n: usize, h: &mut Digest) {
        if feat("avx512bw") {
            let out = _mm256_reduce_or_epi8(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_reduce_pd(n: usize, h: &mut Digest) {
        if feat("avx512dq") {
            let out = _mm256_reduce_pd::<0x19>(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_reduce_ps(n: usize, h: &mut Digest) {
        if feat("avx512dq") {
            let out = _mm256_reduce_ps::<0x19>(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_roundscale_pd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm256_roundscale_pd::<9>(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_roundscale_ps(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm256_roundscale_ps::<9>(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_rsqrt14_pd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm256_rsqrt14_pd(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_rsqrt14_ps(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm256_rsqrt14_ps(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_rsqrt_ps(n: usize, h: &mut Digest) {
        if feat("avx") {
            let out = _mm256_rsqrt_ps(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_set_m128(n: usize, h: &mut Digest) {
        if feat("avx") {
            let out = _mm256_set_m128(float_input(n+1), float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_set_m128d(n: usize, h: &mut Digest) {
        if feat("avx") {
            let out = _mm256_set_m128d(float_input(n+1), float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_setr_m128d(n: usize, h: &mut Digest) {
        if feat("avx") {
            let out = _mm256_setr_m128d(float_input(n+1), float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_shuffle_f32x4(n: usize, h: &mut Digest) {
        if feat("avx512vl") {
            let out = _mm256_shuffle_f32x4::<0xb1>(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_shuffle_f64x2(n: usize, h: &mut Digest) {
        if feat("avx512vl") {
            let out = _mm256_shuffle_f64x2::<0xb1>(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_shuffle_i32x4(n: usize, h: &mut Digest) {
        if feat("avx512vl") {
            let out = _mm256_shuffle_i32x4::<0xb1>(input(n+0), input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_shuffle_i64x2(n: usize, h: &mut Digest) {
        if feat("avx512vl") {
            let out = _mm256_shuffle_i64x2::<0xb1>(input(n+0), input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_shufflehi_epi16(n: usize, h: &mut Digest) {
        if feat("avx2") {
            let out = _mm256_shufflehi_epi16::<0x1b>(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_shufflelo_epi16(n: usize, h: &mut Digest) {
        if feat("avx2") {
            let out = _mm256_shufflelo_epi16::<0x1b>(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_slli_si256(n: usize, h: &mut Digest) {
        if feat("avx2") {
            let out = _mm256_slli_si256::<5>(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_srli_si256(n: usize, h: &mut Digest) {
        if feat("avx2") {
            let out = _mm256_srli_si256::<5>(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_storeu2_m128(n: usize, h: &mut Digest) {
        if feat("avx") {
            let out = _mm256_storeu2_m128(storeptr(n+16).cast::<f32>(), storeptr(n).cast::<f32>(), float_input(n+2));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_storeu2_m128d(n: usize, h: &mut Digest) {
        if feat("avx") {
            let out = _mm256_storeu2_m128d(storeptr(n+16).cast::<f64>(), storeptr(n).cast::<f64>(), float_input(n+2));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_storeu2_m128i(n: usize, h: &mut Digest) {
        if feat("avx") {
            let out = _mm256_storeu2_m128i(storeptr(n+16).cast::<__m128i>(), storeptr(n).cast::<__m128i>(), input(n+2));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_stream_load_si256(n: usize, h: &mut Digest) {
        if feat("avx2") {
            let out = _mm256_stream_load_si256(loadptr(n).cast::<__m256i>());
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_testc_ps(n: usize, h: &mut Digest) {
        if feat("avx") {
            let out = _mm256_testc_ps(float_input(n+0), float_input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_testc_si256(n: usize, h: &mut Digest) {
        if feat("avx2") {
            let out = _mm256_testc_si256(input(n+0), input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_testnzc_ps(n: usize, h: &mut Digest) {
        if feat("avx") {
            let out = _mm256_testnzc_ps(float_input(n+0), float_input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_testnzc_si256(n: usize, h: &mut Digest) {
        if feat("avx2") {
            let out = _mm256_testnzc_si256(input(n+0), input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_testz_ps(n: usize, h: &mut Digest) {
        if feat("avx") {
            let out = _mm256_testz_ps(float_input(n+0), float_input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_undefined_ps(n: usize, h: &mut Digest) {
        if feat("avx2") {
            let out = _mm256_undefined_ps();
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_undefined_si256(n: usize, h: &mut Digest) {
        if feat("avx") {
            let out = _mm256_undefined_si256();
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_zextps128_ps256(n: usize, h: &mut Digest) {
        if feat("avx2") {
            let out = _mm256_zextps128_ps256(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_zextsi128_si256(n: usize, h: &mut Digest) {
        if feat("avx2") {
            let out = _mm256_zextsi128_si256(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_add_round_sd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_add_round_sd::<8>(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_add_round_ss(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_add_round_ss::<8>(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_addsub_ps(n: usize, h: &mut Digest) {
        if feat("sse") {
            let out = _mm_addsub_ps(float_input(n+0), float_input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_alignr_epi32(n: usize, h: &mut Digest) {
        if feat("avx2") {
            let out = _mm_alignr_epi32::<9>(input(n+0), input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_alignr_epi64(n: usize, h: &mut Digest) {
        if feat("avx2") {
            let out = _mm_alignr_epi64::<9>(input(n+0), input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_blend_epi32(n: usize, h: &mut Digest) {
        if feat("sse4.1") {
            let out = _mm_blend_epi32::<5>(input(n+0), input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_blend_ps(n: usize, h: &mut Digest) {
        if feat("sse4.1") {
            let out = _mm_blend_ps::<5>(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_broadcast_i32x2(n: usize, h: &mut Digest) {
        if feat("avx512vl") {
            let out = _mm_broadcast_i32x2(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_broadcast_ss(n: usize, h: &mut Digest) {
        if feat("avx") {
            let out = _mm_broadcast_ss(&float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_broadcastmb_epi64(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_broadcastmb_epi64(kbits(n) as __mmask8);
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_broadcastmw_epi32(n: usize, h: &mut Digest) {
        if feat("avx512cd") {
            let out = _mm_broadcastmw_epi32(kbits(n) as __mmask16);
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_broadcastsi128_si256(n: usize, h: &mut Digest) {
        if feat("avx") {
            let out = _mm_broadcastsi128_si256(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_broadcastss_ps(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_broadcastss_ps(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_bslli_si128(n: usize, h: &mut Digest) {
        if feat("sse2") {
            let out = _mm_bslli_si128::<5>(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_bsrli_si128(n: usize, h: &mut Digest) {
        if feat("sse2") {
            let out = _mm_bsrli_si128::<5>(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_ceil_ss(n: usize, h: &mut Digest) {
        if feat("sse4.1") {
            let out = _mm_ceil_ss(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cmp_ss(n: usize, h: &mut Digest) {
        if feat("sse") {
            let out = _mm_cmp_ss::<3>(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cmpeq_ss(n: usize, h: &mut Digest) {
        if feat("sse") {
            let out = _mm_cmpeq_ss(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cmpge_ss(n: usize, h: &mut Digest) {
        if feat("sse") {
            let out = _mm_cmpge_ss(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cmpgt_ss(n: usize, h: &mut Digest) {
        if feat("sse") {
            let out = _mm_cmpgt_ss(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cmple_ss(n: usize, h: &mut Digest) {
        if feat("sse") {
            let out = _mm_cmple_ss(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cmplt_ss(n: usize, h: &mut Digest) {
        if feat("sse") {
            let out = _mm_cmplt_ss(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cmpneq_ss(n: usize, h: &mut Digest) {
        if feat("sse") {
            let out = _mm_cmpneq_ss(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cmpnge_ps(n: usize, h: &mut Digest) {
        if feat("sse") {
            let out = _mm_cmpnge_ps(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cmpnge_ss(n: usize, h: &mut Digest) {
        if feat("sse") {
            let out = _mm_cmpnge_ss(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cmpngt_ps(n: usize, h: &mut Digest) {
        if feat("sse") {
            let out = _mm_cmpngt_ps(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cmpngt_ss(n: usize, h: &mut Digest) {
        if feat("sse") {
            let out = _mm_cmpngt_ss(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cmpnle_ps(n: usize, h: &mut Digest) {
        if feat("sse") {
            let out = _mm_cmpnle_ps(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cmpnle_ss(n: usize, h: &mut Digest) {
        if feat("sse") {
            let out = _mm_cmpnle_ss(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cmpnlt_ps(n: usize, h: &mut Digest) {
        if feat("sse") {
            let out = _mm_cmpnlt_ps(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cmpnlt_ss(n: usize, h: &mut Digest) {
        if feat("sse") {
            let out = _mm_cmpnlt_ss(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cmpord_ss(n: usize, h: &mut Digest) {
        if feat("sse") {
            let out = _mm_cmpord_ss(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cmpunord_ss(n: usize, h: &mut Digest) {
        if feat("sse") {
            let out = _mm_cmpunord_ss(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_comieq_ss(n: usize, h: &mut Digest) {
        if feat("sse") {
            let out = _mm_comieq_ss(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_comige_ss(n: usize, h: &mut Digest) {
        if feat("sse") {
            let out = _mm_comige_ss(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_comigt_ss(n: usize, h: &mut Digest) {
        if feat("sse") {
            let out = _mm_comigt_ss(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_comile_ss(n: usize, h: &mut Digest) {
        if feat("sse") {
            let out = _mm_comile_ss(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_comilt_ss(n: usize, h: &mut Digest) {
        if feat("sse") {
            let out = _mm_comilt_ss(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_conflict_epi32(n: usize, h: &mut Digest) {
        if feat("avx512cd") {
            let out = _mm_conflict_epi32(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_conflict_epi64(n: usize, h: &mut Digest) {
        if feat("avx512cd") {
            let out = _mm_conflict_epi64(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvt_roundi32_ss(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_cvt_roundi32_ss::<8>(float_input(n+0), (n*97) as i32);
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvt_roundi64_sd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_cvt_roundi64_sd::<8>(float_input(n+0), (n*97) as i64);
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvt_roundi64_ss(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_cvt_roundi64_ss::<8>(float_input(n+0), (n*97) as i64);
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvt_roundsd_i32(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_cvt_roundsd_i32::<8>(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvt_roundsd_i64(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_cvt_roundsd_i64::<8>(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvt_roundsd_si32(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_cvt_roundsd_si32::<4>(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvt_roundsd_si64(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_cvt_roundsd_si64::<4>(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvt_roundsd_ss(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_cvt_roundsd_ss::<8>(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvt_roundsd_u32(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_cvt_roundsd_u32::<8>(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvt_roundsd_u64(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_cvt_roundsd_u64::<8>(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvt_roundsi32_ss(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_cvt_roundsi32_ss::<8>(float_input(n+0), (n*97) as i32);
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvt_roundsi64_sd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_cvt_roundsi64_sd::<8>(float_input(n+0), (n*97) as i64);
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvt_roundsi64_ss(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_cvt_roundsi64_ss::<8>(float_input(n+0), (n*97) as i64);
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvt_roundss_i32(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_cvt_roundss_i32::<8>(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvt_roundss_i64(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_cvt_roundss_i64::<8>(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvt_roundss_sd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_cvt_roundss_sd::<8>(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvt_roundss_si32(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_cvt_roundss_si32::<4>(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvt_roundss_si64(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_cvt_roundss_si64::<4>(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvt_roundss_u32(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_cvt_roundss_u32::<8>(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvt_roundss_u64(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_cvt_roundss_u64::<8>(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvt_roundu32_ss(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_cvt_roundu32_ss::<8>(float_input(n+0), (n*97) as u32);
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvt_roundu64_sd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_cvt_roundu64_sd::<8>(float_input(n+0), (n*97) as u64);
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvt_roundu64_ss(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_cvt_roundu64_ss::<8>(float_input(n+0), (n*97) as u64);
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvt_si2ss(n: usize, h: &mut Digest) {
        if feat("sse") {
            let out = _mm_cvt_si2ss(float_input(n+0), (n*97) as i32);
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvt_ss2si(n: usize, h: &mut Digest) {
        if feat("sse") {
            let out = _mm_cvt_ss2si(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvtepi16_epi32(n: usize, h: &mut Digest) {
        if feat("sse2") {
            let out = _mm_cvtepi16_epi32(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvtepi16_epi64(n: usize, h: &mut Digest) {
        if feat("sse2") {
            let out = _mm_cvtepi16_epi64(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvtepi16_epi8(n: usize, h: &mut Digest) {
        if feat("sse2") {
            let out = _mm_cvtepi16_epi8(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvtepi32_epi16(n: usize, h: &mut Digest) {
        if feat("sse2") {
            let out = _mm_cvtepi32_epi16(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvtepi32_epi64(n: usize, h: &mut Digest) {
        if feat("sse2") {
            let out = _mm_cvtepi32_epi64(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvtepi32_epi8(n: usize, h: &mut Digest) {
        if feat("sse2") {
            let out = _mm_cvtepi32_epi8(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvtepi64_epi16(n: usize, h: &mut Digest) {
        if feat("sse2") {
            let out = _mm_cvtepi64_epi16(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvtepi64_epi32(n: usize, h: &mut Digest) {
        if feat("sse2") {
            let out = _mm_cvtepi64_epi32(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvtepi64_epi8(n: usize, h: &mut Digest) {
        if feat("sse2") {
            let out = _mm_cvtepi64_epi8(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvtepi64_pd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_cvtepi64_pd(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvtepi64_ps(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_cvtepi64_ps(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvtepi8_epi16(n: usize, h: &mut Digest) {
        if feat("sse2") {
            let out = _mm_cvtepi8_epi16(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvtepi8_epi32(n: usize, h: &mut Digest) {
        if feat("sse2") {
            let out = _mm_cvtepi8_epi32(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvtepi8_epi64(n: usize, h: &mut Digest) {
        if feat("sse2") {
            let out = _mm_cvtepi8_epi64(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvtepu16_epi32(n: usize, h: &mut Digest) {
        if feat("sse2") {
            let out = _mm_cvtepu16_epi32(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvtepu16_epi64(n: usize, h: &mut Digest) {
        if feat("sse2") {
            let out = _mm_cvtepu16_epi64(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvtepu32_epi64(n: usize, h: &mut Digest) {
        if feat("sse2") {
            let out = _mm_cvtepu32_epi64(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvtepu32_pd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_cvtepu32_pd(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvtepu64_pd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_cvtepu64_pd(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvtepu64_ps(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_cvtepu64_ps(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvtepu8_epi32(n: usize, h: &mut Digest) {
        if feat("sse2") {
            let out = _mm_cvtepu8_epi32(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvtepu8_epi64(n: usize, h: &mut Digest) {
        if feat("sse2") {
            let out = _mm_cvtepu8_epi64(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvti32_sd(n: usize, h: &mut Digest) {
        if feat("sse2") {
            let out = _mm_cvti32_sd(float_input(n+0), (n*97) as i32);
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvti32_ss(n: usize, h: &mut Digest) {
        if feat("sse") {
            let out = _mm_cvti32_ss(float_input(n+0), (n*97) as i32);
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvti64_sd(n: usize, h: &mut Digest) {
        if feat("sse2") {
            let out = _mm_cvti64_sd(float_input(n+0), (n*97) as i64);
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvti64_ss(n: usize, h: &mut Digest) {
        if feat("sse") {
            let out = _mm_cvti64_ss(float_input(n+0), (n*97) as i64);
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvtpd_epi64(n: usize, h: &mut Digest) {
        if feat("avx512vl") {
            let out = _mm_cvtpd_epi64(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvtpd_epu32(n: usize, h: &mut Digest) {
        if feat("avx512vl") {
            let out = _mm_cvtpd_epu32(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvtpd_epu64(n: usize, h: &mut Digest) {
        if feat("avx512vl") {
            let out = _mm_cvtpd_epu64(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvtps_epi64(n: usize, h: &mut Digest) {
        if feat("avx512vl") {
            let out = _mm_cvtps_epi64(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvtps_epu32(n: usize, h: &mut Digest) {
        if feat("avx512vl") {
            let out = _mm_cvtps_epu32(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvtps_epu64(n: usize, h: &mut Digest) {
        if feat("avx512vl") {
            let out = _mm_cvtps_epu64(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvtsd_i32(n: usize, h: &mut Digest) {
        if feat("sse2") {
            let out = _mm_cvtsd_i32(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvtsd_i64(n: usize, h: &mut Digest) {
        if feat("sse2") {
            let out = _mm_cvtsd_i64(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvtsd_u32(n: usize, h: &mut Digest) {
        if feat("sse2") {
            let out = _mm_cvtsd_u32(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvtsd_u64(n: usize, h: &mut Digest) {
        if feat("sse2") {
            let out = _mm_cvtsd_u64(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvtsepi16_epi8(n: usize, h: &mut Digest) {
        if feat("sse2") {
            let out = _mm_cvtsepi16_epi8(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvtsepi32_epi16(n: usize, h: &mut Digest) {
        if feat("sse2") {
            let out = _mm_cvtsepi32_epi16(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvtsepi32_epi8(n: usize, h: &mut Digest) {
        if feat("sse2") {
            let out = _mm_cvtsepi32_epi8(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvtsepi64_epi16(n: usize, h: &mut Digest) {
        if feat("sse2") {
            let out = _mm_cvtsepi64_epi16(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvtsepi64_epi32(n: usize, h: &mut Digest) {
        if feat("sse2") {
            let out = _mm_cvtsepi64_epi32(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvtsepi64_epi8(n: usize, h: &mut Digest) {
        if feat("sse2") {
            let out = _mm_cvtsepi64_epi8(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvtsi128_si16(n: usize, h: &mut Digest) {
        if feat("avx512fp16") {
            let out = _mm_cvtsi128_si16(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvtsi16_si128(n: usize, h: &mut Digest) {
        if feat("avx512fp16") {
            let out = _mm_cvtsi16_si128((n*37) as i16);
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvtsi32_ss(n: usize, h: &mut Digest) {
        if feat("sse") {
            let out = _mm_cvtsi32_ss(float_input(n+0), (n*97) as i32);
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvtsi64_ss(n: usize, h: &mut Digest) {
        if feat("sse") {
            let out = _mm_cvtsi64_ss(float_input(n+0), (n*97) as i64);
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvtss_i32(n: usize, h: &mut Digest) {
        if feat("sse") {
            let out = _mm_cvtss_i32(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvtss_i64(n: usize, h: &mut Digest) {
        if feat("sse") {
            let out = _mm_cvtss_i64(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvtss_si32(n: usize, h: &mut Digest) {
        if feat("sse") {
            let out = _mm_cvtss_si32(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvtss_si64(n: usize, h: &mut Digest) {
        if feat("sse") {
            let out = _mm_cvtss_si64(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvtss_u32(n: usize, h: &mut Digest) {
        if feat("sse") {
            let out = _mm_cvtss_u32(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvtss_u64(n: usize, h: &mut Digest) {
        if feat("sse") {
            let out = _mm_cvtss_u64(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvtt_roundsd_i32(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_cvtt_roundsd_i32::<8>(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvtt_roundsd_i64(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_cvtt_roundsd_i64::<8>(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvtt_roundsd_si32(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_cvtt_roundsd_si32::<4>(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvtt_roundsd_si64(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_cvtt_roundsd_si64::<4>(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvtt_roundsd_u32(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_cvtt_roundsd_u32::<8>(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvtt_roundsd_u64(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_cvtt_roundsd_u64::<8>(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvtt_roundss_i32(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_cvtt_roundss_i32::<8>(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvtt_roundss_i64(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_cvtt_roundss_i64::<8>(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvtt_roundss_si32(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_cvtt_roundss_si32::<4>(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvtt_roundss_si64(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_cvtt_roundss_si64::<4>(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvtt_roundss_u32(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_cvtt_roundss_u32::<8>(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvtt_roundss_u64(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_cvtt_roundss_u64::<8>(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvtt_ss2si(n: usize, h: &mut Digest) {
        if feat("sse") {
            let out = _mm_cvtt_ss2si(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvttpd_epi64(n: usize, h: &mut Digest) {
        if feat("avx512vl") {
            let out = _mm_cvttpd_epi64(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvttpd_epu32(n: usize, h: &mut Digest) {
        if feat("avx512vl") {
            let out = _mm_cvttpd_epu32(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvttpd_epu64(n: usize, h: &mut Digest) {
        if feat("avx512vl") {
            let out = _mm_cvttpd_epu64(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvttps_epi64(n: usize, h: &mut Digest) {
        if feat("avx512vl") {
            let out = _mm_cvttps_epi64(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvttps_epu32(n: usize, h: &mut Digest) {
        if feat("avx512vl") {
            let out = _mm_cvttps_epu32(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvttps_epu64(n: usize, h: &mut Digest) {
        if feat("avx512vl") {
            let out = _mm_cvttps_epu64(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvttsd_i32(n: usize, h: &mut Digest) {
        if feat("sse2") {
            let out = _mm_cvttsd_i32(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvttsd_i64(n: usize, h: &mut Digest) {
        if feat("sse2") {
            let out = _mm_cvttsd_i64(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvttsd_u32(n: usize, h: &mut Digest) {
        if feat("sse2") {
            let out = _mm_cvttsd_u32(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvttsd_u64(n: usize, h: &mut Digest) {
        if feat("sse2") {
            let out = _mm_cvttsd_u64(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvttss_i32(n: usize, h: &mut Digest) {
        if feat("sse") {
            let out = _mm_cvttss_i32(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvttss_i64(n: usize, h: &mut Digest) {
        if feat("sse") {
            let out = _mm_cvttss_i64(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvttss_si32(n: usize, h: &mut Digest) {
        if feat("sse") {
            let out = _mm_cvttss_si32(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvttss_si64(n: usize, h: &mut Digest) {
        if feat("sse") {
            let out = _mm_cvttss_si64(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvttss_u32(n: usize, h: &mut Digest) {
        if feat("sse") {
            let out = _mm_cvttss_u32(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvttss_u64(n: usize, h: &mut Digest) {
        if feat("sse") {
            let out = _mm_cvttss_u64(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvtu32_sd(n: usize, h: &mut Digest) {
        if feat("sse2") {
            let out = _mm_cvtu32_sd(float_input(n+0), (n*97) as u32);
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvtu32_ss(n: usize, h: &mut Digest) {
        if feat("sse") {
            let out = _mm_cvtu32_ss(float_input(n+0), (n*97) as u32);
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvtu64_sd(n: usize, h: &mut Digest) {
        if feat("sse2") {
            let out = _mm_cvtu64_sd(float_input(n+0), (n*97) as u64);
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvtu64_ss(n: usize, h: &mut Digest) {
        if feat("sse") {
            let out = _mm_cvtu64_ss(float_input(n+0), (n*97) as u64);
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvtusepi16_epi8(n: usize, h: &mut Digest) {
        if feat("avx512vl") {
            let out = _mm_cvtusepi16_epi8(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvtusepi32_epi16(n: usize, h: &mut Digest) {
        if feat("avx512vl") {
            let out = _mm_cvtusepi32_epi16(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvtusepi32_epi8(n: usize, h: &mut Digest) {
        if feat("avx512vl") {
            let out = _mm_cvtusepi32_epi8(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvtusepi64_epi16(n: usize, h: &mut Digest) {
        if feat("avx512vl") {
            let out = _mm_cvtusepi64_epi16(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvtusepi64_epi32(n: usize, h: &mut Digest) {
        if feat("avx512vl") {
            let out = _mm_cvtusepi64_epi32(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_cvtusepi64_epi8(n: usize, h: &mut Digest) {
        if feat("avx512vl") {
            let out = _mm_cvtusepi64_epi8(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_dbsad_epu8(n: usize, h: &mut Digest) {
        if feat("avx512bw") {
            let out = _mm_dbsad_epu8::<0xa5>(input(n+0), input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_div_round_sd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_div_round_sd::<8>(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_div_round_ss(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_div_round_ss::<8>(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_div_ss(n: usize, h: &mut Digest) {
        if feat("sse") {
            let out = _mm_div_ss(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_dp_ps(n: usize, h: &mut Digest) {
        if feat("sse4.1") {
            let out = _mm_dp_ps::<0xff>(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_dpbsud_epi32(n: usize, h: &mut Digest) {
        if feat("avxvnniint8") {
            let out = _mm_dpbsud_epi32(input(n+3), input(n+0), input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_dpbsuds_epi32(n: usize, h: &mut Digest) {
        if feat("avxvnniint8") {
            let out = _mm_dpbsuds_epi32(input(n+3), input(n+0), input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_dpbuud_epi32(n: usize, h: &mut Digest) {
        if feat("avxvnniint8") {
            let out = _mm_dpbuud_epi32(input(n+3), input(n+0), input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_dpbuuds_epi32(n: usize, h: &mut Digest) {
        if feat("avxvnniint8") {
            let out = _mm_dpbuuds_epi32(input(n+3), input(n+0), input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_dpwsud_epi32(n: usize, h: &mut Digest) {
        if feat("avxvnniint16") {
            let out = _mm_dpwsud_epi32(input(n+3), input(n+0), input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_dpwsuds_epi32(n: usize, h: &mut Digest) {
        if feat("avxvnniint16") {
            let out = _mm_dpwsuds_epi32(input(n+3), input(n+0), input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_dpwusd_epi32(n: usize, h: &mut Digest) {
        if feat("avxvnniint16") {
            let out = _mm_dpwusd_epi32(input(n+3), input(n+0), input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_dpwusds_epi32(n: usize, h: &mut Digest) {
        if feat("avxvnniint16") {
            let out = _mm_dpwusds_epi32(input(n+3), input(n+0), input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_dpwuud_epi32(n: usize, h: &mut Digest) {
        if feat("avxvnniint16") {
            let out = _mm_dpwuud_epi32(input(n+3), input(n+0), input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_dpwuuds_epi32(n: usize, h: &mut Digest) {
        if feat("avxvnniint16") {
            let out = _mm_dpwuuds_epi32(input(n+3), input(n+0), input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_extract_epi16(n: usize, h: &mut Digest) {
        if feat("sse2") {
            let out = _mm_extract_epi16::<2>(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_extract_epi8(n: usize, h: &mut Digest) {
        if feat("sse2") {
            let out = _mm_extract_epi8::<2>(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_extract_ps(n: usize, h: &mut Digest) {
        if feat("sse4.1") {
            let out = _mm_extract_ps::<2>(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_extract_si64(n: usize, h: &mut Digest) {
        if feat("sse4a") {
            let out = _mm_extract_si64(input(n+0), input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_fixupimm_pd(n: usize, h: &mut Digest) {
        if feat("avx512vl") {
            let out = _mm_fixupimm_pd::<0x1e>(float_input(n+0), float_input(n+1), input(n+2));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_fixupimm_ps(n: usize, h: &mut Digest) {
        if feat("avx512vl") {
            let out = _mm_fixupimm_ps::<0x1e>(float_input(n+0), float_input(n+1), input(n+2));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_fixupimm_round_sd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_fixupimm_round_sd::<9, 8>(float_input(n+0), float_input(n+1), input(n+2));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_fixupimm_round_ss(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_fixupimm_round_ss::<9, 8>(float_input(n+0), float_input(n+1), input(n+2));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_fixupimm_sd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_fixupimm_sd::<9>(float_input(n+0), float_input(n+1), input(n+2));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_fixupimm_ss(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_fixupimm_ss::<9>(float_input(n+0), float_input(n+1), input(n+2));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_floor_ss(n: usize, h: &mut Digest) {
        if feat("sse4.1") {
            let out = _mm_floor_ss(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_fmadd_round_sd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_fmadd_round_sd::<8>(float_input(n+0), float_input(n+1), float_input(n+2));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_fmadd_round_ss(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_fmadd_round_ss::<8>(float_input(n+0), float_input(n+1), float_input(n+2));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_fmadd_ss(n: usize, h: &mut Digest) {
        if feat("fma") {
            let out = _mm_fmadd_ss(float_input(n+0), float_input(n+1), float_input(n+2));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_fmsub_round_sd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_fmsub_round_sd::<8>(float_input(n+0), float_input(n+1), float_input(n+2));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_fmsub_round_ss(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_fmsub_round_ss::<8>(float_input(n+0), float_input(n+1), float_input(n+2));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_fmsub_ss(n: usize, h: &mut Digest) {
        if feat("fma") {
            let out = _mm_fmsub_ss(float_input(n+0), float_input(n+1), float_input(n+2));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_fnmadd_round_sd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_fnmadd_round_sd::<8>(float_input(n+0), float_input(n+1), float_input(n+2));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_fnmadd_round_ss(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_fnmadd_round_ss::<8>(float_input(n+0), float_input(n+1), float_input(n+2));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_fnmadd_ss(n: usize, h: &mut Digest) {
        if feat("fma") {
            let out = _mm_fnmadd_ss(float_input(n+0), float_input(n+1), float_input(n+2));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_fnmsub_round_sd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_fnmsub_round_sd::<8>(float_input(n+0), float_input(n+1), float_input(n+2));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_fnmsub_round_ss(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_fnmsub_round_ss::<8>(float_input(n+0), float_input(n+1), float_input(n+2));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_fnmsub_ss(n: usize, h: &mut Digest) {
        if feat("fma") {
            let out = _mm_fnmsub_ss(float_input(n+0), float_input(n+1), float_input(n+2));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_getexp_round_sd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_getexp_round_sd::<8>(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_getexp_round_ss(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_getexp_round_ss::<8>(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_getexp_sd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_getexp_sd(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_getexp_ss(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_getexp_ss(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_getmant_round_sd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_getmant_round_sd::<1, 0, 8>(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_getmant_round_ss(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_getmant_round_ss::<1, 0, 8>(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_getmant_sd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_getmant_sd::<1, 0>(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_getmant_ss(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_getmant_ss::<1, 0>(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_hadd_epi16(n: usize, h: &mut Digest) {
        if feat("sse2") {
            let out = _mm_hadd_epi16(input(n+0), input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_hadd_ps(n: usize, h: &mut Digest) {
        if feat("sse") {
            let out = _mm_hadd_ps(float_input(n+0), float_input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_hadds_epi16(n: usize, h: &mut Digest) {
        if feat("sse2") {
            let out = _mm_hadds_epi16(input(n+0), input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_hsub_epi16(n: usize, h: &mut Digest) {
        if feat("sse2") {
            let out = _mm_hsub_epi16(input(n+0), input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_hsub_epi32(n: usize, h: &mut Digest) {
        if feat("sse2") {
            let out = _mm_hsub_epi32(input(n+0), input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_hsub_ps(n: usize, h: &mut Digest) {
        if feat("sse") {
            let out = _mm_hsub_ps(float_input(n+0), float_input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_hsubs_epi16(n: usize, h: &mut Digest) {
        if feat("sse2") {
            let out = _mm_hsubs_epi16(input(n+0), input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_insert_epi16(n: usize, h: &mut Digest) {
        if feat("sse2") {
            let out = _mm_insert_epi16::<2>(input(n+0), (n%97) as i32);
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_insert_epi8(n: usize, h: &mut Digest) {
        if feat("sse2") {
            let out = _mm_insert_epi8::<2>(input(n+0), (n%97) as i32);
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_insert_ps(n: usize, h: &mut Digest) {
        if feat("sse4.1") {
            let out = _mm_insert_ps::<0x8a>(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_insert_si64(n: usize, h: &mut Digest) {
        if feat("sse4a") {
            let out = _mm_insert_si64(input(n+0), input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_lfence(n: usize, h: &mut Digest) {
        if feat("sse2") {
            let out = _mm_lfence();
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_load1_ps(n: usize, h: &mut Digest) {
        if feat("avx") {
            let out = _mm_load1_ps(loadptr(n).cast::<f32>());
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_load_ps1(n: usize, h: &mut Digest) {
        if feat("avx") {
            let out = _mm_load_ps1(loadptr(n).cast::<f32>());
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_load_ss(n: usize, h: &mut Digest) {
        if feat("sse") {
            let out = _mm_load_ss(loadptr(n).cast::<f32>());
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_loadr_ps(n: usize, h: &mut Digest) {
        if feat("sse") {
            let out = _mm_loadr_ps(loadptr(n).cast::<f32>());
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_loadu_si16(n: usize, h: &mut Digest) {
        if feat("sse2") {
            let out = _mm_loadu_si16(loadptr(n).cast::<u8>());
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_loadu_si32(n: usize, h: &mut Digest) {
        if feat("sse2") {
            let out = _mm_loadu_si32(loadptr(n).cast::<u8>());
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_loadu_si64(n: usize, h: &mut Digest) {
        if feat("sse2") {
            let out = _mm_loadu_si64(loadptr(n).cast::<u8>());
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_max_round_sd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_max_round_sd::<8>(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_max_round_ss(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_max_round_ss::<8>(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_max_ss(n: usize, h: &mut Digest) {
        if feat("sse") {
            let out = _mm_max_ss(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_min_round_sd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_min_round_sd::<8>(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_min_round_ss(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_min_round_ss::<8>(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_min_ss(n: usize, h: &mut Digest) {
        if feat("sse") {
            let out = _mm_min_ss(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_minpos_epu16(n: usize, h: &mut Digest) {
        if feat("sse4.1") {
            let out = _mm_minpos_epu16(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_move_ss(n: usize, h: &mut Digest) {
        if feat("sse") {
            let out = _mm_move_ss(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_moveldup_ps(n: usize, h: &mut Digest) {
        if feat("sse") {
            let out = _mm_moveldup_ps(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_movelh_ps(n: usize, h: &mut Digest) {
        if feat("sse") {
            let out = _mm_movelh_ps(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_mpsadbw_epu8(n: usize, h: &mut Digest) {
        if feat("sse4.1") {
            let out = _mm_mpsadbw_epu8::<0x05>(input(n+0), input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_mul_epi32(n: usize, h: &mut Digest) {
        if feat("sse2") {
            let out = _mm_mul_epi32(input(n+0), input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_mul_round_sd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_mul_round_sd::<8>(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_mul_round_ss(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_mul_round_ss::<8>(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_mul_ss(n: usize, h: &mut Digest) {
        if feat("sse") {
            let out = _mm_mul_ss(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_mulhi_epi16(n: usize, h: &mut Digest) {
        if feat("sse2") {
            let out = _mm_mulhi_epi16(input(n+0), input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_mulhrs_epi16(n: usize, h: &mut Digest) {
        if feat("sse2") {
            let out = _mm_mulhrs_epi16(input(n+0), input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_pause(n: usize, h: &mut Digest) {
        if feat("sse2") {
            let out = _mm_pause();
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_permute_ps(n: usize, h: &mut Digest) {
        if feat("avx") {
            let out = _mm_permute_ps::<0x1b>(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_permutevar_ps(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_permutevar_ps(float_input(n+0), input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_range_pd(n: usize, h: &mut Digest) {
        if feat("avx512dq") {
            let out = _mm_range_pd::<9>(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_range_ps(n: usize, h: &mut Digest) {
        if feat("avx512dq") {
            let out = _mm_range_ps::<9>(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_range_round_sd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_range_round_sd::<9, 8>(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_range_round_ss(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_range_round_ss::<9, 8>(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_rcp14_pd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_rcp14_pd(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_rcp14_ps(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_rcp14_ps(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_rcp14_sd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_rcp14_sd(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_rcp14_ss(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_rcp14_ss(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_rcp_ss(n: usize, h: &mut Digest) {
        if feat("sse") {
            let out = _mm_rcp_ss(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_reduce_add_epi16(n: usize, h: &mut Digest) {
        if feat("avx512bw") {
            let out = _mm_reduce_add_epi16(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_reduce_add_epi8(n: usize, h: &mut Digest) {
        if feat("avx512bw") {
            let out = _mm_reduce_add_epi8(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_reduce_and_epi16(n: usize, h: &mut Digest) {
        if feat("avx512bw") {
            let out = _mm_reduce_and_epi16(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_reduce_and_epi8(n: usize, h: &mut Digest) {
        if feat("avx512bw") {
            let out = _mm_reduce_and_epi8(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_reduce_max_epi16(n: usize, h: &mut Digest) {
        if feat("avx512bw") {
            let out = _mm_reduce_max_epi16(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_reduce_max_epi8(n: usize, h: &mut Digest) {
        if feat("avx512bw") {
            let out = _mm_reduce_max_epi8(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_reduce_max_epu16(n: usize, h: &mut Digest) {
        if feat("avx512bw") {
            let out = _mm_reduce_max_epu16(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_reduce_max_epu8(n: usize, h: &mut Digest) {
        if feat("avx512bw") {
            let out = _mm_reduce_max_epu8(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_reduce_min_epi16(n: usize, h: &mut Digest) {
        if feat("avx512bw") {
            let out = _mm_reduce_min_epi16(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_reduce_min_epi8(n: usize, h: &mut Digest) {
        if feat("avx512bw") {
            let out = _mm_reduce_min_epi8(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_reduce_min_epu16(n: usize, h: &mut Digest) {
        if feat("avx512bw") {
            let out = _mm_reduce_min_epu16(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_reduce_min_epu8(n: usize, h: &mut Digest) {
        if feat("avx512bw") {
            let out = _mm_reduce_min_epu8(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_reduce_mul_epi16(n: usize, h: &mut Digest) {
        if feat("avx512bw") {
            let out = _mm_reduce_mul_epi16(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_reduce_mul_epi8(n: usize, h: &mut Digest) {
        if feat("avx512bw") {
            let out = _mm_reduce_mul_epi8(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_reduce_or_epi16(n: usize, h: &mut Digest) {
        if feat("avx512bw") {
            let out = _mm_reduce_or_epi16(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_reduce_or_epi8(n: usize, h: &mut Digest) {
        if feat("avx512bw") {
            let out = _mm_reduce_or_epi8(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_reduce_pd(n: usize, h: &mut Digest) {
        if feat("avx512dq") {
            let out = _mm_reduce_pd::<0x19>(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_reduce_ps(n: usize, h: &mut Digest) {
        if feat("avx512dq") {
            let out = _mm_reduce_ps::<0x19>(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_reduce_round_sd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_reduce_round_sd::<0x19, 8>(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_reduce_round_ss(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_reduce_round_ss::<0x19, 8>(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_reduce_sd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_reduce_sd::<0x19>(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_reduce_ss(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_reduce_ss::<0x19>(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_round_ss(n: usize, h: &mut Digest) {
        if feat("sse4.1") {
            let out = _mm_round_ss::<9>(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_roundscale_pd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_roundscale_pd::<9>(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_roundscale_ps(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_roundscale_ps::<9>(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_roundscale_round_sd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_roundscale_round_sd::<0x19, 8>(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_roundscale_round_ss(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_roundscale_round_ss::<0x19, 8>(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_roundscale_sd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_roundscale_sd::<0x19>(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_roundscale_ss(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_roundscale_ss::<0x19>(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_rsqrt14_pd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_rsqrt14_pd(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_rsqrt14_ps(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_rsqrt14_ps(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_rsqrt14_sd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_rsqrt14_sd(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_rsqrt14_ss(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_rsqrt14_ss(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_rsqrt_ss(n: usize, h: &mut Digest) {
        if feat("sse") {
            let out = _mm_rsqrt_ss(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_scalef_round_sd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_scalef_round_sd::<8>(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_scalef_round_ss(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_scalef_round_ss::<8>(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_scalef_sd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_scalef_sd(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_scalef_ss(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_scalef_ss(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_sfence(n: usize, h: &mut Digest) {
        if feat("sse2") {
            let out = _mm_sfence();
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_sqrt_round_sd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_sqrt_round_sd::<8>(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_sqrt_round_ss(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_sqrt_round_ss::<8>(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_sqrt_ss(n: usize, h: &mut Digest) {
        if feat("sse") {
            let out = _mm_sqrt_ss(float_input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_store1_ps(n: usize, h: &mut Digest) {
        if feat("sse") {
            let out = _mm_store1_ps(storeptr(n).cast::<f32>(), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_store_ps1(n: usize, h: &mut Digest) {
        if feat("sse") {
            let out = _mm_store_ps1(storeptr(n).cast::<f32>(), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_store_ss(n: usize, h: &mut Digest) {
        if feat("sse") {
            let out = _mm_store_ss(storeptr(n).cast::<f32>(), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_storer_ps(n: usize, h: &mut Digest) {
        if feat("sse") {
            let out = _mm_storer_ps(storeptr(n).cast::<f32>(), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_storeu_si16(n: usize, h: &mut Digest) {
        if feat("sse2") {
            let out = _mm_storeu_si16(storeptr(n).cast::<u8>(), input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_storeu_si32(n: usize, h: &mut Digest) {
        if feat("sse2") {
            let out = _mm_storeu_si32(storeptr(n).cast::<u8>(), input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_storeu_si64(n: usize, h: &mut Digest) {
        if feat("sse2") {
            let out = _mm_storeu_si64(storeptr(n).cast::<u8>(), input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_stream_load_si128(n: usize, h: &mut Digest) {
        if feat("sse2") {
            let out = _mm_stream_load_si128(loadptr(n).cast::<__m128i>());
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_stream_sd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_stream_sd(storeptr(n).cast::<f64>(), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_stream_si32(n: usize, h: &mut Digest) {
        if feat("sse2") {
            let out = _mm_stream_si32(storeptr(n).cast::<i32>(), (n*7) as i32);
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_stream_si64(n: usize, h: &mut Digest) {
        if feat("sse2") {
            let out = _mm_stream_si64(storeptr(n).cast::<i64>(), (n*7) as i64);
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_stream_ss(n: usize, h: &mut Digest) {
        if feat("sse") {
            let out = _mm_stream_ss(storeptr(n).cast::<f32>(), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_sub_round_sd(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_sub_round_sd::<8>(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_sub_round_ss(n: usize, h: &mut Digest) {
        if feat("avx512f") {
            let out = _mm_sub_round_ss::<8>(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_sub_ss(n: usize, h: &mut Digest) {
        if feat("sse") {
            let out = _mm_sub_ss(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_test_all_ones(n: usize, h: &mut Digest) {
        if feat("sse4.1") {
            let out = _mm_test_all_ones(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_test_all_zeros(n: usize, h: &mut Digest) {
        if feat("sse4.1") {
            let out = _mm_test_all_zeros(input(n+0), input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_test_mix_ones_zeros(n: usize, h: &mut Digest) {
        if feat("sse4.1") {
            let out = _mm_test_mix_ones_zeros(input(n+0), input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_testc_ps(n: usize, h: &mut Digest) {
        if feat("sse") {
            let out = _mm_testc_ps(float_input(n+0), float_input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_testc_si128(n: usize, h: &mut Digest) {
        if feat("sse2") {
            let out = _mm_testc_si128(input(n+0), input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_testnzc_ps(n: usize, h: &mut Digest) {
        if feat("sse") {
            let out = _mm_testnzc_ps(float_input(n+0), float_input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_testnzc_si128(n: usize, h: &mut Digest) {
        if feat("sse2") {
            let out = _mm_testnzc_si128(input(n+0), input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_testz_ps(n: usize, h: &mut Digest) {
        if feat("sse") {
            let out = _mm_testz_ps(float_input(n+0), float_input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_tzcnt_32(n: usize, h: &mut Digest) {
        if feat("bmi1") {
            let out = _mm_tzcnt_32((n*2654435761) as u32);
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_tzcnt_64(n: usize, h: &mut Digest) {
        if feat("bmi1") {
            let out = _mm_tzcnt_64((n*2654435761) as u64);
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_ucomieq_ss(n: usize, h: &mut Digest) {
        if feat("sse") {
            let out = _mm_ucomieq_ss(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_ucomige_ss(n: usize, h: &mut Digest) {
        if feat("sse") {
            let out = _mm_ucomige_ss(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_ucomigt_ss(n: usize, h: &mut Digest) {
        if feat("sse") {
            let out = _mm_ucomigt_ss(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_ucomile_ss(n: usize, h: &mut Digest) {
        if feat("sse") {
            let out = _mm_ucomile_ss(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_ucomilt_ss(n: usize, h: &mut Digest) {
        if feat("sse") {
            let out = _mm_ucomilt_ss(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm_ucomineq_ss(n: usize, h: &mut Digest) {
        if feat("sse") {
            let out = _mm_ucomineq_ss(float_input(n+0), float_input(n+1));
            mix(h, &out);
        }
    }
#[test]
fn a0001_lzcnt_32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_lzcnt_32(core::hint::black_box(n), &mut d); } }
    d.line("_lzcnt_u32", "none");
}

#[test]
fn a0002_tzcnt_32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_tzcnt_32(core::hint::black_box(n), &mut d); } }
    d.line("_tzcnt_u32", "none");
}

#[test]
fn a0003_andn_32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_andn_32(core::hint::black_box(n), &mut d); } }
    d.line("_andn_u32", "none");
}

#[test]
fn a0004_blsr_32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_blsr_32(core::hint::black_box(n), &mut d); } }
    d.line("_blsr_u32", "none");
}

#[test]
fn a0005_blsi_32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_blsi_32(core::hint::black_box(n), &mut d); } }
    d.line("_blsi_u32", "none");
}

#[test]
fn a0006_blsmsk_32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_blsmsk_32(core::hint::black_box(n), &mut d); } }
    d.line("_blsmsk_u32", "none");
}

#[test]
fn a0007_blsfill_32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_blsfill_32(core::hint::black_box(n), &mut d); } }
    d.line("_blsfill_u32", "none");
}

#[test]
fn a0008_blci_32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_blci_32(core::hint::black_box(n), &mut d); } }
    d.line("_blci_u32", "none");
}

#[test]
fn a0009_blcic_32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_blcic_32(core::hint::black_box(n), &mut d); } }
    d.line("_blcic_u32", "none");
}

#[test]
fn a0010_blcmsk_32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_blcmsk_32(core::hint::black_box(n), &mut d); } }
    d.line("_blcmsk_u32", "none");
}

#[test]
fn a0011_blcfill_32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_blcfill_32(core::hint::black_box(n), &mut d); } }
    d.line("_blcfill_u32", "none");
}

#[test]
fn a0012_blcs_32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_blcs_32(core::hint::black_box(n), &mut d); } }
    d.line("_blcs_u32", "none");
}

#[test]
fn a0013_t1mskc_32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_t1mskc_32(core::hint::black_box(n), &mut d); } }
    d.line("_t1mskc_u32", "none");
}

#[test]
fn a0014_tzmsk_32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_tzmsk_32(core::hint::black_box(n), &mut d); } }
    d.line("_tzmsk_u32", "none");
}

#[test]
fn a0015_bzhi_32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_bzhi_32(core::hint::black_box(n), &mut d); } }
    d.line("_bzhi_u32", "none");
}

#[test]
fn a0016_bextr2_32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_bextr2_32(core::hint::black_box(n), &mut d); } }
    d.line("_bextr2_u32", "none");
}

#[test]
fn a0017_bextr_32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_bextr_32(core::hint::black_box(n), &mut d); } }
    d.line("_bextr_u32", "none");
}

#[test]
fn a0018_bextri_32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_bextri_32(core::hint::black_box(n), &mut d); } }
    d.line("_bextri_u32", "none");
}

#[test]
fn a0019_pdep_32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_pdep_32(core::hint::black_box(n), &mut d); } }
    d.line("_pdep_u32", "none");
}

#[test]
fn a0020_pext_32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_pext_32(core::hint::black_box(n), &mut d); } }
    d.line("_pext_u32", "none");
}

#[test]
fn a0021_mulx_32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mulx_32(core::hint::black_box(n), &mut d); } }
    d.line("_mulx_u32", "none");
}

#[test]
fn a0022_addcarry_32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_addcarry_32(core::hint::black_box(n), &mut d); } }
    d.line("_addcarry_u32", "none");
}

#[test]
fn a0023_addcarryx_32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_addcarryx_32(core::hint::black_box(n), &mut d); } }
    d.line("_addcarryx_u32", "none");
}

#[test]
fn a0024_subborrow_32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_subborrow_32(core::hint::black_box(n), &mut d); } }
    d.line("_subborrow_u32", "none");
}

#[test]
fn a0025_lzcnt_64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_lzcnt_64(core::hint::black_box(n), &mut d); } }
    d.line("_lzcnt_u64", "none");
}

#[test]
fn a0026_tzcnt_64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_tzcnt_64(core::hint::black_box(n), &mut d); } }
    d.line("_tzcnt_u64", "none");
}

#[test]
fn a0027_andn_64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_andn_64(core::hint::black_box(n), &mut d); } }
    d.line("_andn_u64", "none");
}

#[test]
fn a0028_blsr_64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_blsr_64(core::hint::black_box(n), &mut d); } }
    d.line("_blsr_u64", "none");
}

#[test]
fn a0029_blsi_64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_blsi_64(core::hint::black_box(n), &mut d); } }
    d.line("_blsi_u64", "none");
}

#[test]
fn a0030_blsmsk_64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_blsmsk_64(core::hint::black_box(n), &mut d); } }
    d.line("_blsmsk_u64", "none");
}

#[test]
fn a0031_blsfill_64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_blsfill_64(core::hint::black_box(n), &mut d); } }
    d.line("_blsfill_u64", "none");
}

#[test]
fn a0032_blci_64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_blci_64(core::hint::black_box(n), &mut d); } }
    d.line("_blci_u64", "none");
}

#[test]
fn a0033_blcic_64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_blcic_64(core::hint::black_box(n), &mut d); } }
    d.line("_blcic_u64", "none");
}

#[test]
fn a0034_blcmsk_64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_blcmsk_64(core::hint::black_box(n), &mut d); } }
    d.line("_blcmsk_u64", "none");
}

#[test]
fn a0035_blcfill_64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_blcfill_64(core::hint::black_box(n), &mut d); } }
    d.line("_blcfill_u64", "none");
}

#[test]
fn a0036_blcs_64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_blcs_64(core::hint::black_box(n), &mut d); } }
    d.line("_blcs_u64", "none");
}

#[test]
fn a0037_t1mskc_64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_t1mskc_64(core::hint::black_box(n), &mut d); } }
    d.line("_t1mskc_u64", "none");
}

#[test]
fn a0038_tzmsk_64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_tzmsk_64(core::hint::black_box(n), &mut d); } }
    d.line("_tzmsk_u64", "none");
}

#[test]
fn a0039_bzhi_64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_bzhi_64(core::hint::black_box(n), &mut d); } }
    d.line("_bzhi_u64", "none");
}

#[test]
fn a0040_bextr2_64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_bextr2_64(core::hint::black_box(n), &mut d); } }
    d.line("_bextr2_u64", "none");
}

#[test]
fn a0041_bextr_64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_bextr_64(core::hint::black_box(n), &mut d); } }
    d.line("_bextr_u64", "none");
}

#[test]
fn a0042_bextri_64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_bextri_64(core::hint::black_box(n), &mut d); } }
    d.line("_bextri_u64", "none");
}

#[test]
fn a0043_pdep_64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_pdep_64(core::hint::black_box(n), &mut d); } }
    d.line("_pdep_u64", "none");
}

#[test]
fn a0044_pext_64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_pext_64(core::hint::black_box(n), &mut d); } }
    d.line("_pext_u64", "none");
}

#[test]
fn a0045_mulx_64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mulx_64(core::hint::black_box(n), &mut d); } }
    d.line("_mulx_u64", "none");
}

#[test]
fn a0046_addcarry_64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_addcarry_64(core::hint::black_box(n), &mut d); } }
    d.line("_addcarry_u64", "none");
}

#[test]
fn a0047_addcarryx_64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_addcarryx_64(core::hint::black_box(n), &mut d); } }
    d.line("_addcarryx_u64", "none");
}

#[test]
fn a0048_subborrow_64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_subborrow_64(core::hint::black_box(n), &mut d); } }
    d.line("_subborrow_u64", "none");
}

#[test]
fn a0049_tzcnt_u16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_tzcnt_u16(core::hint::black_box(n), &mut d); } }
    d.line("_tzcnt_u16", "none");
}

#[test]
fn a0050_popcnt32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_popcnt32(core::hint::black_box(n), &mut d); } }
    d.line("_popcnt32", "none");
}

#[test]
fn a0051_popcnt64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_popcnt64(core::hint::black_box(n), &mut d); } }
    d.line("_popcnt64", "none");
}

#[test]
fn a0052_bswap() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_bswap(core::hint::black_box(n), &mut d); } }
    d.line("_bswap", "none");
}

#[test]
fn a0053_bswap64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_bswap64(core::hint::black_box(n), &mut d); } }
    d.line("_bswap64", "none");
}

#[test]
fn a0054_bittest() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_bittest(core::hint::black_box(n), &mut d); } }
    d.line("_bittest", "none");
}

#[test]
fn a0055_bittestandset() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_bittestandset(core::hint::black_box(n), &mut d); } }
    d.line("_bittestandset", "none");
}

#[test]
fn a0056_bittestandreset() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_bittestandreset(core::hint::black_box(n), &mut d); } }
    d.line("_bittestandreset", "none");
}

#[test]
fn a0057_bittestandcomplement() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_bittestandcomplement(core::hint::black_box(n), &mut d); } }
    d.line("_bittestandcomplement", "none");
}

#[test]
fn a0058_bittest64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_bittest64(core::hint::black_box(n), &mut d); } }
    d.line("_bittest64", "none");
}

#[test]
fn a0059_bittestandset64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_bittestandset64(core::hint::black_box(n), &mut d); } }
    d.line("_bittestandset64", "none");
}

#[test]
fn a0060_bittestandreset64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_bittestandreset64(core::hint::black_box(n), &mut d); } }
    d.line("_bittestandreset64", "none");
}

#[test]
fn a0061_bittestandcomplement64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_bittestandcomplement64(core::hint::black_box(n), &mut d); } }
    d.line("_bittestandcomplement64", "none");
}

#[test]
fn a0062_rdtsc() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_rdtsc(core::hint::black_box(n), &mut d); } }
    eprintln!("_rdtsc: nondeterministic; output is timing-dependent");
}

#[test]
fn a0063_mm_comineq_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_comineq_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_comineq_ss", "none");
}

#[test]
fn a0064_mm_comi_round_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_comi_round_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_comi_round_ss", "none");
}

#[test]
fn a0065_mm_comi_round_sd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_comi_round_sd(core::hint::black_box(n), &mut d); } }
    d.line("_mm_comi_round_sd", "none");
}

#[test]
fn a0066_mm_extracti_si64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_extracti_si64(core::hint::black_box(n), &mut d); } }
    d.line("_mm_extracti_si64", "none");
}

#[test]
fn a0067_mm_inserti_si64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_inserti_si64(core::hint::black_box(n), &mut d); } }
    d.line("_mm_inserti_si64", "none");
}

#[test]
fn a0068_mm_cmpestra() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cmpestra(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cmpestra", "none");
}

#[test]
fn a0069_mm_cmpestro() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cmpestro(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cmpestro", "none");
}

#[test]
fn a0070_mm_cmpestrc() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cmpestrc(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cmpestrc", "none");
}

#[test]
fn a0071_mm_cmpestrs() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cmpestrs(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cmpestrs", "none");
}

#[test]
fn a0072_mm_cmpestrz() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cmpestrz(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cmpestrz", "none");
}

#[test]
fn a0073_mm_cmpestri() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cmpestri(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cmpestri", "none");
}

#[test]
fn a0074_mm_cmpestrm() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cmpestrm(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cmpestrm", "none");
}

#[test]
fn a0075_mm_cmpistra() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cmpistra(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cmpistra", "none");
}

#[test]
fn a0076_mm_cmpistro() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cmpistro(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cmpistro", "none");
}

#[test]
fn a0077_mm_cmpistrc() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cmpistrc(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cmpistrc", "none");
}

#[test]
fn a0078_mm_cmpistrs() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cmpistrs(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cmpistrs", "none");
}

#[test]
fn a0079_mm_cmpistrz() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cmpistrz(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cmpistrz", "none");
}

#[test]
fn a0080_mm_cmpistri() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cmpistri(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cmpistri", "none");
}

#[test]
fn a0081_mm_cmpistrm() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cmpistrm(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cmpistrm", "none");
}

#[test]
fn a0082_mm_abs_epi16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_abs_epi16(core::hint::black_box(n), &mut d); } }
    d.line("_mm_abs_epi16", "none");
}

#[test]
fn a0083_mm_add_epi16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_add_epi16(core::hint::black_box(n), &mut d); } }
    d.line("_mm_add_epi16", "none");
}

#[test]
fn a0084_mm_add_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_add_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_add_epi32", "none");
}

#[test]
fn a0085_mm_add_epi8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_add_epi8(core::hint::black_box(n), &mut d); } }
    d.line("_mm_add_epi8", "none");
}

#[test]
fn a0086_mm_and_si128() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_and_si128(core::hint::black_box(n), &mut d); } }
    d.line("_mm_and_si128", "none");
}

#[test]
fn a0087_mm_andnot_si128() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_andnot_si128(core::hint::black_box(n), &mut d); } }
    d.line("_mm_andnot_si128", "none");
}

#[test]
fn a0088_mm_avg_epu8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_avg_epu8(core::hint::black_box(n), &mut d); } }
    d.line("_mm_avg_epu8", "none");
}

#[test]
fn a0089_mm_blendv_epi8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_blendv_epi8(core::hint::black_box(n), &mut d); } }
    d.line("_mm_blendv_epi8", "none");
}

#[test]
fn a0090_mm_cmpeq_epi8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cmpeq_epi8(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cmpeq_epi8", "none");
}

#[test]
fn a0091_mm_cmpeq_epi16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cmpeq_epi16(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cmpeq_epi16", "none");
}

#[test]
fn a0092_mm_cmpgt_epi8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cmpgt_epi8(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cmpgt_epi8", "none");
}

#[test]
fn a0093_mm_load_si128() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_load_si128(core::hint::black_box(n), &mut d); } }
    d.line("_mm_load_si128", "none");
}

#[test]
fn a0094_mm_loadu_si128() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_loadu_si128(core::hint::black_box(n), &mut d); } }
    d.line("_mm_loadu_si128", "none");
}

#[test]
fn a0095_mm_madd_epi16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_madd_epi16(core::hint::black_box(n), &mut d); } }
    d.line("_mm_madd_epi16", "none");
}

#[test]
fn a0096_mm_maddubs_epi16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_maddubs_epi16(core::hint::black_box(n), &mut d); } }
    d.line("_mm_maddubs_epi16", "none");
}

#[test]
fn a0097_mm_min_epi16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_min_epi16(core::hint::black_box(n), &mut d); } }
    d.line("_mm_min_epi16", "none");
}

#[test]
fn a0098_mm_or_si128() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_or_si128(core::hint::black_box(n), &mut d); } }
    d.line("_mm_or_si128", "none");
}

#[test]
fn a0099_mm_packus_epi16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_packus_epi16(core::hint::black_box(n), &mut d); } }
    d.line("_mm_packus_epi16", "none");
}

#[test]
fn a0100_mm_sad_epu8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_sad_epu8(core::hint::black_box(n), &mut d); } }
    d.line("_mm_sad_epu8", "none");
}

#[test]
fn a0101_mm_set1_epi16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_set1_epi16(core::hint::black_box(n), &mut d); } }
    d.line("_mm_set1_epi16", "none");
}

#[test]
fn a0102_mm_set1_epi8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_set1_epi8(core::hint::black_box(n), &mut d); } }
    d.line("_mm_set1_epi8", "none");
}

#[test]
fn a0103_mm_setzero_si128() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_setzero_si128(core::hint::black_box(n), &mut d); } }
    d.line("_mm_setzero_si128", "none");
}

#[test]
fn a0104_mm_shuffle_epi32_0() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_shuffle_epi32_0(core::hint::black_box(n), &mut d); } }
    d.line("_mm_shuffle_epi32_0", "none");
}

#[test]
fn a0105_mm_shuffle_epi32_1() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_shuffle_epi32_1(core::hint::black_box(n), &mut d); } }
    d.line("_mm_shuffle_epi32_1", "none");
}

#[test]
fn a0106_mm_shuffle_epi32_7() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_shuffle_epi32_7(core::hint::black_box(n), &mut d); } }
    d.line("_mm_shuffle_epi32_7", "none");
}

#[test]
fn a0107_mm_shuffle_epi32_15() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_shuffle_epi32_15(core::hint::black_box(n), &mut d); } }
    d.line("_mm_shuffle_epi32_15", "none");
}

#[test]
fn a0108_mm_shuffle_epi32_16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_shuffle_epi32_16(core::hint::black_box(n), &mut d); } }
    d.line("_mm_shuffle_epi32_16", "none");
}

#[test]
fn a0109_mm_shuffle_epi32_31() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_shuffle_epi32_31(core::hint::black_box(n), &mut d); } }
    d.line("_mm_shuffle_epi32_31", "none");
}

#[test]
fn a0110_mm_shuffle_epi32_32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_shuffle_epi32_32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_shuffle_epi32_32", "none");
}

#[test]
fn a0111_mm_shuffle_epi32_63() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_shuffle_epi32_63(core::hint::black_box(n), &mut d); } }
    d.line("_mm_shuffle_epi32_63", "none");
}

#[test]
fn a0112_mm_shuffle_epi32_64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_shuffle_epi32_64(core::hint::black_box(n), &mut d); } }
    d.line("_mm_shuffle_epi32_64", "none");
}

#[test]
fn a0113_mm_shuffle_epi32_255() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_shuffle_epi32_255(core::hint::black_box(n), &mut d); } }
    d.line("_mm_shuffle_epi32_255", "none");
}

#[test]
fn a0114_mm_shuffle_epi8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_shuffle_epi8(core::hint::black_box(n), &mut d); } }
    d.line("_mm_shuffle_epi8", "none");
}

#[test]
fn a0115_mm_slli_epi32_0() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_slli_epi32_0(core::hint::black_box(n), &mut d); } }
    d.line("_mm_slli_epi32_0", "none");
}

#[test]
fn a0116_mm_slli_epi32_1() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_slli_epi32_1(core::hint::black_box(n), &mut d); } }
    d.line("_mm_slli_epi32_1", "none");
}

#[test]
fn a0117_mm_slli_epi32_7() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_slli_epi32_7(core::hint::black_box(n), &mut d); } }
    d.line("_mm_slli_epi32_7", "none");
}

#[test]
fn a0118_mm_slli_epi32_15() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_slli_epi32_15(core::hint::black_box(n), &mut d); } }
    d.line("_mm_slli_epi32_15", "none");
}

#[test]
fn a0119_mm_slli_epi32_16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_slli_epi32_16(core::hint::black_box(n), &mut d); } }
    d.line("_mm_slli_epi32_16", "none");
}

#[test]
fn a0120_mm_slli_epi32_31() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_slli_epi32_31(core::hint::black_box(n), &mut d); } }
    d.line("_mm_slli_epi32_31", "none");
}

#[test]
fn a0121_mm_slli_epi32_32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_slli_epi32_32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_slli_epi32_32", "none");
}

#[test]
fn a0122_mm_slli_epi32_63() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_slli_epi32_63(core::hint::black_box(n), &mut d); } }
    d.line("_mm_slli_epi32_63", "none");
}

#[test]
fn a0123_mm_slli_epi32_64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_slli_epi32_64(core::hint::black_box(n), &mut d); } }
    d.line("_mm_slli_epi32_64", "none");
}

#[test]
fn a0124_mm_slli_epi32_255() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_slli_epi32_255(core::hint::black_box(n), &mut d); } }
    d.line("_mm_slli_epi32_255", "none");
}

#[test]
fn a0125_mm_srli_si128_0() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_srli_si128_0(core::hint::black_box(n), &mut d); } }
    d.line("_mm_srli_si128_0", "none");
}

#[test]
fn a0126_mm_srli_si128_1() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_srli_si128_1(core::hint::black_box(n), &mut d); } }
    d.line("_mm_srli_si128_1", "none");
}

#[test]
fn a0127_mm_srli_si128_7() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_srli_si128_7(core::hint::black_box(n), &mut d); } }
    d.line("_mm_srli_si128_7", "none");
}

#[test]
fn a0128_mm_srli_si128_15() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_srli_si128_15(core::hint::black_box(n), &mut d); } }
    d.line("_mm_srli_si128_15", "none");
}

#[test]
fn a0129_mm_srli_si128_16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_srli_si128_16(core::hint::black_box(n), &mut d); } }
    d.line("_mm_srli_si128_16", "none");
}

#[test]
fn a0130_mm_srli_si128_31() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_srli_si128_31(core::hint::black_box(n), &mut d); } }
    d.line("_mm_srli_si128_31", "none");
}

#[test]
fn a0131_mm_srli_si128_32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_srli_si128_32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_srli_si128_32", "none");
}

#[test]
fn a0132_mm_srli_si128_63() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_srli_si128_63(core::hint::black_box(n), &mut d); } }
    d.line("_mm_srli_si128_63", "none");
}

#[test]
fn a0133_mm_srli_si128_64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_srli_si128_64(core::hint::black_box(n), &mut d); } }
    d.line("_mm_srli_si128_64", "none");
}

#[test]
fn a0134_mm_srli_si128_255() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_srli_si128_255(core::hint::black_box(n), &mut d); } }
    d.line("_mm_srli_si128_255", "none");
}

#[test]
fn a0135_mm_store_si128() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_store_si128(core::hint::black_box(n), &mut d); } }
    d.line("_mm_store_si128", "none");
}

#[test]
fn a0136_mm_storeu_si128() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_storeu_si128(core::hint::black_box(n), &mut d); } }
    d.line("_mm_storeu_si128", "none");
}

#[test]
fn a0137_mm_sub_epi16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_sub_epi16(core::hint::black_box(n), &mut d); } }
    d.line("_mm_sub_epi16", "none");
}

#[test]
fn a0138_mm_sub_epi8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_sub_epi8(core::hint::black_box(n), &mut d); } }
    d.line("_mm_sub_epi8", "none");
}

#[test]
fn a0139_mm_unpackhi_epi64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_unpackhi_epi64(core::hint::black_box(n), &mut d); } }
    d.line("_mm_unpackhi_epi64", "none");
}

#[test]
fn a0140_mm_unpackhi_epi8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_unpackhi_epi8(core::hint::black_box(n), &mut d); } }
    d.line("_mm_unpackhi_epi8", "none");
}

#[test]
fn a0141_mm_unpacklo_epi8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_unpacklo_epi8(core::hint::black_box(n), &mut d); } }
    d.line("_mm_unpacklo_epi8", "none");
}

#[test]
fn a0142_mm_xor_si128() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_xor_si128(core::hint::black_box(n), &mut d); } }
    d.line("_mm_xor_si128", "none");
}

#[test]
fn a0143_mm_add_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_add_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm_add_ps", "none");
}

#[test]
fn a0144_mm_and_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_and_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm_and_ps", "none");
}

#[test]
fn a0145_mm_andnot_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_andnot_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm_andnot_ps", "none");
}

#[test]
fn a0146_mm_cmpeq_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cmpeq_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cmpeq_epi32", "none");
}

#[test]
fn a0147_mm_cmpeq_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cmpeq_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cmpeq_ps", "none");
}

#[test]
fn a0148_mm_cmpge_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cmpge_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cmpge_ps", "none");
}

#[test]
fn a0149_mm_cmpgt_epi16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cmpgt_epi16(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cmpgt_epi16", "none");
}

#[test]
fn a0150_mm_cmpgt_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cmpgt_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cmpgt_epi32", "none");
}

#[test]
fn a0151_mm_cmpgt_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cmpgt_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cmpgt_ps", "none");
}

#[test]
fn a0152_mm_cmple_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cmple_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cmple_ps", "none");
}

#[test]
fn a0153_mm_cmplt_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cmplt_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cmplt_ps", "none");
}

#[test]
fn a0154_mm_div_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_div_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm_div_ps", "none");
}

#[test]
fn a0155_mm_loadu_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_loadu_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm_loadu_ps", "none");
}

#[test]
fn a0156_mm_max_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_max_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm_max_ps", "none");
}

#[test]
fn a0157_mm_min_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_min_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm_min_ps", "none");
}

#[test]
fn a0158_mm_mul_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_mul_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm_mul_ps", "none");
}

#[test]
fn a0159_mm_or_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_or_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm_or_ps", "none");
}

#[test]
fn a0160_mm_set1_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_set1_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm_set1_ps", "none");
}

#[test]
fn a0161_mm_slli_epi16_0() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_slli_epi16_0(core::hint::black_box(n), &mut d); } }
    d.line("_mm_slli_epi16_0", "none");
}

#[test]
fn a0162_mm_slli_epi16_1() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_slli_epi16_1(core::hint::black_box(n), &mut d); } }
    d.line("_mm_slli_epi16_1", "none");
}

#[test]
fn a0163_mm_slli_epi16_7() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_slli_epi16_7(core::hint::black_box(n), &mut d); } }
    d.line("_mm_slli_epi16_7", "none");
}

#[test]
fn a0164_mm_slli_epi16_15() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_slli_epi16_15(core::hint::black_box(n), &mut d); } }
    d.line("_mm_slli_epi16_15", "none");
}

#[test]
fn a0165_mm_slli_epi16_16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_slli_epi16_16(core::hint::black_box(n), &mut d); } }
    d.line("_mm_slli_epi16_16", "none");
}

#[test]
fn a0166_mm_slli_epi16_31() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_slli_epi16_31(core::hint::black_box(n), &mut d); } }
    d.line("_mm_slli_epi16_31", "none");
}

#[test]
fn a0167_mm_slli_epi16_32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_slli_epi16_32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_slli_epi16_32", "none");
}

#[test]
fn a0168_mm_slli_epi16_63() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_slli_epi16_63(core::hint::black_box(n), &mut d); } }
    d.line("_mm_slli_epi16_63", "none");
}

#[test]
fn a0169_mm_slli_epi16_64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_slli_epi16_64(core::hint::black_box(n), &mut d); } }
    d.line("_mm_slli_epi16_64", "none");
}

#[test]
fn a0170_mm_slli_epi16_255() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_slli_epi16_255(core::hint::black_box(n), &mut d); } }
    d.line("_mm_slli_epi16_255", "none");
}

#[test]
fn a0171_mm_slli_epi64_0() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_slli_epi64_0(core::hint::black_box(n), &mut d); } }
    d.line("_mm_slli_epi64_0", "none");
}

#[test]
fn a0172_mm_slli_epi64_1() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_slli_epi64_1(core::hint::black_box(n), &mut d); } }
    d.line("_mm_slli_epi64_1", "none");
}

#[test]
fn a0173_mm_slli_epi64_7() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_slli_epi64_7(core::hint::black_box(n), &mut d); } }
    d.line("_mm_slli_epi64_7", "none");
}

#[test]
fn a0174_mm_slli_epi64_15() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_slli_epi64_15(core::hint::black_box(n), &mut d); } }
    d.line("_mm_slli_epi64_15", "none");
}

#[test]
fn a0175_mm_slli_epi64_16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_slli_epi64_16(core::hint::black_box(n), &mut d); } }
    d.line("_mm_slli_epi64_16", "none");
}

#[test]
fn a0176_mm_slli_epi64_31() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_slli_epi64_31(core::hint::black_box(n), &mut d); } }
    d.line("_mm_slli_epi64_31", "none");
}

#[test]
fn a0177_mm_slli_epi64_32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_slli_epi64_32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_slli_epi64_32", "none");
}

#[test]
fn a0178_mm_slli_epi64_63() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_slli_epi64_63(core::hint::black_box(n), &mut d); } }
    d.line("_mm_slli_epi64_63", "none");
}

#[test]
fn a0179_mm_slli_epi64_64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_slli_epi64_64(core::hint::black_box(n), &mut d); } }
    d.line("_mm_slli_epi64_64", "none");
}

#[test]
fn a0180_mm_slli_epi64_255() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_slli_epi64_255(core::hint::black_box(n), &mut d); } }
    d.line("_mm_slli_epi64_255", "none");
}

#[test]
fn a0181_mm_srai_epi32_0() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_srai_epi32_0(core::hint::black_box(n), &mut d); } }
    d.line("_mm_srai_epi32_0", "none");
}

#[test]
fn a0182_mm_srai_epi32_1() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_srai_epi32_1(core::hint::black_box(n), &mut d); } }
    d.line("_mm_srai_epi32_1", "none");
}

#[test]
fn a0183_mm_srai_epi32_7() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_srai_epi32_7(core::hint::black_box(n), &mut d); } }
    d.line("_mm_srai_epi32_7", "none");
}

#[test]
fn a0184_mm_srai_epi32_15() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_srai_epi32_15(core::hint::black_box(n), &mut d); } }
    d.line("_mm_srai_epi32_15", "none");
}

#[test]
fn a0185_mm_srai_epi32_16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_srai_epi32_16(core::hint::black_box(n), &mut d); } }
    d.line("_mm_srai_epi32_16", "none");
}

#[test]
fn a0186_mm_srai_epi32_31() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_srai_epi32_31(core::hint::black_box(n), &mut d); } }
    d.line("_mm_srai_epi32_31", "none");
}

#[test]
fn a0187_mm_srai_epi32_32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_srai_epi32_32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_srai_epi32_32", "none");
}

#[test]
fn a0188_mm_srai_epi32_63() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_srai_epi32_63(core::hint::black_box(n), &mut d); } }
    d.line("_mm_srai_epi32_63", "none");
}

#[test]
fn a0189_mm_srai_epi32_64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_srai_epi32_64(core::hint::black_box(n), &mut d); } }
    d.line("_mm_srai_epi32_64", "none");
}

#[test]
fn a0190_mm_srai_epi32_255() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_srai_epi32_255(core::hint::black_box(n), &mut d); } }
    d.line("_mm_srai_epi32_255", "none");
}

#[test]
fn a0191_mm_srli_epi16_0() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_srli_epi16_0(core::hint::black_box(n), &mut d); } }
    d.line("_mm_srli_epi16_0", "none");
}

#[test]
fn a0192_mm_srli_epi16_1() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_srli_epi16_1(core::hint::black_box(n), &mut d); } }
    d.line("_mm_srli_epi16_1", "none");
}

#[test]
fn a0193_mm_srli_epi16_7() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_srli_epi16_7(core::hint::black_box(n), &mut d); } }
    d.line("_mm_srli_epi16_7", "none");
}

#[test]
fn a0194_mm_srli_epi16_15() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_srli_epi16_15(core::hint::black_box(n), &mut d); } }
    d.line("_mm_srli_epi16_15", "none");
}

#[test]
fn a0195_mm_srli_epi16_16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_srli_epi16_16(core::hint::black_box(n), &mut d); } }
    d.line("_mm_srli_epi16_16", "none");
}

#[test]
fn a0196_mm_srli_epi16_31() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_srli_epi16_31(core::hint::black_box(n), &mut d); } }
    d.line("_mm_srli_epi16_31", "none");
}

#[test]
fn a0197_mm_srli_epi16_32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_srli_epi16_32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_srli_epi16_32", "none");
}

#[test]
fn a0198_mm_srli_epi16_63() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_srli_epi16_63(core::hint::black_box(n), &mut d); } }
    d.line("_mm_srli_epi16_63", "none");
}

#[test]
fn a0199_mm_srli_epi16_64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_srli_epi16_64(core::hint::black_box(n), &mut d); } }
    d.line("_mm_srli_epi16_64", "none");
}

#[test]
fn a0200_mm_srli_epi16_255() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_srli_epi16_255(core::hint::black_box(n), &mut d); } }
    d.line("_mm_srli_epi16_255", "none");
}

#[test]
fn a0201_mm_srli_epi32_0() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_srli_epi32_0(core::hint::black_box(n), &mut d); } }
    d.line("_mm_srli_epi32_0", "none");
}

#[test]
fn a0202_mm_srli_epi32_1() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_srli_epi32_1(core::hint::black_box(n), &mut d); } }
    d.line("_mm_srli_epi32_1", "none");
}

#[test]
fn a0203_mm_srli_epi32_7() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_srli_epi32_7(core::hint::black_box(n), &mut d); } }
    d.line("_mm_srli_epi32_7", "none");
}

#[test]
fn a0204_mm_srli_epi32_15() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_srli_epi32_15(core::hint::black_box(n), &mut d); } }
    d.line("_mm_srli_epi32_15", "none");
}

#[test]
fn a0205_mm_srli_epi32_16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_srli_epi32_16(core::hint::black_box(n), &mut d); } }
    d.line("_mm_srli_epi32_16", "none");
}

#[test]
fn a0206_mm_srli_epi32_31() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_srli_epi32_31(core::hint::black_box(n), &mut d); } }
    d.line("_mm_srli_epi32_31", "none");
}

#[test]
fn a0207_mm_srli_epi32_32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_srli_epi32_32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_srli_epi32_32", "none");
}

#[test]
fn a0208_mm_srli_epi32_63() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_srli_epi32_63(core::hint::black_box(n), &mut d); } }
    d.line("_mm_srli_epi32_63", "none");
}

#[test]
fn a0209_mm_srli_epi32_64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_srli_epi32_64(core::hint::black_box(n), &mut d); } }
    d.line("_mm_srli_epi32_64", "none");
}

#[test]
fn a0210_mm_srli_epi32_255() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_srli_epi32_255(core::hint::black_box(n), &mut d); } }
    d.line("_mm_srli_epi32_255", "none");
}

#[test]
fn a0211_mm_srli_epi64_0() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_srli_epi64_0(core::hint::black_box(n), &mut d); } }
    d.line("_mm_srli_epi64_0", "none");
}

#[test]
fn a0212_mm_srli_epi64_1() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_srli_epi64_1(core::hint::black_box(n), &mut d); } }
    d.line("_mm_srli_epi64_1", "none");
}

#[test]
fn a0213_mm_srli_epi64_7() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_srli_epi64_7(core::hint::black_box(n), &mut d); } }
    d.line("_mm_srli_epi64_7", "none");
}

#[test]
fn a0214_mm_srli_epi64_15() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_srli_epi64_15(core::hint::black_box(n), &mut d); } }
    d.line("_mm_srli_epi64_15", "none");
}

#[test]
fn a0215_mm_srli_epi64_16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_srli_epi64_16(core::hint::black_box(n), &mut d); } }
    d.line("_mm_srli_epi64_16", "none");
}

#[test]
fn a0216_mm_srli_epi64_31() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_srli_epi64_31(core::hint::black_box(n), &mut d); } }
    d.line("_mm_srli_epi64_31", "none");
}

#[test]
fn a0217_mm_srli_epi64_32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_srli_epi64_32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_srli_epi64_32", "none");
}

#[test]
fn a0218_mm_srli_epi64_63() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_srli_epi64_63(core::hint::black_box(n), &mut d); } }
    d.line("_mm_srli_epi64_63", "none");
}

#[test]
fn a0219_mm_srli_epi64_64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_srli_epi64_64(core::hint::black_box(n), &mut d); } }
    d.line("_mm_srli_epi64_64", "none");
}

#[test]
fn a0220_mm_srli_epi64_255() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_srli_epi64_255(core::hint::black_box(n), &mut d); } }
    d.line("_mm_srli_epi64_255", "none");
}

#[test]
fn a0221_mm_sqrt_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_sqrt_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm_sqrt_ps", "none");
}

#[test]
fn a0222_mm_storeu_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_storeu_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm_storeu_ps", "none");
}

#[test]
fn a0223_mm_sub_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_sub_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm_sub_ps", "none");
}

#[test]
fn a0224_mm_unpackhi_epi16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_unpackhi_epi16(core::hint::black_box(n), &mut d); } }
    d.line("_mm_unpackhi_epi16", "none");
}

#[test]
fn a0225_mm_unpackhi_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_unpackhi_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_unpackhi_epi32", "none");
}

#[test]
fn a0226_mm_unpacklo_epi16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_unpacklo_epi16(core::hint::black_box(n), &mut d); } }
    d.line("_mm_unpacklo_epi16", "none");
}

#[test]
fn a0227_mm_unpacklo_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_unpacklo_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_unpacklo_epi32", "none");
}

#[test]
fn a0228_mm_unpacklo_epi64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_unpacklo_epi64(core::hint::black_box(n), &mut d); } }
    d.line("_mm_unpacklo_epi64", "none");
}

#[test]
fn a0229_mm_xor_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_xor_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm_xor_ps", "none");
}

#[test]
fn a0230_mm_add_epi64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_add_epi64(core::hint::black_box(n), &mut d); } }
    d.line("_mm_add_epi64", "none");
}

#[test]
fn a0231_mm_sub_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_sub_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_sub_epi32", "none");
}

#[test]
fn a0232_mm_sub_epi64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_sub_epi64(core::hint::black_box(n), &mut d); } }
    d.line("_mm_sub_epi64", "none");
}

#[test]
fn a0233_mm_adds_epi8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_adds_epi8(core::hint::black_box(n), &mut d); } }
    d.line("_mm_adds_epi8", "none");
}

#[test]
fn a0234_mm_adds_epi16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_adds_epi16(core::hint::black_box(n), &mut d); } }
    d.line("_mm_adds_epi16", "none");
}

#[test]
fn a0235_mm_adds_epu8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_adds_epu8(core::hint::black_box(n), &mut d); } }
    d.line("_mm_adds_epu8", "none");
}

#[test]
fn a0236_mm_adds_epu16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_adds_epu16(core::hint::black_box(n), &mut d); } }
    d.line("_mm_adds_epu16", "none");
}

#[test]
fn a0237_mm_subs_epi8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_subs_epi8(core::hint::black_box(n), &mut d); } }
    d.line("_mm_subs_epi8", "none");
}

#[test]
fn a0238_mm_subs_epi16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_subs_epi16(core::hint::black_box(n), &mut d); } }
    d.line("_mm_subs_epi16", "none");
}

#[test]
fn a0239_mm_subs_epu8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_subs_epu8(core::hint::black_box(n), &mut d); } }
    d.line("_mm_subs_epu8", "none");
}

#[test]
fn a0240_mm_subs_epu16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_subs_epu16(core::hint::black_box(n), &mut d); } }
    d.line("_mm_subs_epu16", "none");
}

#[test]
fn a0241_mm_min_epi8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_min_epi8(core::hint::black_box(n), &mut d); } }
    d.line("_mm_min_epi8", "none");
}

#[test]
fn a0242_mm_min_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_min_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_min_epi32", "none");
}

#[test]
fn a0243_mm_min_epu8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_min_epu8(core::hint::black_box(n), &mut d); } }
    d.line("_mm_min_epu8", "none");
}

#[test]
fn a0244_mm_min_epu16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_min_epu16(core::hint::black_box(n), &mut d); } }
    d.line("_mm_min_epu16", "none");
}

#[test]
fn a0245_mm_min_epu32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_min_epu32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_min_epu32", "none");
}

#[test]
fn a0246_mm_max_epi8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_max_epi8(core::hint::black_box(n), &mut d); } }
    d.line("_mm_max_epi8", "none");
}

#[test]
fn a0247_mm_max_epi16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_max_epi16(core::hint::black_box(n), &mut d); } }
    d.line("_mm_max_epi16", "none");
}

#[test]
fn a0248_mm_max_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_max_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_max_epi32", "none");
}

#[test]
fn a0249_mm_max_epu8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_max_epu8(core::hint::black_box(n), &mut d); } }
    d.line("_mm_max_epu8", "none");
}

#[test]
fn a0250_mm_max_epu16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_max_epu16(core::hint::black_box(n), &mut d); } }
    d.line("_mm_max_epu16", "none");
}

#[test]
fn a0251_mm_max_epu32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_max_epu32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_max_epu32", "none");
}

#[test]
fn a0252_mm_avg_epu16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_avg_epu16(core::hint::black_box(n), &mut d); } }
    d.line("_mm_avg_epu16", "none");
}

#[test]
fn a0253_mm_abs_epi8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_abs_epi8(core::hint::black_box(n), &mut d); } }
    d.line("_mm_abs_epi8", "none");
}

#[test]
fn a0254_mm_abs_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_abs_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_abs_epi32", "none");
}

#[test]
fn a0255_mm_set1_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_set1_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_set1_epi32", "none");
}

#[test]
fn a0256_mm_set1_epi64x() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_set1_epi64x(core::hint::black_box(n), &mut d); } }
    d.line("_mm_set1_epi64x", "none");
}

#[test]
fn a0257_mm_packs_epi16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_packs_epi16(core::hint::black_box(n), &mut d); } }
    d.line("_mm_packs_epi16", "none");
}

#[test]
fn a0258_mm_packs_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_packs_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_packs_epi32", "none");
}

#[test]
fn a0259_mm_packus_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_packus_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_packus_epi32", "none");
}

#[test]
fn a0260_mm_srai_epi16_0() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_srai_epi16_0(core::hint::black_box(n), &mut d); } }
    d.line("_mm_srai_epi16_0", "none");
}

#[test]
fn a0261_mm_srai_epi16_1() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_srai_epi16_1(core::hint::black_box(n), &mut d); } }
    d.line("_mm_srai_epi16_1", "none");
}

#[test]
fn a0262_mm_srai_epi16_7() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_srai_epi16_7(core::hint::black_box(n), &mut d); } }
    d.line("_mm_srai_epi16_7", "none");
}

#[test]
fn a0263_mm_srai_epi16_15() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_srai_epi16_15(core::hint::black_box(n), &mut d); } }
    d.line("_mm_srai_epi16_15", "none");
}

#[test]
fn a0264_mm_srai_epi16_16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_srai_epi16_16(core::hint::black_box(n), &mut d); } }
    d.line("_mm_srai_epi16_16", "none");
}

#[test]
fn a0265_mm_srai_epi16_31() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_srai_epi16_31(core::hint::black_box(n), &mut d); } }
    d.line("_mm_srai_epi16_31", "none");
}

#[test]
fn a0266_mm_srai_epi16_32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_srai_epi16_32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_srai_epi16_32", "none");
}

#[test]
fn a0267_mm_srai_epi16_63() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_srai_epi16_63(core::hint::black_box(n), &mut d); } }
    d.line("_mm_srai_epi16_63", "none");
}

#[test]
fn a0268_mm_srai_epi16_64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_srai_epi16_64(core::hint::black_box(n), &mut d); } }
    d.line("_mm_srai_epi16_64", "none");
}

#[test]
fn a0269_mm_srai_epi16_255() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_srai_epi16_255(core::hint::black_box(n), &mut d); } }
    d.line("_mm_srai_epi16_255", "none");
}

#[test]
fn a0270_mm_cmpneq_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cmpneq_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cmpneq_ps", "none");
}

#[test]
fn a0271_mm_srai_epi64_0() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_srai_epi64_0(core::hint::black_box(n), &mut d); } }
    d.line("_mm_srai_epi64_0", "none");
}

#[test]
fn a0272_mm_srai_epi64_1() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_srai_epi64_1(core::hint::black_box(n), &mut d); } }
    d.line("_mm_srai_epi64_1", "none");
}

#[test]
fn a0273_mm_srai_epi64_7() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_srai_epi64_7(core::hint::black_box(n), &mut d); } }
    d.line("_mm_srai_epi64_7", "none");
}

#[test]
fn a0274_mm_srai_epi64_15() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_srai_epi64_15(core::hint::black_box(n), &mut d); } }
    d.line("_mm_srai_epi64_15", "none");
}

#[test]
fn a0275_mm_srai_epi64_16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_srai_epi64_16(core::hint::black_box(n), &mut d); } }
    d.line("_mm_srai_epi64_16", "none");
}

#[test]
fn a0276_mm_srai_epi64_31() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_srai_epi64_31(core::hint::black_box(n), &mut d); } }
    d.line("_mm_srai_epi64_31", "none");
}

#[test]
fn a0277_mm_srai_epi64_32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_srai_epi64_32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_srai_epi64_32", "none");
}

#[test]
fn a0278_mm_srai_epi64_63() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_srai_epi64_63(core::hint::black_box(n), &mut d); } }
    d.line("_mm_srai_epi64_63", "none");
}

#[test]
fn a0279_mm_srai_epi64_64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_srai_epi64_64(core::hint::black_box(n), &mut d); } }
    d.line("_mm_srai_epi64_64", "none");
}

#[test]
fn a0280_mm_srai_epi64_255() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_srai_epi64_255(core::hint::black_box(n), &mut d); } }
    d.line("_mm_srai_epi64_255", "none");
}

#[test]
fn a0281__mm256_abs_epi8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe__mm256_abs_epi8(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_abs_epi8", "none");
}

#[test]
fn a0282__mm256_sign_epi8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe__mm256_sign_epi8(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_sign_epi8", "none");
}

#[test]
fn a0283__mm_hadd_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe__mm_hadd_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_hadd_epi32", "none");
}

#[test]
fn a0284__mm256_cvtepu8_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe__mm256_cvtepu8_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_cvtepu8_epi32", "none");
}

#[test]
fn a0285__mm256_cvtepi8_epi16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe__mm256_cvtepi8_epi16(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_cvtepi8_epi16", "none");
}

#[test]
fn a0286__mm256_cvtepi16_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe__mm256_cvtepi16_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_cvtepi16_epi32", "none");
}

#[test]
fn a0287__mm256_cvtepu16_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe__mm256_cvtepu16_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_cvtepu16_epi32", "none");
}

#[test]
fn a0288__mm256_permute2f128_si256_0() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe__mm256_permute2f128_si256_0(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_permute2f128_si256_0", "none");
}

#[test]
fn a0289__mm256_permute2f128_si256_10() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe__mm256_permute2f128_si256_10(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_permute2f128_si256_10", "none");
}

#[test]
fn a0290__mm256_permute2f128_si256_21() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe__mm256_permute2f128_si256_21(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_permute2f128_si256_21", "none");
}

#[test]
fn a0291__mm256_permute2f128_si256_88() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe__mm256_permute2f128_si256_88(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_permute2f128_si256_88", "none");
}

#[test]
fn a0292__mm256_permute2f128_si256_ff() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe__mm256_permute2f128_si256_ff(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_permute2f128_si256_ff", "none");
}

#[test]
fn a0293__mm_clflush() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe__mm_clflush(core::hint::black_box(n), &mut d); } }
    d.line("_mm_clflush", "none");
}

#[test]
fn a0294__mm_mfence() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe__mm_mfence(core::hint::black_box(n), &mut d); } }
    d.line("_mm_mfence", "none");
}

#[test]
fn a0295__pext_u32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe__pext_u32(core::hint::black_box(n), &mut d); } }
    d.line("_pext_u32", "none");
}

#[test]
fn a0296_mm256_addsub_pd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_addsub_pd(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_addsub_pd", "none");
}

#[test]
fn a0297_mm256_addsub_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_addsub_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_addsub_ps", "none");
}

#[test]
fn a0298_mm256_alignr_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_alignr_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_alignr_epi32", "none");
}

#[test]
fn a0299_mm256_alignr_epi64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_alignr_epi64(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_alignr_epi64", "none");
}

#[test]
fn a0300_mm256_blend_epi16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_blend_epi16(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_blend_epi16", "none");
}

#[test]
fn a0301_mm256_blend_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_blend_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_blend_ps", "none");
}

#[test]
fn a0302_mm256_broadcast_f32x2() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_broadcast_f32x2(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_broadcast_f32x2", "none");
}

#[test]
fn a0303_mm256_broadcast_f32x4() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_broadcast_f32x4(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_broadcast_f32x4", "none");
}

#[test]
fn a0304_mm256_broadcast_f64x2() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_broadcast_f64x2(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_broadcast_f64x2", "none");
}

#[test]
fn a0305_mm256_broadcast_i32x2() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_broadcast_i32x2(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_broadcast_i32x2", "none");
}

#[test]
fn a0306_mm256_broadcast_i64x2() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_broadcast_i64x2(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_broadcast_i64x2", "none");
}

#[test]
fn a0307_mm256_broadcast_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_broadcast_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_broadcast_ps", "none");
}

#[test]
fn a0308_mm256_broadcast_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_broadcast_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_broadcast_ss", "none");
}

#[test]
fn a0309_mm256_broadcastmb_epi64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_broadcastmb_epi64(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_broadcastmb_epi64", "none");
}

#[test]
fn a0310_mm256_broadcastmw_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_broadcastmw_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_broadcastmw_epi32", "none");
}

#[test]
fn a0311_mm256_broadcastss_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_broadcastss_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_broadcastss_ps", "none");
}

#[test]
fn a0312_mm256_bslli_epi128() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_bslli_epi128(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_bslli_epi128", "none");
}

#[test]
fn a0313_mm256_bsrli_epi128() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_bsrli_epi128(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_bsrli_epi128", "none");
}

#[test]
fn a0314_mm256_castps128_ps256() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_castps128_ps256(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_castps128_ps256", "none");
}

#[test]
fn a0315_mm256_conflict_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_conflict_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_conflict_epi32", "none");
}

#[test]
fn a0316_mm256_conflict_epi64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_conflict_epi64(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_conflict_epi64", "none");
}

#[test]
fn a0317_mm256_cvtepi16_epi64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_cvtepi16_epi64(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_cvtepi16_epi64", "none");
}

#[test]
fn a0318_mm256_cvtepi16_epi8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_cvtepi16_epi8(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_cvtepi16_epi8", "none");
}

#[test]
fn a0319_mm256_cvtepi32_epi16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_cvtepi32_epi16(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_cvtepi32_epi16", "none");
}

#[test]
fn a0320_mm256_cvtepi32_epi64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_cvtepi32_epi64(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_cvtepi32_epi64", "none");
}

#[test]
fn a0321_mm256_cvtepi32_epi8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_cvtepi32_epi8(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_cvtepi32_epi8", "none");
}

#[test]
fn a0322_mm256_cvtepi64_epi16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_cvtepi64_epi16(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_cvtepi64_epi16", "none");
}

#[test]
fn a0323_mm256_cvtepi64_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_cvtepi64_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_cvtepi64_epi32", "none");
}

#[test]
fn a0324_mm256_cvtepi64_epi8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_cvtepi64_epi8(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_cvtepi64_epi8", "none");
}

#[test]
fn a0325_mm256_cvtepi64_pd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_cvtepi64_pd(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_cvtepi64_pd", "none");
}

#[test]
fn a0326_mm256_cvtepi64_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_cvtepi64_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_cvtepi64_ps", "none");
}

#[test]
fn a0327_mm256_cvtepi8_epi64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_cvtepi8_epi64(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_cvtepi8_epi64", "none");
}

#[test]
fn a0328_mm256_cvtepu16_epi64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_cvtepu16_epi64(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_cvtepu16_epi64", "none");
}

#[test]
fn a0329_mm256_cvtepu32_epi64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_cvtepu32_epi64(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_cvtepu32_epi64", "none");
}

#[test]
fn a0330_mm256_cvtepu32_pd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_cvtepu32_pd(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_cvtepu32_pd", "none");
}

#[test]
fn a0331_mm256_cvtepu64_pd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_cvtepu64_pd(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_cvtepu64_pd", "none");
}

#[test]
fn a0332_mm256_cvtepu64_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_cvtepu64_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_cvtepu64_ps", "none");
}

#[test]
fn a0333_mm256_cvtepu8_epi64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_cvtepu8_epi64(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_cvtepu8_epi64", "none");
}

#[test]
fn a0334_mm256_cvtpd_epi64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_cvtpd_epi64(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_cvtpd_epi64", "none");
}

#[test]
fn a0335_mm256_cvtpd_epu32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_cvtpd_epu32(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_cvtpd_epu32", "none");
}

#[test]
fn a0336_mm256_cvtpd_epu64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_cvtpd_epu64(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_cvtpd_epu64", "none");
}

#[test]
fn a0337_mm256_cvtps_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_cvtps_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_cvtps_epi32", "none");
}

#[test]
fn a0338_mm256_cvtps_epi64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_cvtps_epi64(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_cvtps_epi64", "none");
}

#[test]
fn a0339_mm256_cvtps_epu32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_cvtps_epu32(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_cvtps_epu32", "none");
}

#[test]
fn a0340_mm256_cvtps_epu64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_cvtps_epu64(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_cvtps_epu64", "none");
}

#[test]
fn a0341_mm256_cvtsd_f64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_cvtsd_f64(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_cvtsd_f64", "none");
}

#[test]
fn a0342_mm256_cvtsepi16_epi8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_cvtsepi16_epi8(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_cvtsepi16_epi8", "none");
}

#[test]
fn a0343_mm256_cvtsepi32_epi16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_cvtsepi32_epi16(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_cvtsepi32_epi16", "none");
}

#[test]
fn a0344_mm256_cvtsepi32_epi8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_cvtsepi32_epi8(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_cvtsepi32_epi8", "none");
}

#[test]
fn a0345_mm256_cvtsepi64_epi16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_cvtsepi64_epi16(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_cvtsepi64_epi16", "none");
}

#[test]
fn a0346_mm256_cvtsepi64_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_cvtsepi64_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_cvtsepi64_epi32", "none");
}

#[test]
fn a0347_mm256_cvtsepi64_epi8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_cvtsepi64_epi8(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_cvtsepi64_epi8", "none");
}

#[test]
fn a0348_mm256_cvtsi256_si32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_cvtsi256_si32(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_cvtsi256_si32", "none");
}

#[test]
fn a0349_mm256_cvtss_f32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_cvtss_f32(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_cvtss_f32", "none");
}

#[test]
fn a0350_mm256_cvttpd_epi64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_cvttpd_epi64(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_cvttpd_epi64", "none");
}

#[test]
fn a0351_mm256_cvttpd_epu32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_cvttpd_epu32(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_cvttpd_epu32", "none");
}

#[test]
fn a0352_mm256_cvttpd_epu64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_cvttpd_epu64(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_cvttpd_epu64", "none");
}

#[test]
fn a0353_mm256_cvttps_epi64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_cvttps_epi64(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_cvttps_epi64", "none");
}

#[test]
fn a0354_mm256_cvttps_epu32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_cvttps_epu32(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_cvttps_epu32", "none");
}

#[test]
fn a0355_mm256_cvttps_epu64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_cvttps_epu64(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_cvttps_epu64", "none");
}

#[test]
fn a0356_mm256_cvtusepi16_epi8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_cvtusepi16_epi8(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_cvtusepi16_epi8", "none");
}

#[test]
fn a0357_mm256_cvtusepi32_epi16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_cvtusepi32_epi16(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_cvtusepi32_epi16", "none");
}

#[test]
fn a0358_mm256_cvtusepi32_epi8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_cvtusepi32_epi8(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_cvtusepi32_epi8", "none");
}

#[test]
fn a0359_mm256_cvtusepi64_epi16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_cvtusepi64_epi16(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_cvtusepi64_epi16", "none");
}

#[test]
fn a0360_mm256_cvtusepi64_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_cvtusepi64_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_cvtusepi64_epi32", "none");
}

#[test]
fn a0361_mm256_cvtusepi64_epi8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_cvtusepi64_epi8(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_cvtusepi64_epi8", "none");
}

#[test]
fn a0362_mm256_dbsad_epu8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_dbsad_epu8(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_dbsad_epu8", "none");
}

#[test]
fn a0363_mm256_dp_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_dp_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_dp_ps", "none");
}

#[test]
fn a0364_mm256_dpbsud_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_dpbsud_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_dpbsud_epi32", "none");
}

#[test]
fn a0365_mm256_dpbsuds_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_dpbsuds_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_dpbsuds_epi32", "none");
}

#[test]
fn a0366_mm256_dpbuud_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_dpbuud_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_dpbuud_epi32", "none");
}

#[test]
fn a0367_mm256_dpbuuds_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_dpbuuds_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_dpbuuds_epi32", "none");
}

#[test]
fn a0368_mm256_dpwsud_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_dpwsud_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_dpwsud_epi32", "none");
}

#[test]
fn a0369_mm256_dpwsuds_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_dpwsuds_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_dpwsuds_epi32", "none");
}

#[test]
fn a0370_mm256_dpwusd_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_dpwusd_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_dpwusd_epi32", "none");
}

#[test]
fn a0371_mm256_dpwusds_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_dpwusds_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_dpwusds_epi32", "none");
}

#[test]
fn a0372_mm256_dpwuud_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_dpwuud_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_dpwuud_epi32", "none");
}

#[test]
fn a0373_mm256_dpwuuds_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_dpwuuds_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_dpwuuds_epi32", "none");
}

#[test]
fn a0374_mm256_extract_epi16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_extract_epi16(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_extract_epi16", "none");
}

#[test]
fn a0375_mm256_extract_epi8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_extract_epi8(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_extract_epi8", "none");
}

#[test]
fn a0376_mm256_extractf32x4_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_extractf32x4_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_extractf32x4_ps", "none");
}

#[test]
fn a0377_mm256_extractf64x2_pd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_extractf64x2_pd(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_extractf64x2_pd", "none");
}

#[test]
fn a0378_mm256_extracti32x4_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_extracti32x4_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_extracti32x4_epi32", "none");
}

#[test]
fn a0379_mm256_extracti64x2_epi64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_extracti64x2_epi64(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_extracti64x2_epi64", "none");
}

#[test]
fn a0380_mm256_fixupimm_pd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_fixupimm_pd(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_fixupimm_pd", "none");
}

#[test]
fn a0381_mm256_fixupimm_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_fixupimm_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_fixupimm_ps", "none");
}

#[test]
fn a0382_mm256_hadd_epi16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_hadd_epi16(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_hadd_epi16", "none");
}

#[test]
fn a0383_mm256_hadd_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_hadd_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_hadd_epi32", "none");
}

#[test]
fn a0384_mm256_hadd_pd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_hadd_pd(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_hadd_pd", "none");
}

#[test]
fn a0385_mm256_hadd_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_hadd_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_hadd_ps", "none");
}

#[test]
fn a0386_mm256_hadds_epi16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_hadds_epi16(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_hadds_epi16", "none");
}

#[test]
fn a0387_mm256_hsub_epi16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_hsub_epi16(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_hsub_epi16", "none");
}

#[test]
fn a0388_mm256_hsub_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_hsub_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_hsub_epi32", "none");
}

#[test]
fn a0389_mm256_hsub_pd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_hsub_pd(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_hsub_pd", "none");
}

#[test]
fn a0390_mm256_hsub_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_hsub_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_hsub_ps", "none");
}

#[test]
fn a0391_mm256_hsubs_epi16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_hsubs_epi16(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_hsubs_epi16", "none");
}

#[test]
fn a0392_mm256_insert_epi16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_insert_epi16(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_insert_epi16", "none");
}

#[test]
fn a0393_mm256_insert_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_insert_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_insert_epi32", "none");
}

#[test]
fn a0394_mm256_insert_epi64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_insert_epi64(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_insert_epi64", "none");
}

#[test]
fn a0395_mm256_insert_epi8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_insert_epi8(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_insert_epi8", "none");
}

#[test]
fn a0396_mm256_insertf128_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_insertf128_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_insertf128_ps", "none");
}

#[test]
fn a0397_mm256_loadu2_m128() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_loadu2_m128(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_loadu2_m128", "none");
}

#[test]
fn a0398_mm256_loadu2_m128d() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_loadu2_m128d(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_loadu2_m128d", "none");
}

#[test]
fn a0399_mm256_loadu2_m128i() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_loadu2_m128i(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_loadu2_m128i", "none");
}

#[test]
fn a0400_mm256_movehdup_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_movehdup_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_movehdup_ps", "none");
}

#[test]
fn a0401_mm256_moveldup_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_moveldup_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_moveldup_ps", "none");
}

#[test]
fn a0402_mm256_mpsadbw_epu8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_mpsadbw_epu8(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_mpsadbw_epu8", "none");
}

#[test]
fn a0403_mm256_mul_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_mul_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_mul_epi32", "none");
}

#[test]
fn a0404_mm256_mulhi_epi16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_mulhi_epi16(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_mulhi_epi16", "none");
}

#[test]
fn a0405_mm256_mulhrs_epi16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_mulhrs_epi16(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_mulhrs_epi16", "none");
}

#[test]
fn a0406_mm256_permute_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_permute_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_permute_ps", "none");
}

#[test]
fn a0407_mm256_permutevar_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_permutevar_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_permutevar_ps", "none");
}

#[test]
fn a0408_mm256_permutex_epi64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_permutex_epi64(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_permutex_epi64", "none");
}

#[test]
fn a0409_mm256_permutex_pd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_permutex_pd(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_permutex_pd", "none");
}

#[test]
fn a0410_mm256_range_pd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_range_pd(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_range_pd", "none");
}

#[test]
fn a0411_mm256_range_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_range_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_range_ps", "none");
}

#[test]
fn a0412_mm256_rcp14_pd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_rcp14_pd(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_rcp14_pd", "none");
}

#[test]
fn a0413_mm256_rcp14_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_rcp14_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_rcp14_ps", "none");
}

#[test]
fn a0414_mm256_rcp_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_rcp_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_rcp_ps", "none");
}

#[test]
fn a0415_mm256_reduce_add_epi16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_reduce_add_epi16(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_reduce_add_epi16", "none");
}

#[test]
fn a0416_mm256_reduce_add_epi8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_reduce_add_epi8(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_reduce_add_epi8", "none");
}

#[test]
fn a0417_mm256_reduce_and_epi16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_reduce_and_epi16(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_reduce_and_epi16", "none");
}

#[test]
fn a0418_mm256_reduce_and_epi8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_reduce_and_epi8(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_reduce_and_epi8", "none");
}

#[test]
fn a0419_mm256_reduce_max_epi16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_reduce_max_epi16(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_reduce_max_epi16", "none");
}

#[test]
fn a0420_mm256_reduce_max_epi8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_reduce_max_epi8(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_reduce_max_epi8", "none");
}

#[test]
fn a0421_mm256_reduce_max_epu16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_reduce_max_epu16(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_reduce_max_epu16", "none");
}

#[test]
fn a0422_mm256_reduce_max_epu8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_reduce_max_epu8(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_reduce_max_epu8", "none");
}

#[test]
fn a0423_mm256_reduce_min_epi16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_reduce_min_epi16(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_reduce_min_epi16", "none");
}

#[test]
fn a0424_mm256_reduce_min_epi8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_reduce_min_epi8(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_reduce_min_epi8", "none");
}

#[test]
fn a0425_mm256_reduce_min_epu16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_reduce_min_epu16(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_reduce_min_epu16", "none");
}

#[test]
fn a0426_mm256_reduce_min_epu8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_reduce_min_epu8(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_reduce_min_epu8", "none");
}

#[test]
fn a0427_mm256_reduce_mul_epi16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_reduce_mul_epi16(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_reduce_mul_epi16", "none");
}

#[test]
fn a0428_mm256_reduce_mul_epi8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_reduce_mul_epi8(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_reduce_mul_epi8", "none");
}

#[test]
fn a0429_mm256_reduce_or_epi16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_reduce_or_epi16(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_reduce_or_epi16", "none");
}

#[test]
fn a0430_mm256_reduce_or_epi8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_reduce_or_epi8(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_reduce_or_epi8", "none");
}

#[test]
fn a0431_mm256_reduce_pd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_reduce_pd(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_reduce_pd", "none");
}

#[test]
fn a0432_mm256_reduce_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_reduce_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_reduce_ps", "none");
}

#[test]
fn a0433_mm256_roundscale_pd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_roundscale_pd(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_roundscale_pd", "none");
}

#[test]
fn a0434_mm256_roundscale_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_roundscale_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_roundscale_ps", "none");
}

#[test]
fn a0435_mm256_rsqrt14_pd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_rsqrt14_pd(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_rsqrt14_pd", "none");
}

#[test]
fn a0436_mm256_rsqrt14_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_rsqrt14_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_rsqrt14_ps", "none");
}

#[test]
fn a0437_mm256_rsqrt_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_rsqrt_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_rsqrt_ps", "none");
}

#[test]
fn a0438_mm256_set_m128() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_set_m128(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_set_m128", "none");
}

#[test]
fn a0439_mm256_set_m128d() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_set_m128d(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_set_m128d", "none");
}

#[test]
fn a0440_mm256_setr_m128d() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_setr_m128d(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_setr_m128d", "none");
}

#[test]
fn a0441_mm256_shuffle_f32x4() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_shuffle_f32x4(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_shuffle_f32x4", "none");
}

#[test]
fn a0442_mm256_shuffle_f64x2() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_shuffle_f64x2(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_shuffle_f64x2", "none");
}

#[test]
fn a0443_mm256_shuffle_i32x4() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_shuffle_i32x4(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_shuffle_i32x4", "none");
}

#[test]
fn a0444_mm256_shuffle_i64x2() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_shuffle_i64x2(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_shuffle_i64x2", "none");
}

#[test]
fn a0445_mm256_shufflehi_epi16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_shufflehi_epi16(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_shufflehi_epi16", "none");
}

#[test]
fn a0446_mm256_shufflelo_epi16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_shufflelo_epi16(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_shufflelo_epi16", "none");
}

#[test]
fn a0447_mm256_slli_si256() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_slli_si256(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_slli_si256", "none");
}

#[test]
fn a0448_mm256_srli_si256() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_srli_si256(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_srli_si256", "none");
}

#[test]
fn a0449_mm256_storeu2_m128() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_storeu2_m128(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_storeu2_m128", "none");
}

#[test]
fn a0450_mm256_storeu2_m128d() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_storeu2_m128d(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_storeu2_m128d", "none");
}

#[test]
fn a0451_mm256_storeu2_m128i() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_storeu2_m128i(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_storeu2_m128i", "none");
}

#[test]
fn a0452_mm256_stream_load_si256() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_stream_load_si256(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_stream_load_si256", "none");
}

#[test]
fn a0453_mm256_testc_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_testc_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_testc_ps", "none");
}

#[test]
fn a0454_mm256_testc_si256() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_testc_si256(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_testc_si256", "none");
}

#[test]
fn a0455_mm256_testnzc_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_testnzc_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_testnzc_ps", "none");
}

#[test]
fn a0456_mm256_testnzc_si256() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_testnzc_si256(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_testnzc_si256", "none");
}

#[test]
fn a0457_mm256_testz_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_testz_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_testz_ps", "none");
}

#[test]
fn a0458_mm256_undefined_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_undefined_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_undefined_ps", "none");
}

#[test]
fn a0459_mm256_undefined_si256() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_undefined_si256(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_undefined_si256", "none");
}

#[test]
fn a0460_mm256_zextps128_ps256() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_zextps128_ps256(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_zextps128_ps256", "none");
}

#[test]
fn a0461_mm256_zextsi128_si256() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_zextsi128_si256(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_zextsi128_si256", "none");
}

#[test]
fn a0462_mm_add_round_sd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_add_round_sd(core::hint::black_box(n), &mut d); } }
    d.line("_mm_add_round_sd", "none");
}

#[test]
fn a0463_mm_add_round_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_add_round_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_add_round_ss", "none");
}

#[test]
fn a0464_mm_addsub_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_addsub_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm_addsub_ps", "none");
}

#[test]
fn a0465_mm_alignr_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_alignr_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_alignr_epi32", "none");
}

#[test]
fn a0466_mm_alignr_epi64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_alignr_epi64(core::hint::black_box(n), &mut d); } }
    d.line("_mm_alignr_epi64", "none");
}

#[test]
fn a0467_mm_blend_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_blend_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_blend_epi32", "none");
}

#[test]
fn a0468_mm_blend_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_blend_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm_blend_ps", "none");
}

#[test]
fn a0469_mm_broadcast_i32x2() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_broadcast_i32x2(core::hint::black_box(n), &mut d); } }
    d.line("_mm_broadcast_i32x2", "none");
}

#[test]
fn a0470_mm_broadcast_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_broadcast_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_broadcast_ss", "none");
}

#[test]
fn a0471_mm_broadcastmb_epi64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_broadcastmb_epi64(core::hint::black_box(n), &mut d); } }
    d.line("_mm_broadcastmb_epi64", "none");
}

#[test]
fn a0472_mm_broadcastmw_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_broadcastmw_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_broadcastmw_epi32", "none");
}

#[test]
fn a0473_mm_broadcastsi128_si256() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_broadcastsi128_si256(core::hint::black_box(n), &mut d); } }
    d.line("_mm_broadcastsi128_si256", "none");
}

#[test]
fn a0474_mm_broadcastss_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_broadcastss_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm_broadcastss_ps", "none");
}

#[test]
fn a0475_mm_bslli_si128() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_bslli_si128(core::hint::black_box(n), &mut d); } }
    d.line("_mm_bslli_si128", "none");
}

#[test]
fn a0476_mm_bsrli_si128() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_bsrli_si128(core::hint::black_box(n), &mut d); } }
    d.line("_mm_bsrli_si128", "none");
}

#[test]
fn a0477_mm_ceil_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_ceil_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_ceil_ss", "none");
}

#[test]
fn a0478_mm_cmp_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cmp_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cmp_ss", "none");
}

#[test]
fn a0479_mm_cmpeq_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cmpeq_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cmpeq_ss", "none");
}

#[test]
fn a0480_mm_cmpge_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cmpge_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cmpge_ss", "none");
}

#[test]
fn a0481_mm_cmpgt_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cmpgt_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cmpgt_ss", "none");
}

#[test]
fn a0482_mm_cmple_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cmple_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cmple_ss", "none");
}

#[test]
fn a0483_mm_cmplt_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cmplt_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cmplt_ss", "none");
}

#[test]
fn a0484_mm_cmpneq_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cmpneq_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cmpneq_ss", "none");
}

#[test]
fn a0485_mm_cmpnge_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cmpnge_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cmpnge_ps", "none");
}

#[test]
fn a0486_mm_cmpnge_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cmpnge_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cmpnge_ss", "none");
}

#[test]
fn a0487_mm_cmpngt_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cmpngt_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cmpngt_ps", "none");
}

#[test]
fn a0488_mm_cmpngt_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cmpngt_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cmpngt_ss", "none");
}

#[test]
fn a0489_mm_cmpnle_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cmpnle_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cmpnle_ps", "none");
}

#[test]
fn a0490_mm_cmpnle_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cmpnle_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cmpnle_ss", "none");
}

#[test]
fn a0491_mm_cmpnlt_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cmpnlt_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cmpnlt_ps", "none");
}

#[test]
fn a0492_mm_cmpnlt_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cmpnlt_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cmpnlt_ss", "none");
}

#[test]
fn a0493_mm_cmpord_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cmpord_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cmpord_ss", "none");
}

#[test]
fn a0494_mm_cmpunord_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cmpunord_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cmpunord_ss", "none");
}

#[test]
fn a0495_mm_comieq_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_comieq_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_comieq_ss", "none");
}

#[test]
fn a0496_mm_comige_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_comige_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_comige_ss", "none");
}

#[test]
fn a0497_mm_comigt_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_comigt_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_comigt_ss", "none");
}

#[test]
fn a0498_mm_comile_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_comile_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_comile_ss", "none");
}

#[test]
fn a0499_mm_comilt_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_comilt_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_comilt_ss", "none");
}

#[test]
fn a0500_mm_conflict_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_conflict_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_conflict_epi32", "none");
}

#[test]
fn a0501_mm_conflict_epi64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_conflict_epi64(core::hint::black_box(n), &mut d); } }
    d.line("_mm_conflict_epi64", "none");
}

#[test]
fn a0502_mm_cvt_roundi32_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvt_roundi32_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvt_roundi32_ss", "none");
}

#[test]
fn a0503_mm_cvt_roundi64_sd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvt_roundi64_sd(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvt_roundi64_sd", "none");
}

#[test]
fn a0504_mm_cvt_roundi64_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvt_roundi64_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvt_roundi64_ss", "none");
}

#[test]
fn a0505_mm_cvt_roundsd_i32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvt_roundsd_i32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvt_roundsd_i32", "none");
}

#[test]
fn a0506_mm_cvt_roundsd_i64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvt_roundsd_i64(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvt_roundsd_i64", "none");
}

#[test]
fn a0507_mm_cvt_roundsd_si32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvt_roundsd_si32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvt_roundsd_si32", "none");
}

#[test]
fn a0508_mm_cvt_roundsd_si64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvt_roundsd_si64(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvt_roundsd_si64", "none");
}

#[test]
fn a0509_mm_cvt_roundsd_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvt_roundsd_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvt_roundsd_ss", "none");
}

#[test]
fn a0510_mm_cvt_roundsd_u32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvt_roundsd_u32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvt_roundsd_u32", "none");
}

#[test]
fn a0511_mm_cvt_roundsd_u64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvt_roundsd_u64(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvt_roundsd_u64", "none");
}

#[test]
fn a0512_mm_cvt_roundsi32_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvt_roundsi32_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvt_roundsi32_ss", "none");
}

#[test]
fn a0513_mm_cvt_roundsi64_sd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvt_roundsi64_sd(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvt_roundsi64_sd", "none");
}

#[test]
fn a0514_mm_cvt_roundsi64_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvt_roundsi64_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvt_roundsi64_ss", "none");
}

#[test]
fn a0515_mm_cvt_roundss_i32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvt_roundss_i32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvt_roundss_i32", "none");
}

#[test]
fn a0516_mm_cvt_roundss_i64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvt_roundss_i64(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvt_roundss_i64", "none");
}

#[test]
fn a0517_mm_cvt_roundss_sd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvt_roundss_sd(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvt_roundss_sd", "none");
}

#[test]
fn a0518_mm_cvt_roundss_si32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvt_roundss_si32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvt_roundss_si32", "none");
}

#[test]
fn a0519_mm_cvt_roundss_si64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvt_roundss_si64(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvt_roundss_si64", "none");
}

#[test]
fn a0520_mm_cvt_roundss_u32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvt_roundss_u32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvt_roundss_u32", "none");
}

#[test]
fn a0521_mm_cvt_roundss_u64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvt_roundss_u64(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvt_roundss_u64", "none");
}

#[test]
fn a0522_mm_cvt_roundu32_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvt_roundu32_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvt_roundu32_ss", "none");
}

#[test]
fn a0523_mm_cvt_roundu64_sd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvt_roundu64_sd(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvt_roundu64_sd", "none");
}

#[test]
fn a0524_mm_cvt_roundu64_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvt_roundu64_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvt_roundu64_ss", "none");
}

#[test]
fn a0525_mm_cvt_si2ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvt_si2ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvt_si2ss", "none");
}

#[test]
fn a0526_mm_cvt_ss2si() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvt_ss2si(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvt_ss2si", "none");
}

#[test]
fn a0527_mm_cvtepi16_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvtepi16_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvtepi16_epi32", "none");
}

#[test]
fn a0528_mm_cvtepi16_epi64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvtepi16_epi64(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvtepi16_epi64", "none");
}

#[test]
fn a0529_mm_cvtepi16_epi8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvtepi16_epi8(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvtepi16_epi8", "none");
}

#[test]
fn a0530_mm_cvtepi32_epi16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvtepi32_epi16(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvtepi32_epi16", "none");
}

#[test]
fn a0531_mm_cvtepi32_epi64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvtepi32_epi64(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvtepi32_epi64", "none");
}

#[test]
fn a0532_mm_cvtepi32_epi8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvtepi32_epi8(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvtepi32_epi8", "none");
}

#[test]
fn a0533_mm_cvtepi64_epi16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvtepi64_epi16(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvtepi64_epi16", "none");
}

#[test]
fn a0534_mm_cvtepi64_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvtepi64_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvtepi64_epi32", "none");
}

#[test]
fn a0535_mm_cvtepi64_epi8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvtepi64_epi8(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvtepi64_epi8", "none");
}

#[test]
fn a0536_mm_cvtepi64_pd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvtepi64_pd(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvtepi64_pd", "none");
}

#[test]
fn a0537_mm_cvtepi64_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvtepi64_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvtepi64_ps", "none");
}

#[test]
fn a0538_mm_cvtepi8_epi16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvtepi8_epi16(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvtepi8_epi16", "none");
}

#[test]
fn a0539_mm_cvtepi8_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvtepi8_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvtepi8_epi32", "none");
}

#[test]
fn a0540_mm_cvtepi8_epi64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvtepi8_epi64(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvtepi8_epi64", "none");
}

#[test]
fn a0541_mm_cvtepu16_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvtepu16_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvtepu16_epi32", "none");
}

#[test]
fn a0542_mm_cvtepu16_epi64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvtepu16_epi64(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvtepu16_epi64", "none");
}

#[test]
fn a0543_mm_cvtepu32_epi64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvtepu32_epi64(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvtepu32_epi64", "none");
}

#[test]
fn a0544_mm_cvtepu32_pd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvtepu32_pd(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvtepu32_pd", "none");
}

#[test]
fn a0545_mm_cvtepu64_pd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvtepu64_pd(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvtepu64_pd", "none");
}

#[test]
fn a0546_mm_cvtepu64_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvtepu64_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvtepu64_ps", "none");
}

#[test]
fn a0547_mm_cvtepu8_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvtepu8_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvtepu8_epi32", "none");
}

#[test]
fn a0548_mm_cvtepu8_epi64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvtepu8_epi64(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvtepu8_epi64", "none");
}

#[test]
fn a0549_mm_cvti32_sd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvti32_sd(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvti32_sd", "none");
}

#[test]
fn a0550_mm_cvti32_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvti32_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvti32_ss", "none");
}

#[test]
fn a0551_mm_cvti64_sd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvti64_sd(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvti64_sd", "none");
}

#[test]
fn a0552_mm_cvti64_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvti64_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvti64_ss", "none");
}

#[test]
fn a0553_mm_cvtpd_epi64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvtpd_epi64(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvtpd_epi64", "none");
}

#[test]
fn a0554_mm_cvtpd_epu32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvtpd_epu32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvtpd_epu32", "none");
}

#[test]
fn a0555_mm_cvtpd_epu64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvtpd_epu64(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvtpd_epu64", "none");
}

#[test]
fn a0556_mm_cvtps_epi64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvtps_epi64(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvtps_epi64", "none");
}

#[test]
fn a0557_mm_cvtps_epu32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvtps_epu32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvtps_epu32", "none");
}

#[test]
fn a0558_mm_cvtps_epu64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvtps_epu64(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvtps_epu64", "none");
}

#[test]
fn a0559_mm_cvtsd_i32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvtsd_i32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvtsd_i32", "none");
}

#[test]
fn a0560_mm_cvtsd_i64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvtsd_i64(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvtsd_i64", "none");
}

#[test]
fn a0561_mm_cvtsd_u32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvtsd_u32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvtsd_u32", "none");
}

#[test]
fn a0562_mm_cvtsd_u64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvtsd_u64(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvtsd_u64", "none");
}

#[test]
fn a0563_mm_cvtsepi16_epi8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvtsepi16_epi8(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvtsepi16_epi8", "none");
}

#[test]
fn a0564_mm_cvtsepi32_epi16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvtsepi32_epi16(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvtsepi32_epi16", "none");
}

#[test]
fn a0565_mm_cvtsepi32_epi8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvtsepi32_epi8(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvtsepi32_epi8", "none");
}

#[test]
fn a0566_mm_cvtsepi64_epi16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvtsepi64_epi16(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvtsepi64_epi16", "none");
}

#[test]
fn a0567_mm_cvtsepi64_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvtsepi64_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvtsepi64_epi32", "none");
}

#[test]
fn a0568_mm_cvtsepi64_epi8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvtsepi64_epi8(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvtsepi64_epi8", "none");
}

#[test]
fn a0569_mm_cvtsi128_si16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvtsi128_si16(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvtsi128_si16", "none");
}

#[test]
fn a0570_mm_cvtsi16_si128() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvtsi16_si128(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvtsi16_si128", "none");
}

#[test]
fn a0571_mm_cvtsi32_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvtsi32_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvtsi32_ss", "none");
}

#[test]
fn a0572_mm_cvtsi64_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvtsi64_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvtsi64_ss", "none");
}

#[test]
fn a0573_mm_cvtss_i32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvtss_i32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvtss_i32", "none");
}

#[test]
fn a0574_mm_cvtss_i64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvtss_i64(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvtss_i64", "none");
}

#[test]
fn a0575_mm_cvtss_si32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvtss_si32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvtss_si32", "none");
}

#[test]
fn a0576_mm_cvtss_si64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvtss_si64(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvtss_si64", "none");
}

#[test]
fn a0577_mm_cvtss_u32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvtss_u32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvtss_u32", "none");
}

#[test]
fn a0578_mm_cvtss_u64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvtss_u64(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvtss_u64", "none");
}

#[test]
fn a0579_mm_cvtt_roundsd_i32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvtt_roundsd_i32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvtt_roundsd_i32", "none");
}

#[test]
fn a0580_mm_cvtt_roundsd_i64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvtt_roundsd_i64(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvtt_roundsd_i64", "none");
}

#[test]
fn a0581_mm_cvtt_roundsd_si32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvtt_roundsd_si32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvtt_roundsd_si32", "none");
}

#[test]
fn a0582_mm_cvtt_roundsd_si64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvtt_roundsd_si64(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvtt_roundsd_si64", "none");
}

#[test]
fn a0583_mm_cvtt_roundsd_u32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvtt_roundsd_u32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvtt_roundsd_u32", "none");
}

#[test]
fn a0584_mm_cvtt_roundsd_u64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvtt_roundsd_u64(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvtt_roundsd_u64", "none");
}

#[test]
fn a0585_mm_cvtt_roundss_i32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvtt_roundss_i32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvtt_roundss_i32", "none");
}

#[test]
fn a0586_mm_cvtt_roundss_i64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvtt_roundss_i64(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvtt_roundss_i64", "none");
}

#[test]
fn a0587_mm_cvtt_roundss_si32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvtt_roundss_si32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvtt_roundss_si32", "none");
}

#[test]
fn a0588_mm_cvtt_roundss_si64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvtt_roundss_si64(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvtt_roundss_si64", "none");
}

#[test]
fn a0589_mm_cvtt_roundss_u32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvtt_roundss_u32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvtt_roundss_u32", "none");
}

#[test]
fn a0590_mm_cvtt_roundss_u64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvtt_roundss_u64(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvtt_roundss_u64", "none");
}

#[test]
fn a0591_mm_cvtt_ss2si() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvtt_ss2si(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvtt_ss2si", "none");
}

#[test]
fn a0592_mm_cvttpd_epi64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvttpd_epi64(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvttpd_epi64", "none");
}

#[test]
fn a0593_mm_cvttpd_epu32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvttpd_epu32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvttpd_epu32", "none");
}

#[test]
fn a0594_mm_cvttpd_epu64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvttpd_epu64(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvttpd_epu64", "none");
}

#[test]
fn a0595_mm_cvttps_epi64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvttps_epi64(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvttps_epi64", "none");
}

#[test]
fn a0596_mm_cvttps_epu32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvttps_epu32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvttps_epu32", "none");
}

#[test]
fn a0597_mm_cvttps_epu64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvttps_epu64(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvttps_epu64", "none");
}

#[test]
fn a0598_mm_cvttsd_i32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvttsd_i32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvttsd_i32", "none");
}

#[test]
fn a0599_mm_cvttsd_i64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvttsd_i64(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvttsd_i64", "none");
}

#[test]
fn a0600_mm_cvttsd_u32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvttsd_u32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvttsd_u32", "none");
}

#[test]
fn a0601_mm_cvttsd_u64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvttsd_u64(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvttsd_u64", "none");
}

#[test]
fn a0602_mm_cvttss_i32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvttss_i32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvttss_i32", "none");
}

#[test]
fn a0603_mm_cvttss_i64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvttss_i64(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvttss_i64", "none");
}

#[test]
fn a0604_mm_cvttss_si32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvttss_si32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvttss_si32", "none");
}

#[test]
fn a0605_mm_cvttss_si64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvttss_si64(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvttss_si64", "none");
}

#[test]
fn a0606_mm_cvttss_u32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvttss_u32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvttss_u32", "none");
}

#[test]
fn a0607_mm_cvttss_u64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvttss_u64(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvttss_u64", "none");
}

#[test]
fn a0608_mm_cvtu32_sd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvtu32_sd(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvtu32_sd", "none");
}

#[test]
fn a0609_mm_cvtu32_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvtu32_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvtu32_ss", "none");
}

#[test]
fn a0610_mm_cvtu64_sd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvtu64_sd(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvtu64_sd", "none");
}

#[test]
fn a0611_mm_cvtu64_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvtu64_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvtu64_ss", "none");
}

#[test]
fn a0612_mm_cvtusepi16_epi8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvtusepi16_epi8(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvtusepi16_epi8", "none");
}

#[test]
fn a0613_mm_cvtusepi32_epi16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvtusepi32_epi16(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvtusepi32_epi16", "none");
}

#[test]
fn a0614_mm_cvtusepi32_epi8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvtusepi32_epi8(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvtusepi32_epi8", "none");
}

#[test]
fn a0615_mm_cvtusepi64_epi16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvtusepi64_epi16(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvtusepi64_epi16", "none");
}

#[test]
fn a0616_mm_cvtusepi64_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvtusepi64_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvtusepi64_epi32", "none");
}

#[test]
fn a0617_mm_cvtusepi64_epi8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_cvtusepi64_epi8(core::hint::black_box(n), &mut d); } }
    d.line("_mm_cvtusepi64_epi8", "none");
}

#[test]
fn a0618_mm_dbsad_epu8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_dbsad_epu8(core::hint::black_box(n), &mut d); } }
    d.line("_mm_dbsad_epu8", "none");
}

#[test]
fn a0619_mm_div_round_sd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_div_round_sd(core::hint::black_box(n), &mut d); } }
    d.line("_mm_div_round_sd", "none");
}

#[test]
fn a0620_mm_div_round_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_div_round_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_div_round_ss", "none");
}

#[test]
fn a0621_mm_div_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_div_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_div_ss", "none");
}

#[test]
fn a0622_mm_dp_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_dp_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm_dp_ps", "none");
}

#[test]
fn a0623_mm_dpbsud_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_dpbsud_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_dpbsud_epi32", "none");
}

#[test]
fn a0624_mm_dpbsuds_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_dpbsuds_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_dpbsuds_epi32", "none");
}

#[test]
fn a0625_mm_dpbuud_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_dpbuud_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_dpbuud_epi32", "none");
}

#[test]
fn a0626_mm_dpbuuds_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_dpbuuds_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_dpbuuds_epi32", "none");
}

#[test]
fn a0627_mm_dpwsud_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_dpwsud_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_dpwsud_epi32", "none");
}

#[test]
fn a0628_mm_dpwsuds_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_dpwsuds_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_dpwsuds_epi32", "none");
}

#[test]
fn a0629_mm_dpwusd_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_dpwusd_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_dpwusd_epi32", "none");
}

#[test]
fn a0630_mm_dpwusds_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_dpwusds_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_dpwusds_epi32", "none");
}

#[test]
fn a0631_mm_dpwuud_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_dpwuud_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_dpwuud_epi32", "none");
}

#[test]
fn a0632_mm_dpwuuds_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_dpwuuds_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_dpwuuds_epi32", "none");
}

#[test]
fn a0633_mm_extract_epi16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_extract_epi16(core::hint::black_box(n), &mut d); } }
    d.line("_mm_extract_epi16", "none");
}

#[test]
fn a0634_mm_extract_epi8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_extract_epi8(core::hint::black_box(n), &mut d); } }
    d.line("_mm_extract_epi8", "none");
}

#[test]
fn a0635_mm_extract_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_extract_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm_extract_ps", "none");
}

#[test]
fn a0636_mm_extract_si64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_extract_si64(core::hint::black_box(n), &mut d); } }
    d.line("_mm_extract_si64", "none");
}

#[test]
fn a0637_mm_fixupimm_pd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_fixupimm_pd(core::hint::black_box(n), &mut d); } }
    d.line("_mm_fixupimm_pd", "none");
}

#[test]
fn a0638_mm_fixupimm_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_fixupimm_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm_fixupimm_ps", "none");
}

#[test]
fn a0639_mm_fixupimm_round_sd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_fixupimm_round_sd(core::hint::black_box(n), &mut d); } }
    d.line("_mm_fixupimm_round_sd", "none");
}

#[test]
fn a0640_mm_fixupimm_round_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_fixupimm_round_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_fixupimm_round_ss", "none");
}

#[test]
fn a0641_mm_fixupimm_sd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_fixupimm_sd(core::hint::black_box(n), &mut d); } }
    d.line("_mm_fixupimm_sd", "none");
}

#[test]
fn a0642_mm_fixupimm_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_fixupimm_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_fixupimm_ss", "none");
}

#[test]
fn a0643_mm_floor_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_floor_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_floor_ss", "none");
}

#[test]
fn a0644_mm_fmadd_round_sd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_fmadd_round_sd(core::hint::black_box(n), &mut d); } }
    d.line("_mm_fmadd_round_sd", "none");
}

#[test]
fn a0645_mm_fmadd_round_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_fmadd_round_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_fmadd_round_ss", "none");
}

#[test]
fn a0646_mm_fmadd_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_fmadd_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_fmadd_ss", "none");
}

#[test]
fn a0647_mm_fmsub_round_sd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_fmsub_round_sd(core::hint::black_box(n), &mut d); } }
    d.line("_mm_fmsub_round_sd", "none");
}

#[test]
fn a0648_mm_fmsub_round_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_fmsub_round_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_fmsub_round_ss", "none");
}

#[test]
fn a0649_mm_fmsub_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_fmsub_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_fmsub_ss", "none");
}

#[test]
fn a0650_mm_fnmadd_round_sd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_fnmadd_round_sd(core::hint::black_box(n), &mut d); } }
    d.line("_mm_fnmadd_round_sd", "none");
}

#[test]
fn a0651_mm_fnmadd_round_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_fnmadd_round_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_fnmadd_round_ss", "none");
}

#[test]
fn a0652_mm_fnmadd_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_fnmadd_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_fnmadd_ss", "none");
}

#[test]
fn a0653_mm_fnmsub_round_sd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_fnmsub_round_sd(core::hint::black_box(n), &mut d); } }
    d.line("_mm_fnmsub_round_sd", "none");
}

#[test]
fn a0654_mm_fnmsub_round_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_fnmsub_round_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_fnmsub_round_ss", "none");
}

#[test]
fn a0655_mm_fnmsub_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_fnmsub_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_fnmsub_ss", "none");
}

#[test]
fn a0656_mm_getexp_round_sd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_getexp_round_sd(core::hint::black_box(n), &mut d); } }
    d.line("_mm_getexp_round_sd", "none");
}

#[test]
fn a0657_mm_getexp_round_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_getexp_round_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_getexp_round_ss", "none");
}

#[test]
fn a0658_mm_getexp_sd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_getexp_sd(core::hint::black_box(n), &mut d); } }
    d.line("_mm_getexp_sd", "none");
}

#[test]
fn a0659_mm_getexp_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_getexp_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_getexp_ss", "none");
}

#[test]
fn a0660_mm_getmant_round_sd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_getmant_round_sd(core::hint::black_box(n), &mut d); } }
    d.line("_mm_getmant_round_sd", "none");
}

#[test]
fn a0661_mm_getmant_round_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_getmant_round_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_getmant_round_ss", "none");
}

#[test]
fn a0662_mm_getmant_sd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_getmant_sd(core::hint::black_box(n), &mut d); } }
    d.line("_mm_getmant_sd", "none");
}

#[test]
fn a0663_mm_getmant_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_getmant_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_getmant_ss", "none");
}

#[test]
fn a0664_mm_hadd_epi16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_hadd_epi16(core::hint::black_box(n), &mut d); } }
    d.line("_mm_hadd_epi16", "none");
}

#[test]
fn a0665_mm_hadd_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_hadd_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm_hadd_ps", "none");
}

#[test]
fn a0666_mm_hadds_epi16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_hadds_epi16(core::hint::black_box(n), &mut d); } }
    d.line("_mm_hadds_epi16", "none");
}

#[test]
fn a0667_mm_hsub_epi16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_hsub_epi16(core::hint::black_box(n), &mut d); } }
    d.line("_mm_hsub_epi16", "none");
}

#[test]
fn a0668_mm_hsub_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_hsub_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_hsub_epi32", "none");
}

#[test]
fn a0669_mm_hsub_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_hsub_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm_hsub_ps", "none");
}

#[test]
fn a0670_mm_hsubs_epi16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_hsubs_epi16(core::hint::black_box(n), &mut d); } }
    d.line("_mm_hsubs_epi16", "none");
}

#[test]
fn a0671_mm_insert_epi16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_insert_epi16(core::hint::black_box(n), &mut d); } }
    d.line("_mm_insert_epi16", "none");
}

#[test]
fn a0672_mm_insert_epi8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_insert_epi8(core::hint::black_box(n), &mut d); } }
    d.line("_mm_insert_epi8", "none");
}

#[test]
fn a0673_mm_insert_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_insert_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm_insert_ps", "none");
}

#[test]
fn a0674_mm_insert_si64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_insert_si64(core::hint::black_box(n), &mut d); } }
    d.line("_mm_insert_si64", "none");
}

#[test]
fn a0675_mm_lfence() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_lfence(core::hint::black_box(n), &mut d); } }
    d.line("_mm_lfence", "none");
}

#[test]
fn a0676_mm_load1_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_load1_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm_load1_ps", "none");
}

#[test]
fn a0677_mm_load_ps1() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_load_ps1(core::hint::black_box(n), &mut d); } }
    d.line("_mm_load_ps1", "none");
}

#[test]
fn a0678_mm_load_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_load_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_load_ss", "none");
}

#[test]
fn a0679_mm_loadr_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_loadr_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm_loadr_ps", "none");
}

#[test]
fn a0680_mm_loadu_si16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_loadu_si16(core::hint::black_box(n), &mut d); } }
    d.line("_mm_loadu_si16", "none");
}

#[test]
fn a0681_mm_loadu_si32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_loadu_si32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_loadu_si32", "none");
}

#[test]
fn a0682_mm_loadu_si64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_loadu_si64(core::hint::black_box(n), &mut d); } }
    d.line("_mm_loadu_si64", "none");
}

#[test]
fn a0683_mm_max_round_sd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_max_round_sd(core::hint::black_box(n), &mut d); } }
    d.line("_mm_max_round_sd", "none");
}

#[test]
fn a0684_mm_max_round_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_max_round_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_max_round_ss", "none");
}

#[test]
fn a0685_mm_max_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_max_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_max_ss", "none");
}

#[test]
fn a0686_mm_min_round_sd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_min_round_sd(core::hint::black_box(n), &mut d); } }
    d.line("_mm_min_round_sd", "none");
}

#[test]
fn a0687_mm_min_round_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_min_round_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_min_round_ss", "none");
}

#[test]
fn a0688_mm_min_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_min_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_min_ss", "none");
}

#[test]
fn a0689_mm_minpos_epu16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_minpos_epu16(core::hint::black_box(n), &mut d); } }
    d.line("_mm_minpos_epu16", "none");
}

#[test]
fn a0690_mm_move_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_move_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_move_ss", "none");
}

#[test]
fn a0691_mm_moveldup_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_moveldup_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm_moveldup_ps", "none");
}

#[test]
fn a0692_mm_movelh_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_movelh_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm_movelh_ps", "none");
}

#[test]
fn a0693_mm_mpsadbw_epu8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_mpsadbw_epu8(core::hint::black_box(n), &mut d); } }
    d.line("_mm_mpsadbw_epu8", "none");
}

#[test]
fn a0694_mm_mul_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_mul_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_mul_epi32", "none");
}

#[test]
fn a0695_mm_mul_round_sd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_mul_round_sd(core::hint::black_box(n), &mut d); } }
    d.line("_mm_mul_round_sd", "none");
}

#[test]
fn a0696_mm_mul_round_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_mul_round_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_mul_round_ss", "none");
}

#[test]
fn a0697_mm_mul_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_mul_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_mul_ss", "none");
}

#[test]
fn a0698_mm_mulhi_epi16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_mulhi_epi16(core::hint::black_box(n), &mut d); } }
    d.line("_mm_mulhi_epi16", "none");
}

#[test]
fn a0699_mm_mulhrs_epi16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_mulhrs_epi16(core::hint::black_box(n), &mut d); } }
    d.line("_mm_mulhrs_epi16", "none");
}

#[test]
fn a0700_mm_pause() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_pause(core::hint::black_box(n), &mut d); } }
    d.line("_mm_pause", "none");
}

#[test]
fn a0701_mm_permute_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_permute_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm_permute_ps", "none");
}

#[test]
fn a0702_mm_permutevar_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_permutevar_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm_permutevar_ps", "none");
}

#[test]
fn a0703_mm_range_pd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_range_pd(core::hint::black_box(n), &mut d); } }
    d.line("_mm_range_pd", "none");
}

#[test]
fn a0704_mm_range_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_range_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm_range_ps", "none");
}

#[test]
fn a0705_mm_range_round_sd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_range_round_sd(core::hint::black_box(n), &mut d); } }
    d.line("_mm_range_round_sd", "none");
}

#[test]
fn a0706_mm_range_round_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_range_round_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_range_round_ss", "none");
}

#[test]
fn a0707_mm_rcp14_pd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_rcp14_pd(core::hint::black_box(n), &mut d); } }
    d.line("_mm_rcp14_pd", "none");
}

#[test]
fn a0708_mm_rcp14_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_rcp14_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm_rcp14_ps", "none");
}

#[test]
fn a0709_mm_rcp14_sd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_rcp14_sd(core::hint::black_box(n), &mut d); } }
    d.line("_mm_rcp14_sd", "none");
}

#[test]
fn a0710_mm_rcp14_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_rcp14_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_rcp14_ss", "none");
}

#[test]
fn a0711_mm_rcp_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_rcp_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_rcp_ss", "none");
}

#[test]
fn a0712_mm_reduce_add_epi16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_reduce_add_epi16(core::hint::black_box(n), &mut d); } }
    d.line("_mm_reduce_add_epi16", "none");
}

#[test]
fn a0713_mm_reduce_add_epi8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_reduce_add_epi8(core::hint::black_box(n), &mut d); } }
    d.line("_mm_reduce_add_epi8", "none");
}

#[test]
fn a0714_mm_reduce_and_epi16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_reduce_and_epi16(core::hint::black_box(n), &mut d); } }
    d.line("_mm_reduce_and_epi16", "none");
}

#[test]
fn a0715_mm_reduce_and_epi8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_reduce_and_epi8(core::hint::black_box(n), &mut d); } }
    d.line("_mm_reduce_and_epi8", "none");
}

#[test]
fn a0716_mm_reduce_max_epi16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_reduce_max_epi16(core::hint::black_box(n), &mut d); } }
    d.line("_mm_reduce_max_epi16", "none");
}

#[test]
fn a0717_mm_reduce_max_epi8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_reduce_max_epi8(core::hint::black_box(n), &mut d); } }
    d.line("_mm_reduce_max_epi8", "none");
}

#[test]
fn a0718_mm_reduce_max_epu16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_reduce_max_epu16(core::hint::black_box(n), &mut d); } }
    d.line("_mm_reduce_max_epu16", "none");
}

#[test]
fn a0719_mm_reduce_max_epu8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_reduce_max_epu8(core::hint::black_box(n), &mut d); } }
    d.line("_mm_reduce_max_epu8", "none");
}

#[test]
fn a0720_mm_reduce_min_epi16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_reduce_min_epi16(core::hint::black_box(n), &mut d); } }
    d.line("_mm_reduce_min_epi16", "none");
}

#[test]
fn a0721_mm_reduce_min_epi8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_reduce_min_epi8(core::hint::black_box(n), &mut d); } }
    d.line("_mm_reduce_min_epi8", "none");
}

#[test]
fn a0722_mm_reduce_min_epu16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_reduce_min_epu16(core::hint::black_box(n), &mut d); } }
    d.line("_mm_reduce_min_epu16", "none");
}

#[test]
fn a0723_mm_reduce_min_epu8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_reduce_min_epu8(core::hint::black_box(n), &mut d); } }
    d.line("_mm_reduce_min_epu8", "none");
}

#[test]
fn a0724_mm_reduce_mul_epi16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_reduce_mul_epi16(core::hint::black_box(n), &mut d); } }
    d.line("_mm_reduce_mul_epi16", "none");
}

#[test]
fn a0725_mm_reduce_mul_epi8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_reduce_mul_epi8(core::hint::black_box(n), &mut d); } }
    d.line("_mm_reduce_mul_epi8", "none");
}

#[test]
fn a0726_mm_reduce_or_epi16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_reduce_or_epi16(core::hint::black_box(n), &mut d); } }
    d.line("_mm_reduce_or_epi16", "none");
}

#[test]
fn a0727_mm_reduce_or_epi8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_reduce_or_epi8(core::hint::black_box(n), &mut d); } }
    d.line("_mm_reduce_or_epi8", "none");
}

#[test]
fn a0728_mm_reduce_pd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_reduce_pd(core::hint::black_box(n), &mut d); } }
    d.line("_mm_reduce_pd", "none");
}

#[test]
fn a0729_mm_reduce_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_reduce_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm_reduce_ps", "none");
}

#[test]
fn a0730_mm_reduce_round_sd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_reduce_round_sd(core::hint::black_box(n), &mut d); } }
    d.line("_mm_reduce_round_sd", "none");
}

#[test]
fn a0731_mm_reduce_round_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_reduce_round_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_reduce_round_ss", "none");
}

#[test]
fn a0732_mm_reduce_sd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_reduce_sd(core::hint::black_box(n), &mut d); } }
    d.line("_mm_reduce_sd", "none");
}

#[test]
fn a0733_mm_reduce_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_reduce_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_reduce_ss", "none");
}

#[test]
fn a0734_mm_round_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_round_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_round_ss", "none");
}

#[test]
fn a0735_mm_roundscale_pd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_roundscale_pd(core::hint::black_box(n), &mut d); } }
    d.line("_mm_roundscale_pd", "none");
}

#[test]
fn a0736_mm_roundscale_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_roundscale_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm_roundscale_ps", "none");
}

#[test]
fn a0737_mm_roundscale_round_sd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_roundscale_round_sd(core::hint::black_box(n), &mut d); } }
    d.line("_mm_roundscale_round_sd", "none");
}

#[test]
fn a0738_mm_roundscale_round_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_roundscale_round_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_roundscale_round_ss", "none");
}

#[test]
fn a0739_mm_roundscale_sd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_roundscale_sd(core::hint::black_box(n), &mut d); } }
    d.line("_mm_roundscale_sd", "none");
}

#[test]
fn a0740_mm_roundscale_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_roundscale_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_roundscale_ss", "none");
}

#[test]
fn a0741_mm_rsqrt14_pd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_rsqrt14_pd(core::hint::black_box(n), &mut d); } }
    d.line("_mm_rsqrt14_pd", "none");
}

#[test]
fn a0742_mm_rsqrt14_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_rsqrt14_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm_rsqrt14_ps", "none");
}

#[test]
fn a0743_mm_rsqrt14_sd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_rsqrt14_sd(core::hint::black_box(n), &mut d); } }
    d.line("_mm_rsqrt14_sd", "none");
}

#[test]
fn a0744_mm_rsqrt14_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_rsqrt14_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_rsqrt14_ss", "none");
}

#[test]
fn a0745_mm_rsqrt_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_rsqrt_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_rsqrt_ss", "none");
}

#[test]
fn a0746_mm_scalef_round_sd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_scalef_round_sd(core::hint::black_box(n), &mut d); } }
    d.line("_mm_scalef_round_sd", "none");
}

#[test]
fn a0747_mm_scalef_round_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_scalef_round_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_scalef_round_ss", "none");
}

#[test]
fn a0748_mm_scalef_sd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_scalef_sd(core::hint::black_box(n), &mut d); } }
    d.line("_mm_scalef_sd", "none");
}

#[test]
fn a0749_mm_scalef_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_scalef_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_scalef_ss", "none");
}

#[test]
fn a0750_mm_sfence() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_sfence(core::hint::black_box(n), &mut d); } }
    d.line("_mm_sfence", "none");
}

#[test]
fn a0751_mm_sqrt_round_sd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_sqrt_round_sd(core::hint::black_box(n), &mut d); } }
    d.line("_mm_sqrt_round_sd", "none");
}

#[test]
fn a0752_mm_sqrt_round_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_sqrt_round_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_sqrt_round_ss", "none");
}

#[test]
fn a0753_mm_sqrt_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_sqrt_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_sqrt_ss", "none");
}

#[test]
fn a0754_mm_store1_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_store1_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm_store1_ps", "none");
}

#[test]
fn a0755_mm_store_ps1() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_store_ps1(core::hint::black_box(n), &mut d); } }
    d.line("_mm_store_ps1", "none");
}

#[test]
fn a0756_mm_store_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_store_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_store_ss", "none");
}

#[test]
fn a0757_mm_storer_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_storer_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm_storer_ps", "none");
}

#[test]
fn a0758_mm_storeu_si16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_storeu_si16(core::hint::black_box(n), &mut d); } }
    d.line("_mm_storeu_si16", "none");
}

#[test]
fn a0759_mm_storeu_si32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_storeu_si32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_storeu_si32", "none");
}

#[test]
fn a0760_mm_storeu_si64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_storeu_si64(core::hint::black_box(n), &mut d); } }
    d.line("_mm_storeu_si64", "none");
}

#[test]
fn a0761_mm_stream_load_si128() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_stream_load_si128(core::hint::black_box(n), &mut d); } }
    d.line("_mm_stream_load_si128", "none");
}

#[test]
fn a0762_mm_stream_sd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_stream_sd(core::hint::black_box(n), &mut d); } }
    d.line("_mm_stream_sd", "none");
}

#[test]
fn a0763_mm_stream_si32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_stream_si32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_stream_si32", "none");
}

#[test]
fn a0764_mm_stream_si64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_stream_si64(core::hint::black_box(n), &mut d); } }
    d.line("_mm_stream_si64", "none");
}

#[test]
fn a0765_mm_stream_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_stream_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_stream_ss", "none");
}

#[test]
fn a0766_mm_sub_round_sd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_sub_round_sd(core::hint::black_box(n), &mut d); } }
    d.line("_mm_sub_round_sd", "none");
}

#[test]
fn a0767_mm_sub_round_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_sub_round_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_sub_round_ss", "none");
}

#[test]
fn a0768_mm_sub_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_sub_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_sub_ss", "none");
}

#[test]
fn a0769_mm_test_all_ones() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_test_all_ones(core::hint::black_box(n), &mut d); } }
    d.line("_mm_test_all_ones", "none");
}

#[test]
fn a0770_mm_test_all_zeros() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_test_all_zeros(core::hint::black_box(n), &mut d); } }
    d.line("_mm_test_all_zeros", "none");
}

#[test]
fn a0771_mm_test_mix_ones_zeros() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_test_mix_ones_zeros(core::hint::black_box(n), &mut d); } }
    d.line("_mm_test_mix_ones_zeros", "none");
}

#[test]
fn a0772_mm_testc_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_testc_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm_testc_ps", "none");
}

#[test]
fn a0773_mm_testc_si128() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_testc_si128(core::hint::black_box(n), &mut d); } }
    d.line("_mm_testc_si128", "none");
}

#[test]
fn a0774_mm_testnzc_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_testnzc_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm_testnzc_ps", "none");
}

#[test]
fn a0775_mm_testnzc_si128() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_testnzc_si128(core::hint::black_box(n), &mut d); } }
    d.line("_mm_testnzc_si128", "none");
}

#[test]
fn a0776_mm_testz_ps() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_testz_ps(core::hint::black_box(n), &mut d); } }
    d.line("_mm_testz_ps", "none");
}

#[test]
fn a0777_mm_tzcnt_32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_tzcnt_32(core::hint::black_box(n), &mut d); } }
    d.line("_mm_tzcnt_32", "none");
}

#[test]
fn a0778_mm_tzcnt_64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_tzcnt_64(core::hint::black_box(n), &mut d); } }
    d.line("_mm_tzcnt_64", "none");
}

#[test]
fn a0779_mm_ucomieq_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_ucomieq_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_ucomieq_ss", "none");
}

#[test]
fn a0780_mm_ucomige_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_ucomige_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_ucomige_ss", "none");
}

#[test]
fn a0781_mm_ucomigt_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_ucomigt_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_ucomigt_ss", "none");
}

#[test]
fn a0782_mm_ucomile_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_ucomile_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_ucomile_ss", "none");
}

#[test]
fn a0783_mm_ucomilt_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_ucomilt_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_ucomilt_ss", "none");
}

#[test]
fn a0784_mm_ucomineq_ss() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm_ucomineq_ss(core::hint::black_box(n), &mut d); } }
    d.line("_mm_ucomineq_ss", "none");
}
    #[inline(never)]
    unsafe fn probe_mm256_abs_epi16(n: usize, h: &mut Digest) {
        if feat("avx2") {
            let out = _mm256_abs_epi16(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_abs_epi32(n: usize, h: &mut Digest) {
        if feat("avx2") {
            let out = _mm256_abs_epi32(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_abs_epi64(n: usize, h: &mut Digest) {
        if feat("avx2") {
            let out = _mm256_abs_epi64(input(n+0));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_add_pd(n: usize, h: &mut Digest) {
        if feat("avx") {
            let out = _mm256_add_pd(float_input(n+0), float_input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_adds_epi16(n: usize, h: &mut Digest) {
        if feat("avx2") {
            let out = _mm256_adds_epi16(input(n+0), input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_adds_epi8(n: usize, h: &mut Digest) {
        if feat("avx2") {
            let out = _mm256_adds_epi8(input(n+0), input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_adds_epu16(n: usize, h: &mut Digest) {
        if feat("avx2") {
            let out = _mm256_adds_epu16(input(n+0), input(n+5));
            mix(h, &out);
        }
    }

    #[inline(never)]
    unsafe fn probe_mm256_adds_epu8(n: usize, h: &mut Digest) {
        if feat("avx2") {
            let out = _mm256_adds_epu8(input(n+0), input(n+5));
            mix(h, &out);
        }
    }

#[test]
fn a0786_mm256_abs_epi16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_abs_epi16(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_abs_epi16", "none");
}

#[test]
fn a0787_mm256_abs_epi32() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_abs_epi32(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_abs_epi32", "none");
}

#[test]
fn a0788_mm256_abs_epi64() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_abs_epi64(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_abs_epi64", "none");
}

#[test]
fn a0789_mm256_add_pd() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_add_pd(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_add_pd", "none");
}

#[test]
fn a0790_mm256_adds_epi16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_adds_epi16(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_adds_epi16", "none");
}

#[test]
fn a0791_mm256_adds_epi8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_adds_epi8(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_adds_epi8", "none");
}

#[test]
fn a0792_mm256_adds_epu16() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_adds_epu16(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_adds_epu16", "none");
}

#[test]
fn a0793_mm256_adds_epu8() {
    let mut d = Digest::new();
    unsafe { for n in 0..512 { probe_mm256_adds_epu8(core::hint::black_box(n), &mut d); } }
    d.line("_mm256_adds_epu8", "none");
}

// Inside a function with their target features the safe x86 intrinsics (stdarch's) are called
// without `unsafe`, as rustc's are; the implied features count (avx2 implies avx, ssse3, sse2).
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2,sse4.1,sse4.2")]
fn x86_safe_in_target_feature_fn(x: i32, c: u32) -> (i64, i32, u32, i32) {
    use core::arch::x86_64::*;
    let a = _mm_set1_epi32(x);
    let b = _mm_add_epi32(a, _mm_slli_epi32::<1>(a));
    let s = _mm_shuffle_epi8(b, _mm_setzero_si128());
    let w = _mm256_add_epi32(_mm256_set1_epi32(x), _mm256_set1_epi32(1));
    (_mm_cvtsi128_si64(b), _mm_extract_epi32::<2>(s), _mm_crc32_u32(c, 1), _mm256_extract_epi32::<7>(w))
}

#[cfg(target_arch = "x86_64")]
#[test]
fn x86_safe_in_target_feature() {
    if !(feat("avx2") && feat("sse4.1") && feat("sse4.2")) {
        return;
    }
    let (lo, s, crc, w) = unsafe { x86_safe_in_target_feature_fn(core::hint::black_box(3), 7) };
    let mut r = 7u32;
    for i in 0..32 {
        r ^= (1u32 >> i) & 1;
        r = (r >> 1) ^ (0x82f6_3b78 & 0u32.wrapping_sub(r & 1));
    }
    assert_eq!((lo, s, crc, w), (0x0000_0009_0000_0009, 0x0909_0909, r, 4), "set1/slli/add, shuffle_epi8/extract, crc32_u32, avx2 add/extract");
    println!("x86 safe in target_feature fn: ok");
    
}

