use fake::Dummy;
use serde::{Deserialize, Serialize};

use specta::Type;

#[derive(strum_macros::Display, Serialize, Deserialize, Clone, Debug, Dummy, Type)]
pub enum TagLocation {
    #[strum(to_string = "front_matter")]
    FrontMatter,
    #[strum(to_string = "body")]
    Body,
    /// For apps using Conundrum content, this might come from a panel, a modal
    /// or what-not, but not from the note itself. If it comes from the note or
    /// front-matter, use those fields so they can be removed strategically.
    #[strum(to_string = "app_inserted")]
    AppInserted,
    #[strum(to_string = "auto_taggable")]
    AutoTaggable,
    #[strum(to_string = "straggling")]
    /// Straggling when a tag is inserted through the REST api or another means
    /// where it is user-defined, but not necessarily associated with a
    /// note. These will never be automatically cleaned up as part of the
    /// syncing process.
    Straggling,
}
