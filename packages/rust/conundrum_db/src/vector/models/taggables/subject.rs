use std::sync::Arc;

use arrow_array::{RecordBatch, StringArray};
use serde::{Deserialize, Serialize};

use crate::vector::{
    database::db_types::db_entity::DBEntity,
    models::{
        date_time::date_time::DateTime, primitives::case_insensitive_string::CaseInsensitiveString,
        taggables::tag_location::TagLocation,
    },
};

use conundrum::ecosystem::{
    db::traits::database_field_representable::DatabaseFieldRepresentable,
    error_handling::db_error::{DatabaseError, DatabaseResult},
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Subject {
    pub value: CaseInsensitiveString,
    pub location: TagLocation,
    pub ctime: DateTime,
}

// impl DBEntity<String> for Subject {
//     fn primary_key_value(&self) -> String {
//         self.value.to_string()
//     }

//     fn to_arrow_schema() -> Arc<Schema> {
//         Arc::new(Schema::new(vec![Field::new("value", DataType::Utf8, false),
//                                   Field::new("location", DataType::Utf8,
// false),                                   Field::new("ctime", DataType::Utf8,
// false),]))     }

//     fn to_record_batch(items: Vec<Self>) ->
// DatabaseResult<arrow_array::RecordBatch> {         let mut body = Vec::new();
//         let mut location = Vec::new();
//         let mut ctime = Vec::new();
//         for item in items {
//             body.push(item.value.to_db_representation());
//             location.push(item.location.to_db_representation());
//             ctime.push(item.ctime.to_db_representation());
//         }

//         RecordBatch::try_new(Self::to_arrow_schema(),
//                              vec![Arc::new(StringArray::from(body)),
//                                   Arc::new(StringArray::from(location)),
//
// Arc::new(StringArray::from(ctime)),]).map_err(|e| {
// DatabaseError::FailToSerialize(e.to_string())
// })     }
// }
