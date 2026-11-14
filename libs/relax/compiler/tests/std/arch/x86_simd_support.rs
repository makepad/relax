// Feature probing goes through CPUID/XCR0 directly, not is_x86_feature_detected:
// relax's std reports no AVX-512 while its asm! assembler encodes no EVEX, but the
// emulated intrinsics here compute regardless of the reported feature bits, so on an
// AVX-512 host the relax run and the rustc reference would take different branches.
#[cfg(target_arch = "x86_64")]
fn feat(name: &str) -> bool {
    use core::arch::x86_64::{__cpuid, __cpuid_count, _xgetbv, CpuidResult};
    unsafe {
        let root = __cpuid(0);
        if root.eax < 1 {
            return false;
        }
        let p = __cpuid(1);
        let zero = CpuidResult {
            eax: 0,
            ebx: 0,
            ecx: 0,
            edx: 0,
        };
        let e = if root.eax >= 7 {
            __cpuid_count(7, 0)
        } else {
            zero
        };
        let e1 = if root.eax >= 7 && e.eax >= 1 {
            __cpuid_count(7, 1)
        } else {
            zero
        };
        let ext = if __cpuid(0x80000000).eax >= 0x80000001 {
            __cpuid(0x80000001).ecx
        } else {
            0
        };
        let xcr0 = if p.ecx & (3 << 26) == (3 << 26) {
            _xgetbv(0)
        } else {
            0
        };
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
fn feat(_name: &str) -> bool {
    false
}

fn kbits(n: usize) -> u64 {
    (0x9e3779b97f4a7c15u64).wrapping_mul(n as u64 + 1)
}

static mut SIMDBUF: [u8; 2048] = [0xa5; 2048];

// stream_load/stream_store need natural vector-size alignment, so both
// helpers return addresses snapped to a 64-byte boundary.
fn loadptr(n: usize) -> *const u8 {
    unsafe {
        let p = core::ptr::addr_of!(SIMDBUF) as *const u8;
        p.add((64 - (p as usize & 63)) & 63).add((n % 8) * 64)
    }
}

fn storeptr(n: usize) -> *mut u8 {
    unsafe {
        let p = core::ptr::addr_of_mut!(SIMDBUF) as *mut u8;
        p.add(1024 + ((64 - (p as usize & 63)) & 63))
            .add((n % 8) * 64)
    }
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
    // Only integer scalars and padding-free integer vectors up to 64 bytes are valid here.
    // Every initialized bit pattern is valid for those types.
    let mut words = [0u64; 8];
    for i in 0..(core::mem::size_of::<T>() + 7) / 8 {
        words[i] = edges[(n + i * (n / 16)) % 16];
    }
    unsafe { core::ptr::read_unaligned(words.as_ptr() as *const T) }
}

// Each fixture constructs values of the actual lane type. Only padding-free x86 vector
// types with the exact array size below are read as vectors.
fn float_lanes_f32<const N: usize>(n: usize) -> [f32; N] {
    let bits = [
        0x00000000u32,
        0x80000000,
        0x3f800000,
        0xbf800000,
        0x7f800000,
        0xff800000,
        0x7fc00000,
        0x00000001,
        0x80000001,
        0x00800000,
        0x7f7fffff,
        0x40800000,
        0xc0800000,
    ];
    let mut result = [0.0; N];
    let mut i = 0;
    while i < N {
        result[i] = f32::from_bits(bits[(n + i) % bits.len()]);
        i += 1;
    }
    result
}

fn float_lanes_f64<const N: usize>(n: usize) -> [f64; N] {
    let bits = [
        0x0000000000000000u64,
        0x8000000000000000,
        0x3ff0000000000000,
        0xbff0000000000000,
        0x7ff0000000000000,
        0xfff0000000000000,
        0x7ff8000000000000,
        0x0000000000000001,
        0x8000000000000001,
        0x0010000000000000,
        0x7fefffffffffffff,
        0x4010000000000000,
        0xc010000000000000,
    ];
    let mut result = [0.0; N];
    let mut i = 0;
    while i < N {
        result[i] = f64::from_bits(bits[(n + i) % bits.len()]);
        i += 1;
    }
    result
}

trait FloatFixture: Copy {
    fn fixture(n: usize) -> Self;
}
impl FloatFixture for f32 {
    fn fixture(n: usize) -> Self {
        float_lanes_f32::<1>(n)[0]
    }
}
impl FloatFixture for f64 {
    fn fixture(n: usize) -> Self {
        float_lanes_f64::<1>(n)[0]
    }
}

#[cfg(any(target_arch = "x86_64", relax))]
macro_rules! vector_float_fixture {
    ($ty:ty, $lane:ty, $count:expr, $lanes:ident) => {
        impl FloatFixture for $ty {
            fn fixture(n: usize) -> Self {
                let values: [$lane; $count] = $lanes(n);
                unsafe { core::ptr::read_unaligned(values.as_ptr() as *const Self) }
            }
        }
    };
}
#[cfg(any(target_arch = "x86_64", relax))]
vector_float_fixture!(core::arch::x86_64::__m128, f32, 4, float_lanes_f32);
#[cfg(any(target_arch = "x86_64", relax))]
vector_float_fixture!(core::arch::x86_64::__m256, f32, 8, float_lanes_f32);
#[cfg(any(target_arch = "x86_64", relax))]
vector_float_fixture!(core::arch::x86_64::__m512, f32, 16, float_lanes_f32);
#[cfg(any(target_arch = "x86_64", relax))]
vector_float_fixture!(core::arch::x86_64::__m128d, f64, 2, float_lanes_f64);
#[cfg(any(target_arch = "x86_64", relax))]
vector_float_fixture!(core::arch::x86_64::__m256d, f64, 4, float_lanes_f64);
#[cfg(any(target_arch = "x86_64", relax))]
vector_float_fixture!(core::arch::x86_64::__m512d, f64, 8, float_lanes_f64);

fn float_input<T: FloatFixture>(n: usize) -> T {
    T::fixture(n)
}

fn string_length(n: usize) -> i32 {
    const LENGTHS: [i32; 10] = [0, 1, -1, 8, -8, 16, -16, 17, i32::MIN, i32::MAX];
    LENGTHS[n % LENGTHS.len()]
}

// Exercise both NaN operands and signaling/quiet priority as well as signed zeros.
fn minmax_input<T: Copy>(n: usize) -> T {
    let bits = [
        0u32, 0x80000000, 0x3f800000, 0xbf800000, 0x7f800000, 0xff800000, 0x7fc12345, 0xffc54321,
        0x7f800001, 0xff800123, 1, 0x80000001, 0x7f7fffff, 0xff7fffff, 0x00800000, 0x80800000,
    ];
    // Only a padding-free 128-bit f32 vector is passed by the min/max probes.
    let mut words = [0u32; 4];
    for i in 0..4 {
        words[i] = bits[(n + i) % 16];
    }
    unsafe { core::ptr::read_unaligned(words.as_ptr() as *const T) }
}

#[test]
fn input_integer_edges() {
    assert_eq!(input::<u8>(0), 0);
    assert_eq!(input::<u8>(1), u8::MAX);
    assert_eq!(input::<i8>(2), i8::MIN);
    assert_eq!(input::<i8>(3), i8::MAX);
    assert_eq!(input::<u16>(1), u16::MAX);
    assert_eq!(input::<i16>(4), i16::MIN);
    assert_eq!(input::<i16>(5), i16::MAX);
    assert_eq!(input::<u32>(1), u32::MAX);
    assert_eq!(input::<i32>(6), i32::MIN);
    assert_eq!(input::<i32>(7), i32::MAX);
    assert_eq!(input::<u64>(1), u64::MAX);
    assert_eq!(input::<i64>(8), i64::MIN);
    assert_eq!(input::<i64>(9), i64::MAX);
}

#[test]
fn float_fixture_edges() {
    let f32_bits = [
        0x00000000u32,
        0x80000000,
        0x3f800000,
        0xbf800000,
        0x7f800000,
        0xff800000,
        0x7fc00000,
        1,
        0x80000001,
        0x00800000,
        0x7f7fffff,
    ];
    let f64_bits = [
        0x0000000000000000u64,
        0x8000000000000000,
        0x3ff0000000000000,
        0xbff0000000000000,
        0x7ff0000000000000,
        0xfff0000000000000,
        0x7ff8000000000000,
        1,
        0x8000000000000001,
        0x0010000000000000,
        0x7fefffffffffffff,
    ];
    for i in 0..f32_bits.len() {
        assert_eq!(float_input::<f32>(i).to_bits(), f32_bits[i]);
        assert_eq!(float_input::<f64>(i).to_bits(), f64_bits[i]);
    }
    #[cfg(any(target_arch = "x86_64", relax))]
    unsafe {
        let f: [f64; 2] = core::mem::transmute(float_input::<core::arch::x86_64::__m128d>(4));
        assert_eq!(f[0].to_bits(), f64_bits[4]);
        assert_eq!(f[1].to_bits(), f64_bits[5]);
    }
}
