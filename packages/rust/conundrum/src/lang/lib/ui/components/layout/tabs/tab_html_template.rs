use crate::parsers::conundrum::logic::bool::boolean::ConundrumBoolean;

pub struct TabButtonHtmlTemplate {
    /// The _rendered_ children of the button, not the tab itself.
    pub btn_children: String,
    pub tab_group_id: String,
    pub idx: usize,
    /// Set to true to make this the initial tab.
    pub initial: ConundrumBoolean,
    /// The rendered children of the tab itself, not the tab button.
    pub tab_children: String,
    pub is_longest: bool,
    pub is_last: bool,
}
