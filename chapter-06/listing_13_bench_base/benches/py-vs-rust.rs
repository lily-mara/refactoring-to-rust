use criterion::{black_box, criterion_group, criterion_main, Criterion};

criterion_main!(python_vs_rust);
criterion_group!(python_vs_rust, bench_fn);

fn bench_fn(c: &mut Criterion) {
  c.bench_function("u8", |b| {
    b.iter(|| {
      black_box(3u8 + 4);
    });
  });

  c.bench_function("u128", |b| {
    b.iter(|| {
      black_box(3u128 + 4);
    });
  });
}
