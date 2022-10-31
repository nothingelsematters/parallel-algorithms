use criterion::{criterion_group, criterion_main, Criterion, PlottingBackend};

fn bfs_bench(_c: &mut Criterion) {
    todo!()
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
