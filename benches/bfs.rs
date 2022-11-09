use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, PlottingBackend};
use parallel_algorithms::bfs::{parallel, sequential, CubicGraph};

fn bfs_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("bfs");

    let g_size: usize = 500;
    let from = 0;
    let to = g_size.pow(3) - 1;
    let g = CubicGraph::new(g_size);
    let parameter = &format!("cubic-{g_size}");

    group.bench_with_input(BenchmarkId::new("sequential", parameter), &g, |b, g| {
        b.iter(|| sequential::bfs(g, from, to))
    });

    group.bench_with_input(BenchmarkId::new("parallel", parameter), &g, |b, g| {
        b.iter(|| parallel::bfs_with_thread_pool(g, from, to, 4).expect("Successful pool creation"))
    });

    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default()
        .with_plots()
        .plotting_backend(PlottingBackend::Plotters)
        .sample_size(10);
    targets = bfs_bench
);
criterion_main!(benches);
