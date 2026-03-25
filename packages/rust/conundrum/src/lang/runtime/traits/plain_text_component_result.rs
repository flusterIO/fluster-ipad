use crate::{
    lang::runtime::state::parse_state::ParseState, output::parsing_result::mdx_parsing_result::MdxParsingResult,
};

pub trait PlainTextComponentResult {
    /// Useful mostly for searchability. Formatting may be hideous.
    fn to_plain_text(&self, res: &mut ParseState) -> String;
}
