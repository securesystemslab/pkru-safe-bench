use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

use libuntrusted::safe_do_nothing;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("do_nothing_0ms", |b| {
        b.iter(|| safe_do_nothing(Box::new(black_box(10)), 0))
    });
    c.bench_function("do_nothing_10ms", |b| {
        b.iter(|| safe_do_nothing(Box::new(black_box(10)), 10))
    });
    let mut group = c.benchmark_group("Do Nothing");
    for i in [0, 100].iter() {
        group.bench_with_input(BenchmarkId::new("Empty", i), i, |b, i| {
            b.iter(|| safe_do_nothing(Box::new(*i), 0))
        });
        group.bench_with_input(BenchmarkId::new("Some Work", i), i, |b, i| {
            b.iter(|| safe_do_nothing(Box::new(*i), *i as usize))
        });
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
