use bad64::decode;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("decode nop", |b| b.iter(|| decode(0xd503201f, 0x1000)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
