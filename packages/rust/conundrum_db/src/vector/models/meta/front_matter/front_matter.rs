use std::sync::Arc;

use conundrum::{
    ecosystem::db::{
        db_traits::{
            db_entity::{DBEntity, DBSchema},
            db_field::DatabaseField,
        },
        tables::DatabaseTable,
    },
    impl_default_crud,
    lifted_models::primitives::db_id::DatabaseId,
    output::parsing_result::front_matter::FrontMatterResult,
};
use fake::Dummy;
use serde::{Deserialize, Serialize};

use crate::vector::models::meta::front_matter::front_matter_source_type::FrontMatterSourceType;

/// Still unsure about implementing front matter as a separate model like I did
/// previously. I want to, but I'm sooooooo over writing these models...
#[derive(Serialize, Deserialize, Clone, Debug, specta::Type, Dummy)]
pub struct FrontMatter {
    pub id: DatabaseId,
    pub note_id: DatabaseId,
    pub source_type: FrontMatterSourceType,
    pub data: FrontMatterResult,
}

impl<'a> DBEntity<'a, DatabaseId> for FrontMatter {
    type PartialUpdateType = FrontMatter;

    fn table() -> conundrum::ecosystem::db::tables::DatabaseTable {
        DatabaseTable::FrontMatter
    }

    fn merge_keys() -> &'static [&'static str] {
        &["id"]
    }

    fn primary_key() -> &'static str {
        "id"
    }

    fn primary_value(&self) -> DatabaseId {
        self.id.clone()
    }

    fn set_primary_value(&mut self, value: DatabaseId) {
        self.id = value.clone();
    }
}

impl<'a> DBSchema<'a> for FrontMatter {
    fn arrow_fields(
        )
        -> conundrum::ecosystem::error_handling::db_error::DatabaseResult<Vec<std::sync::Arc<arrow_schema::Field>>>
    {
        Ok(vec![Arc::new(DatabaseId::field_definition("id", false)),
                Arc::new(DatabaseId::field_definition("note_id", false)),
                Arc::new(FrontMatterSourceType::field_definition("source_type", false)),
                Arc::new(FrontMatterResult::field_definition("data", false))])
    }
}

impl_default_crud!(FrontMatter, FrontMatter, DatabaseId);
