use criterion::{
    criterion_group, criterion_main, measurement::Measurement, BatchSize, BenchmarkGroup,
    BenchmarkId, Criterion, PlottingBackend,
};
use parallel_algorithms::quick_sort::{parallel, sequential};

fn add_bench<M: Measurement, F>(group: &mut BenchmarkGroup<M>, name: &str, v: &Vec<u32>, f: F)
where
    F: Fn(&mut [u32]),
{
    group.bench_with_input(BenchmarkId::new(name, v.len()), v, |b, v| {
        b.iter_batched(|| v.clone(), |mut v| f(&mut v), BatchSize::SmallInput)
    });
}

fn quick_sort_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("quick-sort");

    for len in &[
        10_000_000,
        50_000_000,
        100_000_000,
        500_000_000,
        1_000_000_000,
    ] {
        let v: Vec<u32> = (0..*len).map(|_| rand::random::<u32>()).collect();

        add_bench(&mut group, "sequential", &v, sequential::quick_sort);

        add_bench(&mut group, "parallel: 4 threads", &v, |v| {
            parallel::quick_sort_with_thread_pool(v, 4, 1).expect("Successful pool creation")
        });
        add_bench(&mut group, "parallel: 4 threads, 1_000 block", &v, |v| {
            parallel::quick_sort_with_thread_pool(v, 4, 1_000).expect("Successful pool creation")
        });
    }

    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default()
        .with_plots()
        .plotting_backend(PlottingBackend::Plotters)
        .sample_size(10);
    targets = quick_sort_bench
);
criterion_main!(benches);
