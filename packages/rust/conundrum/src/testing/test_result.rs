use crate::output::parsing_result::mdx_parsing_result::MdxParsingResult;
use scraper::{Html, Selector};
use syntect::{highlighting::Style, util::as_24_bit_terminal_escaped};

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

    pub fn log_html(&self) {
        let assets = syntect_assets::assets::HighlightingAssets::from_binary();
        let ss = assets.get_syntax_set().expect("Gets syntaxes.");
        let s = ss.find_syntax_by_name("HTML").unwrap();
        let t = assets.get_theme("dracula");
        let mut x = syntect::easy::HighlightLines::new(s, t);
        for l in self.0.content.lines() {
            let z: Vec<(Style, &str)> = x.highlight(l, ss);
            let escaped = as_24_bit_terminal_escaped(&z, true);
            println!("{}", escaped);
        }
    }
}
