use std::sync::Arc;

use conundrum::{
    ecosystem::db::{
        db_traits::{
            db_entity::{DBEntity, DBSchema},
            db_field::DatabaseField,
            into_partial::IntoPartial,
        },
        tables::DatabaseTable,
    },
    impl_default_crud,
    lifted_models::primitives::db_id::DatabaseId,
    output::parsing_result::front_matter::FrontMatterResult,
};
use conundrum_macros::DatabaseEntity;
use fake::Dummy;
use serde::{Deserialize, Serialize};

use crate::vector::models::meta::front_matter::front_matter_source_type::FrontMatterSourceType;

/// Still unsure about implementing front matter as a separate model like I did
/// previously. I want to, but I'm sooooooo over writing these models...
#[derive(Serialize, Deserialize, Clone, Debug, specta::Type, Dummy, DatabaseEntity)]
#[db(table = DatabaseTable::FrontMatter)]
pub struct FrontMatter {
    pub id: DatabaseId,
    pub note_id: DatabaseId,
    pub source_type: FrontMatterSourceType,
    pub data: FrontMatterResult,
}

impl IntoPartial<FrontMatterPartial> for FrontMatter {
    fn into_partial(&self) -> FrontMatterPartial {
        FrontMatterPartial { id: self.id.clone(),
                             note_id: Some(self.note_id.clone()),
                             source_type: Some(self.source_type.clone()),
                             data: Some(self.data.clone()) }
    }
}
