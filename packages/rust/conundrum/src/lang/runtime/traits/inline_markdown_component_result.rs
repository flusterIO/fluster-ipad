use crate::lang::runtime::{state::conundrum_error_variant::ConundrumModalResult, traits::conundrum_input::ArcState};

/// This trait is only necessary when a component outputs some content that is
/// different from it's normal markdown content when .preferInlineMarkdownSyntax
/// is true.
///
/// The goal with this is to be similar in output to SwiftUI's inline markdown
/// syntax support.
pub trait InlineMarkdownComponentResult {
    fn to_inline_markdown(&self, res: ArcState) -> ConundrumModalResult<String>;
}
