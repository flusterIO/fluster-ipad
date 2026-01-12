use regex::Regex;
use serde::{Deserialize, Serialize};
use specta::Type;

use crate::{
    core_types::fluster_error::{FlusterError, FlusterResult},
    tauri::features::database::database_types::FlusterDb,
};

#[derive(strum_macros::Display, Type, Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum TaggableTypeEnum {
    Tag,
    Topic,
    Subject,
}

// TODO: This whole file is likely deletable. Clean this up when you have time.
#[derive(Clone, Type, Deserialize, Serialize, Debug)]
pub struct Taggable {
    pub id: Option<String>,
    pub value: String,
    pub tag_type: TaggableTypeEnum,
}

#[derive(Clone, Type, Deserialize, Serialize, Debug)]
pub struct TagEntity {
    pub id: String,
    pub value: String,
    pub tag_type: TaggableTypeEnum,
}

pub fn get_tag_regular_expression() -> Regex {
    Regex::new(r"\[\[#(?<body>[^\]]+)\]\]").unwrap()
}

impl Taggable {
    pub async fn save(&self, db: &mut FlusterDb<'_>) -> FlusterResult<TagEntity> {
        Err(FlusterError::NotImplemented)
    }

    fn get_table_name(&self) -> &'static str {
        match self.tag_type {
            TaggableTypeEnum::Topic => "topic",
            TaggableTypeEnum::Subject => "subject",
            TaggableTypeEnum::Tag => "tag",
        }
    }
    fn get_sql_template_string(&self) -> &'static str {
        match self.tag_type {
            TaggableTypeEnum::Topic => "UPSERT topic SET val $tag_value RETURN id;",
            TaggableTypeEnum::Subject => "UPSERT subject SET val $tag_value RETURN id;",
            TaggableTypeEnum::Tag => "UPSERT tag SET val $tag_value RETURN id;",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[tokio::test]
    // async fn saves_tag_successfully() {
    //     let db = fluster_test_utils::test_utils::get_memory_database().await;
    //     let t = Taggable {
    //         id: None,
    //         value: "Test tag".to_string(),
    //         tag_type: TaggableTypeEnum::Tag,
    //     };
    //     let res = t.save(&db).await;
    //     assert!(res.is_ok(), "Saves tag without throwing an error.");
    //     // assert_eq!(result, 4);
    // }
}
