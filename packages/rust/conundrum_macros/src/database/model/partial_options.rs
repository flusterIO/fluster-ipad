#[derive(Clone, Debug)]
pub struct PartialOptions {
    /// Skip being added to the partial entirely.
    pub skip: bool,
    /// Skip being added to the arrow fields only.
    pub skip_arrow: bool,
    pub required: bool,
    pub patch: bool,
    /// Optionally applied to serde default.
    pub default: Option<String>,
}

#[allow(clippy::derivable_impls)]
impl Default for PartialOptions {
    fn default() -> Self {
        Self { skip: false,
               skip_arrow: false,
               required: false,
               default: None,
               patch: false }
    }
}
