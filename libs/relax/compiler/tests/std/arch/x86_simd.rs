// Shared helpers for the core_arch_x86_64_* differential tests: deterministic edge-case
// inputs, mask bits, unaligned load/store scratch space, the FNV-1a accumulator, and
// direct CPUID/XCR0 feature probing (relax's detect reports no AVX-512 while the
// emulated intrinsics compute regardless, so is_x86_feature_detected would skip them).
// Differential reference: rustc --edition 2021 --test -O simd_intrinsics.rs
// Run with --nocapture --test-threads=1 and compare the SIMD lines with makepad test.
// Every bridge wrapper is exercised, including its separately compiled body, across
// lane extrema, saturation boundaries, shuffle sign bits, NaNs and signed zero.

// Feature probing goes through CPUID/XCR0 directly, not is_x86_feature_detected:
// relax's std reports no AVX-512 while its asm! assembler encodes no EVEX, but the
// emulated intrinsics here compute regardless of the reported feature bits, so on an
// AVX-512 host the relax run and the rustc reference would take different branches.
#[cfg(target_arch = "x86_64")]
fn feat(name: &str) -> bool {
    use core::arch::x86_64::{__cpuid, __cpuid_count, _xgetbv, CpuidResult};
    unsafe {
        let root = __cpuid(0);
        if root.eax < 1 { return false; }
        let p = __cpuid(1);
        let zero = CpuidResult { eax: 0, ebx: 0, ecx: 0, edx: 0 };
        let e = if root.eax >= 7 { __cpuid_count(7, 0) } else { zero };
        let e1 = if root.eax >= 7 && e.eax >= 1 { __cpuid_count(7, 1) } else { zero };
        let ext = if __cpuid(0x80000000).eax >= 0x80000001 { __cpuid(0x80000001).ecx } else { 0 };
        let xcr0 = if p.ecx & (3 << 26) == (3 << 26) { _xgetbv(0) } else { 0 };
        let avx = xcr0 & 6 == 6 && p.ecx & (1 << 28) != 0;
        let avx512 = avx && xcr0 & 0xe6 == 0xe6 && e.ebx & (1 << 16) != 0;
        let amd = (root.ebx == 0x68747541 && root.edx == 0x69746e65 && root.ecx == 0x444d4163)
            || (root.ebx == 0x6f677948 && root.edx == 0x6e65476e && root.ecx == 0x656e6975);
        match name {
            "sse" => p.edx & (1 << 25) != 0,
            "sse2" => p.edx & (1 << 26) != 0,
            "sse3" => p.ecx & 1 != 0,
            "ssse3" => p.ecx & (1 << 9) != 0,
            "sse4.1" => p.ecx & (1 << 19) != 0,
            "sse4.2" => p.ecx & (1 << 20) != 0,
            "sse4a" => amd && ext & (1 << 6) != 0,
            "popcnt" => p.ecx & (1 << 23) != 0,
            "avx" => avx,
            "avx2" => avx && e.ebx & (1 << 5) != 0,
            "fma" => avx && p.ecx & (1 << 12) != 0,
            "f16c" => avx && p.ecx & (1 << 29) != 0,
            "bmi1" => e.ebx & (1 << 3) != 0,
            "bmi2" => e.ebx & (1 << 8) != 0,
            "lzcnt" => ext & (1 << 5) != 0,
            "tbm" => amd && ext & (1 << 21) != 0,
            "adx" => e.ebx & (1 << 19) != 0,
            "sha" => e.ebx & (1 << 29) != 0,
            "pclmulqdq" => p.ecx & (1 << 1) != 0,
            "vpclmulqdq" => avx && e.ecx & (1 << 10) != 0,
            "aes" => p.ecx & (1 << 25) != 0,
            "vaes" => avx && e.ecx & (1 << 9) != 0,
            "gfni" => e.ecx & (1 << 8) != 0,
            "avx512f" => avx512,
            "avx512bw" => avx512 && e.ebx & (1 << 30) != 0,
            "avx512vl" => avx512 && e.ebx & (1 << 31) != 0,
            "avx512dq" => avx512 && e.ebx & (1 << 17) != 0,
            "avx512cd" => avx512 && e.ebx & (1 << 28) != 0,
            "avx512vbmi" => avx512 && e.ecx & (1 << 1) != 0,
            "avx512vbmi2" => avx512 && e.ecx & (1 << 6) != 0,
            "avx512vnni" => avx512 && e.ecx & (1 << 11) != 0,
            "avx512bitalg" => avx512 && e.ecx & (1 << 12) != 0,
            "avx512vpopcntdq" => avx512 && e.ecx & (1 << 14) != 0,
            "avx512ifma" => avx512 && e.ebx & (1 << 21) != 0,
            "avx512fp16" => avx512 && e.edx & (1 << 23) != 0,
            "avx512bf16" => avx512 && e1.eax & (1 << 5) != 0,
            "avx512vp2intersect" => avx512 && e.edx & (1 << 8) != 0,
            "avxvnni" => avx && e1.eax & (1 << 4) != 0,
            "avxvnniint8" => avx && e1.edx & (1 << 4) != 0,
            "avxvnniint16" => avx && e1.edx & (1 << 10) != 0,
            "avxifma" => avx && e1.eax & (1 << 23) != 0,
            "sha512" => avx512 && e1.eax & 1 != 0,
            "sm3" => avx && e1.eax & (1 << 1) != 0,
            "sm4" => avx && e1.eax & (1 << 2) != 0,
            _ => false,
        }
    }
}
#[cfg(not(target_arch = "x86_64"))]
fn feat(_name: &str) -> bool { false }

