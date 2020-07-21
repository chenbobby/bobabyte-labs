use criterion::{
    Criterion,
    criterion_group,
    criterion_main,
};
use rand::Rng;
use bobabyte_labs::simd;

/// 
/// Constants
///
const VECTOR_LENGTH: usize = 1 << 20;
const MIN: i32 = i32::MIN/2;
const MAX: i32 = i32::MAX/2;

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

pub fn vector_add_benchmark(c: &mut Criterion) {
    let name = format!("simd::vector_add ({})", VECTOR_LENGTH);
    let v1 = gen_random_vector();
    let v2 = gen_random_vector();
    let mut result = vec![0; VECTOR_LENGTH];
    c.bench_function(&name, |b| b.iter(|| {
        simd::vector_add(&v1, &v2, &mut result);
    }));
}

pub fn vector_add_sse2_benchmark(c: &mut Criterion) {
    let name = format!("simd::vector_add_sse2 ({})", VECTOR_LENGTH);
    let v1 = gen_random_vector();
    let v2 = gen_random_vector();
    let mut result = vec![0; VECTOR_LENGTH];
    c.bench_function(&name, |b| b.iter(|| {
        unsafe { simd::vector_add_sse2(&v1, &v2, &mut result); }
    }));
}

/// Benchmarking
criterion_group!(benches, vector_add_benchmark, vector_add_sse2_benchmark);
criterion_main!(benches);
