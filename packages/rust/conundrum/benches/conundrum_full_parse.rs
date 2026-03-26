use conundrum::lang::runtime::run_conundrum::{ParseMdxOptions, run_conundrum};
use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use fluster_core_utilities::{
    core_types::development_utils::benchmark_magnitudes::{BenchmarkGeneratedDateString, BenchmarkMagnitude},
    test_utilities::{
        get_test_mdx_content::{format_benchmark_file_path, get_unchanging_benchmark_test_content},
        get_workspace_root::get_workspace_root,
    },
};
use strum::IntoEnumIterator;
use tokio::runtime::Runtime;

fn conundrum_full_parse_benchmark(c: &mut Criterion) {
    let root = get_workspace_root();
    let rt = Runtime::new().unwrap();

    let benchmark_date = BenchmarkGeneratedDateString::get_latest();

    BenchmarkMagnitude::iter().for_each(|mag| {
                                  c.bench_function(
                                                   // docs/development/generated_benchmark_content/
                                                   &format_benchmark_file_path(&mag, &benchmark_date),
                                                   |b| {
                                                       b.to_async(&rt).iter_batched(
                    // 1. SETUP: This runs before the timer starts
                    || get_unchanging_benchmark_test_content(mag, &benchmark_date),
                    // 2. MEASUREMENT: This is what is timed
                    |file_content| async move {
                        run_conundrum(ParseMdxOptions {
                            note_id: None,
                            content: file_content,
                            modifiers: Vec::new(),
                        })
                        .await
                    },
                    // 3. BATCH SIZE: Controls how many setups happen at once
                    BatchSize::LargeInput,
                );
                                                   },
        );
                              });
}

criterion_group!(benches, conundrum_full_parse_benchmark);
criterion_main!(benches);
