// Shared helpers for the core_arch_x86_64_* differential tests: deterministic edge-case
// inputs, mask bits, unaligned load/store scratch space, the FNV-1a accumulator, and
// direct CPUID/XCR0 feature probing (relax's detect reports no AVX-512 while the
// emulated intrinsics compute regardless, so is_x86_feature_detected would skip them).
// Differential reference: rustc --edition 2021 --test -O simd_intrinsics.rs
// Run with --nocapture --test-threads=1 and compare the SIMD lines with makepad test.
// Every bridge wrapper is exercised, including its separately compiled body, across
// lane extrema, saturation boundaries, shuffle sign bits, NaNs and signed zero.

include!("x86_simd_support.rs");

fn mix<T: Copy>(d: &mut Digest, v: &T) {
    d.add(bytes(v));
}
