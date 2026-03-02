use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use fluster_core_utilities::test_utilities::get_test_mdx_content::get_unchanging_benchmark_test_content;
use fluster_pre_parser::parse::by_regex::parse_mdx_by_regex::{
    ParseMdxOptions, parse_mdx_string_to_mdx_result,
};
use tokio::runtime::Runtime;

fn pre_parse_by_regex_benchmark(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    c.bench_function("process_file_content", |b| {
        b.to_async(&rt).iter_batched(
            // 1. SETUP: This runs before the timer starts
            get_unchanging_benchmark_test_content,
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
    });
}

criterion_group!(benches, pre_parse_by_regex_benchmark);
criterion_main!(benches);
