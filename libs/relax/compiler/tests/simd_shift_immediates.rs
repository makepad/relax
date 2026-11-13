#![cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

fn avx512(bw: bool) -> bool {
    unsafe {
        let p = __cpuid(1);
        if p.ecx & (3 << 26) != (3 << 26) || _xgetbv(0) & 0xe6 != 0xe6 {
            return false;
        }
        let e = __cpuid_count(7, 0);
        e.ebx & (1 << 16) != 0 && (!bw || e.ebx & (1 << 30) != 0)
    }
}

#[inline(never)]
unsafe fn check_u16<const N: u32>(input: [u16; 32]) {
    let a: __m512i = core::mem::transmute(input);
    let out: [u16; 32] = core::mem::transmute(_mm512_slli_epi16::<N>(a));
    for i in 0..32 {
        let expected = if N >= 16 { 0 } else { input[i] << N };
        assert_eq!(out[i], expected);
    }
}

#[inline(never)]
unsafe fn check_u64<const N: u32>(input: [u64; 8]) {
    let a: __m512i = core::mem::transmute(input);
    let out: [u64; 8] = core::mem::transmute(_mm512_slli_epi64::<N>(a));
    for i in 0..8 {
        let expected = if N >= 64 { 0 } else { input[i] << N };
        assert_eq!(out[i], expected);
    }
}

#[test]
fn shifts_accept_u32_constants_and_handle_boundaries() {
    if !avx512(false) {
        return;
    }
    let mut words = [0u16; 32];
    for i in 0..32 {
        words[i] = (i as u16).wrapping_mul(0x1235).wrapping_add(0x8001);
    }
    let qwords = [
        0,
        1,
        u64::MAX,
        0x8000000000000000,
        0x0123456789abcdef,
        0x5555555555555555,
        0xaaaaaaaaaaaaaaaa,
        0xdeadbeef01234567,
    ];
    unsafe {
        let words = core::hint::black_box(words);
        let qwords = core::hint::black_box(qwords);
        if avx512(true) {
            check_u16::<0>(words);
            check_u16::<1>(words);
            check_u16::<15>(words);
            check_u16::<16>(words);
            check_u16::<17>(words);
            check_u16::<31>(words);
            check_u16::<32>(words);
            check_u16::<63>(words);
            check_u16::<64>(words);
            check_u16::<254>(words);
            check_u16::<255>(words);
        }
        check_u64::<0>(qwords);
        check_u64::<1>(qwords);
        check_u64::<15>(qwords);
        check_u64::<16>(qwords);
        check_u64::<31>(qwords);
        check_u64::<32>(qwords);
        check_u64::<63>(qwords);
        check_u64::<64>(qwords);
        check_u64::<65>(qwords);
        check_u64::<254>(qwords);
        check_u64::<255>(qwords);
    }
}
