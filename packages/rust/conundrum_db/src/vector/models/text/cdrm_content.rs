use std::sync::Arc;

use arrow_array::{RecordBatch, StringArray};
use serde::{Deserialize, Serialize};

use crate::vector::{
    database::db_types::db_entity::DBEntity,
    models::{
        ai::ai_generated_status::AIGeneratedStatus,
        date_time::date_time::DateTime,
        primitives::db_id::DatabaseId,
        taggables::{subject::Subject, tag::Tag, taggables::Taggables, topic::Topic},
    },
};

use conundrum::ecosystem::{
    db::traits::database_field_representable::DatabaseFieldRepresentable,
    error_handling::db_error::{DatabaseError, DatabaseResult},
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CdrmContent {
    pub id: DatabaseId,
    pub title: Option<String>,
    pub body: String,
    pub ai_generated: AIGeneratedStatus,
    pub tags: Vec<Tag>,
    pub topic: Option<Topic>,
    pub subject: Option<Subject>,
    pub fs_path: Option<String>,
    pub ctime: DateTime,
    pub utime: DateTime,
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
//
// Arc::new(StringArray::from(utime)),]).map_err(|e| {
// DatabaseError::FailToSerialize(e.to_string())
// })     }
// }
