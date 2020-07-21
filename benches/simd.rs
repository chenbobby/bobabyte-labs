use bobabyte_labs::simd;
use criterion::{criterion_group, criterion_main, Criterion};
use rand::Rng;

///
/// Constants
///
const VECTOR_LENGTH: usize = 1 << 10;
const MIN: i32 = i32::MIN / 2;
const MAX: i32 = i32::MAX / 2;

///
/// Benchmark Helpers
///

/// Returns a vector of i32 elements with random values.
fn gen_random_vector() -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let mut v = vec![0; VECTOR_LENGTH];
    for i in 0..VECTOR_LENGTH {
        v[i] = rng.gen_range(MIN, MAX);
    }
    v
}

/// Returns a Box<Data32> with random values.
fn gen_random_data32() -> Box<simd::Data32> {
    let mut rng = rand::thread_rng();
    let mut v = simd::Data32::new();
    for i in 0..simd::DATA32_LENGTH {
        v.0[i] = rng.gen_range(MIN, MAX);
    }
    Box::new(v)
}

///
/// Benchmarks
///

pub fn vector_add_benchmark(c: &mut Criterion) {
    let name = format!("simd::vector_add ({})", VECTOR_LENGTH);
    let v1 = gen_random_vector();
    let v2 = gen_random_vector();
    let mut result = vec![0; VECTOR_LENGTH];
    c.bench_function(&name, |b| {
        b.iter(|| {
            simd::vector_add(&v1, &v2, &mut result);
        })
    });
}

pub fn vector_add_sse2_benchmark(c: &mut Criterion) {
    let name = format!("simd::vector_add_sse2 ({})", VECTOR_LENGTH);
    let v1 = gen_random_vector();
    let v2 = gen_random_vector();
    let mut result = vec![0; VECTOR_LENGTH];
    c.bench_function(&name, |b| {
        b.iter(|| unsafe {
            simd::vector_add_sse2(&v1, &v2, &mut result);
        })
    });
}

pub fn vector_add_avx2_benchmark(c: &mut Criterion) {
    let name = format!("simd::vector_add_avx2 ({})", simd::DATA32_LENGTH);
    let v1 = gen_random_data32();
    let v2 = gen_random_data32();
    let mut result = simd::Data32::new();
    c.bench_function(&name, |b| {
        b.iter(|| unsafe {
            simd::vector_add_sse2(&v1.0, &v2.0, &mut result.0);
        })
    });
}

criterion_group!(
    benches,
    vector_add_benchmark,
    vector_add_sse2_benchmark,
    vector_add_avx2_benchmark,
);
criterion_main!(benches);
