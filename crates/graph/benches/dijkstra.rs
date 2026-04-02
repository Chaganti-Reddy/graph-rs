use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use graph::prelude::*;

mod gen;
use gen::{first_node, random_directed};

fn bench_dijkstra(c: &mut Criterion) {
    let mut group = c.benchmark_group("dijkstra");

    for &n in &[500usize, 1_000, 5_000] {
        let g = random_directed(n, 4);
        let src = first_node(&g);

        group.bench_with_input(BenchmarkId::new("directed", n), &n, |b, _| {
            b.iter(|| dijkstra(black_box(&g), black_box(src)).unwrap())
        });
    }

    group.finish();
}

criterion_group!(benches, bench_dijkstra);
criterion_main!(benches);