fn kbits(n: usize) -> u64 {
    (0x9e3779b97f4a7c15u64).wrapping_mul(n as u64 + 1)
}

static mut SIMDBUF: [u8; 2048] = [0xa5; 2048];

// stream_load/stream_store need natural vector-size alignment, so both
// helpers return addresses snapped to a 64-byte boundary.
fn loadptr(n: usize) -> *const u8 {
    unsafe {
        let p = SIMDBUF.as_ptr();
        p.add((64 - (p as usize & 63)) & 63).add((n % 8) * 64)
    }
}

fn storeptr(n: usize) -> *mut u8 {
    unsafe {
        let p = SIMDBUF.as_mut_ptr();
        p.add(1024 + ((64 - (p as usize & 63)) & 63)).add((n % 8) * 64)
    }
}

fn mix<T: Copy>(d: &mut Digest, v: &T) {
    d.add(bytes(v));
}

fn input<T: Copy>(n: usize) -> T {
    let edges = [
        0u64,
        u64::MAX,
        0x8080808080808080,
        0x7f7f7f7f7f7f7f7f,
        0x8000800080008000,
        0x7fff7fff7fff7fff,
        0x8000000080000000,
        0x7fffffff7fffffff,
        0x8000000000000000,
        0x7fffffffffffffff,
        0x00ff0100007f0080,
        0xffff00010000ffff,
        0x0000ffff00010000,
        0x000102030e0f1011,
        0x7f80ff1020304050,
        0x0123456789abcdef,
    ];
    // Sized to the widest vector under test (__m512, 8 words).
    let mut words = [0u64; 8];
    for i in 0..core::mem::size_of::<T>() / 8 {
        words[i] = edges[(n + i * (n / 16)) % 16];
    }
    unsafe { core::ptr::read_unaligned(words.as_ptr() as *const T) }
}

fn float_input<T: Copy>(n: usize) -> T {
    // One quiet NaN payload, to avoid unspecified choices between two payloads
    // in commutative arithmetic; min/max still verify NaN and signed-zero bits.
    let bits = [
        0u32, 0x80000000, 0x3f800000, 0xbf800000, 0x7f800000, 0xff800000, 0x7fc00000, 1,
        0x7f7fffff, 0x00800000, 0x40800000, 0xc0800000,
    ];
    // Sized to the widest vector under test (__m512, 16 lanes).
    let mut words = [0u32; 16];
    for i in 0..core::mem::size_of::<T>() / 4 {
        words[i] = bits[(n + i) % 12];
    }
    unsafe { core::ptr::read_unaligned(words.as_ptr() as *const T) }
}

// Exercise both NaN operands and signaling/quiet priority as well as signed zeros.
fn minmax_input<T: Copy>(n: usize) -> T {
    let bits = [
        0u32, 0x80000000, 0x3f800000, 0xbf800000, 0x7f800000, 0xff800000, 0x7fc12345, 0xffc54321,
        0x7f800001, 0xff800123, 1, 0x80000001, 0x7f7fffff, 0xff7fffff, 0x00800000, 0x80800000,
    ];
    let mut words = [0u32; 4];
    for i in 0..4 {
        words[i] = bits[(n + i) % 16];
    }
    unsafe { core::ptr::read_unaligned(words.as_ptr() as *const T) }
}

