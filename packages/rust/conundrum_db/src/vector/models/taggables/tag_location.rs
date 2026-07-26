use serde::{Deserialize, Serialize};

use conundrum::ecosystem::db::traits::database_field_representable::DatabaseFieldRepresentable;

#[derive(strum_macros::Display, Serialize, Deserialize, Clone, Debug)]
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
}

impl DatabaseFieldRepresentable<String> for TagLocation {
    fn to_db_representation(&self) -> String {
        self.to_string()
    }
}
