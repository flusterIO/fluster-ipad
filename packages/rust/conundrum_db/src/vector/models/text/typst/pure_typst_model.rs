use serde::{Deserialize, Serialize};
use surrealdb_types::SurrealValue;

use crate::vector::{
    database::{
        db_traits::{database_field::DatabaseField, pure_model_static::PureModelStaticMethods},
        primitive_field_schema_generators::string_field_def_generator::{
            optional_string_field_definition, string_field_definition,
        },
    },
    models::{
        ai::ai_generated_status::AIGeneratedStatus, date_time::date_time::DateTime, primitives::db_id::DatabaseId,
    },
};

#[derive(Serialize, Deserialize, Clone, Debug, SurrealValue)]
pub struct PureTypstContent {
    pub id: DatabaseId,
    pub title: Option<String>,
    pub content: String,
    pub ai_generated: AIGeneratedStatus,
    pub fs_path: Option<String>,
    pub ctime: DateTime,
    pub utime: DateTime,
}

impl PureModelStaticMethods for PureTypstContent {
    fn table() -> conundrum::ecosystem::db::tables::DatabaseTable {
        conundrum::ecosystem::db::tables::DatabaseTable::TypstContent
    }

    fn schema() -> String {
        let tbl = Self::table();
        indoc::formatdoc! {"
        {};
        {};
        {};
        {};
        {};
        {};
        {};
            ", DatabaseId::field_definition("id", &tbl), 
                optional_string_field_definition("title", &tbl),
                string_field_definition("content", &tbl),
                AIGeneratedStatus::field_definition("ai_generated", &tbl),
                optional_string_field_definition("fs_path", &tbl),
                DateTime::field_definition("ctime", &tbl),
                DateTime::field_definition("utime", &tbl),

        }
    }
}
