use criterion::{
    criterion_group, criterion_main, measurement::Measurement, BenchmarkGroup, BenchmarkId,
    Criterion, PlottingBackend,
};
use parallel_algorithms::bfs::{parallel, sequential, AdjacencyMatrix};

fn add_bench<M: Measurement, F>(
    group: &mut BenchmarkGroup<M>,
    name: &str,
    g: &AdjacencyMatrix,
    f: F,
) where
    F: Fn(&AdjacencyMatrix) -> Vec<usize>,
{
    group.bench_with_input(BenchmarkId::new(name, g.len()), g, |b, g| b.iter(|| f(g)));
}

fn bfs_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("bfs");

    let g = AdjacencyMatrix::new(); // TODO generate cubic graph 500x500
    add_bench(&mut group, "sequential", &g, sequential::bfs);
    add_bench(&mut group, "parallel: 4 threads", &g, |v| {
        parallel::bfs_with_thread_pool(v, 4).expect("Successful pool creation")
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
