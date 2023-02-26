use bnf::*;
use criterion::measurement::WallTime;
use criterion::{
    black_box, criterion_group, criterion_main, BenchmarkGroup, BenchmarkId, Criterion,
    PlottingBackend,
};
use std::time::Duration;

fn bench_generator(group: &mut BenchmarkGroup<WallTime>, input: (Grammar, GenerationStrategy)) {
    let mut seed = 0;
    group.bench_with_input(
        BenchmarkId::new("generate_parameterized", "bnf_uni"),
        &input,
        |b, input| {
            b.iter_batched(
                || {
                    seed += 1;
                    seed
                },
                |seed| {
                    input
                        .0
                        .generate_parameterized(black_box(input.1), black_box(seed))
                        .unwrap()
                },
                criterion::BatchSize::SmallInput,
            );
        },
    );
}

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("generator_benches");
    bench_generator(
        &mut group,
        (
            bnf::playground::grammar_bnf(),
            GenerationStrategy::UniformRHSSampling,
        ),
    );
    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .warm_up_time(Duration::new(2, 0))
        .sample_size(300)
        .plotting_backend(PlottingBackend::Plotters);
    targets = bench
}
criterion_main!(benches);
