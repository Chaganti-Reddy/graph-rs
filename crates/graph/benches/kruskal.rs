use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use graph::prelude::*;

mod gen;
use gen::random_undirected;

fn bench_kruskal(c: &mut Criterion) {
    let mut group = c.benchmark_group("kruskal");

    for &n in &[500usize, 1_000, 5_000] {
        let g = random_undirected(n, 4);

        group.bench_with_input(BenchmarkId::new("undirected", n), &n, |b, _| {
            b.iter(|| kruskal(black_box(&g)))
        });
    }

    group.finish();
}

fn bench_prim(c: &mut Criterion) {
    let mut group = c.benchmark_group("prim");

    for &n in &[500usize, 1_000, 5_000] {
        let g = random_undirected(n, 4);

        group.bench_with_input(BenchmarkId::new("undirected", n), &n, |b, _| {
            b.iter(|| prim(black_box(&g)))
        });
    }

    group.finish();
}

criterion_group!(benches, bench_kruskal, bench_prim);
criterion_main!(benches);
