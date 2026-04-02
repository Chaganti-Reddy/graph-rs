use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use graph::prelude::*;

mod gen;
use gen::{first_node, random_directed};

fn bench_bfs(c: &mut Criterion) {
    let mut group = c.benchmark_group("bfs");

    for &n in &[500usize, 1_000, 5_000] {
        let g = random_directed(n, 4);
        let src = first_node(&g);

        group.bench_with_input(BenchmarkId::new("directed", n), &n, |b, _| {
            b.iter(|| bfs(black_box(&g), black_box(src)))
        });
    }

    group.finish();
}

criterion_group!(benches, bench_bfs);
criterion_main!(benches);
