/// ## AI-Markdown Behavior
///
/// Allows users to customize the behavior of how each component is passed to AI
/// when the target indicates that the output will be consumbed by AI. By
/// default, this means the output will be parsed to return only markdown, not
/// the embeded components, and each component will compile away to either:
///
/// - A descriptive sentence describing the component if content can't be
///   generated for markdown
/// targets like in the case of a video or color component.
/// - A user provided descriptive text. There's a templating syntax planned for
///   the future, but the to-do list is _long_.
/// - Or just hide the component
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub enum AIMDBehavior {
    Hide,
    Describe(Option<String>),
}
