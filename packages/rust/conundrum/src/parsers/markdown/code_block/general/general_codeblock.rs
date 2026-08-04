use std::sync::Arc;

use parking_lot::Mutex;
use syntect_assets::assets::HighlightingAssets;

use crate::parsers::markdown::code_block::{
    supported_languages::SupportedCodeBlockSyntax, supported_themes::SupportedCodeBlockTheme,
};

/// A codeblock that does not render or do anything special, just plain ole'
/// markdown.
pub struct GeneralPresentationCodeBlock {
    pub content: String,
    pub lang: SupportedCodeBlockSyntax,
    /// An optional 'theme' property that can be passed into a specific code
    /// block.
    pub theme: Option<SupportedCodeBlockTheme>,
    /// Indicates whether the codeblock is an inline codeblock or not.
    pub inline: bool,
    pub assets: Arc<Mutex<HighlightingAssets>>,
}

impl GeneralPresentationCodeBlock {
    pub fn new_with_new_assets(content: String,
                               lang: SupportedCodeBlockSyntax,
                               theme: Option<SupportedCodeBlockTheme>,
                               inline: bool)
                               -> Self {
        let assets = Arc::new(Mutex::new(HighlightingAssets::from_binary()));
        GeneralPresentationCodeBlock { content,
                                       lang,
                                       theme,
                                       inline,
                                       assets }
    }
}
