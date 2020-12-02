use criterion::{criterion_group, criterion_main, Criterion};

use day_1::{find_two, input};

fn day_1_benchmark(c: &mut Criterion) {
    c.bench_function("Day 1", |benchmarker| {
        benchmarker.iter(|| find_two(input(), 2020))
    });
}

criterion_group!(benches, day_1_benchmark);
criterion_main!(benches);
