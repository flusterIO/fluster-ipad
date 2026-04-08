use crate::lang::runtime::state::{conundrum_error_variant::ConundrumModalResult, parse_state::ParseState};

/// This trait is only necessary when a component outputs some content that is
/// different from it's normal markdown content when .preferInlineMarkdownSyntax
/// is true.
pub trait InlineMarkdownComponentResult {
    fn to_inline_markdown(&self, res: &mut ParseState) -> ConundrumModalResult<String>;
}
