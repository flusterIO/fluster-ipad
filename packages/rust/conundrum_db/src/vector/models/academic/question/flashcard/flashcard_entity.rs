use std::sync::Arc;

use lancedb::arrow::arrow_schema::Schema;
use serde::{Deserialize, Serialize};

use crate::vector::{
    database::db_traits::{db_entity::DBEntity, db_field::DatabaseField},
    models::{
        academic::question::flashcard::flashcard_value::FlashcardValue, date_time::date_time::DateTime,
        primitives::db_id::DatabaseId,
    },
};

pub fn default_empty() -> u32 {
    0
}

#[derive(Clone, Serialize, Deserialize)]
pub struct FlashCardEntity {
    #[serde(default = "DatabaseId::default")]
    pub id: DatabaseId,
    pub question: String,
    pub answer: FlashcardValue,
    pub explanation: Option<String>,
    #[serde(default = "default_empty")]
    pub correct_responses: u32,
    #[serde(default = "default_empty")]
    pub incorrect_responses: u32,
    /// The difficulty field is not optional for AI. AI should always provide an
    /// estimated difficulty score using a scale where Ph.D. level physics
    /// and M.D. level biology is a 100, and elementary math like 2 + 2 is
    /// 0.
    pub difficulty: Option<f32>,
    #[serde(default = "DateTime::new_now")]
    pub ctime: DateTime,
    pub last_access: DateTime,
}

// impl DBEntity for CdrmContent {
//     fn primary_key_value(&self) ->
// crate::vector::models::primitives::db_id::DatabaseId {         self.id
//     }

//     fn to_arrow_schema() -> Arc<Schema> {
//         Arc::new(Schema::new(vec![Field::new("id", DataType::Utf8, false),
//                                   Field::new("title", DataType::Utf8, true),
//                                   Field::new("body", DataType::LargeUtf8,
// false),                                   Field::new("ai_generated",
// DataType::Boolean, false),
// Field::new("fs_path", DataType::Utf8, true),
// Field::new("ctime", DataType::Date64, false),
// Field::new("utime", DataType::Date64, false),]))     }

//     fn to_record_batch(items: Vec<Self>) ->
// DatabaseResult<arrow_array::RecordBatch> {         let mut id = Vec::new();
//         let mut title = Vec::new();
//         let mut body = Vec::new();
//         let mut ai_generated = Vec::new();
//         let mut fs_path = Vec::new();
//         let mut ctime = Vec::new();
//         let mut utime = Vec::new();
//         for item in items {
//             id.push(item.id.to_db_representation());
//             title.push(item.title);
//             body.push(item.body);
//             ai_generated.push(item.ai_generated);
//             fs_path.push(item.fs_path);
//             ctime.push(item.ctime.to_db_representation());
//             utime.push(item.utime.to_db_representation());
//         }

//         RecordBatch::try_new(Self::to_arrow_schema(),
//                              vec![Arc::new(StringArray::from(id)),
//                                   Arc::new(StringArray::from(title)),
//                                   Arc::new(StringArray::from(body)),
//                                   Arc::new(StringArray::from(ai_generated)),
//                                   Arc::new(StringArray::from(fs_path)),
//                                   Arc::new(StringArray::from(ctime)),

// Arc::new(StringArray::from(utime)),]).map_err(|e| {
// DatabaseError::FailToSerialize(e.to_string())
// })     }
// }

// impl DBEntity for FlashCardEntity {
//     fn arrow_schema() -> Arc<lancedb::arrow::arrow_schema::Schema> {
//         Arc::new(Schema::new(vec![DatabaseId::field_definition("id",
// false)]))     }

//     fn table() -> conundrum::ecosystem::db::tables::DatabaseTable {
//         todo!()
//     }
// }
