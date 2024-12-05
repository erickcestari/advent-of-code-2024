use criterion::{criterion_group, criterion_main, Criterion};

use advent_of_code::*;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("d5-p1", |b| b.iter(day_5::part_1::run));
    c.bench_function("d5-p2", |b| b.iter(day_5::part_2::run));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
