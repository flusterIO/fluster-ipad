use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use fluster_core_utilities::{
    core_types::development_utils::benchmark_magnitudes::{
        BenchmarkGeneratedDateString, BenchmarkMagnitude,
    },
    test_utilities::get_test_mdx_content::get_unchanging_benchmark_test_content,
};
use fluster_pre_parser::parse::by_regex::parse_mdx_by_regex::{
    ParseMdxOptions, parse_mdx_string_to_mdx_result,
};
use strum::IntoEnumIterator;
use tokio::runtime::Runtime;

fn pre_parse_by_regex_benchmark(c: &mut Criterion) {
    let root = env!("CARGO_MANIFEST_DIR");
    let which_file_to_execute_path = std::path::Path::new(root)
        .join("docs")
        .join("development")
        .join("script_inputs")
        .join("benchmark_current_file.txt");
    let which_file_to_test = std::fs::read_to_string(which_file_to_execute_path)
        .expect("Reads cross-language 'benchmark_current_file.txt' file.");
    let which_file_is_in_enum =
        BenchmarkGeneratedDateString::string_is_valid_date(&which_file_to_test);

    if !which_file_is_in_enum {
        eprintln!(
            "The {} date is not in the Rust enum. This is required for data exploration later.",
            which_file_to_test
        )
    }
    let rt = Runtime::new().unwrap();
    let benchmark_date = BenchmarkGeneratedDateString::get_latest();

    BenchmarkMagnitude::iter().for_each(|mag| {
        c.bench_function(
            &format!("parse_mdx_by_regex_{:?}", mag.clone() as u32),
            |b| {
                b.to_async(&rt).iter_batched(
                    // 1. SETUP: This runs before the timer starts
                    || get_unchanging_benchmark_test_content(mag.clone(), &benchmark_date),
                    // 2. MEASUREMENT: This is what is timed
                    |file_content| async move {
                        parse_mdx_string_to_mdx_result(&ParseMdxOptions {
                            content: file_content,
                            note_id: None,
                            citations: Vec::new(),
                        })
                        .await
                    },
                    // 3. BATCH SIZE: Controls how many setups happen at once
                    BatchSize::SmallInput,
                );
            },
        );
    });
}

criterion_group!(benches, pre_parse_by_regex_benchmark);
criterion_main!(benches);
