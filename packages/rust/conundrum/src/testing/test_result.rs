use crate::output::parsing_result::mdx_parsing_result::MdxParsingResult;
use scraper::{Html, Selector};

/// A simple utilty struct that does absolutely nothing for now. Will be more
/// useful when I get back on wifi and can install some html utils.
pub struct TestResult(pub MdxParsingResult);

impl TestResult {
    pub fn count_css_query(&self, query: &str) -> usize {
        let content = Html::parse_fragment(&self.0.content);
        let selector = Selector::parse(query).expect("Parses a valid css selector");
        let mut count = 0;

        for _ in content.select(&selector) {
            count += 1;
        }
        count
    }
}
