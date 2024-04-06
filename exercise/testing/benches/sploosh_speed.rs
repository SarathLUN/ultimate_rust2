use criterion::{black_box, criterion_group, criterion_main, Criterion};
use testing::sploosh;

pub fn sploosh_benchmark(c: &mut Criterion) {
    c.bench_function("sploosh (8,9,10)", |b| {
        b.iter(|| black_box(sploosh(8, 9, 10)))
    });
}

criterion_group!(benches, sploosh_benchmark);
criterion_main!(benches);
