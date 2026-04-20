use conundrum::lang::runtime::{
    queries::get_title::get_title_group,
    state::parse_state::{ConundrumCompileTarget, ConundrumModifier},
};
use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use fluster_core_utilities::test_utilities::get_test_mdx_content::{self};
use tokio::runtime::Runtime;

fn conundrum_get_title(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    c.bench_function("conundrum_get_title", |b| {
         b.to_async(&rt).iter_batched(get_test_mdx_content::get_test_note_content_with_everything,
                                      |file_content| async move {
                                          get_title_group(file_content,
                                                          vec![ConundrumModifier::PreferInlineMarkdownSyntax],
                                                          ConundrumCompileTarget::Html)
                                      },
                                      BatchSize::LargeInput);
     });
}

criterion_group!(benches, conundrum_get_title);
criterion_main!(benches);
