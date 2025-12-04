use aoc_2025_02::solve;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_solve(c: &mut Criterion) {
    c.bench_function("solve input.txt", |b| b.iter(|| solve("data/input.txt")));
}

criterion_group!(benches, bench_solve);
criterion_main!(benches);
