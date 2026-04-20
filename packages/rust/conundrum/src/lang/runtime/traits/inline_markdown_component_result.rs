use crate::lang::runtime::{
    state::{conundrum_error_variant::ConundrumModalResult, parse_state::ParseState},
    traits::conundrum_input::ArcState,
};

/// This trait is only necessary when a component outputs some content that is
/// different from it's normal markdown content when .preferInlineMarkdownSyntax
/// is true.
pub trait InlineMarkdownComponentResult {
    fn to_inline_markdown(&self, res: ArcState) -> ConundrumModalResult<String>;
}
