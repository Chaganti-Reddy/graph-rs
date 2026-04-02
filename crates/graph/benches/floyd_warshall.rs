use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use graph::prelude::*;

mod gen;
use gen::random_directed;

fn bench_floyd_warshall(c: &mut Criterion) {
    let mut group = c.benchmark_group("floyd_warshall");

    // Floyd-Warshall is O(V³) — keep sizes small; 500 nodes = 125 M iterations.
    for &n in &[50usize, 100, 200] {
        let g = random_directed(n, 4);

        group.bench_with_input(BenchmarkId::new("directed", n), &n, |b, _| {
            b.iter(|| floyd_warshall(black_box(&g)).unwrap())
        });
    }

    group.finish();
}

criterion_group!(benches, bench_floyd_warshall);
criterion_main!(benches);
