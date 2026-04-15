/// A codeblock that does not render or do anything special, just plain ole'
/// markdown.
pub struct GeneralPresentationCodeBlock {
    pub content: String,
    pub lang: String,
    /// An optional 'theme' property that can be passed into a specific code
    /// block.
    pub theme: Option<String>,
    /// Indicates whether the codeblock is an inline codeblock or not.
    pub inline: bool,
}
