use criterion::{criterion_group, criterion_main, Criterion};

use advent_of_code::*;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("d1-p1", |b| b.iter(day_1::part_1::run));
    c.bench_function("d1-p2", |b| b.iter(day_1::part_2::run));
    c.bench_function("d2-p1", |b| b.iter(day_2::part_1::run));
    c.bench_function("d2-p2", |b| b.iter(day_2::part_2::run));
    c.bench_function("d3-p1", |b| b.iter(day_3::part_1::run));
    c.bench_function("d3-p2", |b| b.iter(day_3::part_2::run));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);