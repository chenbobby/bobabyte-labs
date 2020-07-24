///
/// Imports
///
use bobabyte_labs::loops;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use rand::Rng;

///
/// Constants
///
const MIN: i32 = i32::MIN;
const MAX: i32 = i32::MAX - 1;

///
/// Benchmark Helpers
///

/// Returns a vector of i32 elements with random values.
fn gen_random_vector(length: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let mut v = vec![0; length];
    for i in 0..length {
        v[i] = rng.gen_range(MIN, MAX);
    }
    v
}

///
/// Benchmarks
///

pub fn vector_increment_fissioned_benchmark(c: &mut Criterion) {
    static KB: usize = 1024;
    let mut group = c.benchmark_group("loops::vector_increment_fissioned");
    for length in [KB, 2 * KB, 4 * KB, 8 * KB, 16 * KB, 32 * KB].iter() {
        let mut v1 = gen_random_vector(*length);
        let mut v2 = gen_random_vector(*length);

        group.throughput(Throughput::Bytes(*length as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(length),
            length,
            |b, _| {
                b.iter(|| loops::vector_increment_fissioned(&mut v1, &mut v2));
            },
        );
    }
    group.finish();
}

pub fn vector_increment_unfissioned_benchmark(c: &mut Criterion) {
    static KB: usize = 1024;
    let mut group = c.benchmark_group("loops::vector_increment_unfissioned");
    for length in [KB, 2 * KB, 4 * KB, 8 * KB, 16 * KB, 32 * KB].iter() {
        let mut v1 = gen_random_vector(*length);
        let mut v2 = gen_random_vector(*length);

        group.throughput(Throughput::Bytes(*length as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(length),
            length,
            |b, _| {
                b.iter(|| loops::vector_increment_unfissioned(&mut v1, &mut v2));
            },
        );
    }
    group.finish();
}

criterion_group!(
    benches,
    vector_increment_fissioned_benchmark,
    // vector_increment_unfissioned_benchmark,
);
criterion_main!(benches);
