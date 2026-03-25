use crate::output::parsing_result::mdx_parsing_result::MdxParsingResult;

pub trait MarkdownComponentResult {
    /// Useful mostly for compatibility and some potential integrations in the
    /// future, but creating now just to add the methods as they come about
    /// instead of writing 30 at once.
    fn to_markdown(&self, res: &mut MdxParsingResult) -> String;
}
