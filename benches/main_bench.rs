use quatro_in_rust::themain;
use criterion::{criterion_group, criterion_main, Criterion};


fn main_benchmark(c: &mut Criterion) {
    c.bench_function("themain", |b| b.iter(|| themain()));
}

criterion_group!(benches, main_benchmark);
criterion_main!(benches);
